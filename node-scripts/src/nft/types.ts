/*
Models
 */
export type NftAttribute = {
  display_type?: string | null;
  trait_type: string;
  value: string;
};

export type NftMetadataExtension = {
  image?: string | null;
  image_data?: string | null;
  external_url?: string | null;
  description?: string | null;
  name?: string | null;
  attributes?: NftAttribute[] | null;
  background_color?: string | null;
  animation_url?: string | null;
  youtube_url?: string | null;
};

/*
Messages
 */

export type Cw721InstantiateMessage = {
  minter: string;
  name: string;
  symbol: string;
};

export type Cw721ExecuteMintMessage = {
  token_id: string;
  owner: string;
  token_uri?: string;
  extension?: NftMetadataExtension;
};

export type Cw721ExecuteTransferNftMessage = {
  recipient: string;
  token_id: string;
};

export type Cw721ExecuteSendNftMessage = {
  contract: string;
  token_id: string;
  msg: string;
};

export type Cw721QueryTokensMessage = {
  owner?: string;
  // Token ID to start after
  start_after?: string;
  limit?: string;
};

export type Cw721QueryNftInfoMessage = {
  token_id: string;
};

/*
Responses
 */
export type Cw721NumTokensResponse = {
  count: number;
};

export type Cw721TokensResponse = {
  // Token IDs
  tokens: string[];
};

export type Cw721NftInfoResponse = {
  token_id: string;
  owner: string;
  token_uri?: string;
  extension?: NftMetadataExtension;
};
