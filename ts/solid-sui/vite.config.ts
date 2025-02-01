import {defineConfig} from 'vite';
import solidPlugin from 'vite-plugin-solid';
import dts from 'vite-plugin-dts';

export default defineConfig({
  plugins: [solidPlugin(), dts({include: ['lib']})],
  server: {
    port: 3000,
  },
  build: {
    target: 'esnext',
    lib: {
      entry: 'lib/index.ts',
      formats: ['es'],
    },
    rollupOptions: {
      external: ['solid-js', '@mysten/sui', '@mysten/wallet-standard'],
    },
  },
});
