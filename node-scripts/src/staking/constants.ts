import { ChainTypeDependent } from '../utils/environment';

export const nftStakingCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 53789,
  MAINNET: -1,
};

export const nftStakingContractAddrs: ChainTypeDependent<string> = {
  LOCAL: '',
  TESTNET: 'terra12edeeudxj0f5uyfgye098crhzy080hvjmw9vt7',
  MAINNET: '',
};
