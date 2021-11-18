import { LCDClient } from '@terra-money/terra.js';

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
  console.log('Query contract with params', {
    contractAddress,
    queryMessage,
  });

  const result = await lcdClient.wasm.contractQuery(
    contractAddress,
    queryMessage
  );

  console.log('Query complete with result', result);

  return result as T;
};

export default queryContract;
