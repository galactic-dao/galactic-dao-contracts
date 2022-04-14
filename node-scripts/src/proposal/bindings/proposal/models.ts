type EmptyParams = Record<string, never>;

/**
 * Models
 */
export interface ProposalOption {
  // Unsigned u16
  id: number;
  name: string;
}

export interface ProposalOptionStatus {
  // ID of the ProposalOption
  id: number;
  votes: number;
}

export interface ProposalConfig {
  title: string;
  nft_contract: string;
  nft_staking_contract: string;
  proposal_uri: string;
  options: ProposalOption[];
  quorum_fraction: string;
  close_time: number;
}

export interface ProposalState {
  creator: string;
  proposer: string;
  is_revoked: boolean;
}

/**
 * Responses
 */

export interface ProposalStatusResponse {
  state: ProposalState;
  config: ProposalConfig;
  tally: ProposalOptionStatus[];
}

export interface ProposalVotesResponse {
  // Voted option IDs in the same order as the token IDs provided in the query, undefined if no vote exists
  votes: (number | null)[];
}

/**
 * Instantiate
 */

export interface ProposalInstantiateParams {
  config: ProposalConfig;
  proposer: string;
}

/**
 * Execute
 */
export interface ProposalExecuteVoteParams {
  option_id: number | null;
  token_id: string;
}

export type ProposalExecuteRevokeParams = EmptyParams;

export interface ProposalExecuteParamsByType {
  vote: ProposalExecuteVoteParams;
  revoke: ProposalExecuteRevokeParams;
}

/**
 * Query
 */

type ContractQuery<TQueryParams, TQueryResp> = {
  query: TQueryParams;
  response: TQueryResp;
};

export interface ProposalQueryVotesParams {
  token_ids: string[];
}

export type ProposalVoteQuery = ContractQuery<
  ProposalQueryVotesParams,
  ProposalVotesResponse
>;

export type ProposalQueryStatusParams = EmptyParams;

export type ProposalStatusQuery = ContractQuery<
  ProposalQueryStatusParams,
  ProposalStatusResponse
>;

export interface ProposalQueryByType {
  votes: ProposalVoteQuery;
  status: ProposalStatusQuery;
}
