// eslint-disable-next-line @typescript-eslint/no-var-requires
require('dotenv').config();

export type ChainType = 'MAINNET' | 'TESTNET' | 'LOCAL';

export type ChainTypeDependent<TValue> = {
  [chainType in ChainType]: TValue;
};

type Environment = {
  chainType: string;
  walletMnemonic: string;
};

const checkNotNull = (envVar?: string, name?: string): string => {
  if (!envVar) {
    throw Error(`${name} env var not defined`);
  }
  return envVar;
};

const chainType = checkNotNull(process.env.TERRA_CHAIN_TYPE, 'chain type');
const walletMnemonic = checkNotNull(process.env.WALLET_MNEMONIC, 'wallet');

export const environment: Environment = {
  chainType,
  walletMnemonic,
};
