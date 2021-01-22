use crate::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64
}

impl Coordinate {
    /// Returns the pair of coordinates rounded to the nearest i32.
    pub fn rounded_as_ints(&self) -> (i32, i32) {
        (self.x.round() as i32, self.y.round() as i32)
    }

    /// Returns a new Coordinate with the Vector added
    pub fn add(self, vec: Vector) -> Self {
        Self {
            x: self.x + vec.x,
            y: self.y + vec.y
        }
    }
}