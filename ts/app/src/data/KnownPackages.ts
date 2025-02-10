import {createAsync, query} from '@solidjs/router';
import axios from 'axios';

export type KnownPackagesProps = {
  network: string;
};

export const getKnownPackages = query(async (props: KnownPackagesProps) => {
  const r = await axios.get(`https://api.dominion.zone/${props.network}/known_packages`);
  return r.data as string[];
}, 'knownPackages');

export const KnownPackages = (props: KnownPackagesProps) => {
  return createAsync(() => getKnownPackages(props));
};
