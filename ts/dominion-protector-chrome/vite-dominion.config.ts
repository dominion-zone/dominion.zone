import { defineConfig } from 'vite';
// @ts-ignore
import packageJson from './package.json';

import { resolve } from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, 'src/scripts/content/dominion.ts'),
      fileName: 'dominion',
      name: packageJson.name,
      formats: ['es'],
    },
    outDir: 'public',
    emptyOutDir: false,
    rollupOptions: {
      output: {
        extend: true,
      },
    },
  },
  plugins: [],
});
