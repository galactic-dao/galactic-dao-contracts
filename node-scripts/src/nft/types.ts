/*
Messages
 */

export type Cw721InstantiateMessage = {
  minter: string;
  name: string;
  symbol: string;
};

/*
Responses
 */
export type Cw721NumTokensResponse = {
  count: number;
};
