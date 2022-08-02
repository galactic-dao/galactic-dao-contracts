/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.5.6.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export type ProposalExecuteMsg =
  | {
      vote: {
        option_id?: number | null;
        token_id: string;
        [k: string]: unknown;
      };
    }
  | {
      revoke: {
        [k: string]: unknown;
      };
    };
export type Decimal = string;

export interface ProposalInstantiateMsg {
  config: ProposalConfig;
  proposer: string;

  [k: string]: unknown;
}

export interface ProposalConfig {
  close_time: number;
  nft_contract: string;
  options: ProposalOption[];
  proposal_uri: string;
  quorum_fraction: Decimal;
  title: string;

  [k: string]: unknown;
}

export interface ProposalOption {
  id: number;
  name: string;

  [k: string]: unknown;
}

export type ProposalQueryMsg =
  | {
      status: {
        [k: string]: unknown;
      };
    }
  | {
      votes: {
        token_ids: string[];
        [k: string]: unknown;
      };
    };

export interface ProposalStatusResponse {
  config: ProposalConfig;
  state: ProposalState;
  tally: ProposalOptionStatus[];

  [k: string]: unknown;
}

export interface ProposalState {
  creator: string;
  is_revoked: boolean;
  proposer: string;

  [k: string]: unknown;
}

export interface ProposalOptionStatus {
  id: number;
  votes: number;

  [k: string]: unknown;
}

export interface VotesQueryResponse {
  votes: (number | null)[];

  [k: string]: unknown;
}