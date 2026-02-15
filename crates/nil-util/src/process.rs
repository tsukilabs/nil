// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use anyhow::Result;
use std::ffi::OsStr;
use std::process::Command;

pub fn command<Args, Arg, Env, Key, Value>(program: &str, args: Args, env: Option<Env>) -> Command
where
  Args: IntoIterator<Item = Arg>,
  Arg: AsRef<OsStr>,
  Env: IntoIterator<Item = (Key, Value)>,
  Key: AsRef<OsStr>,
  Value: AsRef<OsStr>,
{
  let mut command = if cfg!(windows) { Command::new("pwsh") } else { Command::new(program) };

  if cfg!(windows) {
    command
      .args(["-Command", program])
      .args(args);
  } else {
    command.args(args);
  }

  if let Some(envs) = env {
    command.envs(envs);
  }

  command
}

pub fn output<Args, Arg, Env, Key, Value>(
  program: &str,
  args: Args,
  env: Option<Env>,
) -> Result<Vec<u8>>
where
  Args: IntoIterator<Item = Arg>,
  Arg: AsRef<OsStr>,
  Env: IntoIterator<Item = (Key, Value)>,
  Key: AsRef<OsStr>,
  Value: AsRef<OsStr>,
{
  command(program, args, env)
    .output()?
    .exit_ok()
    .map(|it| it.stdout)
    .map_err(Into::into)
}

pub fn spawn<Args, Arg, Env, Key, Value>(program: &str, args: Args, env: Option<Env>) -> Result<()>
where
  Args: IntoIterator<Item = Arg>,
  Arg: AsRef<OsStr>,
  Env: IntoIterator<Item = (Key, Value)>,
  Key: AsRef<OsStr>,
  Value: AsRef<OsStr>,
{
  command(program, args, env)
    .status()?
    .exit_ok()
    .map_err(Into::into)
}

#[doc(hidden)]
#[macro_export]
macro_rules! into_command_parts {
  ($command:expr) => {{
    let env: [(&str, &str); 0] = [];
    $crate::command!($command, env)
  }};
  ($command:expr, $env:expr) => {{
    let mut iter = $command
      .trim()
      .split_whitespace()
      .map(|it| it.trim())
      .filter(|it| !it.is_empty());

    let Some(program) = iter.next() else {
      anyhow::bail!("Missing program");
    };

    (program, iter)
  }};
}

#[macro_export]
macro_rules! output {
  ($command:expr) => {{
    let env: [(&str, &str); 0] = [];
    $crate::output!($command, env)
  }};
  ($command:expr, $env:expr) => {{
    let (program, args) = $crate::into_command_parts!($command, $env);
    $crate::process::output(program, args, Some($env))
  }};
}

#[macro_export]
macro_rules! output_fmt {
  ($($arg:tt)*) => {{
    let command = format!($($arg)*);
    $crate::output!(command)
  }};
}

#[macro_export]
macro_rules! spawn {
  ($command:expr) => {{
    let env: [(&str, &str); 0] = [];
    $crate::spawn!($command, env)
  }};
  ($command:expr, $env:expr) => {{
    let (program, args) = $crate::into_command_parts!($command, $env);
    $crate::process::spawn(program, args, Some($env))
  }};
}

#[macro_export]
macro_rules! spawn_fmt {
  ($($arg:tt)*) => {{
    let command = format!($($arg)*);
    $crate::spawn!(command)
  }};
}
