import globals from "globals";
import pluginJs from "@eslint/js";
import { defineConfig } from "eslint/config";
import pluginTs from "typescript-eslint";
import pluginReact from "eslint-plugin-react";

export default defineConfig([
  {
    files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"],
    languageOptions: { globals: globals.browser },
    rules: {
      "sort-keys": [
        "warn",
        "asc",
        { allowLineSeparatedGroups: true, caseSensitive: false, natural: true },
      ],
    },
  },
  pluginJs.configs.recommended,
  ...pluginTs.configs.recommended,
  pluginReact.configs.flat["jsx-runtime"],
]);
