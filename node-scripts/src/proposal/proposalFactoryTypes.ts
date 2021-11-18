/*
Models
 */

import { ProposalOption } from "./proposalTypes";

export type ProposalFactoryConfig = {
  nft_contract: string;
  proposal_cost: string;
  proposal_code_id: string;
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

export type VotesQueryResponse = {
  // Voted option IDs in the same order as the token IDs provided in the query, undefined if no vote exists
  votes: (number | undefined)[];
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
  close_time: string;
};

export type ProposalFactoryExecuteModifyConfigMessage = {
  proposal_cost?: string;
  proposal_code_id?: string;
};

export type ProposalFactoryExecuteWithdrawMessage = {
  amount_uluna: string;
};
