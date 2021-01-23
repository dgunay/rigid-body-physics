use crate::point::Point;

pub trait Force {
    fn apply(&self, point: &mut Point);
}
