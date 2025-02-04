use nil_core::{Battle, Squad};

#[test]
fn offensive_power() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(1, 100), squad(2, 50)])
      .defender([])
      .build()
  };

  let power = battle.offensive_power();
  assert_eq!(power.total, 2250.0);
  assert_eq!(power.infantry_ratio, 1.0);
}

#[test]
fn offensive_power_cavalry() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(6, 100)])
      .defender([])
      .build()
  };

  let power = battle.offensive_power();
  assert_eq!(power.total, 15000.0);
  assert_eq!(power.cavalry_ratio, 1.0);
}

#[test]
fn offensive_power_mixed() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(6, 100), squad(1, 500)])
      .defender([])
      .build()
  };

  let power = battle.offensive_power();
  assert_eq!(power.total, 20000.0);
  assert_eq!(power.cavalry_ratio, 0.75);
  assert_eq!(power.infantry_ratio, 0.25);
  assert_eq!(power.ranged_ratio, 0.0);
}

#[test]
fn defensive_power() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(3, 100), squad(2, 50)])
      .defender([squad(1, 100), squad(2, 50)])
      .build()
  };

  let power = battle.defensive_power();
  assert_eq!(power.total, 4000.0);
}

#[test]
fn defensive_power_mixed() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(6, 100), squad(1, 500)])
      .defender([squad(1, 100)])
      .build()
  };

  let power = battle.defensive_power();
  assert_eq!(power.total, 3750.0);
}

#[test]
fn winner_losses() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(3, 100), squad(2, 50)])
      .defender([squad(1, 100)])
      .build()
  };

  let losses = battle.winner_losses();
  assert!((losses.total_loss - 22.908).abs() <= 0.001);
  assert_eq!(losses.cavalry_losses, 0.0);
}

#[test]
fn winner_losses_mixed() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(6, 100), squad(1, 500)])
      .defender([squad(1, 100)])
      .build()
  };

  let losses = battle.winner_losses();
  assert!((losses.total_loss - 48.7139).abs() <= 0.001);
}

#[test]
fn overall_test() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(5, 3000), squad(3, 6000)])
      .defender([squad(1, 8000), squad(2, 8000)])
      .build()
  };

  let attack_power = battle.offensive_power();
  let defense_power = battle.defensive_power();
  let winner_losses = battle.winner_losses();
  assert_eq!(attack_power.total, 630000.0);
  assert!((defense_power.total - 495238.0952380952).abs() <= 0.001);
  assert!((winner_losses.total_loss - 6272.674503).abs() <= 0.001);
  assert!((winner_losses.infantry - 4181.783).abs() <= 0.001);
}

#[test]
fn ranged_debuff_test() {
  let battle = unsafe {
    Battle::builder()
      .attacker([squad(4, 3000), squad(3, 7000)])
      .defender([squad(1, 8000), squad(2, 8000)])
      .build()
  };

  let attack_power = battle.offensive_power();
  assert_eq!(attack_power.total, 370000.0);
}

/// # Safety
///
/// My brother in Christ, please use a valid id when calling this function.
unsafe fn squad(id: u32, amount: u32) -> Squad {
  unsafe { Squad::new_unchecked(id, amount) }
}
