use crate::error::ContractError;
use crate::state::{staked_nfts, token_distributions};
use cosmwasm_std::{
    to_binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order, StdResult, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use cw_storage_plus::{Bound, PrimaryKey, U64Key};
use galacticdao_nft_staking_protocol::staking::{
    StakedNft, StakedNftState, TokenBalance, TokenDistribution,
};
use std::collections::HashMap;

/// Computes rewards in a hashmap, starting at a given time in seconds
/// cw20_token -> amount where amount > 0
pub fn reward_map(deps: Deps, start_time: u64) -> StdResult<HashMap<String, Uint128>> {
    let mut reward_map: HashMap<String, Uint128> = HashMap::new();

    let zero = Uint128::zero();
    token_distributions()
        .idx
        .time
        .range(
            deps.storage,
            Some(Bound::inclusive(
                (U64Key::new(start_time), vec![]).joined_key(),
            )),
            None,
            Order::Ascending,
        )
        .for_each(|item| {
            let distribution = item.unwrap().1;
            // Extra check just in case
            if distribution.time >= start_time && distribution.per_token_balance.amount.gt(&zero) {
                // Add to hashmap
                let reward_for_token = reward_map
                    .entry(distribution.per_token_balance.token)
                    .or_insert(Uint128::zero());
                *reward_for_token += distribution.per_token_balance.amount;
            }
        });

    Ok(reward_map)
}

/// Takes a reward hashmap and creates a vec of token balances
pub fn vec_from_rewards(rewards_map: &HashMap<String, Uint128>) -> Vec<TokenBalance> {
    rewards_map
        .iter()
        .map(|(cw20_token, amount)| TokenBalance {
            amount: amount.clone(),
            token: cw20_token.clone(),
        })
        .collect()
}

/// Takes a reward hashmap and creates the CosmosMsgs for sending the tokens
pub fn msgs_from_rewards(
    rewards_map: &HashMap<String, Uint128>,
    recipient: &String,
) -> StdResult<Vec<CosmosMsg>> {
    let msgs = rewards_map
        .iter()
        .map(|(cw20_token, amount)| {
            let transfer_msg = Cw20ExecuteMsg::Transfer {
                recipient: recipient.clone(),
                amount: amount.clone(),
            };

            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: cw20_token.clone(),
                msg: to_binary(&transfer_msg).unwrap(),
                funds: vec![],
            })
        })
        .collect();

    Ok(msgs)
}

/// Util fn to get StakedNftState from a staked NFT
pub fn query_stake_state(deps: Deps, stake: &StakedNft) -> StdResult<StakedNftState> {
    let rewards = vec_from_rewards(&reward_map(deps, stake.last_claim_time)?);

    Ok(StakedNftState {
        stake: stake.clone(),
        unclaimed_rewards: rewards,
    })
}
