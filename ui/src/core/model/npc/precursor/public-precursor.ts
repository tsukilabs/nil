// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import * as commands from '@/commands';
import { CoordImpl } from '../../continent/coord';

export class PublicPrecursorImpl implements PublicPrecursor {
  public readonly id: PrecursorId;
  public readonly origin: Coord;
  public readonly coords: readonly CoordImpl[] = [];

  protected constructor(args: PublicPrecursorImplConstructorArgs) {
    this.id = args.precursor.id;
    this.origin = args.precursor.origin;
    this.coords = args.coords.map((it) => CoordImpl.create(it));
  }

  public hasVillage(key: ContinentKey) {
    return this.coords.some((it) => it.is(key));
  }

  public getVillage(key: ContinentKey) {
    return this.coords.find((it) => it.is(key));
  }

  public static create(args: PublicPrecursorImplConstructorArgs) {
    if (args.precursor instanceof PublicPrecursorImpl) {
      return args.precursor;
    }

    return new PublicPrecursorImpl(args);
  }

  public static async load(id: PrecursorId) {
    const [precursor, coords] = await Promise.all([
      commands.getPublicPrecursor(id),
      commands.getPrecursorCoords(id),
    ]);

    return PublicPrecursorImpl.create({ precursor, coords });
  }
}

export interface PublicPrecursorImplConstructorArgs {
  precursor: PublicPrecursor;
  coords: readonly Coord[];
}
