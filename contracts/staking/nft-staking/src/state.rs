use cw_storage_plus::{Index, IndexedMap, IndexList, Item, MultiIndex};
use galacticdao_nft_staking_protocol::staking::{StakedNftState, StakingConfig};

/// Stores configuration for the contract
pub const CONFIG: Item<StakingConfig> = Item::new("config");

/// Indexed map to retrieve staked NFTs, with primary key being the token ID
pub fn staked_nfts<'a>() -> IndexedMap<'a, &'a str, StakedNftState, TokenIndexes<'a>> {
    let indexes = TokenIndexes {
        owner: MultiIndex::new(
            |d: &StakedNftState, _k: Vec<u8>| d.owner.clone(),
            "staked_nfts",
            "staked__token_owner",
        ),
    };
    IndexedMap::new("staked_nfts", indexes)
}

/// Impl of MultiIndex
pub struct TokenIndexes<'a> {
    /// Index of (owner addr string) -> StakedNftState
    pub owner: MultiIndex<'a, String, StakedNftState>,
}

impl<'a> IndexList<StakedNftState> for TokenIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<StakedNftState>> + '_> {
        let v: Vec<&dyn Index<StakedNftState>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

