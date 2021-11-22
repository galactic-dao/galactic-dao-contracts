/*
Converts amount to "micro" denom in string (i.e. Uint128)
ex. 1 LUNA -> 1000000 uluna
 */
export const convertAmountToMicroDenom = (amount: number): string =>
  (amount * Math.pow(10, 6)).toFixed(0);

/*
Injects a delay, usually to wait for next block
*/
export async function delay(seconds: number) {
  await new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}
