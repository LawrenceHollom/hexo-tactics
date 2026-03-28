use std::ops::{Add, Index, Mul};

/**
 * For clattering around with colours like a moron
 */
#[derive(Clone, Copy)]
pub struct Pixel([f64; 4]);

impl Pixel {
    pub const BLACK: Pixel = Pixel([0.0, 0.0, 0.0, 1.0]);
    pub const WHITE: Pixel = Pixel([1.0, 1.0, 1.0, 1.0]);
    pub const RED: Pixel = Pixel([1.0, 0.0, 0.0, 1.0]);
    pub const YELLOW: Pixel = Pixel([1.0, 1.0, 0.0, 1.0]);
    pub const GREEN: Pixel = Pixel([0.0, 1.0, 0.0, 1.0]);
    pub const CYAN: Pixel = Pixel([0.0, 1.0, 1.0, 1.0]);
    pub const BLUE: Pixel = Pixel([0.0, 0.0, 1.0, 1.0]);
    pub const MAGENTA: Pixel = Pixel([1.0, 0.0, 1.0, 1.0]);

    pub fn new(r: f64, g: f64, b: f64, alpha: f64) -> Pixel {
        Pixel([r, g, b, alpha])
    }

    pub fn to_ints(&self) -> [u8; 4] {
        [
            (self[0].clamp(0.0, 1.0) * 255.99).floor() as u8,
            (self[1].clamp(0.0, 1.0) * 255.99).floor() as u8,
            (self[2].clamp(0.0, 1.0) * 255.99).floor() as u8,
            (self[3].clamp(0.0, 1.0) * 255.99).floor() as u8,
        ]
    }

    pub fn of_ints(v: [u8; 4]) -> Pixel {
        Pixel([
            v[0] as f64 / 255.0,
            v[1] as f64 / 255.0,
            v[2] as f64 / 255.0,
            v[3] as f64 / 255.0,
        ])
    }

    pub fn greyscale(x: f64) -> Pixel {
        Pixel([x, x, x, 1.0])
    }
}

impl Index<usize> for Pixel {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Add for Pixel {
    type Output = Pixel;

    fn add(self, rhs: Self) -> Self::Output {
        Pixel([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ])
    }
}

impl Mul<f64> for Pixel {
    type Output = Pixel;

    fn mul(self, rhs: f64) -> Self::Output {
        Pixel([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3]])
    }
}