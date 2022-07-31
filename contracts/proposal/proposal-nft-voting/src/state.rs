use cw_storage_plus::{Item, Map};

use galacticdao_nft_voting_protocol::proposal::{ProposalConfig, ProposalState};

pub const CONFIG: Item<ProposalConfig> = Item::new("config");
pub const STATE: Item<ProposalState> = Item::new("state");

/// Map of token ID -> voted option ID
pub const VOTE_BY_TOKEN_ID: Map<String, u16> = Map::new("vote_by_token_id");

/// Map of option ID -> number of votes
pub const TALLY: Map<u16, u16> = Map::new("tally");
