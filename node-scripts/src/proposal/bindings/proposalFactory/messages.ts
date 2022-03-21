import {
  ProposalFactoryExecuteParamsByType,
  ProposalFactoryQueryByType,
} from './models';

export type ProposalFactoryExecuteMsgType =
  keyof ProposalFactoryExecuteParamsByType;

export function getProposalFactoryExecuteMsg<
  TMsgType extends ProposalFactoryExecuteMsgType
>(
  type: TMsgType,
  params: ProposalFactoryExecuteParamsByType[TMsgType]
): Record<string, ProposalFactoryExecuteParamsByType[TMsgType]> {
  return {
    [type]: params,
  };
}

export type ProposalFactoryQueryMsgType = keyof ProposalFactoryQueryByType;

export function getProposalFactoryQueryMsg<
  TMsgType extends ProposalFactoryQueryMsgType
>(
  type: TMsgType,
  params: ProposalFactoryQueryByType[TMsgType]['query']
): Record<string, ProposalFactoryQueryByType[TMsgType]['query']> {
  return {
    [type]: params,
  };
}
