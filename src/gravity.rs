use crate::traits::Force;

pub struct Gravity {
    grav_constant: f64,
}

impl Default for Gravity {
    fn default() -> Self {
        Self { grav_constant: 1.0 }
    }
}

impl Force for Gravity {
    fn apply(&self, point: &mut crate::point::Point) {
        // Modify acceleration in the Y direction by the gravitational constant.
        point.velocity.y += self.grav_constant;
    }
}
