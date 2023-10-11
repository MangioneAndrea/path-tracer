pub type Unit = f32;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub nalgebra::Vector3<Unit>);

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(nalgebra::Vector3::default())
    }
}

impl Vec3 {
    pub const fn new(a: Unit, b: Unit, c: Unit) -> Vec3 {
        Vec3(nalgebra::Vector3::new(a, b, c))
    }
}
