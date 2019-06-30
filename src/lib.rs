extern crate wasm_bindgen;
extern crate time;

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
pub fn transform_to_black_and_white(inp: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
  // let start = time::PreciseTime::now();
  let frame: Vec<u8> = frame::create(inp, width, height).transform_to_black_and_white().get_result();
  // log(&format!("{}", start.to(time::PreciseTime::now()))[..]);
  return frame;
}

#[wasm_bindgen]
pub fn transform_to_gray(inp: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
  return frame::create(inp, width, height).transform_to_gray().get_result();
}

#[wasm_bindgen]
pub fn get_surf_features(inp: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
  let frame: frame::Frame = frame::create(inp, width, height);
  frame.surf();
  // log(&format!("{:?}", frame)[..]);
  println!("{:?}", frame);
  return frame.get_result();
}

#[wasm_bindgen]
pub fn canny(inp: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
  return frame::create(inp, width, height).canny().get_result();
}