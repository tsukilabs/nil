// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { Entity } from "./abstract";
import { compare } from "@/lib/intl";
import { CityImpl } from "@/core/model/city/city";
import type { PlayerPayload } from "@/types/event";
import { type Option, panic } from "@tb-dev/utils";
import { ref, type Ref, type ShallowRef } from "vue";
import { asyncComputed, asyncRef } from "@tb-dev/vue";
import { PlayerImpl } from "@/core/model/player/player";
import type { PlayerId } from "@tsukilabs/nil-bindings";

export class PlayerEntity extends Entity {
  private readonly id = ref<Option<PlayerId>>();
  private readonly player: ShallowRef<Option<PlayerImpl>>;
  private readonly cities: Readonly<ShallowRef<readonly CityImpl[]>>;

  public readonly updatePlayer: () => Promise<void>;

  constructor() {
    super();

    const player = usePlayer(this.id);
    this.player = player.state;
    this.updatePlayer = player.load;

    this.cities = useCities(this.player);

    this.initListeners();
  }

  protected override initListeners() {
    this.event.onPlayer(this.onPlayer.bind(this));
  }

  public override async update() {
    await this.updatePlayer();
  }

  private async onPlayer({ player }: PlayerPayload) {
    if (player === this.id.value) {
      await this.update();
    }
  }

  public static use() {
    return super.get(PlayerEntity) as PlayerEntity;
  }

  public static refs() {
    const instance = this.use();
    return {
      id: instance.id as Readonly<typeof instance.id>,
      cities: instance.cities,
      player: instance.player as Readonly<typeof instance.player>,
    } as const;
  }

  public static update() {
    return this.use().update();
  }

  public static getId() {
    return this.use().id.value;
  }

  public static getIdStrict() {
    return this.getId() ?? panic("Missing player id");
  }

  public static async setId(id?: Option<PlayerId>) {
    const player = this.use();
    player.id.value = id;
    await player.update();
  }

  public static getPlayer() {
    return this.use().player.value ?? null;
  }

  public static getRuler() {
    return this.getPlayer()?.toRuler() ?? null;
  }

  public static getCoords() {
    return this.getPlayer()?.coords ?? [];
  }

  public static getCities() {
    return this.use().cities.value;
  }

  public static init() {
    if (!Object.hasOwn(globalThis.NIL, "player")) {
      const player: (typeof globalThis.NIL)["player"] = {
        getCities: PlayerEntity.getCities.bind(PlayerEntity),
        getCoords: PlayerEntity.getCoords.bind(PlayerEntity),
        getId: PlayerEntity.getId.bind(PlayerEntity),
        getIdStrict: PlayerEntity.getIdStrict.bind(PlayerEntity),
        getPlayer: PlayerEntity.getPlayer.bind(PlayerEntity),
        getRuler: PlayerEntity.getRuler.bind(PlayerEntity),
        refs: PlayerEntity.refs.bind(PlayerEntity),
        setId: PlayerEntity.setId.bind(PlayerEntity),
        update: PlayerEntity.update.bind(PlayerEntity),
        use: PlayerEntity.use.bind(PlayerEntity),
      };

      Object.defineProperty(globalThis.NIL, "player", {
        configurable: false,
        enumerable: true,
        writable: false,
        value: player,
      });
    }
  }
}

function usePlayer(id: Ref<Option<PlayerId>>) {
  return asyncRef(null, async () => {
    return id.value ? PlayerImpl.load(id.value) : null;
  });
}

function useCities(player: ShallowRef<Option<PlayerImpl>>) {
  return asyncComputed([], async () => {
    if (player.value && player.value.coords.length > 0) {
      const cities = await CityImpl.loadAll();
      cities.sort((a, b) => compare(a.name, b.name));
      return cities;
    }
    else {
      return [];
    }
  });
}
