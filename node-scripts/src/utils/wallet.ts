import { environment } from './environment';
import { LCDClient, MnemonicKey, Wallet } from '@terra-money/terra.js';
import { defaultLcdClient } from './lcdClient';

export const getWallet = (
  mnemonicOverride?: string,
  lcdClientOverride?: LCDClient
): Wallet => {
  const rk = new MnemonicKey({
    mnemonic: mnemonicOverride ?? environment.walletMnemonic,
  });
  const lcdClientToUse = lcdClientOverride ?? defaultLcdClient;
  return lcdClientToUse.wallet(rk);
};

export const defaultWallet = getWallet();
