import { LCDClient } from '@terra-money/terra.js';
import { getLogger } from '../common/logger';

const logger = getLogger('queryContract');

export type QueryContractVariables = {
  contractAddress: string;
  queryMessage: Record<string, any>;
  lcdClient: LCDClient;
};
const queryContract = async <T>({
  contractAddress,
  queryMessage,
  lcdClient,
}: QueryContractVariables): Promise<T> => {
  logger.debug('Query contract with params', {
    contractAddress,
    queryMessage,
  });

  const result = await lcdClient.wasm.contractQuery(
    contractAddress,
    queryMessage
  );

  logger.debug('Query complete with result', result);

  return result as T;
};

export default queryContract;
