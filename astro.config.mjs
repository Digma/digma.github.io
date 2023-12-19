import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import sitemap from "@astrojs/sitemap";
import tailwind from "@astrojs/tailwind";

import { ViteRsw } from 'vite-plugin-rsw';


// https://astro.build/config
export default defineConfig({
  site: "https://gael.io",
  integrations: [mdx(), sitemap(), tailwind()],
  vite: {
    plugins: [ViteRsw()],
  },
});
