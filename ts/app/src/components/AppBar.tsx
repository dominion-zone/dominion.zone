import styles from '../styles/AppBar.module.css';
import { Show } from 'solid-js';

const AppBar = () => {
  return (
    <header class={styles.header}>
      <div class={styles.headerContainer}>
        <a href='/' class={styles.logo}>
          <img class={styles.logoIcon} src='./dominion.png'/>
          <div class={styles.titleContainer}><div class={styles.title}>Dominion</div><div class={styles.subtitle}>protector</div></div>
        </a>
        {/*
        <div class={styles.menuToggle} onclick={toggleMenu}>
          <SquareMenu />
        </div>
        */}
        <nav
          classList={{
            [styles.navControls]: true,
            // [styles.active]: menuOpen(),
          }}
        >
        </nav>
      </div>
    </header>
  );
};

export default AppBar;
