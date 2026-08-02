// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export type Scene =
  | GameScene
  | "about"
  | "home"
  | "host-local-game"
  | "host-remote-game"
  | "join-local-game"
  | "join-remote-game"
  | "load-local-game"
  | "lobby"
  | "settings"
  | "sign-in"
  | "sign-up";

export type GameScene =
  | ContinentScene
  | InfrastructureScene
  | MarketScene
  | ProfileScene
  | ReportScene
  | WarRoomScene
  | "chat"
  | "city"
  | "continent"
  | "maneuver"
  | "own-cities"
  | "ranking";

export type ContinentScene = "continent" | "continent-cities";

export type MarketScene = "market" | "market-send";

export type ProfileScene = "profile-bot" | "profile-city" | "profile-player" | "profile-precursor";

export type ReportScene = "report" | "report-forward" | "report-view";

export type WarRoomScene = "war-room" | "war-room-simulator";

export type InfrastructureScene =
  | AcademyScene
  | PrefectureScene
  | StableScene
  | WorkshopScene
  | "farm"
  | "iron-mine"
  | "quarry"
  | "sawmill"
  | "silo"
  | "wall"
  | "warehouse";

export type AcademyScene = "academy" | "academy-settings";

export type PrefectureScene = "prefecture" | "prefecture-settings";

export type StableScene = "stable" | "stable-settings";

export type WorkshopScene = "workshop" | "workshop-settings";
