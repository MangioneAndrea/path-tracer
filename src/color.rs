use crate::algebra::Unit;

#[derive(Clone, Copy, Debug)]
pub struct Color(pub Unit, pub Unit, pub Unit);

pub const BLACK: Color = Color(0., 0., 0.);
pub const WHITE: Color = Color(1., 1., 1.);
pub const RED: Color = Color(1., 0., 0.);
pub const BLUE: Color = Color(0., 1., 0.);
pub const GREEN: Color = Color(0., 0., 1.);
pub const PINK: Color = Color(1., 0.6, 0.6);

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(
            (self.0 * 255.) as u8,
            (self.1 * 255.) as u8,
            (self.2 * 255.) as u8,
        )
    }
}
