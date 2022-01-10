import {
  StakingExecuteChangeConfigMessage,
  StakingExecuteOwnerWithdrawTokensMessage,
  StakingExecuteWithdrawNftMessage,
  StakingExecuteWithdrawRewardsMessage,
  StakingQueryAllStakedMessage,
  StakingQueryDistributionMessage,
  StakingQueryDistributionsMessage,
  StakingQueryStakedByAddrMessage,
  StakingQueryStakedByTokenIdMessage,
} from './types';

/*
Execute
 */
export function getStakingExecuteChangeConfigMsg(
  params: StakingExecuteChangeConfigMessage
) {
  return {
    change_config: params,
  };
}

export function getStakingExecuteWithdrawNftMsg(
  params: StakingExecuteWithdrawNftMessage
) {
  return {
    withdraw_nft: params,
  };
}

export function getStakingExecuteWithdrawRewardsMsg(
  params: StakingExecuteWithdrawRewardsMessage
) {
  return {
    withdraw_rewards: params,
  };
}

export function getStakingExecuteOwnerWithdrawTokensMsg(
  params: StakingExecuteOwnerWithdrawTokensMessage
) {
  return {
    owner_withdraw_tokens: params,
  };
}

/*
Query
 */

export function getStakingQueryConfigMsg() {
  return {
    config: {},
  };
}

export function getStakingQueryStakedByAddrMsg(
  params: StakingQueryStakedByAddrMessage
) {
  return {
    staked_by_addr: params,
  };
}

export function getStakingQueryStakedByTokenMsg(
  params: StakingQueryStakedByTokenIdMessage
) {
  return {
    staked_by_token: params,
  };
}

export function getStakingQueryAllStakedMsg(
  params: StakingQueryAllStakedMessage
) {
  return {
    all_staked: params,
  };
}

export function getStakingQueryNumStakedMsg() {
  return {
    num_staked: {},
  };
}

export function getStakingQueryDistributionMsg(
  params: StakingQueryDistributionMessage
) {
  return {
    distribution: params,
  };
}

export function getStakingQueryDistributionsMsg(
  params: StakingQueryDistributionsMessage
) {
  return {
    distributions: params,
  };
}
