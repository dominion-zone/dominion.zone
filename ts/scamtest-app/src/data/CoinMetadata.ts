import {createAsync, query} from '@solidjs/router';
import {suiClientCache} from '../stores/suiClient';

export type CoinMetadataProps = {
  network: string;
  coinType: string;
};

export const getCoinMetadata = query(async (props: CoinMetadataProps) => {
  const client = suiClientCache.client(props.network);
  return await client.getCoinMetadata(props);
}, 'coinMetadata');

export const CoinMetadata = (props: CoinMetadataProps) => {
  return createAsync(() => getCoinMetadata(props));
};
