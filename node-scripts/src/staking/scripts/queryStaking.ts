import { getLogger } from '../../common/logger';
import queryContract from '../../utils/queryContract';
import { getLCDClient } from '../../utils/lcdClient';
import { nftStakingContractAddrs } from '../constants';
import { environment } from '../../utils/environment';
import { getStakingQueryMsg } from '../bindings/messages';
import { StakingQueryAllStakedParams } from '../bindings/models';

/*
Query msgs
 */

const CONFIG_QUERY_MSG = getStakingQueryMsg('config', {});

const NUM_STAKED_QUERY_MSG = getStakingQueryMsg('num_staked', {});

const _allStakedParams: StakingQueryAllStakedParams = {
  start_after_token: undefined,
  limit: undefined,
};
const ALL_STAKED_QUERY_MSG = getStakingQueryMsg('all_staked', _allStakedParams);

const TOTAL_REWARDS_QUERY_MSG = getStakingQueryMsg('total_rewards', {});

const logger = getLogger('queryStaking');

async function queryStaking() {
  const resp = await queryContract({
    contractAddress: nftStakingContractAddrs[environment.chainType],
    queryMessage: NUM_STAKED_QUERY_MSG,
    lcdClient: getLCDClient(),
  });

  logger.info(JSON.stringify(resp, null, 2));
}

queryStaking();
