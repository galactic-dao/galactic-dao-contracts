#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw_storage_plus::U16Key;

use galacticdao_nft_voting_protocol::cw721_querier::query_token_owner;
use galacticdao_nft_voting_protocol::proposal::{
    ProposalExecuteMsg, ProposalInstantiateMsg, ProposalOptionStatus, ProposalQueryMsg,
    ProposalState, ProposalStatusResponse, VotesQueryResponse,
};

use crate::error::ContractError;
use crate::state::{CONFIG, STATE, TALLY, VOTE_BY_TOKEN_ID};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:galacticdao-proposal-nft-voting";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ProposalInstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &msg.config)?;
    STATE.save(
        deps.storage,
        &ProposalState {
            owner: info.sender.to_string(),
            is_revoked: false,
        },
    )?;

    // Initialize tally to 0's for given options
    for option in msg.config.options.iter() {
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
    let token_owner = query_token_owner(&deps.querier, cfg.nft_contract, token_id.to_string())?;
    if token_owner.as_str() != info.sender.as_str() {
        return Err(ContractError::Unauthorized {});
    }

    // Rollback existing vote
    let existing_vote = VOTE_BY_TOKEN_ID.may_load(deps.storage, token_id.to_string())?;
    if let Some(voted_option_id) = existing_vote {
        TALLY.update(
            deps.storage,
            U16Key::from(voted_option_id),
            |curr_tally| -> StdResult<_> { Ok(curr_tally.unwrap() + 1) },
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

    Ok(Response::new())
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
        .map(|id| VOTE_BY_TOKEN_ID.may_load(deps.storage, id.clone()).unwrap())
        .collect();
    Ok(VotesQueryResponse { votes })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use galacticdao_nft_voting_protocol::proposal::{
        ProposalConfig, ProposalInstantiateMsg, ProposalOption, ProposalOptionStatus,
        ProposalState, ProposalStatusResponse,
    };

    use crate::contract::{instantiate, query_status};

    #[test]
    fn instantiate_and_query_status() {
        let mut deps = mock_dependencies();

        let nft_contract = "contract";
        let proposal_uri = "proposal_uri";
        let owner = "owner";
        let options = vec![
            ProposalOption {
                id: 0,
                name: "0".to_string(),
            },
            ProposalOption {
                id: 0,
                name: "1".to_string(),
            },
        ];

        // Instantiate contract
        let instantiate_msg = ProposalInstantiateMsg {
            config: ProposalConfig {
                nft_contract: nft_contract.to_string(),
                proposal_uri: proposal_uri.to_string(),
                options: options.clone(),
                close_time: 100,
            },
        };
        let info = mock_info(&owner, &[]);
        instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

        // Ensure expected initial state
        let expected = ProposalStatusResponse {
            state: ProposalState {
                owner: owner.to_string(),
                is_revoked: false,
            },
            config: ProposalConfig {
                nft_contract: nft_contract.to_string(),
                proposal_uri: proposal_uri.to_string(),
                options: options.clone(),
                close_time: 100,
            },
            tally: options
                .iter()
                .map(|option| ProposalOptionStatus {
                    id: option.id,
                    votes: 0,
                })
                .collect(),
        };
        assert_eq!(query_status(deps.as_ref()).unwrap(), expected);
    }

    // TODO: More testing
}
