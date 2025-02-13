import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  ignores: ['**/components/base/*'],
  project: ['crates/nil/tsconfig.json'],

  features: {
    perfectionist: true,
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
    typescript: {
      '@typescript-eslint/consistent-type-definitions': ['error', 'type'],
    },
    vue: {
      'vue/v-on-handler-style': 'off',
    },
  },
});
