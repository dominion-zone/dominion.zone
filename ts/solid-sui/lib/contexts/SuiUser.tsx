import {
  Component,
  createContext,
  ParentProps,
  Setter,
  useContext,
} from 'solid-js';

export type SuiUserContext = {
  value: string | null;
  set?: Setter<string | null>;
};

const SuiUserContext = createContext<SuiUserContext>();

export type SuiUserProviderProps = ParentProps<{
  value: string | null;
  set?: Setter<string | null>;
}>;

export const SuiUserProvider: Component<SuiUserProviderProps> = props => {
  return (
    <SuiUserContext.Provider
      value={{
        get value() {
          return props.value;
        },
        set: props.set,
      }}
    >
      {props.children}
    </SuiUserContext.Provider>
  );
};

export const useSuiUser = () => {
  const user = useContext(SuiUserContext);

  if (!user) {
    throw new Error('useSuiUser must be used within a SuiUserProvider');
  }

  return user;
};
