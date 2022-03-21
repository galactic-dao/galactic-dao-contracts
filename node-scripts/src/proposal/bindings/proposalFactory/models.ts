import { ProposalConfig } from '../proposal/models';

type EmptyParams = Record<string, never>;

/**
 * Models
 */

export interface Asset {
  amount: string;
  info:
    | {
        token: {
          contract_addr: string;
        };
      }
    | {
        native_token: {
          denom: string;
        };
      };
}

export interface ProposalFactoryConfig {
  nft_contract: string;
  proposal_cost: Asset;
  proposal_code_id: number;
  min_quorum_fraction: string;
}

export interface ProposalFactoryState {
  owner: string;
  num_created_proposals: number;
}

/**
 * Responses
 */

export interface ProposalFactoryStatusResponse {
  state: ProposalFactoryState;
  config: ProposalFactoryConfig;
}

/**
 * Instantiate
 */

export interface ProposalFactoryInstantiateParams {
  config: ProposalFactoryConfig;
}

/**
 * Execute
 */

export interface ProposalFactoryExecuteCreateProposalParams {
  config: ProposalConfig;
}

export interface ProposalFactoryExecuteModifyConfigParams {
  proposal_cost: Asset | null;
  proposal_code_id: number | null;
  owner: string | null;
  min_quorum_frac: number | null;
}

export interface ProposalFactoryExecuteWithdrawFundsParams {
  asset: Asset;
}

export interface ProposalFactoryExecuteParamsByType {
  create_proposal: ProposalFactoryExecuteCreateProposalParams;
  modify_config: ProposalFactoryExecuteModifyConfigParams;
  withdraw_funds: ProposalFactoryExecuteWithdrawFundsParams;
}

/**
 * Query
 */

type ContractQuery<TQueryParams, TQueryResp> = {
  query: TQueryParams;
  response: TQueryResp;
};

export type ProposalFactoryQueryStatusParams = EmptyParams;

export type ProposalFactoryStatusQuery = ContractQuery<
  ProposalFactoryQueryStatusParams,
  ProposalFactoryStatusResponse
>;

export interface ProposalFactoryQueryProposalsParams {
  start_idx: number | null;
  limit: number | null;
}

export type ProposalFactoryProposalsQuery = ContractQuery<
  ProposalFactoryQueryProposalsParams,
  string[]
>;

export interface ProposalFactoryQueryByType {
  status: ProposalFactoryStatusQuery;
  proposals: ProposalFactoryProposalsQuery;
}
