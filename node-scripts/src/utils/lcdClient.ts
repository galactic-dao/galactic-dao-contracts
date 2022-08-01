import { ChainType, environment } from './environment';
import { LCDClient } from '@terra-money/terra.js';
import { LCDClientConfig } from '@terra-money/terra.js/dist/client/lcd/LCDClient';

const TERRA_MAINNET_CONFIG: LCDClientConfig = {
  chainID: 'phoenix-1',
  URL: 'https://phoenix-lcd.terra.dev',
};

const TERRA_MAINNET_CLASSIC_CONFIG: LCDClientConfig = {
  chainID: 'columbus-5',
  URL: 'https://columbus-lcd.terra.dev',
};

const TERRA_TESTNET_CONFIG: LCDClientConfig = {
  chainID: 'pisco-1',
  URL: 'https://pisco-lcd.terra.dev',
};

const TERRA_LOCAL_CONFIG: LCDClientConfig = {
  chainID: 'localterra',
  URL: 'http://localhost:1317',
};

export const getLCDClient = (chainTypeOverride?: ChainType): LCDClient => {
  let config: LCDClientConfig;
  const chainType = chainTypeOverride ?? environment.chainType;

  switch (chainType) {
    case 'MAINNET':
      config = TERRA_MAINNET_CONFIG;
      break;
    case 'MAINNET_CLASSIC':
      config = TERRA_MAINNET_CLASSIC_CONFIG;
      break;
    case 'TESTNET':
      config = TERRA_TESTNET_CONFIG;
      break;
    case 'LOCAL':
      config = TERRA_LOCAL_CONFIG;

      break;
  }
  return new LCDClient(config);
};

export const defaultLcdClient = getLCDClient();
