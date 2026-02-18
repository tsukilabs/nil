// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::client::ClientUserData;
use mlua::{LuaSerdeExt, UserDataMethods, Value};
use nil_payload::cheat::infrastructure::*;

pub fn add_methods<M: UserDataMethods<ClientUserData>>(methods: &mut M) {
  methods.add_async_method(
    "cheatGetAcademyRecruitQueue",
    async |lua, this, req: Value| {
      let req: CheatGetAcademyRecruitQueueRequest = lua.from_value(req)?;
      this
        .client(async |it| it.cheat_get_academy_recruit_queue(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "cheatGetAcademyRecruitQueues",
    async |lua, this, req: Value| {
      let req: CheatGetAcademyRecruitQueuesRequest = lua.from_value(req)?;
      this
        .client(async |it| {
          it.cheat_get_academy_recruit_queues(req)
            .await
        })
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "cheatGetAllAcademyRecruitQueues",
    async |lua, this, req: Value| {
      let req: CheatGetAllAcademyRecruitQueuesRequest = lua.from_value(req)?;
      this
        .client(async |it| {
          it.cheat_get_all_academy_recruit_queues(req)
            .await
        })
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "cheatGetAllPrefectureBuildQueues",
    async |lua, this, req: Value| {
      let req: CheatGetAllPrefectureBuildQueuesRequest = lua.from_value(req)?;
      this
        .client(async |it| {
          it.cheat_get_all_prefecture_build_queues(req)
            .await
        })
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "cheatGetAllStableRecruitQueues",
    async |lua, this, req: Value| {
      let req: CheatGetAllStableRecruitQueuesRequest = lua.from_value(req)?;
      this
        .client(async |it| {
          it.cheat_get_all_stable_recruit_queues(req)
            .await
        })
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method("cheatGetInfrastructure", async |lua, this, req: Value| {
    let req: CheatGetInfrastructureRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_get_infrastructure(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method(
    "cheatGetPrefectureBuildQueue",
    async |lua, this, req: Value| {
      let req: CheatGetPrefectureBuildQueueRequest = lua.from_value(req)?;
      this
        .client(async |it| {
          it.cheat_get_prefecture_build_queue(req)
            .await
        })
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "cheatGetPrefectureBuildQueues",
    async |lua, this, req: Value| {
      let req: CheatGetPrefectureBuildQueuesRequest = lua.from_value(req)?;
      this
        .client(async |it| {
          it.cheat_get_prefecture_build_queues(req)
            .await
        })
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "cheatGetStableRecruitQueue",
    async |lua, this, req: Value| {
      let req: CheatGetStableRecruitQueueRequest = lua.from_value(req)?;
      this
        .client(async |it| it.cheat_get_stable_recruit_queue(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method(
    "cheatGetStableRecruitQueues",
    async |lua, this, req: Value| {
      let req: CheatGetStableRecruitQueuesRequest = lua.from_value(req)?;
      this
        .client(async |it| it.cheat_get_stable_recruit_queues(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );

  methods.add_async_method("cheatGetStorageCapacity", async |lua, this, req: Value| {
    let req: CheatGetStorageCapacityRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_get_storage_capacity(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method("cheatSetBuildingLevel", async |lua, this, req: Value| {
    let req: CheatSetBuildingLevelRequest = lua.from_value(req)?;
    this
      .client(async |it| it.cheat_set_building_level(req).await)
      .await
      .map(|it| lua.to_value(&it))?
  });

  methods.add_async_method(
    "cheatSetMaxInfrastructure",
    async |lua, this, req: Value| {
      let req: CheatSetMaxInfrastructureRequest = lua.from_value(req)?;
      this
        .client(async |it| it.cheat_set_max_infrastructure(req).await)
        .await
        .map(|it| lua.to_value(&it))?
    },
  );
}
