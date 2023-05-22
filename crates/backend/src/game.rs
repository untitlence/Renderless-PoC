use serde::{Deserialize, Serialize};
use crate::player::Player;

#[derive(Debug, Deserialize, Serialize)]
pub struct World {
  pub players: Vec<Player>
}

impl World {
  pub fn new() -> Self {
    World {
      players: Vec::new()
    }
  }
}
