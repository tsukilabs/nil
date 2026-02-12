// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { watch } from 'vue';
import { compare } from '@/lib/intl';
import { asyncRef } from '@tb-dev/vue';
import { getBulkDistance } from '@/commands/continent';
import { CoordImpl } from '@/core/model/continent/coord';
import { PublicCityImpl, type PublicCityImplConstructorArgs } from '@/core/model/city/public-city';

class ContinentCity extends PublicCityImpl {
  public distance: number;

  constructor(args: ContinentCityConstructorArgs) {
    super(args);
    this.distance = args.distance;
  }
}

export function useContinentCities() {
  const { coord: origin } = NIL.city.refs();

  const { state, loading, load } = asyncRef<readonly ContinentCity[]>([], async () => {
    const impl = await PublicCityImpl.loadAll();
    const cities = impl.map((it) => {
      return new ContinentCity({ city: it, score: it.score, distance: 0 });
    });

    if (origin.value) {
      const destinations = cities.map((it) => it.coord);
      if (destinations.length > 0) {
        for (const [coord, distance] of await getBulkDistance(origin.value, destinations)) {
          const city = cities.find((it) => it.coord.is(coord));
          if (city) {
            city.distance = distance;
          }
        }
      }
    }

    return cities.sort((a, b) => {
      if (a.distance === b.distance) {
        return compare(a.name, b.name);
      }
      else {
        return a.distance - b.distance;
      }
    });
  });

  watch(origin, load);

  function getDistance(key: ContinentKey): Option<number> {
    const coord = CoordImpl.fromContinentKey(key);
    return state.value.find((city) => city.coord.is(coord))?.distance;
  }

  return {
    cities: state,
    loading,
    load,
    getDistance,
  };
}

interface ContinentCityConstructorArgs extends PublicCityImplConstructorArgs {
  readonly distance: number;
}
