import { ChainTypeDependent } from '../utils/environment';

export const nftProposalCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 69422,
  MAINNET: 5346,
};

export const nftProposalFactoryCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 21626,
  MAINNET: -1,
};
