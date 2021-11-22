/*
Execute
 */
export function getCw721MintMsg(tokenId: string, owner: string) {
  return {
    mint: {
      token_id: tokenId,
      owner,
    },
  };
}

export function getCw721TransferMsg(recipient: string, tokenId: string) {
  return {
    transfer_nft: {
      recipient,
      token_id: tokenId,
    },
  };
}

/*
Query
 */

export function getCw721NumTokensMsg() {
  return {
    num_tokens: {},
  };
}
