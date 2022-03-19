import { ProposalExecuteParamsByType, ProposalQueryByType } from './models';

export type ProposalExecuteMsgType = keyof ProposalExecuteParamsByType;

export function getProposalExecuteMsg<TMsgType extends ProposalExecuteMsgType>(
  type: TMsgType,
  params: ProposalExecuteParamsByType[TMsgType]
): Record<string, ProposalExecuteParamsByType[TMsgType]> {
  return {
    [type]: params,
  };
}

export type ProposalQueryMsgType = keyof ProposalQueryByType;

export function getProposalQueryMsg<TMsgType extends ProposalQueryMsgType>(
  type: TMsgType,
  params: ProposalQueryByType[TMsgType]['query']
): Record<string, ProposalQueryByType[TMsgType]['query']> {
  return {
    [type]: params,
  };
}
