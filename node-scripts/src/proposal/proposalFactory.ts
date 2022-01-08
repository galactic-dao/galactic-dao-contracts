import { LCDClient, Wallet } from '@terra-money/terra.js';
import { Coins } from '@terra-money/terra.js/dist/core/Coins';
import {
  ProposalFactoryExecuteCreateMessage,
  ProposalFactoryExecuteModifyConfigMessage,
  ProposalFactoryExecuteWithdrawMessage,
  ProposalFactoryStatusResponse,
} from './proposalFactoryTypes';
import executeContract from '../utils/executeContract';
import {
  getProposalFactoryExecuteCreateMsg,
  getProposalFactoryModifyConfigMsg,
  getProposalFactoryQueryStatusMsg,
  getProposalFactoryWithdrawMsg,
} from './proposalFactoryMessages';
import { BlockTxBroadcastResult } from '@terra-money/terra.js/dist/client/lcd/api/TxAPI';
import queryContract from '../utils/queryContract';

/*
Execute
 */

/**
 * Executes proposal creation through the factory contract
 */
export async function executeCreateProposal(
  wallet: Wallet,
  proposalFactoryContract: string,
  coins: Coins.Input,
  params: ProposalFactoryExecuteCreateMessage
) {
  return executeContract({
    contractAddress: proposalFactoryContract,
    wallet,
    operations: [
      {
        message: getProposalFactoryExecuteCreateMsg(params),
        coins,
      },
    ],
  });
}

/**
 * Modifies configuration for a proposal factory
 */
export async function executeModifyProposalFactoryConfig(
  proposalFactoryContract: string,
  wallet: Wallet,
  params: ProposalFactoryExecuteModifyConfigMessage
): Promise<BlockTxBroadcastResult> {
  return executeContract({
    contractAddress: proposalFactoryContract,
    wallet,
    operations: [
      {
        message: getProposalFactoryModifyConfigMsg(params),
      },
    ],
  });
}

/**
 * Attempts to withdraw a given amount of luna from the proposal factory
 */
export async function executeWithdrawFundsFromProposalFactory(
  proposalFactoryContract: string,
  wallet: Wallet,
  params: ProposalFactoryExecuteWithdrawMessage
): Promise<BlockTxBroadcastResult> {
  return executeContract({
    contractAddress: proposalFactoryContract,
    wallet,
    operations: [
      {
        message: getProposalFactoryWithdrawMsg(params),
      },
    ],
  });
}

/*
Query
 */

/**
 * Queries proposal factory status
 */
export async function queryProposalFactoryStatus(
  proposalFactoryContract: string,
  lcd: LCDClient
): Promise<ProposalFactoryStatusResponse> {
  return queryContract({
    contractAddress: proposalFactoryContract,
    lcdClient: lcd,
    queryMessage: getProposalFactoryQueryStatusMsg(),
  });
}
