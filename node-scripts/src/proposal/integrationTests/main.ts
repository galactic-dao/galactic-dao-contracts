import {
  createTestNft,
  createTestProposalFactory,
  mintTestNfts,
} from './testUtils';
import {
  testWallet1,
  testWallet2,
  testWallet3,
} from '../../common/testingShims';
import { convertAmountToMicroDenom, delay } from '../../utils/misc';
import { ProposalOption } from '../proposalTypes';
import { Coins } from '@terra-money/terra.js/dist/core/Coins';
import { getCreatedProposalAddress } from '../bindings/utils';
import {
  executeCreateProposal,
  executeModifyProposalFactoryConfig,
  executeWithdrawFundsFromProposalFactory,
  queryProposalFactoryStatus,
} from '../proposalFactory';
import {
  executeRevokeOnProposal,
  executeVoteOnProposal,
  queryProposalStatus,
  queryProposalVotes,
} from '../proposal';
import { Numeric } from '@terra-money/terra.js';

function getSecondsSinceEpoch(): number {
  return Math.round(Date.now() / 1000);
}

async function main() {
  // Create a test NFT
  const nftContractAddr = await createTestNft(testWallet1);

  console.log('Deployed test NFT contract at', nftContractAddr);

  // Wait for next block
  await delay(5);

  // Mint to a test wallet
  const initialMintTx = await mintTestNfts(testWallet1, nftContractAddr, {
    [testWallet2.key.accAddress]: 2,
    [testWallet1.key.accAddress]: 2,
  });
  console.log('Minted test NFTs', initialMintTx.txhash);

  await delay(5);

  // Deploy proposal factory
  const proposalFactoryAddr = await createTestProposalFactory(
    testWallet1,
    nftContractAddr
  );
  console.log('Created proposal factory', proposalFactoryAddr);

  // Wait for next block
  await delay(5);

  // Query status
  const factoryStatus = await queryProposalFactoryStatus(
    proposalFactoryAddr,
    testWallet1.lcd
  );
  console.log('Proposal factory status', factoryStatus);

  // Try creating a proposal

  // Create a proposal
  const closeTime = getSecondsSinceEpoch() + 5 * 60;
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
  const proposalLunaCost = 1;

  const proposalCreationTx = await executeCreateProposal(
    testWallet2,
    proposalFactoryAddr,
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

  // Test withdrawing funds from creation
  const [prevTestWalletCoins] = await testWallet1.lcd.bank.balance(
    testWallet1.key.accAddress
  );
  const prevTestWalletUluna = prevTestWalletCoins.get('uluna')!;
  await executeWithdrawFundsFromProposalFactory(
    proposalFactoryAddr,
    testWallet1,
    {
      amount_uluna: convertAmountToMicroDenom(proposalLunaCost),
    }
  );

  await delay(5);

  const [newTestWalletCoins] = await testWallet1.lcd.bank.balance(
    testWallet1.key.accAddress
  );
  const newTestWalletUluna = newTestWalletCoins.get('uluna')!;

  if (
    newTestWalletUluna.amount
      .sub(prevTestWalletUluna.amount)
      .sub(Numeric.parse(convertAmountToMicroDenom(proposalLunaCost)).mul(0.95))
      .lt(0)
  ) {
    throw Error(
      `Did not successfully withdraw. New balance: ${newTestWalletUluna.amount} | prev balance: ${prevTestWalletUluna.amount}`
    );
  }

  // Now test the proposal status

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

  if (votesForTokenId1.votes[0] !== 0) {
    throw Error('Invalid current vote for tokenID 1 after voting');
  }
  if (proposalStatus.tally[0].id != 0 || proposalStatus.tally[0].votes != 1) {
    throw Error('Invalid # votes after voting');
  }

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

  if (votesForTokenId1.votes[0] != null) {
    throw Error('Invalid current vote for tokenID 1 after retracting vote');
  }
  if (proposalStatus.tally[0].id != 0 || proposalStatus.tally[0].votes != 0) {
    throw Error('Invalid # votes after retracting vote');
  }

  // Voting with wallet2 & tokenId 3 should fail as wallet2 doesn't own this token
  try {
    await executeVoteOnProposal(
      testProposalAddr,
      {
        token_id: '3',
      },
      testWallet2
    );
    throw Error('Voting with unowned token ID 3 succeeded');
  } catch (err) {
    console.log('Revoking with unowned token ID threw error as expected');
  }

  // Revoke should fail with non-proposer
  try {
    await executeRevokeOnProposal(testProposalAddr, testWallet2);
    throw Error('Revoking with non-proposer succeeded');
  } catch (err) {
    console.log('Revoking with non-proposer threw error as expected');
  }

  // Wait for close - revoke proposal & vote should fail
  while (getSecondsSinceEpoch() < closeTime) {
    console.log('Waiting for proposal close');
    await delay(30);
  }

  // Revoke & vote should both fail
  try {
    await executeVoteOnProposal(
      testProposalAddr,
      {
        token_id: '1',
      },
      testWallet2
    );
    throw Error('Voting after proposal close succeeded');
  } catch (err) {
    console.log('Voting after proposal close threw error as expected');
  }
  try {
    await executeRevokeOnProposal(testProposalAddr, testWallet1);
    throw Error('Revoking after proposal close succeeded');
  } catch (err) {
    console.log('Voting after proposal close threw error as expected');
  }

  // Try to modify factory config
  const newProposalCost = convertAmountToMicroDenom(0.2);
  try {
    await executeModifyProposalFactoryConfig(proposalFactoryAddr, testWallet2, {
      proposal_cost: newProposalCost,
    });
    throw Error('Modifying factory config with non-owner succeeded');
  } catch (err) {
    console.log(
      'Modifying factory config with non-owner threw error as expected'
    );
  }
  await executeModifyProposalFactoryConfig(proposalFactoryAddr, testWallet1, {
    proposal_cost: newProposalCost,
  });

  await delay(5);

  const updatedFactoryStatus = await queryProposalFactoryStatus(
    proposalFactoryAddr,
    testWallet1.lcd
  );
  if (updatedFactoryStatus.config.proposal_cost != newProposalCost) {
    throw Error('Proposal cost did not update');
  }

  // Try creating a proposal from non-GP owner
  try {
    await executeCreateProposal(
      testWallet3,
      proposalFactoryAddr,
      new Coins({
        uluna: newProposalCost,
      }),
      {
        close_time: closeTime,
        options: proposalOptions,
        proposal_uri: proposalUri,
      }
    );
    throw Error('Creating proposal from non-owner succeeded');
  } catch (err) {
    console.log('Creating proposal with non-owner threw error as expected');
  }

  // Create another proposal to test revoke
  const proposalCreationTx2 = await executeCreateProposal(
    testWallet2,
    proposalFactoryAddr,
    new Coins({
      uluna: newProposalCost,
    }),
    {
      close_time: closeTime,
      options: proposalOptions,
      proposal_uri: proposalUri,
    }
  );
  const testProposalAddr2 = getCreatedProposalAddress(proposalCreationTx2);
  console.log('Created test proposal with address', testProposalAddr2);

  await delay(5);

  await executeRevokeOnProposal(testProposalAddr2, testWallet2);

  await delay(5);

  try {
    await executeVoteOnProposal(
      testProposalAddr2,
      {
        token_id: '1',
      },
      testWallet2
    );
    throw Error('Voting on revoked proposal succeeded');
  } catch (err) {
    console.log('Voting on revoked proposal threw error as expected');
  }

  const updatedProposalStatusAfterRevoke = await queryProposalStatus(
    testProposalAddr2,
    testWallet1.lcd
  );
  if (!updatedProposalStatusAfterRevoke.state.is_revoked) {
    throw Error('Proposal revoked state did not update');
  }

  console.log('Integration tests passed!');
}

main();
