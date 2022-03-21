type EmptyParams = Record<string, never>;

/**
 * Models
 */

export interface StakingConfig {
  owner: string;
  nft_contract: string;
  whitelisted_tokens: string[];
  trusted_token_sender: string;
  reward_withdrawal_timeout: string;
}

export interface TokenBalance {
  amount: string;
  token: string;
}

export interface StakedNft {
  token_id: string;
  time_deposited: number;
  can_withdraw_rewards_time: number;
  owner: string;
  // Total contract distributions snapshotted at time of staking
  beginning_reward_snapshot: TokenBalance[];
  // Total contract distributions since last claim
  last_reward_snapshot: TokenBalance[];
}

export interface StakedNftState {
  stake: StakedNft;
  // Unclaimed rewards for this NFT
  unclaimed_rewards: TokenBalance[];
  // Total accumulated rewards for this NFT
  total_rewards: TokenBalance[];
}

/**
 * Instantiate
 */

export interface StakingInstantiateParams {
  nft_contract: string;
  whitelisted_tokens: string[];
  trusted_token_sender: string;
  reward_withdrawal_timeout: number;
}

/**
 * Execute
 */

export interface StakingExecuteChangeConfigParams {
  whitelisted_tokens: string[] | null;
  trusted_token_sender: string | null;
  reward_withdrawal_timeout: string | null;
}

export interface StakingExecuteWithdrawNftParams {
  token_id: string;
}

export interface StakingExecuteWithdrawRewardsParams {
  token_id: string;
}

export interface StakingExecuteOwnerWithdrawTokensParams {
  balance: TokenBalance;
}

// Expected CW20 Params to send tokens to stkaing contract
export interface StakingCw20ExecuteSendParams {
  // Destination contract
  contract: string;
  amount: string;
  msg: string;
}

export interface StakingExecuteParamsByType {
  change_config: StakingExecuteChangeConfigParams;
  withdraw_nft: StakingExecuteWithdrawNftParams;
  withdraw_rewards: StakingExecuteWithdrawRewardsParams;
  owner_withdraw_tokens: StakingExecuteOwnerWithdrawTokensParams;
}

/**
 * Query
 */

type ContractQuery<TQueryParams, TQueryResp> = {
  query: TQueryParams;
  response: TQueryResp;
};

export type StakingConfigQuery = ContractQuery<EmptyParams, StakingConfig>;

export type StakingNumStakedQuery = ContractQuery<EmptyParams, number>;

export type StakingTotalRewardsQuery = ContractQuery<
  EmptyParams,
  TokenBalance[]
>;

export interface StakingQueryStakedByAddrParams {
  address: string;
  start_after_token?: string;
  limit?: number;
}

export type StakingStakedByAddrQuery = ContractQuery<
  StakingQueryStakedByAddrParams,
  StakedNftState[]
>;

export interface StakingQueryStakedByTokenParams {
  address: string;
}

export type StakingStakedByTokenQuery = ContractQuery<
  StakingQueryStakedByTokenParams,
  StakedNftState | null
>;

export interface StakingQueryAllStakedParams {
  start_after_token?: string;
  limit?: number;
}

export type StakingAllStakedQuery = ContractQuery<
  StakingQueryAllStakedParams,
  StakedNftState[]
>;

export interface StakingQueryByType {
  config: StakingConfigQuery;
  staked_by_addr: StakingStakedByAddrQuery;
  staked_by_token: StakingStakedByTokenQuery;
  all_staked: StakingAllStakedQuery;
  num_staked: StakingNumStakedQuery;
  total_rewards: StakingTotalRewardsQuery;
}
