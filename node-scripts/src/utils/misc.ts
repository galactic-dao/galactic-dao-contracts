/*
Converts amount to "micro" denom in string (i.e. Uint128)
ex. 1 LUNA -> 1000000 uluna
 */
export const convertAmountToMicroDenom = (amount: number): string =>
  (amount * Math.pow(10, 6)).toFixed(0);
