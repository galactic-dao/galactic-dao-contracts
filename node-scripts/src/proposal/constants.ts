import { ChainTypeDependent } from '../utils/environment';

export const nftProposalCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 2902,
  MAINNET_CLASSIC: 5346,
  MAINNET: 298,
};

export const nftProposalFactoryCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: -1,
  MAINNET_CLASSIC: -1,
  MAINNET: -1,
};
