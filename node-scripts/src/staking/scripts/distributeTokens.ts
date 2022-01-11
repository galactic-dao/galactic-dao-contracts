import { getLogger } from '../../common/logger';
import { nftStakingContractAddrs } from '../bindings/constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import executeContract from '../../utils/executeContract';
import { getStakingCw20SendTokenMsg } from '../bindings/messages';

const TOKEN_ADDR = 'terra15mc3pc999xn5j9e59z8lhz8hg2tavdfhze0f85';
const AMOUNT = '1';

const logger = getLogger('distributeTokens');

async function distributeTokens() {
  const wallet = getWallet(environment.royaltiesWalletMnemonic);

  await executeContract({
    contractAddress: TOKEN_ADDR,
    operations: [
      {
        message: getStakingCw20SendTokenMsg({
          contract: nftStakingContractAddrs[environment.chainType],
          amount: AMOUNT,
          msg: '',
        }),
      },
    ],
    wallet,
  });
}

distributeTokens().then(() => {
  logger.info('Done!');
});
