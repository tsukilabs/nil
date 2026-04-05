import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: [
    'tsconfig.json',
    'app/tsconfig.json',
    'docs/tsconfig.json',
  ],
  ignores: ['**/components/ui/*'],
  features: {
    perfectionist: true,
    unicorn: true,
    vue: true,
  },
  overrides: {
    typescript: {
      'consistent-type-definitions': 'off',
      'naming-convention': 'off',
      'no-explicit-any': 'off',
      'no-non-null-assertion': 'off',
      'no-unnecessary-type-arguments': 'off',
      'no-unsafe-call': 'off',
      'no-unsafe-member-access': 'off',
      'no-unused-private-class-members': 'off',
    },
    vue: {
      'enforce-style-attribute': ['error', { allow: ['module', 'scoped'] }],
    },
  },
});
