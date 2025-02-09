import {createAsync, query} from '@solidjs/router';
import axios from 'axios';

export type KnownPackagesProps = {
  network: string;
};

export const getKnownPackages = query(async (props: KnownPackagesProps) => {
  const r = await axios.get('http://api.dominion.zone/devnet/known_packages');
  return r.data as string[];
}, 'knownPackages');

export const KnownPackages = (props: KnownPackagesProps) => {
  return createAsync(() => getKnownPackages(props));
};
