use std::ops::{Add, AddAssign, Mul, Sub};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Vec2D(pub i32, pub i32);

impl AddAssign for Vec2D {
  fn add_assign(&mut self, rhs: Self) {
    self.0 += rhs.0;
    self.1 += rhs.1;
  }
}
