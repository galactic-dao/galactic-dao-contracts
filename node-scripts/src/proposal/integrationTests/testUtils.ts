import { Wallet } from '@terra-money/terra.js';
import {
  Cw721InstantiateMessage,
  Cw721NumTokensResponse,
} from '../../nft/types';
import instantiateContract from '../../utils/instantiateContract';
import queryContract from '../../utils/queryContract';
import {
  getCw721ExecuteMintMsg,
  getCw721QueryNumTokensMsg,
} from '../../nft/messages';
import executeContract, {
  ExecuteContractOperation,
} from '../../utils/executeContract';
import { range } from 'lodash';
import { cw721NftCodeIds } from '../../nft/constants';
import { ProposalFactoryInstantiateMessage } from '../proposalFactoryTypes';
import { TEST_PROPOSAL_COST } from './constants';
import { nftProposalCodeIds, nftProposalFactoryCodeIds } from '../constants';

/**
 * Creates a test NFT contract with no token balances
 */
export async function createTestNft(wallet: Wallet): Promise<string> {
  const instantiateMsg: Cw721InstantiateMessage = {
    minter: wallet.key.accAddress,
    name: 'Test',
    symbol: 'TST',
  };
  return instantiateContract({
    contractCodeId: cw721NftCodeIds['TESTNET'],
    initMessage: instantiateMsg,
    wallet,
  });
}

/**
 * Mint certain NFTs to the given addresses with certain quantities
 */
export async function mintTestNfts(
  wallet: Wallet,
  testNftContract: string,
  addressToQty: Record<string, number>
) {
  const numTokensResponse = await queryContract<Cw721NumTokensResponse>({
    contractAddress: testNftContract,
    lcdClient: wallet.lcd,
    queryMessage: getCw721QueryNumTokensMsg(),
  });
  let currentTokenId = numTokensResponse.count + 1;

  // Messages
  const contractOperations: ExecuteContractOperation[] = [];
  Object.keys(addressToQty).forEach((address) => {
    contractOperations.push(
      ...range(0, addressToQty[address]).map<ExecuteContractOperation>(
        (itemNumForAddress) => {
          return {
            coins: [],
            message: getCw721ExecuteMintMsg({
              token_id: (currentTokenId + itemNumForAddress).toFixed(0),
              owner: address,
            }),
          };
        }
      )
    );

    currentTokenId += addressToQty[address];
  });

  return executeContract({
    contractAddress: testNftContract,
    wallet,
    operations: contractOperations,
  });
}

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
