import * as options from './options';

export function isPlayerOptions(value: unknown): value is PlayerOptions {
  return options.player.safeParse(value).success;
}

export function isWorldOptions(value: unknown): value is WorldOptions {
  return options.world.safeParse(value).success;
}
