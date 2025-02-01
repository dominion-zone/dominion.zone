import {createAsync, query} from '@solidjs/router';
import {suiClientCache} from '../stores/suiClient';

export type SuiObjectProps = {
  network: string;
  id: string;
};

export const getSuiObject = query(async (props: SuiObjectProps) => {
  const client = suiClientCache.client(props.network);
  return await client.getObject({...props, options: {showContent: true}});
}, 'object');

export const SuiObject = (props: SuiObjectProps) => {
  return createAsync(() => getSuiObject(props));
};
