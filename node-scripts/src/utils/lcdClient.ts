import { ChainType, environment } from './environment';
import { LCDClient } from '@terra-money/terra.js';
import { LCDClientConfig } from '@terra-money/terra.js/dist/client/lcd/LCDClient';

const TERRA_MAINNET_CONFIG: LCDClientConfig = {
  chainID: 'columbus-5',
  URL: 'https://lcd.terra.dev',
};

const TERRA_TESTNET_CONFIG: LCDClientConfig = {
  chainID: 'bombay-12',
  URL: 'https://bombay-lcd.terra.dev',
};

const TERRA_LOCAL_CONFIG: LCDClientConfig = {
  chainID: 'localterra',
  URL: 'http://localhost:1317',
};

export const getLCDClient = (chainTypeOverride?: ChainType): LCDClient => {
  let config: LCDClientConfig;
  const chainType = chainTypeOverride ?? environment.chainType;

  if (chainType === 'MAINNET') {
    config = TERRA_MAINNET_CONFIG;
  } else if (chainType === 'LOCAL') {
    config = TERRA_LOCAL_CONFIG;
  } else {
    config = TERRA_TESTNET_CONFIG;
  }

  return new LCDClient(config);
};

export const defaultLcdClient = getLCDClient();
