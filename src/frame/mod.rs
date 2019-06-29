extern crate imageproc;
extern crate image;

mod hessian;

use imageproc::integral_image::{integral_image, sum_image_pixels};

pub const RGB2_YUM: &'static [f32; 3] = &[0.299, 0.587, 0.114];

#[derive(Debug)]
pub struct Frame {
  frame: Vec<u8>,
  image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
  surf_features: Vec<Point>,
}

#[derive(Debug)]
pub struct Point {
  x: u32,
  y: u32,
}

impl Frame {

  pub fn transform_to_black_and_white(mut self) -> Frame {
    let l: usize = self.frame.len() / 4;
    for i in 0..l {
      let gray: u32 = (self.frame[i * 4 + 0] as u32 + self.frame[i * 4 + 1] as u32 + self.frame[i * 4 + 2] as u32) / 3;
      self.frame[i * 4 + 0] = gray as u8;
      self.frame[i * 4 + 1] = gray as u8;
      self.frame[i * 4 + 2] = gray as u8;
    }

    self
  }

  pub fn transform_to_gray(mut self) -> Frame {
    let l: usize = self.frame.len() / 4;
    for i in 0..l {
      let gray: f32 = self.frame[i * 4 + 0] as f32 * RGB2_YUM[0] + self.frame[i * 4 + 1] as f32 * RGB2_YUM[1] + self.frame[i * 4 + 2] as f32 * RGB2_YUM[2];
      self.frame[i * 4 + 0] = gray as u8;
      self.frame[i * 4 + 1] = gray as u8;
      self.frame[i * 4 + 2] = gray as u8;
    }

    self
  }

  pub fn surf(&self) {
    let integral_image = integral_image::<_, u8>(&self.image_buffer);
    let mut h_pyramid: hessian::HessianPyramid = hessian::new(4, 6, 2);
    h_pyramid.build_pyramid(integral_image);
  }

  pub fn get_result(self) -> Vec<u8> {
    return self.frame;
  }
}

pub fn create(f: Vec<u8>, width: u32, height: u32) -> Frame {
  return Frame {
    frame: f.to_vec(),
    image_buffer: image::ImageBuffer::from_vec(width, height, f).unwrap(),
    surf_features: Vec::new(),
  };
}