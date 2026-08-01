<!-- Copyright (C) Call of Nil contributors -->
<!-- SPDX-License-Identifier: AGPL-3.0-only -->

<script vapor lang="ts">
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import { useI18n } from "vue-i18n";
import { Button } from "@ui/button";
import * as commands from "@/commands";
import { useRouter } from "vue-router";
import type { Option } from "@tb-dev/utils";
import { computed, onBeforeMount } from "vue";
import { useSettings } from "@/stores/settings";
import { localRef, useMutex } from "@tb-dev/vue";
import enUS from "@/locale/en-US/scenes/online.json";
import ptBR from "@/locale/pt-BR/scenes/online.json";
import { isValidPassword, isValidPlayerId } from "@/lib/schema";
import ButtonSpinner from "@/components/button/ButtonSpinner.vue";
import { go, QUERY_SIGN_IN_USER, QUERY_SIGN_UP_USER } from "@/router";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@ui/card";

const { t } = useI18n({
  messages: {
    "en-US": enUS,
    "pt-BR": ptBR,
  },
});

const router = useRouter();
const settings = useSettings();

interface User {
  name: Option<string>;
  password: Option<string>;
}

const userName = localRef<User["name"]>(key("user.name"), null);
const userPassword = localRef<User["password"]>(key("user.password"), null);

const { locked, lock } = useMutex();
const canSignIn = computed(() => {
  return (
    isValidPlayerId(userName.value) &&
    isValidPassword(userPassword.value)
  );
});

onBeforeMount(() => {
  const url = new URL(window.location.href);
  userName.value = url.searchParams.get(QUERY_SIGN_IN_USER);
});

async function signIn() {
  await lock(async () => {
    if (
      isValidPlayerId(userName.value) &&
      isValidPassword(userPassword.value)
    ) {
      const token = await commands.authorize(userName.value, userPassword.value);
      await commands.updateClient({
        server: { kind: "remote" },
        playerId: userName.value,
        playerPassword: userPassword.value,
        authorizationToken: token,
      });

      settings.auth.token = token;
      await go("lobby");
    }
  });
}

async function goToSignUpScene() {
  await go("sign-up", { query: { [QUERY_SIGN_UP_USER]: userName.value } });
}

function key(name: string) {
  return `sign-in:${name}`;
}
</script>

<template>
  <div class="card-layout">
    <Card>
      <CardHeader>
        <CardTitle>{{ t("sign-in") }}</CardTitle>
      </CardHeader>

      <CardContent class="max-md:px-2">
        <Label>
          <span>{{ t("user") }}</span>
          <Input
            v-model.trim="userName"
            type="text"
            :disabled="locked"
            :minlength="1"
            :maxlength="20"
            @keydown.enter="signIn"
          />
        </Label>
        <Label>
          <span>{{ t("password") }}</span>
          <Input
            v-model="userPassword"
            type="password"
            :disabled="locked"
            :minlength="3"
            :maxlength="50"
            @keydown.enter="signIn"
          />
        </Label>
      </CardContent>

      <CardFooter class="grid grid-cols-3">
        <ButtonSpinner :loading="locked" :disabled="locked || !canSignIn" @click="signIn">
          {{ t("sign-in") }}
        </ButtonSpinner>

        <Button
          variant="secondary"
          :disabled="locked"
          role="link"
          tabindex="0"
          @click="goToSignUpScene"
        >
          <span>{{ t("sign-up") }}</span>
        </Button>

        <Button variant="secondary" :disabled="locked" @click="() => router.back()">
          <span>{{ t("cancel") }}</span>
        </Button>
      </CardFooter>
    </Card>
  </div>
</template>
