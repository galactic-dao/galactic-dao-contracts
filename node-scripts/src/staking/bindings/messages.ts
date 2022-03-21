import {
  StakingCw20ExecuteSendParams,
  StakingExecuteParamsByType,
  StakingQueryByType,
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

export type StakingQueryMsgType = keyof StakingQueryByType;

export function getStakingQueryMsg<TMsgType extends StakingQueryMsgType>(
  type: TMsgType,
  params: StakingQueryByType[TMsgType]['query']
): Record<string, StakingQueryByType[TMsgType]['query']> {
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
