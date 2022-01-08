import { ProposalOption } from './proposalTypes';

/*
Models
 */
export type ProposalFactoryConfig = {
  nft_contract: string;
  proposal_cost: string;
  proposal_code_id: number;
};

export type ProposalFactoryState = {
  owner: string;
};

/*
Responses
 */
export type ProposalFactoryStatusResponse = {
  state: ProposalFactoryState;
  config: ProposalFactoryConfig;
};

/*
Messages
 */
export type ProposalFactoryInstantiateMessage = {
  config: ProposalFactoryConfig;
};

export type ProposalFactoryExecuteCreateMessage = {
  proposal_uri: String;
  options: ProposalOption[];
  close_time: number;
};

export type ProposalFactoryExecuteModifyConfigMessage = {
  proposal_cost?: string;
  proposal_code_id?: string;
};

export type ProposalFactoryExecuteWithdrawMessage = {
  amount_uluna: string;
};
