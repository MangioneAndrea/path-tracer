use nalgebra::{RealField, Vector3};

use crate::{
    algebra::{Unit, Vec3},
    color::{Color, BLACK},
};

pub(crate) mod cuboid;
pub(crate) mod sphere;

pub struct MeshProperties {
    pub center: Vec3,
    pub color: Color,
    pub reflectivity: Option<Unit>,
    pub emission: Option<Color>,
}

impl Default for MeshProperties {
    fn default() -> Self {
        MeshProperties {
            center: Vec3::default(),
            color: BLACK,
            reflectivity: None,
            emission: None,
        }
    }
}

pub trait Mesh {
    fn closest_intersection(&self, from: &Vec3, to: &Vec3) -> Option<Vec3>;

    fn get_properties(&self) -> &MeshProperties;

    fn brdf(&self, direction: &Vec3, normal: &Vec3, w: &Vec3) -> Color {
        let nl = normal.0.magnitude();

        if self.get_properties().reflectivity.is_some() {}

        return self.get_properties().color * (1. / f32::pi());
    }
}
