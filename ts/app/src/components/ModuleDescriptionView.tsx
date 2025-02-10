import {Component, For, JSX, Show, splitProps, Suspense} from 'solid-js';
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
        <h3>Structs:</h3>
        <ul>
          <For each={info()?.structs}>{struct => <li><h4>{struct.struct_name}</h4>
          <ul>
            <li>Description: {struct.description}</li>
            <Show when={struct.address_owned}><li>Address Owned: {struct.address_owned}</li></Show>
            <Show when={struct.object_owned}><li>Object Owned: {struct.object_owned}</li></Show>
            <Show when={struct.wrapped}><li>Wrapped: {struct.wrapped}</li></Show>
            <Show when={struct.shared}><li>Shared: {struct.shared}</li></Show>
            <Show when={struct.immutable}><li>Immutable: {struct.immutable}</li></Show>
            <Show when={struct.event}><li>Event: {struct.event}</li></Show>
            <h5>Warnings:</h5>
            <ul>
              <For each={struct.warnings}>{warning => <li>⚠ {warning}</li>}</For>
              </ul>
          </ul>
          </li>}</For>
        </ul>
      </section>
    </Suspense>
  );
};

export default ModuleDescriptionView;
