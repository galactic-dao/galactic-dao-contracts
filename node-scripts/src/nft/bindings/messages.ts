import { Cw721ExecuteParamsByType, Cw721QueryByType } from './models';

export type Cw721ExecuteMsgType = keyof Cw721ExecuteParamsByType;

export function getCw721ExecuteMsg<TMsgType extends Cw721ExecuteMsgType>(
  type: TMsgType,
  params: Cw721ExecuteParamsByType[TMsgType]
): Record<string, Cw721ExecuteParamsByType[TMsgType]> {
  return {
    [type]: params,
  };
}

export type Cw721QueryMsgType = keyof Cw721QueryByType;

export function getCw721QueryMsg<TMsgType extends Cw721QueryMsgType>(
  type: TMsgType,
  params: Cw721QueryByType[TMsgType]['query']
): Record<string, Cw721QueryByType[TMsgType]['query']> {
  return {
    [type]: params,
  };
}
