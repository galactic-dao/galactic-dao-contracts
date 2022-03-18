import { getLogger } from '../../common/logger';
import { nftStakingContractAddrs } from '../constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import executeContract from '../../utils/executeContract';
import { getStakingExecuteMsg } from '../bindings/messages';

const logger = getLogger('withdrawNft');

async function withdrawNft() {
  const wallet = getWallet();

  await executeContract({
    contractAddress: nftStakingContractAddrs[environment.chainType],
    operations: [
      {
        message: getStakingExecuteMsg('withdraw_nft', {
          token_id: '100633496447795476300046186918197367338',
        }),
      },
    ],
    wallet,
  });
}

withdrawNft().then(() => {
  logger.info('Done!');
});
