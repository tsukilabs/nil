// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface InfrastructureQueue<Order extends InfrastructureQueueOrder> {
  readonly orders: readonly Order[];
}

interface InfrastructureQueueOrder {
  readonly id: InfrastructureQueueOrderId;
  readonly resources: Resources;
  readonly workforce: number;
  readonly state: InfrastructureQueueOrderState;
}

type InfrastructureQueueOrderId = string;

type InfrastructureQueueOrderState =
  | InfrastructureQueueOrderStatePending
  | InfrastructureQueueOrderStateDone;

interface InfrastructureQueueOrderStatePending {
  readonly kind: 'pending';
  readonly workforce: number;
}

interface InfrastructureQueueOrderStateDone {
  readonly kind: 'done';
}
