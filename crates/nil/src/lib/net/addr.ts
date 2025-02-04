import { isIp } from '../schema';
import type { Option } from '@tb-dev/utils';

export class SocketAddrV4 {
  private constructor(
    public readonly ip: string,
    public readonly port: number
  ) {}

  public format() {
    return `${this.ip}:${this.port}`;
  }

  public static parse(addr: string) {
    const array = addr.split(':');
    const port = parsePort(array[1]);

    if (isIp(array[0]) && port) {
      return new SocketAddrV4(array[0], port);
    }

    throw new Error(`invalid socket address: ${addr}`);
  }

  public static tryParse(addr: string) {
    try {
      return SocketAddrV4.parse(addr);
    } catch {
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

  return null;
}
