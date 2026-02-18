-- Copyright (C) Call of Nil contributors
-- SPDX-License-Identifier: AGPL-3.0-only

local worlds = client:getRemoteWorlds();

for _, world in ipairs(worlds) do
  println(world.config.name)
  println("  id: ", world.config.id)
  println("  round: ", world.currentRound)
  println("  size: ", world.continentSize)
  println("  active players: ", world.activePlayers)
  println("  total players: ", world.totalPlayers)
  println("  created by: ", world.createdBy)
end
