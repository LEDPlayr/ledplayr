import type { UserConfig } from "vite";

import { env } from "process";
import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import Icons from "unplugin-icons/vite";
import { defineConfig } from "vite";

const cfg: UserConfig = {
  css: {
    preprocessorOptions: {
      scss: {},
    },
  },
  plugins: [
    sveltekit(),
    Icons({
      compiler: "svelte",
    }),
    tailwindcss(),
  ],
};

if (env["PROXY"]) {
  cfg["server"] = {
    proxy: {
      "/api": {
        target: env["PROXY"],
        changeOrigin: true,
      },
      "/rapidoc": {
        target: env["PROXY"],
        changeOrigin: true,
      },
    },
  };
}

export default defineConfig(cfg);
