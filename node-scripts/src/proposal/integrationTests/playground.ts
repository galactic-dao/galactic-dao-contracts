import { createTestProposal } from './proposalFactoryUtils';
import { getCreatedProposalAddress } from '../proposalFactoryUtils';
import { testWallet2 } from '../../common/testingShims';
import { convertAmountToMicroDenom } from '../../utils/misc';
import { Coins } from '@terra-money/terra.js';

async function main() {
  // NFT contract addr: terra1hh4xp8ejnma6tnv3cf36eeu5dq2kufa6fqnw3n
  // Test proposal factory addr: terra170g3dcvf2ugegkknspynzgn0w2uactsddndvms

  // TODO: try creating a proposal
  const proposalCreationTx = await createTestProposal(
    testWallet2,
    'terra170g3dcvf2ugegkknspynzgn0w2uactsddndvms',
    new Coins({
      uluna: convertAmountToMicroDenom(0.1),
    }),
    {
      // TODO close time
      close_time: 1,
      options: [
        {
          id: 0,
          name: 'Option 1',
        },
        {
          id: 1,
          name: 'Option 2',
        },
      ],
      proposal_uri: '',
    }
  );

  const testProposalAddr = getCreatedProposalAddress(proposalCreationTx);
  console.log('Created test proposal with address', testProposalAddr);

  // TODO query status

  // TODO vote
}

main();
