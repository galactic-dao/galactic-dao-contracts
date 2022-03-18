use std::collections::HashMap;

use cosmwasm_std::{to_binary, CosmosMsg, Deps, StdResult, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;

use galacticdao_nft_staking_protocol::staking::{StakedNft, StakedNftState, TokenBalance};

/// Maps an array of token balances into a hashmap of token_id -> balance
pub fn balance_vec_to_map(balances: &Vec<TokenBalance>) -> HashMap<String, Uint128> {
    balances.iter().fold(
        HashMap::<String, Uint128>::new(),
        |mut balances, balance| {
            balances
                .entry(balance.token.clone())
                .and_modify(|bal| *bal += balance.amount)
                .or_insert(balance.amount);
            balances
        },
    )
}

/// Takes a balance hashmap of token_id -> amount and creates a vec of token balances
pub fn balance_map_to_vec(rewards_map: &HashMap<String, Uint128>) -> Vec<TokenBalance> {
    rewards_map
        .iter()
        .map(|(cw20_token, amount)| TokenBalance {
            amount: amount.clone(),
            token: cw20_token.clone(),
        })
        .collect()
}

/// Computes balance diffs between `old` and `new`
pub fn balance_map_diff(
    new: &HashMap<String, Uint128>,
    old: &HashMap<String, Uint128>,
) -> HashMap<String, Uint128> {
    new.iter().fold(
        HashMap::<String, Uint128>::new(),
        |mut diff, (token, amount)| {
            let old_amount = *old.get(token).unwrap_or(&Uint128::zero());
            diff.insert(token.clone(), *amount - old_amount);
            diff
        },
    )
}

/// Takes a reward hashmap and creates the CosmosMsgs for sending the tokens
pub fn msgs_from_rewards(
    rewards_map: &HashMap<String, Uint128>,
    recipient: &String,
) -> StdResult<Vec<CosmosMsg>> {
    let msgs = rewards_map
        .iter()
        .filter_map(|(cw20_token, amount)| {
            if amount.is_zero() {
                return None;
            }

            let transfer_msg = Cw20ExecuteMsg::Transfer {
                recipient: recipient.clone(),
                amount: amount.clone(),
            };

            return Some(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: cw20_token.clone(),
                msg: to_binary(&transfer_msg).unwrap(),
                funds: vec![],
            }));
        })
        .collect();

    Ok(msgs)
}

/// Util fn to get StakedNftState from a staked NFT
pub fn query_stake_state(
    _deps: Deps,
    stake: &StakedNft,
    total_rewards: &Vec<TokenBalance>,
) -> StdResult<StakedNftState> {
    let total_rewards_map = balance_vec_to_map(total_rewards);

    let unclaimed_map = balance_map_diff(
        &total_rewards_map,
        &balance_vec_to_map(&stake.last_reward_snapshot),
    );
    let total_map = balance_map_diff(
        &total_rewards_map,
        &balance_vec_to_map(&stake.beginning_reward_snapshot),
    );

    Ok(StakedNftState {
        stake: stake.clone(),
        unclaimed_rewards: balance_map_to_vec(&unclaimed_map),
        total_rewards: balance_map_to_vec(&total_map),
    })
}

/// Validates token addrs
pub fn validate_tokens(deps: Deps, tokens: &Vec<String>) -> StdResult<()> {
    for token in tokens {
        match deps.api.addr_validate(token.as_str()) {
            Err(err) => {
                return Err(err);
            }
            _ => {}
        }
    }

    Ok(())
}
