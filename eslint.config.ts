import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['tsconfig.json', 'app/tsconfig.json'],
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
      '@typescript-eslint/no-unused-private-class-members': 'off',
    },
  },
});
