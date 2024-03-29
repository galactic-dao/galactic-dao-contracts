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
    /// Owner of the staking contract
    pub owner: String,
    /// The NFT contract associated with NFT staking
    pub nft_contract: String,
    /// Whitelisted token contracts that will be distributed
    pub whitelisted_tokens: Vec<String>,
    /// Trusted token sender, only tokens sent from this address will be distributed
    pub trusted_token_sender: String,
    /// Timeout from time of NFT staking to reward withdrawal eligibility
    pub reward_withdrawal_timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakedNft {
    /// Token ID of the NFT
    pub token_id: String,
    /// UNIX seconds of when the NFT was deposited
    pub time_deposited: u64,
    /// UNIX seconds of when the NFT will be eligible for withdrawals
    pub can_withdraw_rewards_time: u64,
    /// Owner of the NFT
    pub owner: String,
    /// Snapshot of rewards when the NFT was staked, used to calculated total rewards accumulated
    pub beginning_reward_snapshot: Vec<TokenBalance>,
    /// Last snapshot of total cumulative staking rewards (at time of last claim OR time of stake)
    /// Used to calculate eligible rewards for next claim
    pub last_reward_snapshot: Vec<TokenBalance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenBalance {
    pub amount: Uint128,
    pub token: String,
}

/*
Responses
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakedNftState {
    pub stake: StakedNft,
    /// Rewards that have yet to be claimed
    pub unclaimed_rewards: Vec<TokenBalance>,
    /// Total rewards (including unclaimed) accumulated since initial stake
    pub total_rewards: Vec<TokenBalance>,
}

/*
Messages
 */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakingInstantiateMsg {
    pub nft_contract: String,
    pub whitelisted_tokens: Vec<String>,
    pub trusted_token_sender: String,
    pub reward_withdrawal_timeout: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StakingExecuteMsg {
    /// Changes fields on the current config, other than the immutable ones
    /// Null fields won't be changed
    ChangeConfig {
        whitelisted_tokens: Option<Vec<String>>,
        trusted_token_sender: Option<String>,
        reward_withdrawal_timeout: Option<u64>,
    },
    /// Receive an NFT to begin the staking process, with the NFT contract address
    /// encoded in info.sender
    ReceiveNft(Cw721ReceiveMsg),
    /// Receive tokens from CW20 for distribution, with the CW20 token address
    /// encoded in info.sender
    Receive(Cw20ReceiveMsg),
    /// Withdraw NFT from staking
    WithdrawNft { token_id: String },
    /// Withdraw rewards
    WithdrawRewards { token_id: String },
    /// Used only by the contract owner to decommission the contract
    /// by withdrawing any unclaimed tokens
    OwnerWithdrawTokens { balance: TokenBalance },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StakingQueryMsg {
    /// Queries config of contract
    Config {},
    /// Queries staked NFT states by address
    StakedByAddr {
        address: String,
        start_after_token: Option<String>,
        limit: Option<u32>,
    },
    /// Queries staked NFT state by token ID
    StakedByToken {
        token_id: String,
    },
    /// Queries all staked NFT states, paginated by token ID
    AllStaked {
        start_after_token: Option<String>,
        limit: Option<u32>,
    },
    /// Returns number of staked NFTs
    NumStaked {},
    // Total rewards since the beginning of time
    TotalRewards {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}
