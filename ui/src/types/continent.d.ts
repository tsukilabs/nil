// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

type PublicField = PublicFieldEmpty | PublicFieldVillage;

type PublicFieldKind = PublicField['kind'];

interface PublicFieldEmpty {
  readonly kind: 'empty';
}

interface PublicFieldVillage {
  readonly kind: 'village';
  readonly village: Village;
}
