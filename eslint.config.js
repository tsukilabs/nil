import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  ignores: ['**/components/base/*'],
  project: ['crates/nil/tsconfig.json'],

  features: {
    perfectionist: true,
    react: false,
    svelte: false,
    tailwind: true,
    unicorn: true,
    vue: true,
  },

  overrides: {
    perfectionist: {
      'perfectionist/sort-objects': [
        'error',
        {
          order: 'asc',
          partitionByNewLine: true,
          type: 'natural',
        },
      ],

      '@typescript-eslint/sort-type-constituents': 'off',
      'perfectionist/sort-union-types': [
        'error',
        {
          order: 'asc',
          partitionByNewLine: true,
          type: 'natural',
        },
      ],
    },
    vue: {
      'vue/v-on-handler-style': 'off',
    },
  },
});
