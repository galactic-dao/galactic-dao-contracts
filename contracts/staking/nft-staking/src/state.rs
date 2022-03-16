use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

use galacticdao_nft_staking_protocol::staking::{StakedNft, StakingConfig, TokenBalance};

pub const STAKED_NFTS_PK_NAMESPACE: &'static str = "staked_nfts";

/// Stores configuration for the contract
pub const CONFIG: Item<StakingConfig> = Item::new("config");

/// Helper to store number of staked NFTs to increase computational efficiency
pub const NUM_STAKED: Item<u64> = Item::new("num_staked");

/// Stores total token rewards PER UNIT NFT since the beginning of time, keyed by CW20 address
pub const TOTAL_REWARDS: Item<Vec<TokenBalance>> = Item::new("total_rewards");

/// Indexed map to retrieve staked NFTs, with primary key being the token ID
/// Important notes:
///     - We're bound to CW storage 0.9.1, which has a few drawbacks
///     - The last tuple item for each index (U64Key, Vec<u8>) is always the primary key, which is required
///     - A follow up to the above - later versions have removed this, so online examples / docs may not be relevant
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

/*
Indexed Map - NFTs
 */

pub struct StakedNftIndices<'a> {
    /// Index of (owner addr string, addr as byte array) -> StakedNftState
    pub owner: MultiIndex<'a, (String, Vec<u8>), StakedNft>,
}

impl<'a> IndexList<StakedNft> for StakedNftIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<StakedNft>> + '_> {
        let v: Vec<&dyn Index<StakedNft>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}
