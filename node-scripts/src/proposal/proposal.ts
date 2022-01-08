import { LCDClient, Wallet } from '@terra-money/terra.js';
import {
  ProposalExecuteVoteMessage,
  ProposalQueryVotesMessage,
  ProposalStatusResponse,
  ProposalVotesResponse,
} from './proposalTypes';
import queryContract from '../utils/queryContract';
import {
  getProposalExecuteVoteMsg,
  getProposalQueryStatusMsg,
  getProposalQueryVotesMsg,
  getProposalRevokeMsg,
} from './proposalMessages';
import { BlockTxBroadcastResult } from '@terra-money/terra.js/dist/client/lcd/api/TxAPI';
import executeContract from '../utils/executeContract';

/*
Execute
 */

/**
 * Executes a vote with a given token ID
 */
export async function executeVoteOnProposal(
  proposalContract: string,
  params: ProposalExecuteVoteMessage,
  wallet: Wallet
): Promise<BlockTxBroadcastResult> {
  return executeContract({
    contractAddress: proposalContract,
    wallet,
    operations: [
      {
        message: getProposalExecuteVoteMsg(params),
        coins: [],
      },
    ],
  });
}

/**
 * Executes a revoke command on the proposal, must be the proposer to do so
 */
export async function executeRevokeOnProposal(
  proposalContract: string,
  wallet: Wallet
): Promise<BlockTxBroadcastResult> {
  return executeContract({
    contractAddress: proposalContract,
    wallet,
    operations: [
      {
        message: getProposalRevokeMsg(),
        coins: [],
      },
    ],
  });
}

/*
Query
 */

/**
 * Queries proposal status
 */
export async function queryProposalStatus(
  proposalContract: string,
  lcd: LCDClient
): Promise<ProposalStatusResponse> {
  return queryContract({
    contractAddress: proposalContract,
    lcdClient: lcd,
    queryMessage: getProposalQueryStatusMsg(),
  });
}

/**
 * Queries proposal votes
 */
export async function queryProposalVotes(
  proposalContract: string,
  params: ProposalQueryVotesMessage,
  lcd: LCDClient
): Promise<ProposalVotesResponse> {
  return queryContract({
    contractAddress: proposalContract,
    lcdClient: lcd,
    queryMessage: getProposalQueryVotesMsg(params),
  });
}
