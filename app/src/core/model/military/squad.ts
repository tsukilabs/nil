// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { toU32 } from '@/lib/number';

export class SquadImpl implements Squad {
  public readonly unit: UnitId;
  public readonly size: number;

  private constructor(squad: Squad) {
    this.unit = squad.unit;
    this.size = squad.size;
  }

  public isEmpty() {
    return SquadImpl.isEmpty(this);
  }

  public normalize() {
    return SquadImpl.normalize(this);
  }

  public add(rhs: Squad) {
    return SquadImpl.add(this, rhs);
  }

  public sub(rhs: Squad) {
    return SquadImpl.sub(this, rhs);
  }

  public static create(squad: Squad) {
    if (squad instanceof SquadImpl) {
      return squad;
    }

    return new SquadImpl(squad);
  }

  public static createEmpty(unit: UnitId) {
    return SquadImpl.create(SquadImpl.createEmptyRaw(unit));
  }

  public static createEmptyRaw(unit: UnitId): Squad {
    return { unit, size: 0 };
  }

  public static isEmpty(squad: Squad) {
    return !Number.isFinite(squad.size) || squad.size <= 0;
  }

  public static normalize(squad: Squad) {
    return SquadImpl.create({
      unit: squad.unit,
      size: toU32(squad.size),
    });
  }

  public static add(lhs: Squad, rhs: Squad) {
    if (lhs.unit === rhs.unit) {
      const size = lhs.size + rhs.size;
      return SquadImpl.create({ unit: lhs.unit, size });
    }
    else {
      return lhs;
    }
  }

  public static sub(lhs: Squad, rhs: Squad) {
    if (lhs.unit === rhs.unit) {
      const size = lhs.size - rhs.size;
      return SquadImpl.create({ unit: lhs.unit, size });
    }
    else {
      return lhs;
    }
  }
}
