use cosmwasm_std::{to_binary, Binary, Deps, Env, Order, StdResult};
use cw_storage_plus::Bound;

use galacticdao_nft_staking_protocol::staking::{StakedNft, StakedNftState};

use crate::state::{staked_nfts, CONFIG, NUM_STAKED, TOTAL_REWARDS};
use crate::util::query_stake_state;

fn query_limit(limit: &Option<u32>) -> usize {
    limit.unwrap_or(32).min(32) as usize
}

/// Return current config of the contract
pub fn query_config(deps: Deps, _env: Env) -> StdResult<Binary> {
    Ok(to_binary(&CONFIG.load(deps.storage)?)?)
}

/// Returns the StakedNftState associated with a token ID, or null if stake does not exist
pub fn query_stake_by_token(deps: Deps, _env: Env, token_id: &String) -> StdResult<Binary> {
    let total_rewards = TOTAL_REWARDS.load(deps.storage)?;
    let stake = staked_nfts().may_load(deps.storage, token_id.as_str())?;

    let response: Option<StakedNftState> = stake.map_or(
        None,
        |retrieved_stake: StakedNft| -> Option<StakedNftState> {
            Some(query_stake_state(deps, &retrieved_stake, &total_rewards).unwrap())
        },
    );

    Ok(to_binary(&response)?)
}

/// Returns all StakedNftStates associated with a staker address (Vec<StakedNftState>)
pub fn query_stake_by_addr(
    deps: Deps,
    _env: Env,
    addr: &String,
    start_after_token: &Option<String>,
    limit: &Option<u32>,
) -> StdResult<Binary> {
    let total_rewards = TOTAL_REWARDS.load(deps.storage)?;
    let min_bound: Option<Bound> = start_after_token
        .clone()
        .map(|token_id| Bound::Exclusive(token_id.as_bytes().to_vec()));

    let staked_states: Vec<StakedNftState> = staked_nfts()
        .idx
        .owner
        .prefix(addr.clone())
        .range(deps.storage, min_bound, None, Order::Ascending)
        .map(|stake_kv_pair| {
            let stake = stake_kv_pair.unwrap().1;
            query_stake_state(deps, &stake, &total_rewards).unwrap()
        })
        .take(query_limit(&limit))
        .collect();

    Ok(to_binary(&staked_states)?)
}

/// Returns a paginated response for all staked NFTs
pub fn query_all_staked(
    deps: Deps,
    _env: Env,
    start_after_token: &Option<String>,
    limit: &Option<u32>,
) -> StdResult<Binary> {
    let total_rewards = TOTAL_REWARDS.load(deps.storage)?;

    let min_bound: Option<Bound> = start_after_token
        .clone()
        .map(|token_id| Bound::Exclusive(token_id.as_bytes().to_vec()));
    let staked: Vec<StakedNftState> = staked_nfts()
        .range(deps.storage, min_bound, None, Order::Ascending)
        .take(query_limit(limit))
        .map(|item| {
            let stake = item.unwrap().1;
            query_stake_state(deps, &stake, &total_rewards).unwrap()
        })
        .collect();

    return to_binary(&staked);
}

/// Returns number of staked tokens
pub fn query_num_staked(deps: Deps, _env: Env) -> StdResult<Binary> {
    to_binary(&NUM_STAKED.load(deps.storage)?)
}

/// Returns all rewards under staking contract
pub fn query_total_rewards(deps: Deps, _env: Env) -> StdResult<Binary> {
    Ok(to_binary(&TOTAL_REWARDS.load(deps.storage)?)?)
}
