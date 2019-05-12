extern crate imageproc;
extern crate image;

pub struct HessianPyramid {
  num_octaves: u64,
  num_intervals: u64,
  initial_step_size: u64,
}

impl HessianPyramid {
  pub fn build_pyramid(&mut self, img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {

    let mut pyramid: Vec<Vec<Vec<f64>>> = Vec::new();
    // resize pyramid
    pyramid.resize(self.num_intervals as usize * self.num_octaves as usize, Vec::new());
    for i in 0..self.num_octaves {
      let step = self.get_step_size(i);
      for j in 0..self.num_intervals as usize {
        pyramid[self.num_intervals as usize * i as usize + j] = vec![vec![0.0; (img.width() as u64 / step) as usize]; (img.height() as u64 / step) as usize];
      }
    }

    

  }

  fn get_step_size(&self, octave: u64) -> u64 {
    return (self.initial_step_size as f64 * (2.0f64.powf(octave as f64) + 0.5)) as u64;
  }
}

pub fn new(octaves: u64, intervals: u64, step: u64) -> HessianPyramid {
  HessianPyramid {
    num_octaves: octaves,
    num_intervals: intervals,
    initial_step_size: step,
  }
}