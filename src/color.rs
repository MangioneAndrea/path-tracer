use nalgebra::Vector3;

use crate::algebra::{Unit, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Color(pub Unit, pub Unit, pub Unit);

pub const BLACK: Color = Color(0., 0., 0.);
pub const WHITE: Color = Color(1., 1., 1.);
pub const RED: Color = Color(1., 0., 0.);
pub const BLUE: Color = Color(0., 1., 0.);
pub const GREEN: Color = Color(0., 0., 1.);
pub const PINK: Color = Color(1., 0.6, 0.6);

impl Color {}

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(
            (self.0 * 255.) as u8,
            (self.1 * 255.) as u8,
            (self.2 * 255.) as u8,
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        return BLACK;
    }
}

impl std::ops::Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl std::ops::Div<Unit> for Color {
    type Output = Color;
    fn div(self, rhs: Unit) -> Self::Output {
        Color(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl std::ops::Mul<Unit> for Color {
    type Output = Color;
    fn mul(self, rhs: Unit) -> Self::Output {
        Color(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(BLACK, |a, b| a + b)
    }
}

pub trait ColorOps {
    fn avg(self) -> Color;
}

impl ColorOps for Vec<Color> {
    fn avg(self) -> Color {
        let size = self.len() as Unit;
        self.into_iter().sum::<Color>() / size
    }
}
