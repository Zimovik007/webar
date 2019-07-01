extern crate wasm_bindgen;
extern crate time;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use std::panic;

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
  return frame::create(inp, width, height).transform_to_black_and_white().get_result();
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

#[wasm_bindgen]
pub fn test_webgl() -> Result<(), JsValue>{
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvasElement").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();  
  let context = canvas.get_context("webgl").unwrap().unwrap().dyn_into::<web_sys::WebGlRenderingContext>().unwrap();

  let vert_shader = compile_shader(&context, WebGlRenderingContext::VERTEX_SHADER,
    r#"
      attribute vec4 position;
      void main() {
        gl_Position = position;
      }
    "#,
  )?;

  let frag_shader = compile_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER,
    r#"
      void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
      }
    "#,
  )?;

  let program = link_program(&context, &vert_shader, &frag_shader)?;
  context.use_program(Some(&program));

  let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

  let buffer = context.create_buffer().ok_or("failed to create buffer")?;
  context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

  unsafe {
    let vert_array = js_sys::Float32Array::view(&vertices);
    context.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vert_array, WebGlRenderingContext::STATIC_DRAW);
  }

  context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
  context.enable_vertex_attrib_array(0);

  context.clear_color(0.0, 0.0, 0.0, 1.0);
  context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

  context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (vertices.len() / 3) as i32);
  Ok(())
}

#[wasm_bindgen]
pub fn draw_smile() -> Result<(), JsValue> {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvasElement").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
  let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();
  context.begin_path();
  context.arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
  context.move_to(110.0, 75.0);
  context.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI).unwrap();
  context.move_to(65.0, 65.0);
  context.arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
  context.move_to(95.0, 65.0);
  context.arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
  context.stroke();
  Ok(())
}

pub fn compile_shader(context: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
  let shader = context.create_shader(shader_type).ok_or_else(|| String::from("Unable to create shader object"))?;

  context.shader_source(&shader, source);
  context.compile_shader(&shader);
  
  if context.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
    Ok(shader)
  } else {
    Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
  }
}

pub fn link_program(context: &WebGlRenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
  let program = context.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
    Ok(program)
  } else {
    Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object")))
  }
}