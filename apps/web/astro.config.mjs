// @ts-check
import { defineConfig, svgoOptimizer } from 'astro/config';

import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  vite: {
    plugins: [tailwindcss()]
  },
  experimental: {
    svgOptimizer: svgoOptimizer()
  }
});
