use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::asset::Asset;
use crate::proposal::ProposalConfig;

/*
Models
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalFactoryConfig {
    /// The NFT contract associated with NFT voting
    pub nft_contract: String,
    /// Cost for creating a proposal
    pub proposal_cost: Asset,
    /// Code ID of the proposal to instantiate
    pub proposal_code_id: u64,
    /// Minimum allowed quorum fraction
    pub min_quorum_fraction: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalFactoryState {
    /// Owner of the proposal factory - all funds are routed to the owner
    pub owner: Addr,
    /// Number of created proposals
    pub num_created_proposals: u64,
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
    CreateProposal { config: ProposalConfig },
    /// Updates the configuration with any specified items
    ModifyConfig {
        proposal_cost: Option<Asset>,
        proposal_code_id: Option<u64>,
        owner: Option<String>,
        min_quorum_fraction: Option<Decimal>,
    },
    /// Withdraws funds within the contract to the owner
    WithdrawFunds { asset: Asset },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProposalFactoryQueryMsg {
    /// Retrieves current status for the proposal factory
    Status {},
    /// Retrieves paginated list of created proposal addresses, limited to max of 100
    Proposals {
        /// Pagination index
        start_idx: Option<u64>,
        /// Max number of proposals to return
        limit: Option<u8>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
