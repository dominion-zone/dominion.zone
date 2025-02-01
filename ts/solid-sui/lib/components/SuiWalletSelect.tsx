import {
  Component,
  createMemo,
  JSX,
  mergeProps,
  Setter,
  Show,
  splitProps,
} from 'solid-js';
import {CheckIcon, ChevronsUpDown, ExternalLinkIcon} from 'lucide-solid';
import {For} from 'solid-js';
import {isWalletWithRequiredFeatureSet} from '@mysten/wallet-standard';
import {SuiWallet} from '../contexts/SuiWallet';
import {Wallet, wallets} from '../contexts/wallets';
import {
  DisclosureStateChild,
  Listbox,
  ListboxButton,
  ListboxLabel,
  ListboxOption,
  ListboxOptions,
  ListboxSingleProps,
  Transition,
} from 'terracotta';

export type SuiWalletSelectStyle = {
  root?: string;
  container?: string;
  button?: string;
  ['button-text']?: string;
  ['button-icon']?: string;
  ['transition--enter']?: string;
  ['transition--enter-from']?: string;
  ['transition--enter-to']?: string;
  ['transition--leave']?: string;
  ['transition--leave-from']?: string;
  ['transition--leave-to']?: string;
  options?: string;
  option?: string;
  ['option-active']?: string;
  ['option-content']?: string;
  ['option-text']?: string;
  ['option-text--selected']?: string;
  ['check-icon']?: string;
  ['link-icon']?: string;
  icon?: string;
};

export type SuiWalletSelectProps = Omit<
  ListboxSingleProps<string | null>,
  'value' | 'onSelectChange'
> & {
  style?: SuiWalletSelectStyle;
  preferredWallets?: {name: string; url: string}[];
  wallet: SuiWallet | null;
  setWallet: Setter<SuiWallet | null>;
  walletFilter?: (wallet: Wallet) => boolean;
};

export const SuiWalletSelect: Component<SuiWalletSelectProps> = props => {
  const defaultedProps = mergeProps(
    {
      get class() {
        return props.style?.root ?? 'listbox';
      },
      defaultOpen: false,
    },
    props,
  );
  const [myProps, listboxProps] = splitProps(defaultedProps, [
    'preferredWallets',
    'wallet',
    'setWallet',
    'walletFilter',
  ]);

  const collection = createMemo(() => {
    const checkedWallets = wallets.filter(
      (wallet): wallet is SuiWallet =>
        isWalletWithRequiredFeatureSet(wallet) &&
        wallet.chains.some(chain => chain.split(':')[0] === 'sui') &&
        (!myProps.walletFilter || myProps.walletFilter(wallet)),
    );

    const installed: SuiWallet[] = [
      // Preferred wallets, in order:
      ...((myProps.preferredWallets ?? [])
        .map(({name}) => checkedWallets.find(wallet => wallet.id === name))
        .filter(Boolean) as SuiWallet[]),

      // Wallets in default order:
      ...checkedWallets.filter(
        wallet =>
          !(myProps.preferredWallets ?? []).find(
            ({name}) => wallet.id === name,
          ),
      ),
    ];

    const nonInstalled = (myProps.preferredWallets ?? []).filter(({name}) =>
      wallets.every(wallet => wallet.id !== name),
    );

    return {installed, nonInstalled};
  });

  return (
    <Listbox
      value={props.wallet?.id ?? null}
      onSelectChange={w =>
        props.setWallet(
          (wallets as SuiWallet[]).find(({id}) => id === w) ?? null,
        )
      }
      {...listboxProps}
    >
      <div class={props.style?.container ?? 'listbox__container'}>
        <ListboxButton
          class={props.style?.button ?? 'listbox__button'}
          type="button"
        >
          <span class={props.style?.['button-text'] ?? 'listbox__button-text'}>
            {myProps.wallet?.id ?? '...'}
          </span>
          <span class={props.style?.['button-icon'] ?? 'listbox__button-icon'}>
            <ChevronsUpDown
              class={props.style?.icon ?? 'listbox__icon'}
              aria-hidden="true"
            />
          </span>
        </ListboxButton>
        <DisclosureStateChild>
          {({isOpen}): JSX.Element => (
            <Transition
              show={isOpen()}
              enter={
                props.style?.['transition--enter'] ??
                'listbox__transition--enter'
              }
              enterFrom={
                props.style?.['transition--enter-from'] ??
                'listbox__transition--enter-from'
              }
              enterTo={
                props.style?.['transition--enter-to'] ??
                'listbox__transition--enter-to'
              }
              leave={
                props.style?.['transition--leave'] ??
                'listbox__transition--leave'
              }
              leaveFrom={
                props.style?.['transition--leave-from'] ??
                'listbox__transition--leave-from'
              }
              leaveTo={
                props.style?.['transition--leave-to'] ??
                'listbox__transition--leave-to'
              }
            >
              <ListboxOptions
                unmount={false}
                class={props.style?.options ?? 'listbox__options'}
              >
                <For each={collection().installed}>
                  {(wallet): JSX.Element => (
                    <ListboxOption
                      class={props.style?.option ?? 'listbox__option'}
                      value={wallet.id}
                    >
                      {({isActive, isSelected}): JSX.Element => (
                        <div
                          classList={{
                            [props.style?.['option-content'] ??
                            'listbox__option-content']: true,
                            [props.style?.['option-active'] ??
                            'listbox__option--active']: isActive(),
                          }}
                        >
                          <span
                            classList={{
                              [props.style?.['option-text'] ??
                              'listbox__option-text']: true,
                              [props.style?.['option-text--selected'] ??
                              'listbox__option-text--selected']: isSelected(),
                            }}
                          >
                            {wallet.id}
                          </span>
                          <Show when={isSelected()}>
                            <span
                              class={
                                props.style?.['check-icon'] ??
                                'listbox__check-icon'
                              }
                            >
                              <CheckIcon
                                class={props.style?.icon ?? 'listbox__icon'}
                                aria-hidden="true"
                              />
                            </span>
                          </Show>
                        </div>
                      )}
                    </ListboxOption>
                  )}
                </For>
                <Show when={collection().nonInstalled.length > 0}>
                  <ListboxLabel>Install:</ListboxLabel>
                  <For each={collection().nonInstalled}>
                    {({name, url}) => (
                      <div
                        class={
                          props.style?.['option-content'] ??
                          'listbox__option-content'
                        }
                      >
                        <a
                          class={
                            props.style?.['option-text'] ??
                            'listbox__option-text'
                          }
                          href={url}
                          target="_blank"
                        >
                          {name}
                          <span
                            class={
                              props.style?.['link-icon'] ?? 'listbox__link-icon'
                            }
                          >
                            <ExternalLinkIcon />
                          </span>
                        </a>
                      </div>
                    )}
                  </For>
                </Show>
              </ListboxOptions>
            </Transition>
          )}
        </DisclosureStateChild>
      </div>
    </Listbox>
  );
};
