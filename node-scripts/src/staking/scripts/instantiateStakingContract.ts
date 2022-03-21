import { getLogger } from '../../common/logger';
import { nftStakingCodeIds } from '../constants';
import { environment } from '../../utils/environment';
import { getWallet } from '../../utils/wallet';
import instantiateContract from '../../utils/instantiateContract';
import { StakingInstantiateParams } from '../bindings/models';

// const INSTANTIATE_MSG: StakingInstantiateParams = {
//   // GP testnet contract
//   nft_contract: 'terra1pk646xtdgwym74k46cajttdu6uvypa5jw5wa3j',
//   // 100 min
//   reward_withdrawal_timeout: 60 * 100,
//   // Royalties
//   trusted_token_sender: 'terra1qzpww3mw0t9aep0x6ksf7nfsvhk3tjy7xt65g7',
//   whitelisted_tokens: ['terra1p2z4y8gjceuyrlqdkgmmt9zyg45qdsrvltv3gn'],
// };

const INSTANTIATE_MSG: StakingInstantiateParams = {
  // GP testnet contract
  nft_contract: 'terra103z9cnqm8psy0nyxqtugg6m7xnwvlkqdzm4s4k',
  // 21 days
  reward_withdrawal_timeout: 21 * 24 * 60 * 60,
  // Royalties
  trusted_token_sender: 'terra1qzpww3mw0t9aep0x6ksf7nfsvhk3tjy7xt65g7',
  // Luart
  whitelisted_tokens: ['terra1vwz7t30q76s7xx6qgtxdqnu6vpr3ak3vw62ygk'],
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
