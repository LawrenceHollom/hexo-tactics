use std::ops::{Add, Index, Mul};

/**
 * For clattering around with colours like a moron
 */
#[derive(Clone, Copy)]
pub struct Pixel([f64; 4]);

impl Pixel {
    pub const YELLOW: Pixel = Pixel([1.0, 0.75, 0.14, 1.0]);
    pub const CYAN: Pixel = Pixel([0.22, 0.75, 1.0, 1.0]);
    pub const BACKGROUND: Pixel = Pixel([0.0588, 0.0902, 0.165, 1.0]);
    pub const LINES: Pixel = Pixel([0.141, 0.17, 0.25, 1.0]);


    pub fn to_ints(&self) -> [u8; 4] {
        [
            (self[0].clamp(0.0, 1.0) * 255.99).floor() as u8,
            (self[1].clamp(0.0, 1.0) * 255.99).floor() as u8,
            (self[2].clamp(0.0, 1.0) * 255.99).floor() as u8,
            (self[3].clamp(0.0, 1.0) * 255.99).floor() as u8,
        ]
    }

    // pub fn of_ints(v: [u8; 4]) -> Pixel {
    //     Pixel([
    //         v[0] as f64 / 255.0,
    //         v[1] as f64 / 255.0,
    //         v[2] as f64 / 255.0,
    //         v[3] as f64 / 255.0,
    //     ])
    // }

    // pub fn greyscale(x: f64) -> Pixel {
    //     Pixel([x, x, x, 1.0])
    // }
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