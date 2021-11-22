import { ChainTypeDependent } from '../utils/environment';

export const nftProposalCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 20366,
  MAINNET: -1,
};

export const nftProposalFactoryCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 21185,
  MAINNET: -1,
};
