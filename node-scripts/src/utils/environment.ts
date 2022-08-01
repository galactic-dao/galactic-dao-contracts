// eslint-disable-next-line @typescript-eslint/no-var-requires
require('dotenv').config();

export type ChainType = 'MAINNET' | 'MAINNET_CLASSIC' | 'TESTNET' | 'LOCAL';

export type ChainTypeDependent<TValue> = {
  [chainType in ChainType]: TValue;
};

type Environment = {
  chainType: ChainType;
  defaultWalletMnemonic: string;
  royaltiesWalletMnemonic: string;
};

const checkNotNull = (envVar?: string, name?: string): string => {
  if (!envVar) {
    throw Error(`${name} env var not defined`);
  }
  return envVar;
};

const chainType = checkNotNull(process.env.TERRA_CHAIN_TYPE, 'chain type');

// Wallets
const defaultWalletMnemonic = checkNotNull(
  process.env.DEFAULT_WALLET_MNEMONIC,
  'default wallet'
);

const royaltiesWalletMnemonic = checkNotNull(
  process.env.ROYALTIES_WALLET_MNEMONIC,
  'royalties wallet'
);

export const environment: Environment = {
  chainType: chainType as ChainType,
  defaultWalletMnemonic,
  royaltiesWalletMnemonic,
};
