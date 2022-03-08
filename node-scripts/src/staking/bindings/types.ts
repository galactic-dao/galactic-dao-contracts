/*
Models
 */
export interface StakingConfig {
  owner: string;
  nft_contract: string;
  whitelisted_tokens: string[];
  trusted_token_sender: string;
  reward_withdrawal_timeout: string;
}

export interface StakedNft {
  token_id: string;
  time_deposited: number;
  can_withdraw_rewards_time: number;
  owner: string;
  last_claim_time: number;
}

export interface TokenBalance {
  amount: string;
  token: string;
}

export interface StakedNftState {
  stake: StakedNft;
  unclaimed_rewards: TokenBalance[];
}

export interface TokenDistribution {
  time: number;
  per_token_balance: TokenBalance;
}

/*
Messages
 */

export interface StakingInstantiateMessage {
  nft_contract: string;
  whitelisted_tokens: string[];
  trusted_token_sender: string;
  reward_withdrawal_timeout: number;
}

export interface StakingExecuteChangeConfigMessage {
  whitelisted_tokens: string[] | null;
  trusted_token_sender: string | null;
  reward_withdrawal_timeout: string | null;
}

export interface StakingExecuteWithdrawNftMessage {
  token_id: string;
}

export interface StakingExecuteWithdrawRewardsMessage {
  token_id: string;
}

export interface StakingExecuteOwnerWithdrawTokensMessage {
  balance: TokenBalance;
}

export interface StakingQueryStakedByAddrMessage {
  address: string;
  start_after_token?: string;
  limit?: string;
}

export interface StakingQueryStakedByTokenIdMessage {
  address: string;
}

export interface StakingQueryAllStakedMessage {
  start_after_token?: string;
  limit?: string;
}

export interface StakingQueryDistributionMessage {
  token_addr: string;
  time: number;
}

export interface StakingQueryDistributionsMessage {
  start_after_time?: string;
  token_addr?: string;
  limit?: string;
}

// CW20
export interface StakingCw20ExecuteSendMessage {
  // Destination contract
  contract: string;
  amount: string;
  msg: string;
}

/*
Responses
 */
