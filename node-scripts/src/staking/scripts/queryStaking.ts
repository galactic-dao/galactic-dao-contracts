import { getLogger } from '../../common/logger';
import queryContract from '../../utils/queryContract';
import { getLCDClient } from '../../utils/lcdClient';
import {
  getStakingQueryAllStakedMsg,
  getStakingQueryConfigMsg,
  getStakingQueryDistributionsMsg,
  getStakingQueryNumStakedMsg,
} from '../bindings/messages';
import { nftStakingContractAddrs } from '../constants';
import { environment } from '../../utils/environment';
import {
  StakingQueryAllStakedMessage,
  StakingQueryDistributionsMessage,
} from '../bindings/types';

/*
Query msgs
 */

const CONFIG_QUERY_MSG = getStakingQueryConfigMsg();

const NUM_STAKED_QUERY_MSG = getStakingQueryNumStakedMsg();

const _allStakedParams: StakingQueryAllStakedMessage = {
  start_after_token: undefined,
  limit: undefined,
};
const ALL_STAKED_QUERY_MSG = getStakingQueryAllStakedMsg(_allStakedParams);

const _allDistributionsParams: StakingQueryDistributionsMessage = {
  start_after_time: undefined,
  limit: undefined,
};
const ALL_DISTRIBUTIONS_QUERY_MSG = getStakingQueryDistributionsMsg(
  _allDistributionsParams
);

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
