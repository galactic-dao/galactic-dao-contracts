import { ChainTypeDependent } from '../utils/environment';

export const cw721NftCodeIds: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 12429,
  MAINNET_CLASSIC: 590,
  MAINNET: -1,
};
