import {createAsync, query} from '@solidjs/router';
import axios from 'axios';

export type StructInfo = {
    network: string,
    struct_name: string,
    description: string,
    address_owned?: string,
    object_owned?: string,
    wrapped?: string,
    shared?: string,
    immutable?: string,
    event?: string,
    warnings: string[]
};

export type ModuleInfo = {
  packageId: string;
  network: string;
  module: string;
  description: string;
  security_level: string;
  warnings: string[];
  structs: StructInfo[];
};

export type ModuleDescriptionProps = {
  packageId: string;
  network: string;
  module: string;
};

export const getModuleDescription = query(
  async (props: ModuleDescriptionProps) => {
    const r = await axios.get(
      `https://api.dominion.zone/${props.network}/module/${props.packageId}::${props.module}`,
    );
    return {...props, ...r.data} as ModuleInfo;
  },
  'knownPackages',
);

export const ModuleDescription = (props: ModuleDescriptionProps) => {
  return createAsync(() => getModuleDescription(props));
};
