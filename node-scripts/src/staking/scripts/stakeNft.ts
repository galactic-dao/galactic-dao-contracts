import { getLogger } from '../../common/logger';
import { nftStakingContractAddrs } from '../constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import executeContract from '../../utils/executeContract';
import { getCw721ExecuteSendNftMsg } from '../../nft/bindings/messages';

const NFT_ADDR = 'terra1pk646xtdgwym74k46cajttdu6uvypa5jw5wa3j';

const logger = getLogger('stakeNft');

async function stakeNft() {
  const wallet = getWallet();

  await executeContract({
    contractAddress: NFT_ADDR,
    operations: [
      {
        message: getCw721ExecuteSendNftMsg({
          contract: nftStakingContractAddrs[environment.chainType],
          msg: '',
          token_id: '100633496447795476300046186918197367338',
        }),
      },
    ],
    wallet,
  });
}

stakeNft().then(() => {
  logger.info('Done!');
});
