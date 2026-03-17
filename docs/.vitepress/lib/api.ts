// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

export const BASE_URL = 'https://tsukilabs.dev.br/nil';

export function url(endpoint: string) {
  return `${BASE_URL}/${endpoint}`;
}

export async function get(endpoint: string, init?: RequestInit) {
  const response = await fetch(url(endpoint), {
    ...init,
    method: 'GET',
  });

  if (response.ok) {
    return response;
  }
  else {
    throw new Error(await response.text());
  }
}

export async function post(endpoint: string, body: unknown, init?: RequestInit) {
  const response = await fetch(url(endpoint), {
    ...init,
    method: 'POST',
    body: JSON.stringify(body, null, 0),
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (response.ok) {
    return response;
  }
  else {
    throw new Error(await response.text());
  }
}
