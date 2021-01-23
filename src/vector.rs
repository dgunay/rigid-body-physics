use anyhow::Result;
use sdl2::keyboard::Scancode;
use thiserror::Error;

use std::{
    convert::TryFrom,
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Attempted to access axis at invalid offset {0}")]
    InvalidAxis(usize),

    #[error("Invalid Scancode {0}")]
    InvalidScancode(Scancode),
}

impl Vector {
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn zero_out(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    /// Accesses the Vector's fields as if they were an array of f64s.
    // TODO: this is mostly a crutch to handle the awkwardness of generically
    // handling axis selection from the bounding box (e.g. iterating over
    // the components of a point or vector and comparing them to the limits
    // of the bounding box, we have to do runtime checking of which axis we
    // are looking at)
    pub fn at(&self, i: usize) -> Result<f64> {
        match i {
            0 => Ok(self.x),
            1 => Ok(self.y),
            _ => Err(Error::InvalidAxis(i).into()),
        }
    }

    pub fn scale(self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    // Returns a vector with the components either as-is or set to `limit` if
    // they are less than `limit`.
    pub fn max(self, limit: f64) -> Self {
        Self {
            x: self.x.max(limit),
            y: self.y.max(limit),
        }
    }

    pub fn min(self, limit: f64) -> Self {
        Self {
            x: self.x.min(limit),
            y: self.y.min(limit),
        }
    }

    // Floors the components if they are positive, ceils them if they are negative.
    // pub fn floor_or_ceil(self, limit: f64) -> Self {
    //     Self {
    //         x: if x > 0 {},
    //         y: if y > 0 {},
    //     }
    // }

    /// Given another vector and a function F, constructs a new Vector out of
    /// the result of passing the components of both vectors to F. e.g., you
    /// may implement vector addition by using something like:
    /// ```
    /// use fixed_point_physics::vector::Vector;
    /// let v1 = Vector { x: 1.0, y: 1.0 };
    /// let v2 = v1.map(&v1, |l, r| l+r);
    /// assert_eq!(v2, v1 + v1);
    /// ```
    pub fn map(self, rhs: &Vector, func: impl Fn(&f64, &f64) -> f64) -> Self {
        Self {
            x: func(&self.x, &rhs.x),
            y: func(&self.y, &rhs.y),
        }
    }
}

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl TryFrom<Scancode> for Vector {
    type Error = Error;

    fn try_from(k: Scancode) -> Result<Self, Self::Error> {
        let mut v = Vector::zero();
        match k {
            Scancode::A => v.x = -1.0,
            Scancode::D => v.x = 1.0,
            Scancode::S => v.y = 1.0,
            Scancode::W => v.y = -1.0,
            Scancode::Left => v.x = -1.0,
            Scancode::Right => v.x = 1.0,
            Scancode::Down => v.y = 1.0,
            Scancode::Up => v.y = -1.0,
            _ => return Err(Error::InvalidScancode(k).into()),
        }

        Ok(v)
    }
}
