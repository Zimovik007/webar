pub struct Frame {
  frame: Vec<u8>,
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
    
  }

  pub fn get_result(self) -> Vec<u8> {
    return self.frame;
  }
}

pub fn create(f: Vec<u8>) -> Frame {
  return Frame {
    frame: f,
    surf_features: Vec::new(),
  };
}