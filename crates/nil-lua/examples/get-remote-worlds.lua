-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

local worlds = client:getRemoteWorlds();

for _, world in ipairs(worlds) do
  print(world.config.name)
  print("  id: ", world.config.id)
  print("  created by: ", world.createdBy)
  print("  round: ", world.currentRound)
  print("  size: ", world.continentSize)
  print("  active players: ", world.activePlayers)
  print("  total players: ", world.totalPlayers)
  print("  world speed: ", world.config.speed)
  print("  unit speed: ", world.config.unitSpeed)
end
