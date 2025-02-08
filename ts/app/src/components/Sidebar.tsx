import {JSX} from 'solid-js';
import styles from '../styles/Sidebar.module.css';
import {A, useLocation} from '@solidjs/router';

export type SidebarProps = Omit<JSX.HTMLAttributes<HTMLDivElement>, 'class'>;

const Sidebar = (props: SidebarProps) => {
  const location = useLocation();

  return (
    <div {...props} class={styles.sidebar}>
      <ul class={styles.menu}>
        <li>
          <A href="/" activeClass={styles.disabled} end={true}>
            Home
          </A>
        </li>
        <li>
          <A href="/wallet-protector" activeClass={styles.disabled} end={true}>
            Wallet protector
          </A>
        </li>
        <li>
          <a href="https://scamtest.xyz" target="_blank">
            Test your wallet
          </a>
        </li>
        <li>
          <A href="/explorer" activeClass={styles.disabled} end={true}>
            AI blockchain explorer
          </A>
        </li>
        <li>
          <A href="/docs" activeClass={styles.disabled} end={true}>Docs</A>
        </li>
        <li>
          <A href="/roadmap" activeClass={styles.disabled} end={true}>Roadmap</A>
        </li>
        <li>
          <A href="/contacts" activeClass={styles.disabled} end={true}>Contacts</A>
        </li>
      </ul>
    </div>
  );
};

export default Sidebar;
