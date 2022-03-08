import { getLogger } from '../../common/logger';
import { nftStakingContractAddrs } from '../constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import executeContract from '../../utils/executeContract';
import { getStakingExecuteChangeConfigMsg } from '../bindings/messages';

const logger = getLogger('changeConfig');

async function changeConfig() {
  const wallet = getWallet(environment.royaltiesWalletMnemonic);

  await executeContract({
    contractAddress: nftStakingContractAddrs[environment.chainType],
    operations: [
      {
        message: getStakingExecuteChangeConfigMsg({
          reward_withdrawal_timeout: null,
          whitelisted_tokens: [
            'terra15mc3pc999xn5j9e59z8lhz8hg2tavdfhze0f85',
            'terra13zcjjgwfawzkx62acgxfmmgwu7704r36wms5uv',
            'terra1p2z4y8gjceuyrlqdkgmmt9zyg45qdsrvltv3gn',
          ],
          trusted_token_sender: null,
        }),
      },
    ],
    wallet,
  });
}

changeConfig().then(() => {
  logger.info('Done!');
});
