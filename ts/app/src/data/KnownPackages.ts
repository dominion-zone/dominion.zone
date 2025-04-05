import {queryOptions, createQuery} from '@tanstack/solid-query';
import axios from 'axios';
import {query} from './client';

export type KnownPackagesProps = {
  network: string;
};

export const knownPackagesQueryKey = ({network}: KnownPackagesProps) => [
  'knownPackages',
  network,
];

export const getKnownPackages = async (props: KnownPackagesProps) => {
  const r = await axios.get(
    `https://api.dominion.zone/${props.network}/known_packages`,
  );
  return r.data as string[];
};

export const knownPackagesQueryOptions = (props: KnownPackagesProps) =>
  queryOptions({
    queryKey: knownPackagesQueryKey(props),
    queryFn: () => getKnownPackages(props),
  });

export const knownPackagesQuery = (props: KnownPackagesProps) =>
  createQuery(
    () => knownPackagesQueryOptions(props),
    () => query,
  );
