use super::Client;
use crate::error::Result;
use nil_core::{Coord, Village};

impl Client {
  pub async fn get_village(&self, coord: Coord) -> Result<Village> {
    self.post_json("village", coord).await
  }
}
