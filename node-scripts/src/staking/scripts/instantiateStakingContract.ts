import { StakingInstantiateMessage } from '../bindings/types';
import { getLogger } from '../../common/logger';
import { nftStakingCodeIds } from '../bindings/constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import instantiateContract from '../../utils/instantiateContract';

const INSTANTIATE_MSG: StakingInstantiateMessage = {
  // GP testnet contract
  nft_contract: 'terra1pk646xtdgwym74k46cajttdu6uvypa5jw5wa3j',
  // 100 min
  reward_withdrawal_timeout: 60 * 100,
  // Royalties
  trusted_token_sender: 'terra1qzpww3mw0t9aep0x6ksf7nfsvhk3tjy7xt65g7',
  whitelisted_tokens: ['terra15mc3pc999xn5j9e59z8lhz8hg2tavdfhze0f85'],
};

const logger = getLogger('instantiateStakingContract');

async function instantiateStakingContract() {
  const wallet = getWallet(environment.royaltiesWalletMnemonic);
  const codeId = nftStakingCodeIds[environment.chainType];

  const stakingContract = await instantiateContract({
    contractCodeId: codeId,
    initMessage: INSTANTIATE_MSG,
    wallet,
  });

  logger.info('Staking contract instantiated', stakingContract);
}

instantiateStakingContract().then(() => {
  logger.info('Done!');
});
