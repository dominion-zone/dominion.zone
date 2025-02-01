import {
  Component,
  createContext,
  ParentProps,
  Setter,
  useContext,
} from 'solid-js';
import {Wallet} from './wallets';
import {
  IdentifierString,
  MinimallyRequiredFeatures,
  StandardDisconnectFeature,
  SuiFeatures,
} from '@mysten/wallet-standard';

export type SuiWallet = Wallet<
  MinimallyRequiredFeatures &
    Partial<SuiFeatures> &
    Partial<StandardDisconnectFeature> &
    Record<IdentifierString, unknown>
>;

export type SuiWalletContext = {
  value: SuiWallet | null;
  set?: Setter<SuiWallet | null>;
};

const SuiWalletContext = createContext<SuiWalletContext>();

export type SuiWalletProviderProps = ParentProps<{
  value: SuiWallet | null;
  set?: Setter<SuiWallet | null>;
}>;

export const SuiWalletProvider: Component<SuiWalletProviderProps> = props => {
  return (
    <SuiWalletContext.Provider
      value={{
        get value() {
          return props.value;
        },
        set: props.set,
      }}
    >
      {props.children}
    </SuiWalletContext.Provider>
  );
};

export const useSuiWallet = (): SuiWalletContext => {
  const wallet = useContext(SuiWalletContext);

  if (!wallet) {
    throw new Error('useSuiWallet must be used within a SuiWalletProvider');
  }

  return wallet;
};
