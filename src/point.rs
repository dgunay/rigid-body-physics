use anyhow::Result;
use sdl2::rect::Rect;

use crate::{bounding_box::BoundingBox, coordinate::Coordinate, traits::Force, vector::Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub position: Coordinate,
    pub velocity: Vector,
}

impl Point {
    // TODO: is it better/more efficient to mutate a Point or return a new one?
    pub fn travel(
        mut self,
        bounding_box: &BoundingBox,
        forces: &Vec<Box<dyn Force>>,
    ) -> Result<Point> {
        // Apply forces such as gravity
        for f in forces {
            f.apply(&mut self);
        }

        // If it is the edge of the bounding box, bounce off of it.
        // e.g. if we are velocity 3.0, and the edge is 2.0 away, we end up 1.0 from the edge.
        bounding_box.bounce(self)
    }

    pub fn components_mut(&mut self) -> impl Iterator<Item = (&mut f64, &mut f64)> {
        use std::iter::once;
        once((&mut self.position.x, &mut self.velocity.x))
            .chain(once((&mut self.position.y, &mut self.velocity.y)))
    }
}

impl Into<Rect> for Point {
    fn into(self) -> Rect {
        let point_width: u32 = 6;
        let midpt = point_width / 2;
        let (x, y) = self.position.rounded_as_ints();
        Rect::new(x + midpt as i32, y + midpt as i32, point_width, point_width)
    }
}
