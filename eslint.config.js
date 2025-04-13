/* eslint-disable perfectionist/sort-objects */
import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['crates/nil/tsconfig.json'],
  ignores: ['**/components/base/*'],
  features: {
    perfectionist: true,
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
      '@typescript-eslint/no-non-null-assertion': 'off',
    },
  },
});
