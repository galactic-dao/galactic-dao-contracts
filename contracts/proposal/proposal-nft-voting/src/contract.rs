use cosmwasm_std::{
    attr, entry_point, to_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use cw_storage_plus::U16Key;

use galacticdao_nft_voting_protocol::nft_querier::{query_staked_token_owner, query_token_owner};
use galacticdao_nft_voting_protocol::proposal::{
    MigrateMsg, ProposalExecuteMsg, ProposalInstantiateMsg, ProposalOptionStatus, ProposalQueryMsg,
    ProposalState, ProposalStatusResponse, VotesQueryResponse,
};

use crate::error::ContractError;
use crate::state::{CONFIG, STATE, TALLY, VOTE_BY_TOKEN_ID};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ProposalInstantiateMsg,
) -> Result<Response, ContractError> {
    // Check config
    if msg.config.quorum_fraction.lt(&Decimal::zero())
        || msg.config.quorum_fraction.gt(&Decimal::one())
    {
        return Err(ContractError::InvalidConfig {});
    }

    if msg.config.options.is_empty() {
        return Err(ContractError::InvalidConfig {});
    }

    CONFIG.save(deps.storage, &msg.config)?;
    STATE.save(
        deps.storage,
        &ProposalState {
            creator: info.sender.to_string(),
            proposer: msg.proposer.to_string(),
            is_revoked: false,
        },
    )?;

    // Initialize tally to 0's for given options
    for option in msg.config.options.iter() {
        if TALLY.has(deps.storage, U16Key::from(option.id)) {
            return Err(ContractError::InvalidConfig {});
        }
        TALLY.save(deps.storage, U16Key::from(option.id), &0u16)?;
    }

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ProposalExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ProposalExecuteMsg::Vote {
            option_id,
            token_id,
        } => execute_vote(deps, env, info, option_id, &token_id),
        ProposalExecuteMsg::Revoke {} => execute_revoke(deps, env, info),
    }
}

pub fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    option_id: Option<u16>,
    token_id: &String,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    // Check whether voting is allowed
    if cfg.close_time < env.block.time.seconds() || state.is_revoked {
        return Err(ContractError::Closed {});
    }

    // Check owner
    let token_owner = query_token_owner(&deps.querier, &cfg.nft_contract, token_id)?;
    if token_owner == cfg.nft_staking_contract {
        let staked_token_owner =
            query_staked_token_owner(&deps.querier, &cfg.nft_contract, token_id)?;
        if staked_token_owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }
    } else if token_owner.as_str() != info.sender.as_str() {
        return Err(ContractError::Unauthorized {});
    }

    // Rollback existing vote
    let existing_vote = VOTE_BY_TOKEN_ID.may_load(deps.storage, token_id.to_string())?;
    if let Some(voted_option_id) = existing_vote {
        TALLY.update(
            deps.storage,
            U16Key::from(voted_option_id),
            |curr_tally| -> StdResult<_> { Ok(curr_tally.unwrap() - 1) },
        )?;
        VOTE_BY_TOKEN_ID.remove(deps.storage, token_id.to_string());
    }

    // Update to new vote
    if let Some(new_option_id) = option_id {
        TALLY.update(
            deps.storage,
            U16Key::from(new_option_id),
            |curr_tally| -> StdResult<_> { Ok(curr_tally.unwrap() + 1) },
        )?;
        VOTE_BY_TOKEN_ID.save(deps.storage, token_id.to_string(), &new_option_id)?;
    }

    Ok(Response::new().add_attributes(vec![
        attr("action", "proposal_vote"),
        attr("token_id", token_id),
        attr(
            "option_id",
            if option_id.is_some() {
                option_id.unwrap().to_string()
            } else {
                "".to_string()
            },
        ),
    ]))
}

pub fn execute_revoke(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let cfg = CONFIG.load(deps.storage)?;

    if info.sender.as_str() != state.proposer.as_str() {
        return Err(ContractError::Unauthorized {});
    }
    if cfg.close_time < env.block.time.seconds() {
        return Err(ContractError::Closed {});
    }

    state.is_revoked = true;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "revoke_proposal"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: ProposalQueryMsg) -> StdResult<Binary> {
    match msg {
        ProposalQueryMsg::Status {} => to_binary(&query_status(deps)?),
        ProposalQueryMsg::Votes { token_ids } => to_binary(&query_votes(deps, token_ids)?),
    }
}

pub fn query_status(deps: Deps) -> StdResult<ProposalStatusResponse> {
    let state = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;
    let tally = config
        .options
        .iter()
        .map(|option| ProposalOptionStatus {
            id: option.id,
            votes: TALLY.load(deps.storage, U16Key::from(option.id)).unwrap(),
        })
        .collect();

    Ok(ProposalStatusResponse {
        state,
        config,
        tally,
    })
}

pub fn query_votes(deps: Deps, token_ids: Vec<String>) -> StdResult<VotesQueryResponse> {
    let votes: Vec<Option<u16>> = token_ids
        .iter()
        .map(|id| {
            VOTE_BY_TOKEN_ID
                .may_load(deps.storage, id.to_string())
                .unwrap()
        })
        .collect();
    Ok(VotesQueryResponse { votes })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
