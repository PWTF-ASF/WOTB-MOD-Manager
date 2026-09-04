const skipFormatting = require('@vue/eslint-config-prettier/skip-formatting')
const typescriptEslint = require('@typescript-eslint/eslint-plugin')
const security = require('eslint-plugin-security')
const pluginVue = require('eslint-plugin-vue')

const typescriptParser = typescriptEslint.configs['flat/recommended'][0].languageOptions.parser

/** @type {import('eslint').Linter.Config[]} */
module.exports = [
  {
    name: 'app/files-to-ignore',
    ignores: [
      '**/dist/**',
      '**/dist-ssr/**',
      '**/coverage/**',
      '**/test-results/**',
      '**/src-tauri/target/**',
      '**/src-tauri/gen/**',
      '**/*.d.ts',
      'bump-version.cjs',
      '*.config.*',
    ],
  },

  ...typescriptEslint.configs['flat/recommended'],
  ...pluginVue.configs['flat/recommended'],

  {
    name: 'app/vue-typescript-parser',
    files: ['**/*.vue'],
    languageOptions: {
      parserOptions: {
        parser: typescriptParser,
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
    },
  },

  {
    name: 'app/rules',
    files: ['**/*.{ts,mts,tsx,vue}'],
    rules: {
      'no-var': 'error',
      'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
      'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
      'vue/multi-word-component-names': 'off',
      'comma-dangle': ['error', 'only-multiline'],
      'id-length': [2, { exceptions: ['i', 'j', 'e', 'z', '_'] }],
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          args: 'all',
          argsIgnorePattern: '^_',
          caughtErrors: 'all',
          caughtErrorsIgnorePattern: '^_',
          destructuredArrayIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          ignoreRestSiblings: true,
        },
      ],
    },
  },

  skipFormatting,

  security.configs.recommended,

  {
    name: 'app/test-rules',
    files: ['tests/**/*.{ts,tsx}'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
      'security/detect-object-injection': 'off',
    },
  },
]
