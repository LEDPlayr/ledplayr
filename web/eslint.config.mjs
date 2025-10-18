import js from "@eslint/js";
import typescriptEslint from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import esLintConfigPrettier from "eslint-config-prettier";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import parser from "svelte-eslint-parser";
import ts from "typescript-eslint";

/** @type {import('eslint').Linter.Config[] } */
export default [
  {
    ignores: [
      "**/.DS_Store",
      "**/node_modules",
      "build",
      ".svelte-kit",
      "package",
      "**/.env",
      "**/.env.*",
      "!**/.env.example",
      "**/pnpm-lock.yaml",
      "**/package-lock.json",
      "**/yarn.lock",
      "src/lib/client/**",
    ],
  },

  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs.recommended,
  esLintConfigPrettier,

  {
    plugins: {
      "@typescript-eslint": typescriptEslint,
    },

    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },

      parser: tsParser,
      ecmaVersion: 2020,
      sourceType: "module",

      parserOptions: {
        extraFileExtensions: [".svelte"],
      },
    },

    rules: {
      "no-unused-vars": "off",

      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          argsIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          caughtErrorsIgnorePattern: "^_",
        },
      ],

      "svelte/no-at-html-tags": "warn",
    },
  },

  {
    files: [
      "**/*.svelte",
      "*.svelte",
      "**/*.svelte.js",
      "*.svelte.js",
      "**/*.svelte.ts",
      "*.svelte.ts",
      "src/lib/client/**",
    ],

    languageOptions: {
      parser: parser,
      ecmaVersion: 5,
      sourceType: "script",

      parserOptions: {
        parser: "@typescript-eslint/parser",
      },
    },
  },
];
