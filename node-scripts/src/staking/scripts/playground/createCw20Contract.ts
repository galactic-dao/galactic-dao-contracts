import { ChainTypeDependent, environment } from '../../../utils/environment';
import { getWallet } from '../../../utils/wallet';
import { getLogger } from '../../../common/logger';
import instantiateContract from '../../../utils/instantiateContract';
import { convertAmountToMicroDenom } from '../../../utils/misc';

const CW20_CODE_IDS: ChainTypeDependent<number> = {
  LOCAL: -1,
  TESTNET: 16194,
  MAINNET: 808,
};

const ADDR_TO_BALANCE: Record<string, string> = {
  // Royalties
  terra1qzpww3mw0t9aep0x6ksf7nfsvhk3tjy7xt65g7:
    convertAmountToMicroDenom(100000),
};

const logger = getLogger('createCw20Contract');

/**
 * Creates a CW20 contract for testing with initial balances
 */
export async function createCw20Contract() {
  const wallet = getWallet(environment.royaltiesWalletMnemonic);

  const minterAddress = wallet.key.accAddress;

  logger.info('Creating CW20 with minter', minterAddress);

  const initMessage: Record<string, any> = {
    name: 'Galactic Gold',
    symbol: 'GGLD',
    decimals: 6,
    initial_balances: Object.keys(ADDR_TO_BALANCE).map((addr) => {
      const balance = ADDR_TO_BALANCE[addr];

      return {
        address: addr,
        amount: balance,
      };
    }),
    // Set ourselves as minter
    mint: {
      minter: minterAddress,
    },
  };

  const contractAddress = await instantiateContract({
    contractCodeId: CW20_CODE_IDS[environment.chainType],
    initMessage,
    wallet,
  });
  logger.info('CW20 address', contractAddress);
}
