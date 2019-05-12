extern crate imageproc;
extern crate image;

pub struct Hessian_pyramid {
  num_octaves: u32,
  num_intervals: u32,
  initial_step_size: u32,
}

impl Hessian_pyramid {
  pub fn build_pyramid(mut self, img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    
  }
}

pub fn new(octaves: u32, intervals: u32, step: u32) -> Hessian_pyramid {
  Hessian_pyramid {
    num_octaves: octaves,
    num_intervals: intervals,
    initial_step_size: step,
  }
}