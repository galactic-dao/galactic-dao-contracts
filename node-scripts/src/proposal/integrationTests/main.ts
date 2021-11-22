import { createTestNft, mintNfts } from './nftUtils';
import { testWallet1, testWallet2 } from '../../common/testingShims';
import {
  createTestProposalFactory,
  queryProposalFactoryStatus,
} from './proposalFactoryUtils';
import { delay } from '../../utils/misc';

async function main() {
  // Create a test NFT
  const nftContractAddr = await createTestNft(testWallet1);

  console.log('Deployed test NFT contract at', nftContractAddr);

  // Wait for next block
  await delay(5);

  // Mint to a test wallet
  const initialMintTx = await mintNfts(testWallet1, nftContractAddr, {
    [testWallet2.key.accAddress]: 2,
  });
  console.log('Minted test NFTs', initialMintTx.txhash);

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
}

main();
