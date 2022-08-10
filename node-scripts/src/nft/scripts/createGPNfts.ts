import * as fs from 'fs';
import * as path from 'path';
import { getWallet } from '../../utils/wallet';
import { chunk, range } from 'lodash';
import { getLogger } from '../../common/logger';
import executeContract from '../../utils/executeContract';
import { delay } from '../../utils/misc';
import { HolderData } from './types';
import {
  CW721ExecuteMsg,
  CW721InstantiateMsg,
  Metadata,
} from '../../bindings/CW721Contract';
import { cw721NftCodeIds } from '../constants';
import instantiateContract from '../../utils/instantiateContract';
import { environment } from '../../utils/environment';

// JSON files from 1-10921 for the GP punk metadata
const METADATA_FOLDER = '../assets/gp_metadata';
// Snapshot of holders
const HOLDERS_JSON_FILE = './holders/gp_holders.json';

interface MetadataFileData {
  token_id: string;
  classic_token_id: string;
  token_uri: string;
  extension: Metadata;
}

function getHolderDataByClassicTokenId(): Record<string, HolderData> {
  const holders: HolderData[] = JSON.parse(
    fs.readFileSync(path.join(__dirname, HOLDERS_JSON_FILE)).toString()
  );

  return holders.reduce((acc, holder) => {
    const tokenId = holder.token_id;
    acc[tokenId] = holder;
    return acc;
  }, {} as Record<string, HolderData>);
}

function getMetadata(punkNumber: number): MetadataFileData {
  return JSON.parse(
    fs
      .readFileSync(path.join(__dirname, METADATA_FOLDER, `${punkNumber}.json`))
      .toString()
  );
}

const logger = getLogger('createGpNfts');

// Used to continue minting in case of failure
const startAtPunkNum = -1;

/**
 * Creates a complete Galactic Punk CW721 NFT contract
 * - Extract `metadata/gp_metadata.zip` into `assets/`
 */
async function main() {
  const holdersByTokenId = getHolderDataByClassicTokenId();
  const wallet = getWallet();

  // Dont do this on mainnet
  if (wallet.lcd.config.chainID !== 'pisco-1') {
    throw Error('Are you on the right network?');
  }

  const cw721InitMessage: CW721InstantiateMsg = {
    minter: wallet.key.accAddress,
    name: 'Galactic Punks',
    symbol: 'GP',
  };
  const nftContract = await instantiateContract({
    contractCodeId: cw721NftCodeIds[environment.chainType],
    initMessage: cw721InitMessage,
    wallet,
    label: 'Galactic Punks NFT',
  });

  logger.info('NFT contract address', nftContract);

  for (const batchedPunkNums of chunk(range(1, 10922), 200)) {
    if (batchedPunkNums[0] < startAtPunkNum) {
      continue;
    }

    // Use a longer delay to avoid hitting rate limits
    await delay(10);

    // Only mint half of the punks on V2
    const filteredPunkNums = batchedPunkNums.filter(() => Math.random() > 0.5);

    const mintMessages = filteredPunkNums.map((punkNumber) => {
      const metadata = getMetadata(punkNumber);
      const mintMsg: CW721ExecuteMsg = {
        mint: {
          extension: metadata.extension,
          owner: holdersByTokenId[metadata.classic_token_id].user_addr,
          token_id: metadata.token_id, // Uses V2 token ID
          token_uri: metadata.token_uri,
        },
      };

      return mintMsg;
    });

    const tx = await executeContract({
      contractAddress: nftContract,
      operations: mintMessages.map((mintMsg) => ({
        message: mintMsg,
      })),
      wallet,
    });

    logger.info(
      'Minted batch starting with punk number: ',
      batchedPunkNums[0],
      '| Tx Hash: ',
      tx.txhash
    );
  }

  logger.info('Finished creating NFTs');
}

main();
