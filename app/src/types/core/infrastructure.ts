// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { BuildingLevel, Resources } from '@tsukilabs/nil-bindings';

export interface Building {
  readonly enabled: boolean;
  readonly level: BuildingLevel;
}

export interface InfrastructureQueue<Order extends InfrastructureQueueOrder> {
  readonly orders: Order[];
}

export interface InfrastructureQueueOrder {
  readonly id: InfrastructureQueueOrderId;
  readonly resources: Resources;
  readonly workforce: number;
  readonly state: InfrastructureQueueOrderState;
}

export type InfrastructureQueueOrderId = string;

export type InfrastructureQueueOrderState =
  | InfrastructureQueueOrderStatePending
  | InfrastructureQueueOrderStateDone;

export interface InfrastructureQueueOrderStatePending {
  readonly kind: 'pending';
  readonly workforce: number;
}

export interface InfrastructureQueueOrderStateDone {
  readonly kind: 'done';
}
