import {
  ProposalInstantiateParams,
  ProposalOption,
} from '../../bindings/proposal/models';
import { getLogger } from '../../../common/logger';
import { getWallet } from '../../../utils/wallet';
import { nftProposalCodeIds } from '../../constants';
import { environment } from '../../../utils/environment';
import instantiateContract from '../../../utils/instantiateContract';

const PROPOSAL_OPTIONS: ProposalOption[] = [
  {
    id: 0,
    name: 'Abstain',
  },
  {
    id: 1,
    name: 'Yes',
  },
  {
    id: 2,
    name: 'No',
  },
];

const CLOSE_TIME = Math.round(Date.now() / 1000 + 60 * 60 * 24 * 3); // 3 Days

const logger = getLogger('instantiateProposalContract');

async function instantiateProposalContract() {
  const wallet = getWallet();
  const codeId = nftProposalCodeIds[environment.chainType];

  console.log('Start date', new Date().toISOString());
  console.log('Close date', new Date(CLOSE_TIME * 1000).toISOString());

  const instantiateMsg: ProposalInstantiateParams = {
    config: {
      // GP testnet contracts
      // nft_contract: 'terra1pk646xtdgwym74k46cajttdu6uvypa5jw5wa3j',
      // nft_staking_contract: 'terra12edeeudxj0f5uyfgye098crhzy080hvjmw9vt7',
      // GP mainnet contracts
      nft_contract: 'terra103z9cnqm8psy0nyxqtugg6m7xnwvlkqdzm4s4k',
      nft_staking_contract: 'terra10t4pgfs6s3qeykqgfq9r74s89jmu7zx5gfkga5',
      title: 'PROPOSAL 023 - TERRA 2.0 GALACTIC DAO VALIDATOR',
      proposal_uri:
        'ipfs://bafkreicowqqm2vaklmn6k4uoerqgkefoc52tdgsxjwmshq6bga2csqltv4',
      options: PROPOSAL_OPTIONS,
      quorum_fraction: '0.05',
      close_time: CLOSE_TIME,
    },
    proposer: wallet.key.accAddress,
  };

  const contract = await instantiateContract({
    contractCodeId: codeId,
    initMessage: instantiateMsg,
    wallet,
  });

  logger.info('Contract instantiated', contract);
}

instantiateProposalContract().then(() => {
  logger.info('Done!');
});
