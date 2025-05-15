use std::f64::consts::PI;

pub struct Circle {
  radius: f64,
}

impl Circle {
  pub fn new(radius: f64) -> Circle {
    Circle { radius }
  }

  pub fn area(&self) -> f64 {
    PI * self.radius * self.radius
  }

  pub fn circumference(&self) -> f64 {
    PI * self.radius
  }
}