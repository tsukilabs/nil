// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export class ResourcesImpl implements Resources {
  public readonly food: number;
  public readonly iron: number;
  public readonly stone: number;
  public readonly wood: number;

  private constructor(resources: Resources) {
    this.food = resources.food;
    this.iron = resources.iron;
    this.stone = resources.stone;
    this.wood = resources.wood;
  }

  public add(value: number | PartialNullish<Resources>) {
    if (typeof value === 'number') {
      return ResourcesImpl.create({
        food: this.food + value,
        iron: this.iron + value,
        stone: this.stone + value,
        wood: this.wood + value,
      });
    }

    return ResourcesImpl.create({
      food: this.food + (value.food ?? 0),
      iron: this.iron + (value.iron ?? 0),
      stone: this.stone + (value.stone ?? 0),
      wood: this.wood + (value.wood ?? 0),
    });
  }

  public sub(value: number | PartialNullish<Resources>) {
    if (typeof value === 'number') {
      return ResourcesImpl.create({
        food: this.food - value,
        iron: this.iron - value,
        stone: this.stone - value,
        wood: this.wood - value,
      });
    }

    return ResourcesImpl.create({
      food: this.food - (value.food ?? 0),
      iron: this.iron - (value.iron ?? 0),
      stone: this.stone - (value.stone ?? 0),
      wood: this.wood - (value.wood ?? 0),
    });
  }

  public mul(value: number | PartialNullish<Resources>) {
    if (typeof value === 'number') {
      return ResourcesImpl.create({
        food: this.food * value,
        iron: this.iron * value,
        stone: this.stone * value,
        wood: this.wood * value,
      });
    }

    return ResourcesImpl.create({
      food: this.food * (value.food ?? 0),
      iron: this.iron * (value.iron ?? 0),
      stone: this.stone * (value.stone ?? 0),
      wood: this.wood * (value.wood ?? 0),
    });
  }

  public div(value: number | PartialNullish<Resources>) {
    if (typeof value === 'number') {
      return ResourcesImpl.create({
        food: this.food / value,
        iron: this.iron / value,
        stone: this.stone / value,
        wood: this.wood / value,
      });
    }

    return ResourcesImpl.create({
      food: this.food / (value.food ?? 0),
      iron: this.iron / (value.iron ?? 0),
      stone: this.stone / (value.stone ?? 0),
      wood: this.wood / (value.wood ?? 0),
    });
  }

  public has(resources: PartialNullish<Resources>) {
    return (
      this.food >= (resources.food ?? 0) &&
      this.iron >= (resources.iron ?? 0) &&
      this.stone >= (resources.stone ?? 0) &&
      this.wood >= (resources.wood ?? 0)
    );
  }

  public static create(resources?: PartialNullish<Resources>) {
    return new ResourcesImpl({
      food: resources?.food ?? 0,
      iron: resources?.iron ?? 0,
      stone: resources?.stone ?? 0,
      wood: resources?.wood ?? 0,
    });
  }

  public static zero() {
    return ResourcesImpl.create();
  }
}
