import { defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
  input: "../openapi.json",
  output: {
    clean: true,
    lint: "eslint",
    format: "prettier",
    path: "src/lib/client",
  },
  plugins: [
    "@hey-api/client-fetch",
    "@hey-api/sdk",
    {
      enums: "javascript",
      name: "@hey-api/typescript",
    },
  ],
});
