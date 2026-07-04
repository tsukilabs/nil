// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Theme } from "vitepress";
import DefaultTheme from "vitepress/theme";
import { setCurrentApp } from "@tb-dev/vue";

const theme: Theme = {
  extends: DefaultTheme,
  enhanceApp(ctx) {
    setCurrentApp(ctx.app);
  },
};

export default theme;
