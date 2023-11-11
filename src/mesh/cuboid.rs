use nalgebra::Vector1;

use crate::algebra::{Unit, Vec3};
use crate::color::Color;
use crate::mesh::MeshProperties;

use super::Mesh;

pub struct Cuboid {
    pub mesh_properties: MeshProperties,
    pub height: Unit,
    pub width: Unit,
    pub depth: Unit,
}

impl Cuboid {
    pub fn new(
        x: Unit,
        y: Unit,
        z: Unit,
        height: Unit,
        width: Unit,
        depth: Unit,
        color: Color,
    ) -> Cuboid {
        Cuboid {
            mesh_properties: MeshProperties {
                center: Vec3::new(x, y, z),
                color,
                reflectivity: None,
                emission: None,
            },
            height,
            width,
            depth,
        }
    }
}

impl Mesh for Cuboid {
    fn get_properties(&self) -> &MeshProperties {
        return &self.mesh_properties;
    }

    fn closest_intersection(&self, origin: &Vec3, direction: &Vec3) -> Option<Vec3> {
        // TODO
        None
    }
}
