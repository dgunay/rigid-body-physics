use crate::{point::Point, vector::Vector};

pub trait Force {
    fn apply(&self, point: &mut Point);
}

#[derive(Default)]
pub struct GenericForce {
    pub vec: Vector,
}

impl Force for GenericForce {
    fn apply(&self, point: &mut Point) {
        point.velocity = point.velocity + self.vec;
    }
}
