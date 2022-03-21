import { ChainTypeDependent } from '../utils/environment';

export const nftStakingCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 53789,
  MAINNET: 3801,
};

export const nftStakingContractAddrs: ChainTypeDependent<string> = {
  LOCAL: '',
  TESTNET: 'terra12edeeudxj0f5uyfgye098crhzy080hvjmw9vt7',
  MAINNET: 'terra10t4pgfs6s3qeykqgfq9r74s89jmu7zx5gfkga5',
};
