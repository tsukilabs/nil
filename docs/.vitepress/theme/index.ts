// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Theme } from 'vitepress';
import DefaultTheme from 'vitepress/theme';

const theme: Theme = {
  extends: DefaultTheme,
  enhanceApp() {},
};

export default theme;
