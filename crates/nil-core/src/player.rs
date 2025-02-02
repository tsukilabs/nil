use crate::village::Village;
use crate::world::Coord;
use std::collections::HashMap;
use std::net::SocketAddr;

pub struct Player {
  pub name: String,
  pub address: SocketAddr,
  pub villages: HashMap<Coord, Village>,
}
