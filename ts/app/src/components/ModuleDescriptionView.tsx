import {Component, For, JSX, Show, splitProps, Suspense} from 'solid-js';
import {ModuleDescription} from '../data/ModuleDescription';
import {Tab, TabGroup, TabList, TabPanel} from 'terracotta';
import StructDescriptionView from './StructDescriptionView';

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
        <h2>Module {myProps.module}: {info()?.security_level}</h2>
        <p>{info()?.description}</p>
        <h2>Warnings:</h2>
        <ul>
          <For each={info()?.warnings}>{warning => <li>⚠ {warning}</li>}</For>
        </ul>
        <Show when={info()?.structs.length > 0}>
          <h2>Structs:</h2>
          <div class="tabs">
            <TabGroup
              defaultValue={info()?.structs?.[0]?.struct_name}
              horizontal={false}
              class="tabs__container"
            >
              {({isSelected, isActive}): JSX.Element => (
                <>
                  <TabList class="tabs__list">
                    <For each={info()?.structs}>
                      {(struct): JSX.Element => (
                        <Tab
                          value={struct.struct_name}
                          classList={{
                            tabs__tab: true,
                            'tabs__tab--selected': isSelected(
                              struct.struct_name,
                            ),
                          }}
                        >
                          {struct.struct_name}
                        </Tab>
                      )}
                    </For>
                  </TabList>
                  <div class="tabs__content">
                    <For each={info()?.structs}>
                      {(struct): JSX.Element => (
                        <TabPanel
                          value={struct.struct_name}
                          classList={{
                            tabs__panel: true,
                          }}
                        >
                          <StructDescriptionView struct={struct} />
                        </TabPanel>
                      )}
                    </For>
                  </div>
                </>
              )}
            </TabGroup>
          </div>
        </Show>
      </section>
    </Suspense>
  );
};

export default ModuleDescriptionView;
