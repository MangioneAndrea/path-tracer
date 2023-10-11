use crate::algebra::{Unit, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Color(Unit, Unit, Unit);

pub const BLACK: Color = Color(0., 0., 0.);
pub const WHITE: Color = Color(1., 1., 1.);
pub const RED: Color = Color(1., 0., 0.);
pub const BLUE: Color = Color(0., 1., 0.);
pub const GREEN: Color = Color(0., 0., 1.);

