use super::*;

use crate::random;

use std::fmt;
use std::fmt::Display;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Color
{
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        return Color {
            r,
            g,
            b,
        };
    }

    pub fn new_default() -> Color {
        return Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
    }

    pub fn random_unit() -> Color {
        return Color::new(random::double_unit(), random::double_unit(), random::double_unit())
    }

    pub fn random_range(min: f64, max: f64) -> Color {
        return Color::new(random::double_range(min, max), random::double_range(min, max), random::double_range(min, max))
    }
}

impl Add for Color
{
    type Output = Color;

    fn add(self, other: Color) -> Color {
        return Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color)
    {
        *self = Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Div<f64> for Color
{
    type Output = Color;

    fn div(self, other: f64) -> Color {
        return Color::new(self.r / other, self.g / other, self.b / other);
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, other: f64)
    {
        *self = Color::new(self.r / other, self.g / other, self.b / other);
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut r = self.r;
        let mut g = self.g;
        let mut b = self.b;

        r = r.powf(1.0/2.2);
        g = g.powf(1.0/2.2);
        b = b.powf(1.0/2.2);

        let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
        let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
        let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

        write!(f, "{} {} {}", ir, ig, ib)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        return Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
         };
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        return Color {
            r: other * self.r,
            g: other * self.g,
            b: other * self.b,
         };
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        return other * self;
    }
}
