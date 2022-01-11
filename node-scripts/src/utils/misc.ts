import { ChainTypeDependent, environment } from './environment';

/*
Converts amount to "micro" denom in string (i.e. Uint128)
ex. 1 LUNA -> 1000000 uluna
 */
export const convertAmountToMicroDenom = (amount: number): string =>
  (amount * Math.pow(10, 6)).toFixed(0);

/*
Injects a delay, usually to wait for next block
*/
const BLOCK_DELAY_SEC: ChainTypeDependent<number> = {
  LOCAL: 1,
  TESTNET: 10,
  MAINNET: 10,
};
export async function delay(seconds?: number) {
  const delaySeconds = seconds ?? BLOCK_DELAY_SEC[environment.chainType];
  await new Promise((resolve) => setTimeout(resolve, delaySeconds * 1000));
}
