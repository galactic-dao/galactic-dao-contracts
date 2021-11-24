import { LCDClient, Wallet } from "@terra-money/terra.js";
import instantiateContract from "../../utils/instantiateContract";
import executeContract from "../../utils/executeContract";
import {
  ProposalFactoryExecuteCreateMessage,
  ProposalFactoryExecuteModifyConfigMessage,
  ProposalFactoryInstantiateMessage,
  ProposalFactoryStatusResponse
} from "../proposalFactoryTypes";
import { nftProposalCodeIds, nftProposalFactoryCodeIds } from "../constants";
import { TEST_PROPOSAL_COST } from "./constants";
import {
  getProposalFactoryExecuteCreateMsg,
  getProposalFactoryModifyConfigMsg,
  getProposalFactoryQueryStatusMsg
} from "../proposalFactoryMessages";
import queryContract from "../../utils/queryContract";
import { Coins } from "@terra-money/terra.js/dist/core/Coins";
import {
  ProposalExecuteVoteMessage,
  ProposalQueryVotesMessage,
  ProposalStatusResponse,
  ProposalVotesResponse
} from "../proposalTypes";
import {
  getProposalExecuteVoteMsg,
  getProposalQueryStatusMsg,
  getProposalQueryVotesMsg,
  getProposalRevokeMsg
} from "../proposalMessages";
import { BlockTxBroadcastResult } from "@terra-money/terra.js/dist/client/lcd/api/TxAPI";

// TODO: These need cleanup

/**
 * Creates a test proposal factory
 */
export async function createTestProposalFactory(
  wallet: Wallet,
  nftContract: string
): Promise<string> {
  const instantiateMsg: ProposalFactoryInstantiateMessage = {
    config: {
      nft_contract: nftContract,
      proposal_cost: TEST_PROPOSAL_COST,
      proposal_code_id: nftProposalCodeIds['TESTNET'],
    },
  };
  return instantiateContract({
    contractCodeId: nftProposalFactoryCodeIds['TESTNET'],
    initMessage: instantiateMsg,
    wallet,
  });
}

/**
 * Executes proposal creation through the factory contract
 */
export async function createTestProposal(
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
