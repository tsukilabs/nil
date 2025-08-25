import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: [
    'crates/nil-namegen/scripts/tsconfig.json',
    'crates/plugin-mobile/tsconfig.json',
    'docs/tsconfig.json',
    'registry/tsconfig.json',
    'registry/scripts/tsconfig.json',
    'ui/tsconfig.json',
  ],
  ignores: ['crates/nil-continent/**'],
  features: {
    perfectionist: true,
    unicorn: true,
    vue: true,
  },
  overrides: {
    typescript: {
      '@typescript-eslint/consistent-type-definitions': 'off',
      '@typescript-eslint/naming-convention': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-non-null-assertion': 'off',
      '@typescript-eslint/no-unsafe-call': 'off',
      '@typescript-eslint/no-unsafe-member-access': 'off',
    },
  },
});
