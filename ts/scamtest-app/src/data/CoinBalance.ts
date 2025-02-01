import {createAsync, query} from '@solidjs/router';
import {suiClientCache} from '../stores/suiClient';

export type CoinBalanceProps = {
  network: string;
  coinType: string;
  owner: string | null;
};

export const getCoinBalance = query(async (props: CoinBalanceProps) => {
  if (!props.owner) {
    return null;
  }
  const client = suiClientCache.client(props.network);
  return await client.getBalance({
    coinType: props.coinType,
    owner: props.owner,
  });
}, 'coinBalance');

export const CoinBalance = (props: CoinBalanceProps) => {
  return createAsync(() => getCoinBalance(props));
};
