import {
  Cw721ExecuteMintParams,
  Cw721InstantiateParams,
  NftMetadataExtension,
} from '../bindings/models';
import * as fs from 'fs';
import * as path from 'path';
import { getWallet } from '../../utils/wallet';
import instantiateContract from '../../utils/instantiateContract';
import { cw721NftCodeIds } from '../constants';
import { environment } from '../../utils/environment';
import { chunk, range } from 'lodash';
import { getLogger } from '../../common/logger';
import executeContract from '../../utils/executeContract';
import { delay } from '../../utils/misc';
import { getCw721ExecuteMsg } from '../bindings/messages';

// JSON files from 1-10921 for the GP punk metadata
const METADATA_FOLDER = './assets/gp_metadata';
// Snapshot of holders
const HOLDERS_JSON_FILE = './assets/gp_holders.json';

interface MetadataFileData {
  token_id: string;
  token_uri: string;
  extension: NftMetadataExtension;
}

interface HolderData {
  token_id: string;
  user_addr: string;
}

function getHolderDataByTokenId(): Record<string, HolderData> {
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

/**
 * Creates a complete Galactic Punk CW721 NFT contract
 * - Extract `gp_metadata.zip` into `assets/`
 * - Place `gp_holders.json` into `assets/`
 */
async function main() {
  const holdersByTokenId = getHolderDataByTokenId();
  const wallet = getWallet();

  const cw721InitMessage: Cw721InstantiateParams = {
    minter: wallet.key.accAddress,
    name: 'Galactic Punks',
    symbol: 'GP',
  };
  const nftContract = await instantiateContract({
    contractCodeId: cw721NftCodeIds[environment.chainType],
    initMessage: cw721InitMessage,
    wallet,
  });

  logger.info('NFT contract address', nftContract);

  for (const batchedPunkNums of chunk(range(1, 10922), 100)) {
    const mintMessages = batchedPunkNums.map((punkNumber) => {
      const metadata = getMetadata(punkNumber);
      const mintMsgParams: Cw721ExecuteMintParams = {
        extension: metadata.extension,
        owner: holdersByTokenId[metadata.token_id].user_addr,
        token_id: metadata.token_id,
        token_uri: metadata.token_uri,
      };

      return getCw721ExecuteMsg('mint', mintMsgParams);
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

    // Use a longer delay to avoid hitting rate limits
    await delay(10);
  }

  logger.info('Finished creating NFTs');
}

main();
