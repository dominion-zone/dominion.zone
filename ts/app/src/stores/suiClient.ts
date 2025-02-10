import {
  createSuiClientCache,
  DEFAULT_NETWORK_CONFIGS,
} from '@dominion.zone/solid-sui';
import {createStore} from 'solid-js/store';

const [configs, setConfigs] = createStore(DEFAULT_NETWORK_CONFIGS);

export const suiClientCache = createSuiClientCache({configs});
export {setConfigs};
