import { LCDClient, Wallet } from "@terra-money/terra.js";
import instantiateContract from "../../utils/instantiateContract";
import executeContract from "../../utils/executeContract";
import {
  ProposalFactoryExecuteCreateMessage,
  ProposalFactoryInstantiateMessage,
  ProposalFactoryStatusResponse
} from "../proposalFactoryTypes";
import { nftProposalCodeIds, nftProposalFactoryCodeIds } from "../constants";
import { TEST_PROPOSAL_COST } from "./constants";
import { getProposalFactoryExecuteCreateMsg, getProposalFactoryQueryStatusMsg } from "../proposalMessages";
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
  getProposalQueryVotesMsg
} from "../proposalFactoryMessages";
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
