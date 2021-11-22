import { getWallet } from '../utils/wallet';
import { getLCDClient } from '../utils/lcdClient';

export const testLcdClient = getLCDClient('TESTNET');

// terra104xmc66he33fsxjwkaj6arq04szftn3kwh48hr
export const testWallet1 = getWallet(
  'inherit myself term crystal grain butter lyrics inhale scene round border ramp radar just breeze razor bus meat allow swear sentence village rug fragile',
  testLcdClient
);

// terra1ktd2esx85k4k7vf5n82py2k5m4ghaeukj04l2w
export const testWallet2 = getWallet(
  'expose apple ugly sick robust throw large citizen oxygen theme coil material old scorpion scorpion truth ignore sure very opera tilt dwarf day treat',
  testLcdClient
);

// terra1dh88zsmgaezd26lmal24jp3e4j4vep5546zeeg
export const testWallet3 = getWallet(
  'decide cactus system unveil extra force victory author wild climb burger inhale joy ready hair moon capable town cruise deposit course spot wrap derive',
  testLcdClient
);
