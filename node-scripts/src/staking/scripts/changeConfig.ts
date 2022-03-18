import { getLogger } from '../../common/logger';
import { nftStakingContractAddrs } from '../constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import executeContract from '../../utils/executeContract';
import { getStakingExecuteMsg } from '../bindings/messages';

const logger = getLogger('changeConfig');

async function changeConfig() {
  const wallet = getWallet(environment.royaltiesWalletMnemonic);

  await executeContract({
    contractAddress: nftStakingContractAddrs[environment.chainType],
    operations: [
      {
        message: getStakingExecuteMsg('change_config', {
          reward_withdrawal_timeout: null,
          whitelisted_tokens: [
            // GGOLD
            'terra1p2z4y8gjceuyrlqdkgmmt9zyg45qdsrvltv3gn',
            // GPUNK
            'terra16mx96nqfded5ecml4qpq5ww53wa9985hsgv0fz',
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
