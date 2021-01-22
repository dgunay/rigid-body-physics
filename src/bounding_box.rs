use anyhow::Result;
use thiserror::Error;

use crate::{coordinate::Coordinate, vector::Vector};

pub struct BoundingBox {
    width: f64,
    height: f64,
}

#[derive(Error, Debug)]
pub enum BoundingBoxError {
    #[error("Dimensions must be positive and nonzero")]
    DimensionError,
}

impl BoundingBox {
    pub fn new(width: f64, height: f64) -> Result<Self> {
        if width <= 0.0 || height <= 0.0 {
            return Err(BoundingBoxError::DimensionError)?;
        }

        Ok(BoundingBox { width, height })
    }

    /// True if the Coordinate is within the bounding box
    pub fn contains(&self, coordinate: Coordinate) -> bool {
        coordinate.x >= 0.0
            && coordinate.x <= self.width
            && coordinate.y >= 0.0
            && coordinate.y <= self.height
    }

    /// Bounces the coordinate off of the edge(s) of the bounding box, if the
    /// given vector would place it outside the bounding box
    pub fn bounce(&self, coordinate: Coordinate, vec: Vector) -> Coordinate {
        let new_pos = coordinate.add(vec);
        if self.contains(new_pos) {
            new_pos
        } else {
            // TODO:
            coordinate
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{coordinate::Coordinate, vector::Vector};

    use super::BoundingBox;

    #[test]
    pub fn test_bounce() {
        let bb = BoundingBox::new(10.0, 10.0).unwrap();
        
        // Do not bounce
        let pos = Coordinate { x: 5.0, y:  5.0 };
        assert_eq!(bb.bounce(pos, Vector::zero()), pos);

        // Bounce off of x
        let pos = Coordinate { x: 9.0, y:  9.0 };
        assert_eq!(bb.bounce(pos, Vector { x: 3.0, y: 0.0} ), Coordinate {x: 8.0, y: 9.0 });

        // Bounce off of y
        let pos = Coordinate { x: 9.0, y:  9.0 };
        assert_eq!(bb.bounce(pos, Vector { x: 0.0, y: 3.0} ), Coordinate {x: 9.0, y: 8.0 });

        // Bounce off of a corner
        let pos = Coordinate { x: 9.0, y:  9.0 };
        assert_eq!(bb.bounce(pos, Vector { x: 2.5, y: 3.0} ), Coordinate {x: 8.5, y: 8.0 });
    }
}
