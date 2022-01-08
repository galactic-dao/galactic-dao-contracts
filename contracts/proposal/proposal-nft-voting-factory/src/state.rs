use cw_storage_plus::Item;

use galacticdao_nft_voting_protocol::proposal_factory::{
    ProposalFactoryConfig, ProposalFactoryState,
};

pub const CONFIG: Item<ProposalFactoryConfig> = Item::new("config");
pub const STATE: Item<ProposalFactoryState> = Item::new("state");
