use std::fmt::Display;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::random;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new_default() -> Vec3 {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn x(&self) -> f64 {
        return self.x;
    }

    pub fn y(&self) -> f64 {
        return self.y;
    }

    pub fn z(&self) -> f64 {
        return self.z;
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }

    pub fn normalize(&mut self) -> &mut Self
    {
        *self /= self.length();
        return self;
    }

    pub fn random_unit() -> Vec3 {
        return Vec3::random_range(-1.0, 1.0)
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        return Vec3 {
            x: random::double_range(min, max),
            y: random::double_range(min, max),
            z: random::double_range(min, max),
        };
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3
    {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3)
    {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3
    {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64)
    {
        *self = Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3
    {
        Vec3 {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, i: i32) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!()
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3
    {
        return other * self;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64)
    {
        *self = Vec3 {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        };
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3
    {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3
    {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3)
    {
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    return u.x *v.x + u.y*v.y + u.z*v.z;
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3
{
    return Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };
}

pub fn unit_vector(mut v: Vec3) -> Vec3
{
    v.normalize();
    return v;
}
