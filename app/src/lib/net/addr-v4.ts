// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';

type IpV4 = readonly [number, number, number, number];

export class IpAddrV4 {
  private constructor(public readonly ip: IpV4) {}

  public format() {
    return this.ip.join('.');
  }

  public static local() {
    return new this([127, 0, 0, 1]);
  }

  public static parse(ip: string) {
    const value = ip.split('.').map((n) => Number.parseInt(n));
    if (value.length === 4 && value.every((n) => Number.isInteger(n) && n >= 0 && n <= 255)) {
      return new this(value as [number, number, number, number]);
    }

    throw new Error(`Invalid ipv4 address: ${ip}`);
  }

  public static tryParse(ip: string) {
    try {
      return IpAddrV4.parse(ip);
    }
    catch {
      return null;
    }
  }
}

export class SocketAddrV4 {
  private constructor(
    public readonly ip: IpAddrV4,
    public readonly port: number,
  ) {}

  public format() {
    return `${this.ip.format()}:${this.port}`;
  }

  public asLocal() {
    return new SocketAddrV4(IpAddrV4.local(), this.port);
  }

  public static parse(addr: string) {
    const array = addr.split(':');
    const ip = IpAddrV4.parse(array.at(0) ?? '');
    const port = parsePort(array.at(1));
    return new SocketAddrV4(ip, port);
  }

  public static tryParse(addr: Option<string>) {
    if (!addr) return null;
    try {
      return SocketAddrV4.parse(addr);
    }
    catch {
      return null;
    }
  }
}

function parsePort(port: Option<string>) {
  if (port) {
    const value = Number.parseInt(port, 10);
    if (Number.isInteger(value) && value > 0 && value <= 65535) {
      return value;
    }
  }

  throw new Error(`Invalid port: ${port}`);
}
