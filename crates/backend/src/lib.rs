use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::game::World;
use crate::malib::Vec2D;
use crate::player::Player;

pub mod player;
pub mod malib;
pub mod game;

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct Backend {
  world: World
}

#[wasm_bindgen]
impl Backend {
  pub fn new() -> Self {
    Self {
      world: World::new()
    }
  }

  pub fn render(&self) -> JsValue {
    serde_wasm_bindgen::to_value(&self).unwrap()
  }

  pub fn add_player(&mut self) {
    self.world.players.push(Player {
      position: Vec2D(0, 0)
    });
  }

  pub fn move_player(&mut self, id: usize, delta_js_raw: JsValue) {
    let delta: Vec2D = serde_wasm_bindgen::from_value(delta_js_raw).unwrap();

    let player = self.world.players.get_mut(id).unwrap();

    player.position += delta;
  }
}
