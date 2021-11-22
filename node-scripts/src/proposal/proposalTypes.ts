/*
Models
 */

export type ProposalOption = {
  // Unsigned u16
  id: number;
  name: string;
};

export type ProposalOptionStatus = {
  // ID of the ProposalOption
  id: number;
  votes: number;
};

export type ProposalConfig = {
  nft_contract: string;
  proposal_uri: string;
  options: ProposalOption[];
  close_time: number;
  proposer: string;
};

export type ProposalState = {
  creator: string;
  is_revoked: boolean;
};

/*
Responses
 */
export type ProposalStatusResponse = {
  state: ProposalState;
  config: ProposalConfig;
  tally: ProposalOptionStatus[];
};

export type VotesQueryResponse = {
  // Voted option IDs in the same order as the token IDs provided in the query, undefined if no vote exists
  votes: (number | undefined)[];
};

/*
Messages
 */
export type ProposalInstantiateMessage = {
  config: ProposalConfig;
};

export type ProposalExecuteVoteMessage = {
  option_id?: number;
  token_id: string;
};

export type ProposalQueryVotesMessage = {
  token_ids: string[];
};
