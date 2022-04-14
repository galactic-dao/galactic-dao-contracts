use cosmwasm_std::{
    attr, entry_point, to_binary, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo,
    Reply, ReplyOn, Response, StdError, StdResult, SubMsg, WasmMsg,
};
use protobuf::Message;

use galacticdao_nft_voting_protocol::asset::Asset;
use galacticdao_nft_voting_protocol::nft_querier::query_has_tokens;
use galacticdao_nft_voting_protocol::proposal::{
    ProposalConfig, ProposalInstantiateMsg, ProposalOption,
};
use galacticdao_nft_voting_protocol::proposal_factory::{
    MigrateMsg, ProposalFactoryExecuteMsg, ProposalFactoryInstantiateMsg, ProposalFactoryQueryMsg,
    ProposalFactoryState, ProposalFactoryStatusResponse,
};

use crate::error::ContractError;
use crate::response::MsgInstantiateContractResponse;
use crate::state::{CONFIG, PROPOSALS, STATE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ProposalFactoryInstantiateMsg,
) -> Result<Response, ContractError> {
    CONFIG.save(deps.storage, &msg.config)?;
    STATE.save(
        deps.storage,
        &ProposalFactoryState {
            owner: info.sender,
            num_created_proposals: 0,
        },
    )?;
    PROPOSALS.save(deps.storage, &vec![])?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let res: MsgInstantiateContractResponse =
        Message::parse_from_bytes(msg.result.unwrap().data.unwrap().as_slice()).map_err(|_| {
            StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
        })?;

    let proposal_addr = res.get_contract_address();
    STATE.update(deps.storage, |mut state| -> StdResult<_> {
        state.num_created_proposals += 1;
        Ok(state)
    })?;
    PROPOSALS.update(deps.storage, |mut proposals| -> StdResult<_> {
        proposals.push(proposal_addr.to_string());
        Ok(proposals)
    })?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "create_proposal_success"),
        attr("proposal_address", proposal_addr),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ProposalFactoryExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ProposalFactoryExecuteMsg::CreateProposal { config } => {
            execute_create_proposal(deps, env, info, &config)
        }
        ProposalFactoryExecuteMsg::ModifyConfig {
            proposal_cost,
            proposal_code_id,
            owner,
            min_quorum_fraction,
        } => execute_modify_config(
            deps,
            env,
            info,
            proposal_cost,
            proposal_code_id,
            owner,
            min_quorum_fraction,
        ),
        ProposalFactoryExecuteMsg::WithdrawFunds { asset } => {
            execute_withdraw_funds(deps, env, info, &asset)
        }
    }
}

pub fn execute_create_proposal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    config: &ProposalConfig,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let cfg = CONFIG.load(deps.storage)?;

    let mut proposal_cfg = config.clone();

    // Check proposal config
    if proposal_cfg.quorum_fraction < cfg.min_quorum_fraction {
        return Err(ContractError::Std(StdError::generic_err(
            "Minimum quorum is too low.",
        )));
    }

    // Add abstain option
    proposal_cfg.options.push(ProposalOption {
        // This assumes that we don't have an ID created with max u16
        // which is hacky, but why not
        id: 65535,
        name: "Abstain".to_string(),
    });

    let instantiate_proposal_msg = ProposalInstantiateMsg {
        config: proposal_cfg,
        proposer: info.sender.to_string(),
    };
    let instantiate_submsg = SubMsg {
        id: 0u64,
        msg: WasmMsg::Instantiate {
            admin: None,
            code_id: cfg.proposal_code_id,
            msg: to_binary(&instantiate_proposal_msg)?,
            funds: vec![],
            label: "".to_string(),
        }
        .into(),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    // Conduct checks if not owner
    let mut payment_req_msgs: Vec<CosmosMsg> = vec![];
    if state.owner.as_str() != info.sender.as_str() {
        // Only allow people with an NFT to create a contract
        if !query_has_tokens(&deps.querier, &cfg.nft_contract, &info.sender.to_string())? {
            return Err(ContractError::Unauthorized {});
        }

        // Check that sufficient funds are sent
        if let Some(req_msg) = cfg
            .proposal_cost
            .check_sent_or_into_request_msg(&info, env.contract.address)?
        {
            payment_req_msgs.push(req_msg);
        }
    }

    Ok(Response::new()
        .add_submessage(instantiate_submsg)
        .add_messages(payment_req_msgs)
        .add_attribute("action", "create_proposal_attempt"))
}

pub fn execute_modify_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proposal_cost: Option<Asset>,
    proposal_code_id: Option<u64>,
    owner: Option<String>,
    min_quorum_fraction: Option<Decimal>,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let mut cfg = CONFIG.load(deps.storage)?;

    // Authorization
    if state.owner.as_str() != info.sender.as_str() {
        return Err(ContractError::Unauthorized {});
    }

    if let Some(min_quorum) = min_quorum_fraction {
        if min_quorum.lt(&Decimal::zero()) || min_quorum.gt(&Decimal::one()) {
            return Err(ContractError::Std(StdError::generic_err(
                "Invalid minimum quorum",
            )));
        }
        cfg.min_quorum_fraction = min_quorum;
    }
    cfg.proposal_cost = proposal_cost.unwrap_or(cfg.proposal_cost);
    cfg.proposal_code_id = proposal_code_id.unwrap_or(cfg.proposal_code_id);
    CONFIG.save(deps.storage, &cfg)?;

    state.owner = owner
        .map(|unverified_owner| deps.api.addr_validate(unverified_owner.as_str()).unwrap())
        .unwrap_or(state.owner);
    STATE.save(deps.storage, &state)?;

    return Ok(Response::new().add_attribute("action", "modify_config"));
}

pub fn execute_withdraw_funds(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    asset: &Asset,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    Ok(Response::new()
        .add_message(asset.clone().into_outgoing_msg(state.owner)?)
        .add_attribute("action", "withdraw_funds"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: ProposalFactoryQueryMsg) -> StdResult<Binary> {
    match msg {
        ProposalFactoryQueryMsg::Status {} => to_binary(&query_status(deps)?),
        ProposalFactoryQueryMsg::Proposals { start_idx, limit } => {
            to_binary(&query_proposals(deps, start_idx, limit)?)
        }
    }
}

pub fn query_status(deps: Deps) -> StdResult<ProposalFactoryStatusResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    Ok(ProposalFactoryStatusResponse {
        state: state.clone(),
        config: cfg.clone(),
    })
}

pub fn query_proposals(
    deps: Deps,
    start_idx: Option<u64>,
    limit: Option<u8>,
) -> StdResult<Vec<String>> {
    let proposals = PROPOSALS.load(deps.storage)?;

    Ok(proposals
        .iter()
        .skip(start_idx.unwrap_or(0) as usize)
        .take(limit.unwrap_or(100) as usize)
        .map(|proposal| proposal.to_string())
        .collect())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
