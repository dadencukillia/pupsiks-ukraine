import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { enhancedImages } from '@sveltejs/enhanced-img';
import { sveltekitOG } from '@ethercorps/sveltekit-og/plugin';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [
    tailwindcss(), 
    sveltekit(),
    enhancedImages(), 
    sveltekitOG({
      esmImport: false
    }),
  ]
});
