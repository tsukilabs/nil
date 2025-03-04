/* eslint-disable @typescript-eslint/no-explicit-any */
import * as commands from '@/commands';
import { asyncRef, type AsyncRefOptions } from './async-ref';

type Command = typeof commands;

export type CommandOptions = AsyncRefOptions;

export function command<T extends keyof Command, U>(name: T, initial: U, options?: CommandOptions) {
  type Value = Awaited<ReturnType<Command[T]>> | U;
  return asyncRef<Value>(initial, commands[name] as any, options);
}
