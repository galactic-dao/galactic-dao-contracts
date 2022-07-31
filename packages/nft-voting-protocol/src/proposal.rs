use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/*
Models
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalOption {
    /// ID of the option
    pub id: u16,
    /// Display name
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalOptionStatus {
    /// ID of the option
    pub id: u16,
    /// Number of votes associated with the option
    pub votes: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalConfig {
    /// The NFT contracts associated with NFT voting
    pub nft_contract: String,
    /// Title for proposal
    pub title: String,
    /// A URI with details for the proposal
    pub proposal_uri: String,
    /// The allowed options for voting
    pub options: Vec<ProposalOption>,
    /// The time at which the proposal closes, in seconds
    pub close_time: u64,
    /// Quorum in fractional form (<=1) - not used for any computation but for immutability
    pub quorum_fraction: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalState {
    /// Address of the factory contract that created this proposal
    pub creator: String,
    /// Address that initiated the proposal by calling the factory
    pub proposer: String,
    /// Whether the proposal has been revoked by the proposer
    pub is_revoked: bool,
}

/*
Responses
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalStatusResponse {
    /// Current state of the proposal
    pub state: ProposalState,
    /// Original configuration for the proposal
    pub config: ProposalConfig,
    /// The current statuses for each option
    pub tally: Vec<ProposalOptionStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VotesQueryResponse {
    /// Voted option IDs in the same order as the token IDs provided in the query, null if no vote exists
    pub votes: Vec<Option<u16>>,
}

/*
Messages
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalInstantiateMsg {
    /// Config associated with the proposal
    pub config: ProposalConfig,
    /// Initiator of the proposal
    pub proposer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProposalExecuteMsg {
    /// Casts a vote with the given token ID, if null, indicates a no-vote, revoking any existing votes
    Vote {
        option_id: Option<u16>,
        token_id: String,
    },
    /// Revokes the proposal, can only be called by the proposer
    Revoke {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProposalQueryMsg {
    /// Retrieves general configuration & status for the proposal
    Status {},
    /// Retrieves the current votes for the given token IDs
    Votes { token_ids: Vec<String> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
