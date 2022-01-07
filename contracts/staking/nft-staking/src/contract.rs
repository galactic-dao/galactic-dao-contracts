use cosmwasm_std::{
    attr, entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env,
    MessageInfo, Order, Pair, Reply, ReplyOn, Response, StdError, StdResult, SubMsg, Uint128,
    Uint64, WasmMsg,
};
use cw20::Cw20ReceiveMsg;
use cw721::Cw721ReceiveMsg;
use cw_storage_plus::{Bound, PrimaryKey, U64Key};
use galacticdao_nft_staking_protocol::staking::{
    MigrateMsg, StakedNft, StakingConfig, StakingExecuteMsg, StakingInstantiateMsg,
    StakingQueryMsg, TokenBalance, TokenDistribution,
};
use std::borrow::{Borrow, BorrowMut};
use std::convert::TryInto;

use crate::error::ContractError;
use crate::execute::{
    execute_change_config, execute_receive_nft, execute_receive_token, execute_withdraw_nft,
    execute_withdraw_rewards, execute_withdraw_tokens,
};
use crate::query::{
    query_all_staked, query_config, query_distributions, query_num_staked, query_stake_by_addr,
    query_stake_by_token,
};
use crate::state::{staked_nfts, token_distribution_key, token_distributions, CONFIG, NUM_STAKED};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: StakingInstantiateMsg,
) -> Result<Response, ContractError> {
    let cfg = StakingConfig {
        owner: info.sender.to_string(),
        nft_contract: msg.nft_contract.clone(),
        whitelisted_tokens: msg.whitelisted_tokens.clone(),
        trusted_token_sender: msg.trusted_token_sender.clone(),
        reward_withdrawal_timeout: msg.reward_withdrawal_timeout,
    };
    CONFIG.save(deps.storage, &cfg)?;
    NUM_STAKED.save(deps.storage, &0)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: StakingExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        StakingExecuteMsg::ChangeConfig {
            whitelisted_tokens,
            trusted_token_sender,
            reward_withdrawal_timeout,
        } => execute_change_config(
            deps,
            env,
            info,
            whitelisted_tokens,
            trusted_token_sender,
            reward_withdrawal_timeout,
        ),
        StakingExecuteMsg::ReceiveNft(receive_msg) => {
            execute_receive_nft(deps, env, info, &receive_msg)
        }
        StakingExecuteMsg::Receive(receive_msg) => {
            execute_receive_token(deps, env, info, &receive_msg)
        }
        StakingExecuteMsg::WithdrawRewards { token_id } => {
            execute_withdraw_rewards(deps, env, info, &token_id)
        }
        StakingExecuteMsg::WithdrawNft { token_id } => {
            execute_withdraw_nft(deps, env, info, &token_id)
        }
        StakingExecuteMsg::WithdrawTokens { balance } => {
            execute_withdraw_tokens(deps, env, info, &balance)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: StakingQueryMsg) -> StdResult<Binary> {
    match msg {
        StakingQueryMsg::Config {} => query_config(deps, env),
        StakingQueryMsg::StakedByAddr { address } => query_stake_by_addr(deps, env, &address),
        StakingQueryMsg::StakedByToken { token_id } => query_stake_by_token(deps, env, &token_id),
        StakingQueryMsg::AllStaked {
            start_after_token,
            limit,
        } => query_all_staked(deps, env, &start_after_token, &limit),
        StakingQueryMsg::NumStaked {} => query_num_staked(deps, env),
        StakingQueryMsg::Distributions {
            after_time,
            limit,
            token_addr,
        } => query_distributions(deps, env, &token_addr, &after_time, &limit),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
