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
  width: u32,
  height: u32,
}

#[derive(Debug)]
pub struct Point {
  x: u32,
  y: u32,
}

impl Frame {

  fn gauss_func(x: i32, y: i32, _omega: f64) -> f64 {
    let e: f64 = 1.0_f64.exp();
    1.0_f64 / (2.0_f64 * std::f64::consts::PI * _omega) * e.powf((0 - x.pow(2) - y.pow(2)) as f64 / (2.0_f64 * _omega.powf(2.0)))
  }

  fn get_gauss_kernel(_size: u32, _omega: f64) -> Vec<Vec<f64>> {
    let mut m: Vec<Vec<f64>> = vec![vec![0_f64; _size as usize]; _size as usize];
    let shift: i32 = (_size / 2) as i32;
    for i in 0.._size {
      for j in 0.._size {
        m[i as usize][j as usize] = Frame::gauss_func(i as i32 - shift, j as i32 - shift, _omega);
      }
    }
    m
  }

  pub fn gauss_filter(mut self, _size: u32, _omega: f64) -> Frame {
    let gauss_kernel: Vec<Vec<f64>> = Frame::get_gauss_kernel(_size, _omega);
    let shift: u32 = (_size / 2) as u32;
    
    let mut gauss_frame: Vec<u8> = self.frame.to_vec();
    let mut index: u32;

    for i in shift..(self.height - shift) {
      for j in shift..(self.width - shift) {

        let mut val: f64 = 0_f64;
        for k in (i - shift)..(i + shift) {
          for l in (j - shift)..(j + shift) {
            index = k * self.width + l;
            val += self.frame[(index * 4) as usize] as f64 * gauss_kernel[(k - (i - shift)) as usize][(l - (j - shift)) as usize];
          }
        }

        index = i * self.width + j;
        gauss_frame[(index * 4) as usize] = val as u8;
        gauss_frame[(index * 4 + 1) as usize] = val as u8;
        gauss_frame[(index * 4 + 2) as usize] = val as u8;

      }
    }
    
    self.frame = gauss_frame;
    self
  }

  pub fn sobel_operator(mut self) -> Frame {
    let m_g_x: Vec<Vec<i32>> = vec![vec![1, 0, -1], vec![2, 0, -2], vec![1, 0, -1]];
    let m_g_y: Vec<Vec<i32>> = vec![vec![1, 2, 1], vec![0, 0, 0], vec![-1, -2, -1]];

    let mut sobel_matrix: Vec<u8> = self.frame.to_vec();
    let mut index: u32;

    for i in 1..self.height - 2 {
      for j in 1..self.width - 2 {

        let mut g_x: i32 = 0;
        let mut g_y: i32 = 0;

        for k in i - 1..i + 1 {
          for l in j - 1..j + 1 {
            index = k * self.width + l;
            g_x += m_g_x[(k - (i - 1)) as usize][(l - (j - 1)) as usize] * self.frame[(index * 4) as usize] as i32;
            g_y += m_g_y[(k - (i - 1)) as usize][(l - (j - 1)) as usize] * self.frame[(index * 4) as usize] as i32;
          } 
        }

        let g: u8 = ((g_x.pow(2) + g_y.pow(2)) as f64).sqrt() as u8;
        if g == 0 {
          continue;
        }

        let theta: u8 = ((4.0_f64 * (g_x as f64).atan2(g_y as f64) / std::f64::consts::PI).round() * std::f64::consts::PI / 4.0_f64 - std::f64::consts::PI / 2.0_f64) as u8;

        index = i * self.width + j;

        sobel_matrix[(index * 4) as usize] = g;
        sobel_matrix[((index * 4) + 1) as usize] = g;
        sobel_matrix[((index * 4) + 2) as usize] = g;

        index += self.width + 1;

        sobel_matrix[(index * 4) as usize] = theta;
        sobel_matrix[((index * 4) + 1) as usize] = theta;
        sobel_matrix[((index * 4) + 2) as usize] = theta;

      }
    }

    self.frame = sobel_matrix;
    self
  }

  pub fn canny(mut self) -> Frame {
    
    let _size: u32 = 10;
    let _omega: f64 = 1.0_f64;

    self = Frame::transform_to_gray(self);
    self = Frame::gauss_filter(self, _size, _omega);
    self = Frame::sobel_operator(self);
    self
  }

  pub fn transform_to_black_and_white(mut self) -> Frame {
    let len: usize = self.frame.len() / 4;
    for i in 0..len {
      let gray: u32 = (self.frame[i * 4 + 0] as u32 + self.frame[i * 4 + 1] as u32 + self.frame[i * 4 + 2] as u32) / 3;
      self.frame[i * 4 + 0] = gray as u8;
      self.frame[i * 4 + 1] = gray as u8;
      self.frame[i * 4 + 2] = gray as u8;
    }

    self
  }

  pub fn transform_to_gray(mut self) -> Frame {
    let len: usize = self.frame.len() / 4;
    for i in 0..len {
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
    width: width,
    height: height,
  };
}