extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod frame;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub fn transform_to_black_and_white(inp: Vec<u8>) -> Vec<u8> {
  return frame::create(inp).transform_to_black_and_white().get_result();
}

#[wasm_bindgen]
pub fn get_surf_features(inp: Vec<u8>) -> Vec<u8> {
  let mut frame: frame::Frame = frame::create(inp);
  frame.surf();
  return frame.get_result();
}