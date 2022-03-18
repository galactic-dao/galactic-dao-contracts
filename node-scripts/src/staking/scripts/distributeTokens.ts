import { getLogger } from '../../common/logger';
import { nftStakingContractAddrs } from '../constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import executeContract from '../../utils/executeContract';
import { convertAmountToMicroDenom } from '../../utils/misc';
import { getStakingCw20SendTokenMsg } from '../bindings/messages';

const LUART_TOKEN_ADDR = 'terra15mc3pc999xn5j9e59z8lhz8hg2tavdfhze0f85';
const GPUNK_TOKEN_ADDR = 'terra16mx96nqfded5ecml4qpq5ww53wa9985hsgv0fz';
const GGOLD_TOKEN_ADDR = 'terra1p2z4y8gjceuyrlqdkgmmt9zyg45qdsrvltv3gn';

const AMOUNT = convertAmountToMicroDenom(5);

const logger = getLogger('distributeTokens');

async function distributeTokens() {
  const wallet = getWallet(environment.royaltiesWalletMnemonic);

  await executeContract({
    contractAddress: GPUNK_TOKEN_ADDR,
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
