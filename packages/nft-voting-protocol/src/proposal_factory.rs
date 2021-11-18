use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::proposal::ProposalOption;

/*
Models
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalFactoryConfig {
    /// The NFT contract associated with NFT voting
    pub nft_contract: String,
    /// Cost, in uluna, for creating a proposal
    pub proposal_cost: u64,
    /// Code ID of the proposal to instantiate
    pub proposal_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalFactoryState {
    /// Owner of the proposal factory - all funds are routed to the owner
    pub owner: String,
}

/*
Responses
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalFactoryStatusResponse {
    /// Current state
    pub state: ProposalFactoryState,
    /// Current factory config
    pub config: ProposalFactoryConfig,
}

/*
Messages
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalFactoryInstantiateMsg {
    /// Config associated with the proposal factory
    pub config: ProposalFactoryConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProposalFactoryExecuteMsg {
    /// Creates a proposal with the given config
    CreateProposal {
        proposal_uri: String,
        options: Vec<ProposalOption>,
        close_time: u64,
    },
    /// Updates the configuration with any specified items
    ModifyConfig {
        proposal_cost: Option<u64>,
        proposal_code_id: Option<u64>,
    },
    /// Withdraws funds within the contract to the owner
    WithdrawFunds { amount_uluna: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProposalFactoryQueryMsg {
    /// Retrieves current status for the proposal factory
    Status {},
}
