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

    fn normal_at(&self, hit_point: &Vec3) -> Vec3 {
        let epsilon = 1e-6; // Small value to handle precision issues
        let half_width = self.width * 0.5 - epsilon;
        let half_height = self.height * 0.5 - epsilon;
        let half_depth = self.depth * 0.5 - epsilon;

        if hit_point.0.y > self.get_properties().center.0.y + half_height {
            return Vec3::new(0., 1., 0.);
        }
        if hit_point.0.y < self.get_properties().center.0.y - half_height {
            return Vec3::new(0., -1., 0.);
        }
        if hit_point.0.x > self.get_properties().center.0.x + half_width {
            return Vec3::new(1., 0., 0.);
        }
        if hit_point.0.x < self.get_properties().center.0.x - half_width {
            return Vec3::new(-1., 0., 0.);
        }
        if hit_point.0.z > self.get_properties().center.0.z + half_depth {
            return Vec3::new(0., 0., 1.);
        }

        Vec3::new(0., 0., -1.)
    }

    fn closest_intersection(&self, origin: &Vec3, direction: &Vec3) -> Option<Vec3> {
        // TODO

        None
    }
}
