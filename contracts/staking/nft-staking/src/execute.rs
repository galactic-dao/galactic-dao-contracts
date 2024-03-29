use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
    WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw721::{Cw721ExecuteMsg, Cw721ReceiveMsg};

use galacticdao_nft_staking_protocol::staking::{StakedNft, StakingConfig, TokenBalance};

use crate::error::ContractError;
use crate::error::ContractError::Unauthorized;
use crate::state::{staked_nfts, CONFIG, NUM_STAKED, TOTAL_REWARDS};
use crate::util::{balance_map_diff, balance_vec_to_map, msgs_from_rewards, validate_tokens};

/// Change current configuration for the staking contract
pub fn execute_change_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    whitelisted_tokens: Option<Vec<String>>,
    trusted_token_sender: Option<String>,
    reward_withdrawal_timeout: Option<u64>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;

    if info.sender != cfg.owner {
        return Err(ContractError::Unauthorized {});
    }

    let whitelisted_tokens = whitelisted_tokens.map_or(cfg.whitelisted_tokens.clone(), |tokens| {
        validate_tokens(deps.as_ref(), &tokens).unwrap();
        tokens
    });

    CONFIG.save(
        deps.storage,
        &StakingConfig {
            whitelisted_tokens: whitelisted_tokens.clone(),
            trusted_token_sender: trusted_token_sender.unwrap_or(cfg.trusted_token_sender.clone()),
            reward_withdrawal_timeout: reward_withdrawal_timeout
                .unwrap_or(cfg.reward_withdrawal_timeout),
            ..cfg
        },
    )?;

    Ok(Response::new().add_attributes(vec![
        ("action", "change_config"),
        ("whitelisted_tokens", &whitelisted_tokens.join(",")),
        ("trusted_token_sender", &cfg.trusted_token_sender),
        (
            "reward_withdrawal_timeout",
            &cfg.reward_withdrawal_timeout.to_string(),
        ),
    ]))
}

/// Handle receiving an NFT (i.e. staking)
pub fn execute_receive_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receive_msg: &Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    let nft_addr = info.sender.to_string();
    let nft_owner = receive_msg.sender.clone();
    let token_id = receive_msg.token_id.clone();

    let cfg = CONFIG.load(deps.storage)?;
    let curr_rewards = TOTAL_REWARDS.load(deps.storage)?;
    if cfg.nft_contract != nft_addr {
        return Err(ContractError::InvalidNft {});
    }

    let stake_time = env.block.time.seconds();
    staked_nfts().save(
        deps.storage,
        token_id.clone().as_str(),
        &StakedNft {
            token_id: token_id.clone(),
            time_deposited: stake_time,
            can_withdraw_rewards_time: stake_time + cfg.reward_withdrawal_timeout,
            owner: nft_owner.clone(),
            beginning_reward_snapshot: curr_rewards.clone(),
            last_reward_snapshot: curr_rewards.clone(),
        },
    )?;
    NUM_STAKED.update(deps.storage, |num: u64| -> StdResult<u64> { Ok(num + 1) })?;

    Ok(Response::new().add_attributes(vec![
        ("action", "receive_nft"),
        ("token_id", &token_id),
        ("nft_owner", &nft_owner),
    ]))
}

/// Handle receiving a CW20 token
pub fn execute_receive_token(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    receive_msg: &Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;

    let token_addr = info.sender.to_string();
    let token_sender = receive_msg.sender.clone();
    let amount = receive_msg.amount.clone();

    if token_sender != cfg.trusted_token_sender {
        return Err(ContractError::Unauthorized {});
    }
    if !cfg.whitelisted_tokens.contains(&token_addr) {
        return Err(ContractError::InvalidToken {});
    }

    // Calculate amount to distribute
    let num_staked = NUM_STAKED.load(deps.storage)?;
    let amount_per_stake = amount
        .checked_div(Uint128::from(num_staked as u64))
        .unwrap();

    if amount_per_stake.is_zero() {
        return Err(ContractError::Std(StdError::generic_err(
            "Amount per stake is zero",
        )));
    }

    // Update total cumulative rewards
    let mut total_rewards = TOTAL_REWARDS.load(deps.storage)?;
    let curr_rewards_for_token = total_rewards
        .iter_mut()
        .find(|balance| balance.token == token_addr);
    match curr_rewards_for_token {
        Some(balance) => {
            balance.amount += amount_per_stake;
        }
        None => {
            total_rewards.push(TokenBalance {
                token: token_addr.clone(),
                amount: amount_per_stake,
            });
        }
    }
    TOTAL_REWARDS.save(deps.storage, &total_rewards)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "receive_token"),
        ("token_addr", &token_addr),
        ("token_sender", &token_sender),
        ("total_amount", &amount.to_string()),
        ("amount_per_stake", &amount_per_stake.to_string()),
        ("num_staked", &num_staked.to_string()),
    ]))
}

/// Withdraw rewards for the staked NFT
pub fn execute_withdraw_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: &String,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    let mut stake = staked_nfts().load(deps.storage, token_id)?;
    let current_time = env.block.time.seconds();

    // Check rewards time
    if stake.can_withdraw_rewards_time > current_time {
        return Err(ContractError::CannotWithdrawLockup {});
    }
    // Check sender
    let msg_sender = info.sender;
    if stake.owner != msg_sender && cfg.owner != msg_sender {
        return Err(Unauthorized {});
    }

    // Calculate claimable rewards
    let current_rewards = TOTAL_REWARDS.load(deps.storage)?;
    let claimable_rewards = balance_map_diff(
        &balance_vec_to_map(&current_rewards),
        &balance_vec_to_map(&stake.last_reward_snapshot),
    );

    // Update last reward snapshot
    stake.last_reward_snapshot = current_rewards.clone();
    staked_nfts().save(deps.storage, token_id, &stake)?;

    // Get rewards as send messages
    let send_reward_msgs: Vec<CosmosMsg> = msgs_from_rewards(&claimable_rewards, &stake.owner)?;

    Ok(Response::new()
        .add_messages(send_reward_msgs)
        .add_attributes(vec![
            ("action", "withdraw_rewards"),
            ("token_id", &token_id),
        ]))
}

/// Withdraw the NFT from staking, callable by either the stake owner OR the owner of staking contract
pub fn execute_withdraw_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    token_id: &String,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    let stake = staked_nfts().load(deps.storage, token_id)?;

    // Check sender
    if stake.owner != info.sender {
        return Err(Unauthorized {});
    }

    let mut withdraw_msgs: Vec<CosmosMsg> = vec![];

    // First transfer the NFT back to owner
    let transfer_nft_msg: Cw721ExecuteMsg = Cw721ExecuteMsg::TransferNft {
        recipient: stake.owner.clone(),
        token_id: token_id.to_string(),
    };
    withdraw_msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cfg.nft_contract.clone(),
        msg: to_binary(&transfer_nft_msg)?,
        funds: vec![],
    }));

    // Delete the stake
    staked_nfts().remove(deps.storage, token_id)?;
    NUM_STAKED.update(deps.storage, |num: u64| -> StdResult<u64> { Ok(num - 1) })?;

    // Transfer remaining rewards to owner of staking contract
    let total_rewards = TOTAL_REWARDS.load(deps.storage)?;
    let remaining_rewards = balance_map_diff(
        &balance_vec_to_map(&total_rewards),
        &balance_vec_to_map(&stake.last_reward_snapshot),
    );
    withdraw_msgs.extend(msgs_from_rewards(&remaining_rewards, &cfg.owner)?);

    Ok(Response::new()
        .add_messages(withdraw_msgs)
        .add_attributes(vec![("action", "withdraw_nft"), ("token_id", &token_id)]))
}

/// Withdraw ARBITRARY tokens from this contract. Callable only by the owner
/// This should be used SPARINGLY and very carefully, as it does not update past distributions
/// Ideally, this is only used to decommission the contract or in emergencies
pub fn execute_withdraw_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    balance: &TokenBalance,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if cfg.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::new()
        .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: balance.token.clone(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: cfg.owner.clone(),
                amount: balance.amount.clone(),
            })?,
            funds: vec![],
        }))
        .add_attributes(vec![("action", "owner_withdraw_tokens")]))
}
