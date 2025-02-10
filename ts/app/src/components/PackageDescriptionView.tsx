import {useSuiClient} from '@dominion.zone/solid-sui';
import {
  Component,
  createEffect,
  createMemo,
  For,
  JSX,
  Show,
  splitProps,
} from 'solid-js';
import {SuiObject} from '../data/SuiObject';
import {Tab, TabGroup, TabList, TabPanel} from 'terracotta';
import ModuleDescriptionView from './ModuleDescriptionView';
import { formatAddress } from '@mysten/sui/utils';

export type PackageDescriptionProps = JSX.HTMLAttributes<HTMLElement> & {
  packageId: string;
  network: string;
};

const PackageDescriptionView: Component<PackageDescriptionProps> = props => {
  const [myProps, sectionProps] = splitProps(props, ['network', 'packageId']);

  createEffect(() => {
    console.log('PackageDescription', myProps.network, myProps.packageId);
  });
  const object = SuiObject({
    get network() {
      return myProps.network;
    },
    get id() {
      return myProps.packageId;
    },
  });
  const modules = createMemo(() => {
    const moduleMap = object()?.data?.bcs?.['moduleMap'];
    return moduleMap ? Object.keys(moduleMap) : [];
  });

  return (
    <Show when={object()?.data?.bcs?.dataType === 'package'}>
      <section class="card" {...sectionProps}>
        <h2>
          Package: <a href={`https://suiscan.xyz/${myProps.network}/object/${myProps.packageId}`}>{formatAddress(myProps.packageId)}</a> ({myProps.network})
        </h2>

        <div class="tabs">
          <TabGroup
            defaultValue={modules()[0]}
            horizontal={false}
            class="tabs__container"
          >
            {({isSelected, isActive}): JSX.Element => (
              <>
                <TabList class="tabs__list">
                  <For each={modules()}>
                    {(module): JSX.Element => (
                      <Tab
                        value={module}
                        classList={{
                          tabs__tab: true,
                          'tabs__tab--selected': isSelected(module),
                        }}
                      >
                        {module}
                      </Tab>
                    )}
                  </For>
                </TabList>
                <div class="tabs__content">
                  <For each={modules()}>
                    {(module): JSX.Element => (
                      <TabPanel
                        value={module}
                        classList={{
                          tabs__panel: true,
                        }}
                      >
                        <ModuleDescriptionView
                          packageId={myProps.packageId}
                          network={myProps.network}
                          module={module}/>
                      </TabPanel>
                    )}
                  </For>
                </div>
              </>
            )}
          </TabGroup>
        </div>
      </section>
    </Show>
  );
};

export default PackageDescriptionView;
