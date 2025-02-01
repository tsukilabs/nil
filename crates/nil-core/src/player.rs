use crate::village::Village;
use std::net::SocketAddr;

pub struct Player {
  name: String,
  address: SocketAddr,
  villages: Vec<Village>,
}
