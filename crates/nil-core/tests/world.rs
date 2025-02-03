use nil_core::{Coord, World};

#[test]
fn cell() {
  each_coord(|world, coord| {
    assert!(world.cell(coord).is_ok());
  });
}

#[test]
fn index_to_coord() {
  each_coord(|world, coord| {
    let index = world.index(coord);
    assert_eq!(coord, world.coord(index).unwrap());
  });
}

#[test]
fn default_world_is_empty() {
  each_coord(|world, coord| {
    let cell = world.cell(coord).unwrap();
    assert!(cell.is_empty());
  });
}

fn each_coord(f: impl Fn(&mut World, Coord)) {
  let mut world = World::default();
  (0..100).into_iter().for_each(|x| {
    (0..100).into_iter().for_each(|y| {
      f(&mut world, Coord::new(x, y));
    });
  })
}
