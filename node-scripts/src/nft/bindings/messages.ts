import {
  Cw721ExecuteMintMessage,
  Cw721ExecuteSendNftMessage,
  Cw721ExecuteTransferNftMessage,
  Cw721QueryNftInfoMessage,
  Cw721QueryTokensMessage,
} from './types';

/*
Execute
 */
export function getCw721ExecuteMintMsg(params: Cw721ExecuteMintMessage) {
  return {
    mint: params,
  };
}

export function getCw721ExecuteTransferMsg(
  params: Cw721ExecuteTransferNftMessage
) {
  return {
    transfer_nft: params,
  };
}

export function getCw721ExecuteSendNftMsg(params: Cw721ExecuteSendNftMessage) {
  return {
    send_nft: params,
  };
}

/*
Query
 */

export function getCw721QueryNumTokensMsg() {
  return {
    num_tokens: {},
  };
}

export function getCw721QueryTokensMsg(params: Cw721QueryTokensMessage) {
  return {
    tokens: params,
  };
}

export function getCw721QueryNftInfoMsg(params: Cw721QueryNftInfoMessage) {
  return {
    nft_info: params,
  };
}
