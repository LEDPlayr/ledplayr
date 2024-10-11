import { purgeCss } from "vite-plugin-tailwind-purgecss";

import type { UserConfig } from "vite";

import { env } from "process";
import { sveltekit } from "@sveltejs/kit/vite";
import Icons from "unplugin-icons/vite";
import { defineConfig } from "vite";
import { kitRoutes } from "vite-plugin-kit-routes";

const cfg: UserConfig = {
  plugins: [
    sveltekit(),
    Icons({
      compiler: "svelte",
    }),
    kitRoutes({
      post_update_run: "pnpm exec prettier -w ./src/lib/ROUTES.ts",
    }),
    purgeCss(),
  ],
};

if (env["PROXY"]) {
  cfg["server"] = {
    proxy: {
      "/api": {
        target: env["PROXY"],
        changeOrigin: true,
      },
    },
  };
}

export default defineConfig(cfg);
