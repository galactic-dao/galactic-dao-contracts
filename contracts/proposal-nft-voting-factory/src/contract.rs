#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply,
    ReplyOn, Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use protobuf::Message;

use galacticdao_nft_voting_protocol::cw721_querier::query_has_tokens;
use galacticdao_nft_voting_protocol::proposal::{
    ProposalConfig, ProposalInstantiateMsg, ProposalOption,
};
use galacticdao_nft_voting_protocol::proposal_factory::{
    ProposalFactoryExecuteMsg, ProposalFactoryInstantiateMsg, ProposalFactoryQueryMsg,
    ProposalFactoryState, ProposalFactoryStatusResponse,
};

use crate::error::ContractError;
use crate::response::MsgInstantiateContractResponse;
use crate::state::{CONFIG, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:galacticdao-proposal-nft-voting-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ProposalFactoryInstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &msg.config)?;
    STATE.save(
        deps.storage,
        &ProposalFactoryState {
            owner: info.sender.to_string(),
        },
    )?;

    Ok(Response::new())
}

// TODO: Replies on creation, add attributes, queries
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let res: MsgInstantiateContractResponse =
        Message::parse_from_bytes(msg.result.unwrap().data.unwrap().as_slice()).map_err(|_| {
            StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
        })?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "create_proposal_success"),
        attr("proposal_address", res.get_contract_address().to_string()),
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
        ProposalFactoryExecuteMsg::CreateProposal {
            proposal_uri,
            options,
            close_time,
        } => execute_create_proposal(deps, env, info, &proposal_uri, &options, close_time),
        ProposalFactoryExecuteMsg::ModifyConfig {
            proposal_cost,
            proposal_code_id,
        } => execute_modify_config(deps, env, info, proposal_cost, proposal_code_id),
        ProposalFactoryExecuteMsg::WithdrawFunds { amount_uluna } => {
            execute_withdraw_funds(deps, env, info, &amount_uluna)
        }
    }
}

pub fn execute_create_proposal(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proposal_uri: &String,
    options: &Vec<ProposalOption>,
    close_time: u64,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;

    // Only allow people with an NFT to create a contract
    if !query_has_tokens(&deps.querier, &cfg.nft_contract, &info.sender.to_string())? {
        return Err(ContractError::Unauthorized {});
    }

    let instantiate_proposal_msg = ProposalInstantiateMsg {
        config: ProposalConfig {
            nft_contract: cfg.nft_contract.clone(),
            proposal_uri: proposal_uri.clone(),
            options: options.clone(),
            proposer: info.sender.to_string(),
            close_time,
        },
    };

    Ok(Response::new()
        .add_submessage(SubMsg {
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
        })
        .add_attribute("action", "create_proposal_attempt"))
}

pub fn execute_modify_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proposal_cost: Option<u64>,
    proposal_code_id: Option<u64>,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let mut cfg = CONFIG.load(deps.storage)?;

    if state.owner.as_str() != info.sender.as_str() {
        return Err(ContractError::Unauthorized {});
    }

    cfg.proposal_cost = proposal_cost.unwrap_or(cfg.proposal_cost);
    cfg.proposal_code_id = proposal_code_id.unwrap_or(cfg.proposal_code_id);
    CONFIG.save(deps.storage, &cfg)?;

    return Ok(Response::new().add_attribute("action", "modify_config"));
}

pub fn execute_withdraw_funds(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount_uluna: &Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    Ok(Response::new()
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: state.owner.clone(),
            amount: vec![Coin::new(amount_uluna.u128(), "uluna")],
        }))
        .add_attribute("action", "withdraw_funds"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: ProposalFactoryQueryMsg) -> StdResult<Binary> {
    match msg {
        ProposalFactoryQueryMsg::Status {} => to_binary(&query_status(deps)?),
    }
}

pub fn query_status(deps: Deps) -> StdResult<ProposalFactoryStatusResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    Ok(ProposalFactoryStatusResponse {
        owner: state.owner.clone(),
        config: cfg.clone(),
    })
}
