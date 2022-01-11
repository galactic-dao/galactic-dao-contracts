import { getLogger } from '../../common/logger';
import { getCw721QueryTokensMsg } from '../bindings/messages';
import queryContract from '../../utils/queryContract';
import { getLCDClient } from '../../utils/lcdClient';

const NFT_ADDR = 'terra1pk646xtdgwym74k46cajttdu6uvypa5jw5wa3j';

const TOKENS_QUERY_MSG = getCw721QueryTokensMsg({
  owner: 'terra13krd5pty4d9cw0ynm7v4hcw854zw4ehqy6c482',
});

const logger = getLogger('queryNft');

async function queryNft() {
  const resp = await queryContract({
    contractAddress: NFT_ADDR,
    queryMessage: TOKENS_QUERY_MSG,
    lcdClient: getLCDClient(),
  });

  logger.info(resp);
}

queryNft();
