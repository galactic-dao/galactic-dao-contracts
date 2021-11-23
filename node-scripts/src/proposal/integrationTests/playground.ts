import {
  createTestProposal,
  executeVoteOnProposal,
  queryProposalStatus,
  queryProposalVotes,
} from './proposalFactoryUtils';
import { testWallet1, testWallet2 } from '../../common/testingShims';
import { convertAmountToMicroDenom, delay } from '../../utils/misc';
import { ProposalOption } from '../proposalTypes';
import { Coins } from '@terra-money/terra.js/dist/core/Coins';
import { getCreatedProposalAddress } from '../proposalFactoryUtils';

async function main() {
  // NFT contract addr: terra1hh4xp8ejnma6tnv3cf36eeu5dq2kufa6fqnw3n
  // Test proposal factory addr: terra170g3dcvf2ugegkknspynzgn0w2uactsddndvms

  // Create a proposal
  const secondsSinceEpoch = Math.round(Date.now() / 1000);
  const closeTime = secondsSinceEpoch + 5 * 60;
  const proposalUri = 'testUri';
  const proposalOptions: ProposalOption[] = [
    {
      id: 0,
      name: 'Option 1',
    },
    {
      id: 1,
      name: 'Option 2',
    },
  ];
  const proposalLunaCost = 0.1;

  const proposalCreationTx = await createTestProposal(
    testWallet2,
    'terra170g3dcvf2ugegkknspynzgn0w2uactsddndvms',
    new Coins({
      uluna: convertAmountToMicroDenom(proposalLunaCost),
    }),
    {
      close_time: closeTime,
      options: proposalOptions,
      proposal_uri: proposalUri,
    }
  );

  const testProposalAddr = getCreatedProposalAddress(proposalCreationTx);
  console.log('Created test proposal with address', testProposalAddr);

  await delay(5);

  let proposalStatus = await queryProposalStatus(
    testProposalAddr,
    testWallet1.lcd
  );

  // TODO: Nice checks
  console.log(
    'Fetched proposal status',
    JSON.stringify(proposalStatus, null, 2)
  );
  if (proposalStatus.state.is_revoked) {
    throw Error('Proposal is revoked');
  }
  if (proposalStatus.config.proposer != testWallet2.key.accAddress) {
    throw Error('Incorrect proposal proposer');
  }

  let votesForTokenId1 = await queryProposalVotes(
    testProposalAddr,
    { token_ids: ['1'] },
    testWallet2.lcd
  );
  console.log(
    'Votes for token ID 1',
    JSON.stringify(votesForTokenId1, null, 2)
  );

  // Now vote on first option
  await executeVoteOnProposal(
    testProposalAddr,
    {
      token_id: '1',
      option_id: proposalOptions[0].id,
    },
    testWallet2
  );

  await delay(5);

  votesForTokenId1 = await queryProposalVotes(
    testProposalAddr,
    { token_ids: ['1'] },
    testWallet2.lcd
  );
  console.log(
    'Votes for token ID 1 after voting',
    JSON.stringify(votesForTokenId1, null, 2)
  );

  proposalStatus = await queryProposalStatus(testProposalAddr, testWallet1.lcd);
  console.log(
    'Fetched proposal status after voting',
    JSON.stringify(proposalStatus, null, 2)
  );

  // Now retract the vote
  await executeVoteOnProposal(
    testProposalAddr,
    {
      token_id: '1',
    },
    testWallet2
  );

  await delay(5);

  votesForTokenId1 = await queryProposalVotes(
    testProposalAddr,
    { token_ids: ['1'] },
    testWallet2.lcd
  );
  console.log(
    'Votes for token ID 1 after retracting vote',
    JSON.stringify(votesForTokenId1, null, 2)
  );

  proposalStatus = await queryProposalStatus(testProposalAddr, testWallet1.lcd);
  console.log(
    'Fetched proposal status after retracting vote',
    JSON.stringify(proposalStatus, null, 2)
  );

  // TODO: Voting with wallet2 tokenId 3 should fail (TODO: Should mint tokenId3 though)

  // TODO: Voting with wallet3 should fail

  // TODO: Wait for close - revoke proposal & vote should fail

  // TODO: Change config on factory and create a proposal

  // TODO: Then revoke proposal - should fail
}

main();
