
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex, U64Key};
use galacticdao_nft_staking_protocol::staking::{
    StakedNft, StakingConfig, TokenDistribution,
};
pub const TOKEN_DISTRIBUTIONS_PK_NAMESPACE: &'static str = "token_distributions";
pub const STAKED_NFTS_PK_NAMESPACE: &'static str = "staked_nfts";

/// Stores configuration for the contract
pub const CONFIG: Item<StakingConfig> = Item::new("config");

/// Indexed map for past distributions, with primary key being cw20_addr + stake_time in string
/// Important notes on both this indexed map & the one for staked NFTs:
///     - We're bound to CW storage 0.9.1, which has a few drawbacks
///     - The last tuple item for each index (U64Key, Vec<u8>) is always the primary key, which is required
///     - A follow up to the above - later versions have removed this, so online examples / docs may not be relevant
pub fn token_distributions<'a>(
) -> IndexedMap<'a, &'a str, TokenDistribution, TokenDistributionIndices<'a>> {
    let indices = TokenDistributionIndices {
        time: MultiIndex::new(
            |d: &TokenDistribution, k: Vec<u8>| (U64Key::from(d.time), k),
            TOKEN_DISTRIBUTIONS_PK_NAMESPACE,
            "token_distributions__time",
        ),
        token_and_time: MultiIndex::new(
            |d: &TokenDistribution, k: Vec<u8>| {
                (d.per_token_balance.token.clone(), U64Key::from(d.time), k)
            },
            TOKEN_DISTRIBUTIONS_PK_NAMESPACE,
            "token_distributions__token",
        ),
    };
    IndexedMap::new(TOKEN_DISTRIBUTIONS_PK_NAMESPACE, indices)
}

/// Util to get a unique PK for token distribution
pub fn token_distribution_key(distribution: &TokenDistribution) -> String {
    format!(
        "{}_{}",
        distribution.per_token_balance.token, distribution.time
    )
}

/// Indexed map to retrieve staked NFTs, with primary key being the token ID
pub fn staked_nfts<'a>() -> IndexedMap<'a, &'a str, StakedNft, StakedNftIndices<'a>> {
    let indexes = StakedNftIndices {
        owner: MultiIndex::new(
            |d: &StakedNft, k: Vec<u8>| (d.owner.clone(), k),
            STAKED_NFTS_PK_NAMESPACE,
            "staked_nfts__owner",
        ),
    };
    IndexedMap::new(STAKED_NFTS_PK_NAMESPACE, indexes)
}

/// Helper to store number of staked NFTs to increase computational efficiency
pub const NUM_STAKED: Item<u64> = Item::new("num_staked");

/*
Indexed Map - Token Distributions
 */

pub struct TokenDistributionIndices<'a> {
    /// Index of (distribution_time) -> TokenDistribution
    pub time: MultiIndex<'a, (U64Key, Vec<u8>), TokenDistribution>,
    /// Index of (cw_20 addr, distribution_time) -> TokenDistribution
    pub token_and_time: MultiIndex<'a, (String, U64Key, Vec<u8>), TokenDistribution>,
}

impl<'a> IndexList<TokenDistribution> for TokenDistributionIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TokenDistribution>> + '_> {
        let v: Vec<&dyn Index<TokenDistribution>> = vec![&self.time, &self.token_and_time];
        Box::new(v.into_iter())
    }
}

/*
Indexed Map - NFTs
 */

pub struct StakedNftIndices<'a> {
    /// Index of (owner addr string) -> StakedNftState
    pub owner: MultiIndex<'a, (String, Vec<u8>), StakedNft>,
}

impl<'a> IndexList<StakedNft> for StakedNftIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<StakedNft>> + '_> {
        let v: Vec<&dyn Index<StakedNft>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}
