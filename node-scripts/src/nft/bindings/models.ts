/**
 * Models
 */
export interface NftAttribute {
  display_type?: string | null;
  trait_type: string;
  value: string;
}

export interface NftMetadataExtension {
  image?: string | null;
  image_data?: string | null;
  external_url?: string | null;
  description?: string | null;
  name?: string | null;
  attributes?: NftAttribute[] | null;
  background_color?: string | null;
  animation_url?: string | null;
  youtube_url?: string | null;
}

/**
 * Responses
 */
export interface Cw721NumTokensResponse {
  count: number;
}

export interface Cw721TokensResponse {
  // Token IDs
  tokens: string[];
}

export interface Cw721NftInfoResponse {
  token_uri?: string;
  extension?: NftMetadataExtension;
}

/**
 * Instantiate
 */

export interface Cw721InstantiateParams {
  minter: string;
  name: string;
  symbol: string;
}

/**
 * Execute
 */

export interface Cw721ExecuteMintParams {
  token_id: string;
  owner: string;
  token_uri?: string;
  extension?: NftMetadataExtension;
}

export interface Cw721ExecuteTransferNftParams {
  recipient: string;
  token_id: string;
}

export interface Cw721ExecuteSendNftParams {
  contract: string;
  token_id: string;
  msg: string;
}

export interface Cw721ExecuteParamsByType {
  transfer_nft: Cw721ExecuteTransferNftParams;
  send_nft: Cw721ExecuteSendNftParams;
  mint: Cw721ExecuteMintParams;
}

/**
 * Query
 */

type ContractQuery<TQueryParams, TQueryResp> = {
  query: TQueryParams;
  response: TQueryResp;
};

export interface Cw721QueryAllTokensParams {
  start_after?: string;
  limit?: number;
}

export interface Cw721QueryTokensParams {
  owner?: string;
  // Token ID to start after
  start_after?: string;
  limit?: number;
}

export type Cw721AllTokensQuery = ContractQuery<
  Cw721QueryAllTokensParams,
  Cw721TokensResponse
>;

export type Cw721TokensQuery = ContractQuery<
  Cw721QueryTokensParams,
  Cw721TokensResponse
>;

export interface Cw721QueryNftInfoParams {
  token_id: string;
}

export type Cw721NftInfoQuery = ContractQuery<
  Cw721QueryNftInfoParams,
  Cw721NftInfoResponse
>;

export interface Cw721QueryByType {
  tokens: Cw721TokensQuery;
  all_tokens: Cw721AllTokensQuery;
  nft_info: Cw721NftInfoQuery;
}
