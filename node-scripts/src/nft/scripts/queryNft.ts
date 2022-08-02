import { getLogger } from '../../common/logger';
import queryContract from '../../utils/queryContract';
import { getLCDClient } from '../../utils/lcdClient';
import { CW721QueryMsg } from '../../bindings/CW721Contract';

const NFT_ADDR = 'terra1tf0ns9mvytce37l4kf7vrslayjplwaufhvh3mu';

const QUERY_MSG: CW721QueryMsg = {
  nft_info: {
    token_id: '1',
  },
};

const logger = getLogger('queryNft');

async function queryNft() {
  const resp = await queryContract({
    contractAddress: NFT_ADDR,
    queryMessage: QUERY_MSG,
    lcdClient: getLCDClient(),
  });

  logger.info(resp);
}

queryNft();
