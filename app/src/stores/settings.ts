// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { ref } from 'vue';
import { defineStore } from 'pinia';
import { AuthSettingsImpl } from '@/settings/auth';
import { StableSettingsImpl } from '@/settings/stable';
import { AcademySettingsImpl } from '@/settings/academy';
import { GeneralSettingsImpl } from '@/settings/general';
import { WorkshopSettingsImpl } from '@/settings/workshop';
import { AppearanceSettingsImpl } from '@/settings/appearance';
import { PrefectureSettingsImpl } from '@/settings/prefecture';

export const useSettings = defineStore('settings', () => {
  const academy = ref(new AcademySettingsImpl());
  const appearance = ref(new AppearanceSettingsImpl());
  const auth = ref(new AuthSettingsImpl());
  const general = ref(new GeneralSettingsImpl());
  const prefecture = ref(new PrefectureSettingsImpl());
  const stable = ref(new StableSettingsImpl());
  const workshop = ref(new WorkshopSettingsImpl());

  return {
    academy,
    appearance,
    auth,
    general,
    prefecture,
    stable,
    workshop,
  };
});
