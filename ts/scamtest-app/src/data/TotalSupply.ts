import {createAsync, query} from '@solidjs/router';
import {suiClientCache} from '../stores/suiClient';

export type TotalSupplyProps = {
  network: string;
  coinType: string;
};

export const getTotalSupply = query(async (props: TotalSupplyProps) => {
  const client = suiClientCache.client(props.network);
  return await client.getTotalSupply(props);
}, 'totalSupply');

export const TotalSupply = (props: TotalSupplyProps) => {
  return createAsync(() => getTotalSupply(props));
};
