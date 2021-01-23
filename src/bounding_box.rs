use anyhow::anyhow;
use anyhow::Result;
use thiserror::Error;

use crate::{coordinate::Coordinate, point::Point, vector::Vector};

pub struct BoundingBox {
    width: f64,
    height: f64,
}

#[derive(Error, Debug)]
pub enum BoundingBoxError {
    #[error("Dimensions must be positive and nonzero")]
    DimensionError,

    #[error("Bounce resulted in out-of-bounds Point at ({}, {})", .0.position.x, .0.position.y)]
    BounceError(Point),
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

    /// Bounces the Point off of the edge(s) of the bounding box, if the
    /// given point's position and velocity would place it outside the bounding
    /// box
    pub fn bounce(&self, point: Point) -> Result<Point> {
        // let mut point = point;
        let new_pos = point.position.add(point.velocity);
        let mut new_point = Point {
            position: new_pos,
            velocity: point.velocity,
        };

        if self.contains(new_pos) {
            Ok(new_point)
        } else {
            // The coordinate will bounce off one or more edges of the bounding
            // box.

            let dist_beyond_edge = self.distance_beyond_edge(new_pos);

            // TODO: probably replace with an enum if we don't move away from this
            let mut axis = 0;
            for (axis_pos, axis_vel) in &mut new_point.components_mut() {
                // TODO: can we find a way to not have to compute which axis we are on?
                let upper_limit = match axis {
                    0 => self.width,
                    1 => self.height,
                    _ => return Err(anyhow!("invalid index")),
                };

                // If we would pass the edge, we want to bounce off of it. If
                // we would not reach the edge, merely use the component as-is.
                if *axis_pos < 0.0 || *axis_pos > upper_limit {
                    // subtract 2* the distance beyond edge from the position on this axis
                    *axis_pos -= 2.0 * dist_beyond_edge.at(axis)?;

                    // negate the velocity on this axis
                    *axis_vel = -*axis_vel;
                }

                axis += 1;
            }

            // if it's in bounds after bouncing, we're good.
            // TODO: handle an out-of-bounds after bouncing
            self.contains(new_point.position)
                .then_some(new_point)
                .ok_or(BoundingBoxError::BounceError(new_point).into())
        }
    }

    /// How far beyond the edges is the given coordinate?
    fn distance_beyond_edge(&self, pos: Coordinate) -> Vector {
        Vector {
            x: if pos.x < 0.0 {
                pos.x
            } else {
                pos.x - self.width
            },
            y: if pos.y < 0.0 {
                pos.y
            } else {
                pos.y - self.height
            },
        }
    }

    fn distance_to_edge(&self, pos: Coordinate, vec: Vector) -> Vector {
        // if the vector is negative, find distance from zero.
        let x_edge = if vec.x > 0.0 { self.width } else { 0.0 };
        let y_edge = if vec.y > 0.0 { self.height } else { 0.0 };

        Vector {
            x: x_edge - pos.x,
            y: y_edge - pos.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{coordinate::Coordinate, point::Point, vector::Vector};

    use super::BoundingBox;

    #[test]
    pub fn test_bounce() {
        let bb = BoundingBox::new(10.0, 10.0).unwrap();

        // Do not bounce
        let point = Point {
            position: Coordinate { x: 5.0, y: 5.0 },
            velocity: Vector::zero(),
        };
        assert_eq!(bb.bounce(point).unwrap(), point);

        // Bounce off of x
        let point = Point {
            position: Coordinate { x: 9.0, y: 9.0 },
            velocity: Vector { x: 3.0, y: 0.0 },
        };
        assert_eq!(
            bb.bounce(point).unwrap(),
            Point {
                position: Coordinate { x: 8.0, y: 9.0 },
                velocity: Vector { x: -3.0, y: 0.0 }
            }
        );

        // Bounce off of y
        let point = Point {
            position: Coordinate { x: 9.0, y: 9.0 },
            velocity: Vector { x: 0.0, y: 3.0 },
        };
        assert_eq!(
            bb.bounce(point).unwrap(),
            Point {
                position: Coordinate { x: 9.0, y: 8.0 },
                velocity: Vector { x: 0.0, y: -3.0 }
            }
        );

        // Bounce off of a corner
        let point = Point {
            position: Coordinate { x: 9.0, y: 9.0 },
            velocity: Vector { x: 2.5, y: 3.0 },
        };
        assert_eq!(
            bb.bounce(point).unwrap(),
            Point {
                position: Coordinate { x: 8.5, y: 8.0 },
                velocity: -point.velocity
            }
        );

        // Heading x and y negative, bounce off of y
        let point = Point {
            position: Coordinate { x: 5.0, y: 1.0 },
            velocity: Vector { x: -2.0, y: -3.0 },
        };
        assert_eq!(
            bb.bounce(point).unwrap(),
            Point {
                position: Coordinate { x: 3.0, y: 2.0 },
                velocity: Vector { x: -2.0, y: 3.0 },
            }
        );

        // TODO: need to test what happens if the "bounce" would still
        // result in an OOB condition
    }
}
