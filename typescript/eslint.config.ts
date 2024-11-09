import functional from "eslint-plugin-functional";
import tseslint from "@typescript-eslint/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import { Linter } from "eslint";

const config: Linter.FlatConfig[] = [
  {
    files: ["**/*.ts"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        projectService: true,
      },
    },
    plugins: {
      "@typescript-eslint": tseslint,
      functional,
    },
    rules: {
      ...tseslint.configs.recommended.rules,
      ...functional.configs.externalTypeScriptRecommended.rules,
      ...functional.configs.recommended.rules,
      ...functional.configs.stylistic.rules,
      "@typescript-eslint/prefer-readonly": "error",
      // 他のルール
    },
  },
];

export default config;
