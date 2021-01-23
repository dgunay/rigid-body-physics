use crate::traits::Force;

pub struct Drag {
    coefficient: f64,
}

impl Drag {
    // coefficient is limited from 0.0 (instantly stops movement) to 1.0 (does nothing)
    pub fn new(coefficient: f64) -> Self {
        Self {
            coefficient: coefficient.clamp(0.0, 1.0),
        }
    }
}

impl Force for Drag {
    fn apply(&self, point: &mut crate::point::Point) {
        // Reduce velocity by the drag coefficient
        point.velocity = point.velocity * self.coefficient;
    }
}
