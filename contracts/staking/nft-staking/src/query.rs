use cosmwasm_std::{to_binary, Binary, Deps, Env, Order, StdResult};
use cw_storage_plus::{Bound, PrimaryKey, U64Key};

use galacticdao_nft_staking_protocol::staking::{StakedNft, StakedNftState, TokenDistribution};

use crate::state::{staked_nfts, token_distribution_key, token_distributions, CONFIG, NUM_STAKED};
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
    let stake = staked_nfts().may_load(deps.storage, token_id.as_str())?;

    let response: Option<StakedNftState> = stake.map_or(
        None,
        |retrieved_stake: StakedNft| -> Option<StakedNftState> {
            Some(query_stake_state(deps, &retrieved_stake).unwrap())
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
            query_stake_state(deps, &stake).unwrap()
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
    let min_bound: Option<Bound> = start_after_token
        .clone()
        .map(|token_id| Bound::Exclusive(token_id.as_bytes().to_vec()));
    let staked: Vec<StakedNftState> = staked_nfts()
        .range(deps.storage, min_bound, None, Order::Ascending)
        .take(query_limit(limit))
        .map(|item| {
            let stake = item.unwrap().1;
            query_stake_state(deps, &stake).unwrap()
        })
        .collect();

    return to_binary(&staked);
}

/// Returns number of staked tokens
pub fn query_num_staked(deps: Deps, _env: Env) -> StdResult<Binary> {
    to_binary(&NUM_STAKED.load(deps.storage)?)
}

/// Returns a single distribution, or null if not found
pub fn query_distribution(
    deps: Deps,
    _env: Env,
    token_addr: &String,
    time: u64,
) -> StdResult<Binary> {
    let distribution: Option<TokenDistribution> = token_distributions().may_load(
        deps.storage,
        token_distribution_key(token_addr, time).as_str(),
    )?;

    return to_binary(&distribution);
}

/// Returns paginated token distributions
pub fn query_distributions(
    deps: Deps,
    env: Env,
    token_addr: &Option<String>,
    start_after_time: &Option<u64>,
    limit: &Option<u32>,
) -> StdResult<Binary> {
    let min_time = start_after_time.unwrap_or(0);
    let query_lim = query_limit(limit);

    match token_addr {
        Some(token) => query_distributions_by_token_and_time(deps, env, token, min_time, query_lim),
        None => query_distributions_by_time(deps, env, min_time, query_lim),
    }
}

/// Query distributions by time only
pub fn query_distributions_by_time(
    deps: Deps,
    _env: Env,
    start_after_time: u64,
    limit: usize,
) -> StdResult<Binary> {
    let distributions: Vec<TokenDistribution> = token_distributions()
        .idx
        .time
        // Hack for CW Storage Plus - no support for IndexedMap & exclusive bounds
        .range(
            deps.storage,
            Some(Bound::inclusive(
                // Last key is the PK, which we define to be empty vec for inclusive bound
                (U64Key::new(start_after_time), vec![]).joined_key(),
            )),
            None,
            Order::Ascending,
        )
        .map(|item| item.unwrap().1)
        // Filter out to get exclusive bound
        .filter(|distribution| distribution.time > start_after_time)
        .take(limit)
        .collect();

    return Ok(to_binary(&distributions)?);
}

/// Query distributions by token and time. Most of this is shared with query_distributions_by_time
pub fn query_distributions_by_token_and_time(
    deps: Deps,
    _env: Env,
    token_addr: &String,
    after_time: u64,
    limit: usize,
) -> StdResult<Binary> {
    let distributions: Vec<TokenDistribution> = token_distributions()
        .idx
        .token_and_time
        .sub_prefix(token_addr.clone())
        // Hack for CW Storage Plus - no support for IndexedMap & exclusive bounds
        .range(
            deps.storage,
            Some(Bound::inclusive(
                // Last key is the PK, which we define to be empty vec for inclusive bound
                (U64Key::new(after_time), vec![]).joined_key(),
            )),
            None,
            Order::Ascending,
        )
        .map(|item| item.unwrap().1)
        // Filter out to get exclusive bound
        .filter(|distribution| distribution.time > after_time)
        .take(limit)
        .collect();

    return Ok(to_binary(&distributions)?);
}
