use serde::{Deserialize, Serialize};
use crate::malib::Vec2D;

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
  pub position: Vec2D
}
