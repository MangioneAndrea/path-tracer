use crate::algebra::{Unit, Vec3};
use crate::color::Color;
use crate::mesh::MeshProperties;

pub struct Sphere {
    mesh_properties: MeshProperties,
    radius: Unit,
}

impl Sphere {
    pub fn new(x: Unit, y: Unit, z: Unit, radius: Unit, color: Color) -> Sphere {
        Sphere {
            mesh_properties: MeshProperties {
                center: Vec3::new(x, y, z),
                color,
                reflectivity: 0.
            },
            radius
        }
    }
}
