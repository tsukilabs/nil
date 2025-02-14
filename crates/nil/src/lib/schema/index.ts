import * as base from './base';
import * as options from './options';

export function isIp(value: unknown) {
  return base.ip.safeParse(value).success;
}

export function isPlayerOptions(value: unknown) {
  return options.player.safeParse(value).success;
}

export function isWorldOptions(value: unknown) {
  return options.world.safeParse(value).success;
}
