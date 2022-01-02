use cosmwasm_std::{
    attr, entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Reply, ReplyOn, Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};
use galacticdao_nft_staking_protocol::staking::{MigrateMsg, StakingInstantiateMsg, StakingQueryMsg};

use crate::error::ContractError;
use crate::state::{CONFIG};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: StakingInstantiateMsg,
) -> Result<Response, ContractError> {
    CONFIG.save(deps.storage, &msg.config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: StakingInstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
    // match msg {
    //     ProposalFactoryExecuteMsg::CreateProposal {
    //         proposal_uri,
    //         options,
    //         close_time,
    //     } => execute_create_proposal(deps, env, info, &proposal_uri, &options, close_time),
    //     ProposalFactoryExecuteMsg::ModifyConfig {
    //         proposal_cost,
    //         proposal_code_id,
    //     } => execute_modify_config(deps, env, info, proposal_cost, proposal_code_id),
    //     ProposalFactoryExecuteMsg::WithdrawFunds { amount_uluna } => {
    //         execute_withdraw_funds(deps, env, info, &amount_uluna)
    //     }
    // }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: StakingQueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err(""))
    // match msg {
    //     ProposalFactoryQueryMsg::Status {} => to_binary(&query_status(deps)?),
    // }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
