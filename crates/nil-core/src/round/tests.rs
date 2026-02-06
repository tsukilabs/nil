// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use super::Round;
use crate::error::{Error, Result};
use crate::player::PlayerId;
use std::assert_matches;

#[test]
fn lifecycle() -> Result<()> {
  let mut round = Round::default();
  let players = vec![
    PlayerId::from("Player A"),
    PlayerId::from("Player B"),
    PlayerId::from("Player C"),
  ];

  assert_eq!(round.id(), 1u32);
  assert!(round.is_idle());

  assert_matches!(round.next([]), Err(Error::RoundNotStarted));

  round.start(players.clone())?;
  assert!(round.is_waiting());

  assert_matches!(round.start([]), Err(Error::RoundAlreadyStarted));

  round.set_ready(&players[0], true);

  for player in &players {
    assert_eq!(round.is_player_ready(player), player == &players[0]);
    assert_eq!(round.is_player_pending(player), player != &players[0]);
    assert!(round.is_waiting_player(player));
  }

  round.set_ready(&players[0], false);

  for player in &players {
    assert!(!round.is_player_ready(player));
    assert!(round.is_player_pending(player));
    assert!(round.is_waiting_player(player));
  }

  assert_matches!(round.next([]), Err(Error::RoundHasPendingPlayers));

  for player in &players {
    round.set_ready(player, true);
  }

  assert!(round.is_done());
  round.next(players.clone())?;

  assert_eq!(round.id(), 2u32);
  assert!(round.is_waiting());

  for player in &players {
    assert!(!round.is_player_ready(player));
    assert!(round.is_player_pending(player));
    assert!(round.is_waiting_player(player));
  }

  Ok(())
}
