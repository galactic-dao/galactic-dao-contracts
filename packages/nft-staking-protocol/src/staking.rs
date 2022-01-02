use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/*
Models
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingConfig {
    /// The NFT contract associated with NFT staking
    pub nft_contract: String,
    /// The token contract for reward distribution
    pub token_contract: String,
    /// Trusted token sender, only tokens sent from this address will be distributed
    pub trusted_token_sender: String,
    /// Timeout from time of NFT staking to reward eligibility
    pub staking_reward_timeout: u64,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingTokenDistribution {
    /// UNIX seconds of the distribution
    pub time: u64,
    /// Amount allocated to each eligible staked NFT
    pub amount_per_nft: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakedNftState {
    /// UNIX seconds of when the NFT was deposited
    pub time_deposited: u64,
    /// Owner of the NFT
    pub owner: String,
    /// Time of last claimed token distribution
    pub last_claim_time: u64,
    /// Balance that can be withdrawn
    pub available_balance: Uint128,
}

/*
Responses
 */

// TODO

/*
Messages
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingInstantiateMsg {
    /// Config associated with the proposal
    pub config: StakingConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StakingExecuteMsg {
    /// Receive an NFT to begin the staking process, with the NFT contract address
    /// encoded in info.sender
    ReceiveNft(Cw721ReceiveMsg),
    /// Receive tokens from CW20 for distribution, with the CW20 token address
    /// encoded in info.sender
    Receive(Cw20ReceiveMsg),
    /// Withdraw NFT from staking
    WithdrawNft {
        token_id: String,
    },
    /// Claims rewards for a staked NFT up to the current time
    ClaimRewards {
        token_id: String,
    },
    /// Withdraw rewards
    WithdrawRewards {
        amount: Uint128
    },
    /// Used only by the contract owner to decommission the contract
    /// by withdrawing any unclaimed tokens
    WithdrawTokens {
        amount: Uint128
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StakingQueryMsg {
    /// Queries config of contract
    Config {},
    /// Queries staked NFT states by address
    StakedByAddr {
        address: String,
    },
    /// Queries staked NFT state by token ID
    StakedByToken {
        token_id: String,
    },
    /// Queries all staked NFT states, paginated by token ID
    AllStaked {
        start_after_token: Option<String>,
        limit: Option<u32>
    },
    /// Queries distribution history, paginated by ???
    DistributionHistory {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
