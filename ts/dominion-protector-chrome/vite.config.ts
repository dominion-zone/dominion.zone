import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import Pages from 'vite-plugin-pages';
import { crx } from '@crxjs/vite-plugin'
import manifest from './manifest.json'
import typedCssModulesPlugin from "vite-plugin-typed-css-modules";

export default defineConfig({
  plugins: [
    Pages({
      dirs: ['src/pages'],
    }),
    solidPlugin(),
    typedCssModulesPlugin(),
    crx({ manifest }),
  ],
  server: {
    port: 3000,
    cors: true,
  },
  build: {
    target: 'esnext',
  },
  legacy: {
    skipWebSocketTokenCheck: true
  }
});
