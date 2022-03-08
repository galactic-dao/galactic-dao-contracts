import { getLogger } from '../../common/logger';
import { nftStakingContractAddrs } from '../constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import executeContract from '../../utils/executeContract';
import { getStakingExecuteWithdrawRewardsMsg } from '../bindings/messages';

const logger = getLogger('withdrawRewards');

async function withdrawRewards() {
  const wallet = getWallet();

  await executeContract({
    contractAddress: nftStakingContractAddrs[environment.chainType],
    operations: [
      {
        message: getStakingExecuteWithdrawRewardsMsg({
          token_id: '100633496447795476300046186918197367338',
        }),
      },
    ],
    wallet,
  });
}

withdrawRewards().then(() => {
  logger.info('Done!');
});
