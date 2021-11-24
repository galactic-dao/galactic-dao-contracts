import { ChainTypeDependent } from '../utils/environment';

export const nftProposalCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 21625,
  MAINNET: -1,
};

export const nftProposalFactoryCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 21626,
  MAINNET: -1,
};
