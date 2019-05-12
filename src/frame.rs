extern crate imageproc;
extern crate image;

use imageproc::integral_image::{integral_image, sum_image_pixels};

pub struct Frame {
  frame: Vec<u8>,
  image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
  surf_features: Vec<Point>,
}

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

  pub fn surf(&self) {
    let integral_image = integral_image::<_, u32>(&self.image_buffer);
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