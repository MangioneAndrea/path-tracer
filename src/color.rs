use crate::algebra::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Color(Vec3);

pub const BLACK: Color = Color(Vec3::new(0., 0., 0.));
pub const WHITE: Color = Color(Vec3::new(1., 1., 1.));
pub const RED: Color = Color(Vec3::new(1., 0., 0.));
pub const BLUE: Color = Color(Vec3::new(0., 1., 0.));
pub const GREEN: Color = Color(Vec3::new(0., 0., 1.));
