use crate::{
    algebra::{Unit, Vec3},
    color::{Color, BLACK},
};

pub(crate) mod cuboid;
pub(crate) mod sphere;

pub struct MeshProperties {
    center: Vec3,
    color: Color,
    reflectivity: Unit,
}

impl Default for MeshProperties {
    fn default() -> Self {
        MeshProperties {
            center: Vec3::default(),
            color: BLACK,
            reflectivity: 0.,
        }
    }
}

pub trait Mesh {
    fn closest_intersection(from: Vec3, to: Vec3);
}
