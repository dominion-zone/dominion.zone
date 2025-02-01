/* @refresh reload */
import './index.css';

import { render, Suspense } from 'solid-js/web';

import App from './app';
import { Router } from '@solidjs/router';
import routes from '~solid-pages';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

routes.find(route => {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const preload = (route.component as any)?.routePreload;
  if (preload) {
    route.preload = preload;
  }
});

render(
  () => <Router root={App}>{routes}</Router>,
  root,
);
