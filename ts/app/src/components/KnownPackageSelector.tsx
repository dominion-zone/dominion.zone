import {
  Component,
  createEffect,
  For,
  JSX,
  mergeProps,
  Setter,
  Show,
  splitProps,
} from 'solid-js';

import {
  DisclosureStateChild,
  Listbox,
  ListboxButton,
  ListboxOption,
  ListboxOptions,
  ListboxProps,
  ListboxSingleProps,
  Transition,
} from 'terracotta';
import {KnownPackages} from '../data/KnownPackages';
import {CheckIcon, ChevronsUpDown, ExternalLinkIcon} from 'lucide-solid';

export type KnownPackageSelectorProps = Omit<
  ListboxSingleProps<string | null>,
  'value' | 'onSelectChange'
> & {
  network: string;
  packageId: string | null;
  setPackageId: Setter<string>;
};

const KnownPackageSelector: Component<KnownPackageSelectorProps> = props => {
  const defaultedProps = mergeProps(
    {
      defaultOpen: false,
    },
    props,
  );

  const [myProps, listboxProps] = splitProps(defaultedProps, [
    'network',
    'packageId',
    'setPackageId',
  ]);

  const packages = KnownPackages({
    get network() {
      return myProps.network;
    },
  });

  createEffect((network) => {
    if (network !== myProps.network) {
      myProps.setPackageId(null);
    }
    return myProps.network;
  }, myProps.network);

  return (
    <Listbox
      value={props.packageId}
      onSelectChange={(w: string) => props.setPackageId(w)}
      {...listboxProps}
    >
      <div class="listbox__container">
        <ListboxButton class="listbox__button">
          <input
            type="text"
            class="listbox__button-text"
            value={myProps.packageId ?? ''}
            onChange={e => myProps.setPackageId(e.currentTarget.value)}
          />
          <span class="listbox__button-icon">
            <ChevronsUpDown class="listbox__icon" aria-hidden="true" />
          </span>
        </ListboxButton>
        <DisclosureStateChild>
          {({isOpen}): JSX.Element => (
            <Transition
              show={isOpen()}
              enter="listbox__transition--enter"
              enterFrom="listbox__transition--enter-from"
              enterTo="listbox__transition--enter-to"
              leave="listbox__transition--leave"
              leaveFrom="listbox__transition--leave-from"
              leaveTo="listbox__transition--leave-to"
            >
              <ListboxOptions unmount={false} class="listbox__options">
                <For each={packages()}>
                  {(packageId): JSX.Element => (
                    <ListboxOption class="listbox__option" value={packageId}>
                      {({isActive, isSelected}): JSX.Element => (
                        <div
                          classList={{
                            'listbox__option-content': true,
                            'listbox__option--active': isActive(),
                          }}
                        >
                          <span
                            classList={{
                              'listbox__option-text': true,
                              'listbox__option-text--selected': isSelected(),
                            }}
                          >
                            {packageId}
                          </span>
                          <Show when={isSelected()}>
                            <span class="listbox__check-icon">
                              <CheckIcon
                                class="listbox__icon"
                                aria-hidden="true"
                              />
                            </span>
                          </Show>
                        </div>
                      )}
                    </ListboxOption>
                  )}
                </For>
              </ListboxOptions>
            </Transition>
          )}
        </DisclosureStateChild>
      </div>
    </Listbox>
  );
};

export default KnownPackageSelector;
