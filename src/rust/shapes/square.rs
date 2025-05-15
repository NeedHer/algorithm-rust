pub struct Square {
  side: f64,
}

impl Square {
  pub fn new(side: f64) -> Square {
    Square { side }
  }

  pub fn area(&self) -> f64 {
    self.side * self.side
  }

  pub fn perimeter(&self) -> f64 {
    4.0 * self.side
  }
}