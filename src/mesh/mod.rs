use crate::{
    algebra::{Unit, Vec3},
    color::{Color, BLACK},
};

pub(crate) mod cuboid;
pub(crate) mod sphere;

pub struct MeshProperties {
    pub center: Vec3,
    pub color: Color,
    pub reflectivity: Unit,
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
