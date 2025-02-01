use nil_core::Battle;
use nil_unit::new_unchecked as unit;

#[test]
fn offensive_power() {
  let battle = unsafe {
    Battle::builder()
      .attacker([unit(1, 100), unit(2, 50)])
      .defender([])
      .build()
  };

  let power = battle.offensive_power();
  assert_eq!(power.total, 2250.0);
  assert_eq!(power.general_ratio, 1.0);
}

#[test]
fn offensive_power_cavalry() {
  let battle = unsafe {
    Battle::builder()
      .attacker([unit(6, 100)])
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
      .attacker([unit(6, 100), unit(1, 500)])
      .defender([])
      .build()
  };

  let power = battle.offensive_power();
  assert_eq!(power.total, 20000.0);
  assert_eq!(power.cavalry_ratio, 0.75);
  assert_eq!(power.general_ratio, 0.25);
  assert_eq!(power.ranged_ratio, 0.0);
}

#[test]
fn defensive_power() {
  let battle = unsafe {
    Battle::builder()
      .attacker([unit(3, 100), unit(2, 50)])
      .defender([unit(1, 100), unit(2, 50)])
      .build()
  };

  let power = battle.defensive_power();
  assert_eq!(power.total, 4000.0);
}

#[test]
fn defensive_power_mixed() {
  let battle = unsafe {
    Battle::builder()
      .attacker([unit(6, 100), unit(1, 500)])
      .defender([unit(1, 100)])
      .build()
  };

  let power = battle.defensive_power();
  assert_eq!(power.total, 3750.0);
}
