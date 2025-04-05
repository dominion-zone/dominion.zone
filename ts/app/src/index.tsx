/* @refresh reload */
import './index.css';

import {render, Suspense} from 'solid-js/web';

import App from './app';
import {Router} from '@solidjs/router';
import routes from '~solid-pages';
import {QueryClientProvider} from '@tanstack/solid-query';
import {query} from './data/client';

const root = document.getElementById('root');

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(
  () => (
    <QueryClientProvider client={query}>
      <Router root={App}>{routes}</Router>{' '}
    </QueryClientProvider>
  ),
  root,
);
