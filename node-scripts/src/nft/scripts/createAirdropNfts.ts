import {
  Cw721ExecuteMintParams,
  Cw721InstantiateParams,
} from '../bindings/models';
import * as fs from 'fs';
import * as path from 'path';
import { getWallet } from '../../utils/wallet';
import instantiateContract from '../../utils/instantiateContract';
import { cw721NftCodeIds } from '../constants';
import { environment } from '../../utils/environment';
import { chunk } from 'lodash';
import { getLogger } from '../../common/logger';
import executeContract from '../../utils/executeContract';
import { delay } from '../../utils/misc';
import { getCw721ExecuteMsg } from '../bindings/messages';
import { HolderData } from './types';
import { getComicAirdropMetadata } from './metadata/airdropMetadata';

const HOLDERS_JSON_FILE = './holders/comic_holders.json';
const NAME = 'The Galactic Punks Comic: Volume 1';
const SYMBOL = 'GPC';
const getMetadata = getComicAirdropMetadata;

const logger = getLogger('createAirdropNfts');

async function main() {
  const wallet = getWallet();

  const holders: HolderData[] = JSON.parse(
    fs.readFileSync(path.join(__dirname, HOLDERS_JSON_FILE)).toString()
  );

  const cw721InitMessage: Cw721InstantiateParams = {
    minter: wallet.key.accAddress,
    name: NAME,
    symbol: SYMBOL,
  };

  const nftContract = await instantiateContract({
    contractCodeId: cw721NftCodeIds[environment.chainType],
    initMessage: cw721InitMessage,
    wallet,
  });

  logger.info('NFT contract address', nftContract);

  for (const batchedHolders of chunk(holders, 100)) {
    // Use a longer delay to avoid hitting rate limits
    await delay(10);

    const mintMessages = batchedHolders.map((holderData) => {
      const metadata = getMetadata(holderData.token_id);
      const mintMsgParams: Cw721ExecuteMintParams = {
        extension: metadata.extension,
        owner: holderData.user_addr,
        token_id: holderData.token_id.toString(),
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
      'Minted batch starting with token ID: ',
      batchedHolders[0].token_id,
      '| Tx Hash: ',
      tx.txhash
    );
  }

  logger.info('Finished creating NFTs');
}

main();
