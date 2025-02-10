import {createAsync, query} from '@solidjs/router';
import {suiClientCache} from '../stores/suiClient';

export type SuiObjectProps = {
  network: string;
  id: string;
};

export const getSuiObject = query(async (props: SuiObjectProps) => {
  if (!props.network || !props.id) {
    return null;
  }
  const client = suiClientCache.client(props.network);
  const r = await client.getObject({...props, options: {showBcs: true}});
  console.log('getObject', r);
  return r;
}, 'object');

export const SuiObject = (props: SuiObjectProps) => {
  return createAsync(() => getSuiObject(props));
};
