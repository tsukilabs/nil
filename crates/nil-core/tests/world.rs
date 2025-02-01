use nil_core::{Coord, World};
use rayon::prelude::*;

#[test]
pub fn world_cells() {
  let world = World::new(100);
  (0..100).into_par_iter().for_each(|x| {
    (0..100).into_par_iter().for_each(|y| {
      let coord = Coord::new(x, y);
      assert!(world.get(coord).is_ok());
    });
  });
}
