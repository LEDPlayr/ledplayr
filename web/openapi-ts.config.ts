import { defaultPlugins } from "@hey-api/openapi-ts";

export default {
  client: "@hey-api/client-fetch",
  input: "../openapi.json",
  output: "src/lib/client",
  plugins: [
    ...defaultPlugins,
    {
      enums: "javascript",
      name: "@hey-api/typescript",
    },
  ],
};
