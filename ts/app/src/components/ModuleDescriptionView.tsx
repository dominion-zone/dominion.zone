import {Component, For, JSX, splitProps, Suspense} from 'solid-js';
import {ModuleDescription} from '../data/ModuleDescription';

export type ModuleDescriptionProps = JSX.HTMLAttributes<HTMLElement> & {
  packageId: string;
  network: string;
  module: string;
};

const ModuleDescriptionView: Component<ModuleDescriptionProps> = props => {
  const [myProps, sectionProps] = splitProps(props, [
    'network',
    'packageId',
    'module',
  ]);
  const info = ModuleDescription({
    get packageId() {
      return myProps.packageId;
    },
    get network() {
      return myProps.network;
    },
    get module() {
      return myProps.module;
    },
  });
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <section class="card" {...sectionProps}>
        <h2>Module: {myProps.module}</h2>
        <p>{info()?.description}</p>
        <h3>Security Level: {info()?.security_level}</h3>
        <h3>Warnings:</h3>
        <ul>
          <For each={info()?.warnings}>{warning => <li>⚠ {warning}</li>}</For>
        </ul>
      </section>
    </Suspense>
  );
};

export default ModuleDescriptionView;
