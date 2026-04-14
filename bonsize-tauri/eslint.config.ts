import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import stylistic from "@stylistic/eslint-plugin";
import pluginVue from "eslint-plugin-vue";
import css from "@eslint/css";
import { defineConfig } from "eslint/config";

export default defineConfig([
    {
        files: ["**/*.{js,mjs,cjs,ts,mts,cts,vue}"],
        plugins: { js, "@stylistic": stylistic },
        extends: ["js/recommended"],
        languageOptions: { globals: { ...globals.browser, ...globals.node } },
        rules: {
            "@stylistic/semi": ["error", "always"],
            "@stylistic/quotes": ["error", "double"],
            "eqeqeq": ["error", "always"],
            "@stylistic/comma-dangle": ["error", "always-multiline"],
            "no-console": "warn",
            "@stylistic/arrow-parens": ["error", "always"],
            "@stylistic/indent": ["error", 4],
            "no-unused-vars": "warn",
            "prefer-const": "error",
            "no-var": "error",
            "@stylistic/object-curly-spacing": ["error", "always"],
            "curly": ["error", "all"],
            "@stylistic/keyword-spacing": ["error", {
                before: true, after: true,
            }],
            "@stylistic/spaced-comment": ["error", "always"],
            "@stylistic/no-multiple-empty-lines": ["error", {
                max: 1,
            }],
            "@stylistic/max-len": ["error", 120],
            "arrow-body-style": ["error", "as-needed"],
            "@stylistic/jsx-quotes": ["error", "prefer-double"],
            "no-duplicate-imports": "error",
            "no-shadow": "error",
            "yoda": ["error", "never"],
            "@stylistic/eol-last": ["error", "always"],
        },
    },
    tseslint.configs.recommended,
    pluginVue.configs["flat/essential"],
    { files: ["**/*.vue"], languageOptions: { parserOptions: { parser: tseslint.parser } } },
    { files: ["**/*.css"], plugins: { css }, language: "css/css", extends: ["css/recommended"] },
]);
