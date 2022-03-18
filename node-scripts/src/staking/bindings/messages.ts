import {
  StakingCw20ExecuteSendParams,
  StakingExecuteParamsByType,
  StakingQueryParamsByType,
} from './models';

export type StakingExecuteMsgType = keyof StakingExecuteParamsByType;

export function getStakingExecuteMsg<TMsgType extends StakingExecuteMsgType>(
  type: TMsgType,
  params: StakingExecuteParamsByType[TMsgType]
): Record<string, StakingExecuteParamsByType[TMsgType]> {
  return {
    [type]: params,
  };
}

export type StakingQueryMsgType = keyof StakingQueryParamsByType;

export function getStakingQueryMsg<TMsgType extends StakingQueryMsgType>(
  type: TMsgType,
  params: StakingQueryParamsByType[TMsgType]
): Record<string, StakingQueryParamsByType[TMsgType]> {
  return {
    [type]: params,
  };
}

// Util for sending CW20 token
export function getStakingCw20SendTokenMsg(
  params: StakingCw20ExecuteSendParams
) {
  return {
    send: params,
  };
}
