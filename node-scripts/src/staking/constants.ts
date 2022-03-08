import { ChainTypeDependent } from '../utils/environment';

export const nftStakingCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 32949,
  MAINNET: -1,
};

export const nftStakingContractAddrs: ChainTypeDependent<string> = {
  LOCAL: '',
  TESTNET: 'terra1vkhq4dj3pus3un7eydr5h60efcv0jdfgkll3s8',
  MAINNET: '',
};
