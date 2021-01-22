use std::collections::vec_deque;

use crate::{bounding_box::BoundingBox, coordinate::Coordinate, vector::Vector};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub position: Coordinate,
    pub velocity: Vector,
}

impl Point {
    // TODO: is it better/more efficient to mutate a Point or return a new one?
    pub fn travel(self, bounding_box: &BoundingBox) -> Point {
        // If it is the edge of the bounding box, bounce off of it.
        // e.g. if we are velocity 3.0, and the edge is 2.0 away, we end up 1.0 from the edge.
        Self {
            position: bounding_box.bounce(self.position, self.velocity),
            velocity: self.velocity,
        }
    }
}
