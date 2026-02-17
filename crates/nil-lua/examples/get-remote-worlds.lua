--- Copyright (C) Call of Nil contributors
--- SPDX-License-Identifier: AGPL-3.0-only

local worlds = client:getRemoteWorlds();

for _, world in ipairs(worlds) do
  println(world.config.name, " created by ", world.createdBy)
end
