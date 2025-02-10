import {For, JSX, Show, splitProps} from 'solid-js';
import {StructInfo} from '../data/ModuleDescription';

export type ModuleDescriptionProps = JSX.HTMLAttributes<HTMLElement> & {
  struct: StructInfo;
};

const StructDescriptionView = props => {
  const [myProps, sectionProps] = splitProps(props, ['struct']);
  return (
    <>
      <section class="card" {...sectionProps}>
        <h2>Description:</h2>
        <div>{myProps.struct.description}</div>
      </section>
      <section class="card" {...sectionProps}>
        <h2>Ownership:</h2>
        <ul>
          <Show when={myProps.struct.address_owned}>
            <li>Address Owned: {myProps.struct.address_owned}</li>
          </Show>
          <Show when={myProps.struct.object_owned}>
            <li>Object Owned: {myProps.struct.object_owned}</li>
          </Show>
          <Show when={myProps.struct.wrapped}>
            <li>Wrapped: {myProps.struct.wrapped}</li>
          </Show>
          <Show when={myProps.struct.shared}>
            <li>Shared: {myProps.struct.shared}</li>
          </Show>
          <Show when={myProps.struct.immutable}>
            <li>Immutable: {myProps.struct.immutable}</li>
          </Show>
          <Show when={myProps.struct.event}>
            <li>Event: {myProps.struct.event}</li>
          </Show>
        </ul>
      </section>
      <section class="card" {...sectionProps}>
        <h2>Warnings:</h2>
        <ul>
          <For each={myProps.struct.warnings}>
            {warning => <li>⚠ {warning}</li>}
          </For>
        </ul>
      </section>
    </>
  );
};

export default StructDescriptionView;
