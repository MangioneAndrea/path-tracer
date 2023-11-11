use nalgebra::Vector1;

use crate::algebra::{Unit, Vec3};
use crate::color::Color;
use crate::mesh::MeshProperties;

use super::Mesh;

pub struct Sphere {
    pub mesh_properties: MeshProperties,
    pub radius: Unit,
}

impl Sphere {
    pub fn new(x: Unit, y: Unit, z: Unit, radius: Unit, color: Color) -> Sphere {
        Sphere {
            mesh_properties: MeshProperties {
                center: Vec3::new(x, y, z),
                color,
                reflectivity: None,
                emission: None,
            },
            radius,
        }
    }
}

impl Mesh for Sphere {
    fn get_properties(&self) -> &MeshProperties {
        return &self.mesh_properties;
    }

    fn closest_intersection(&self, origin: &Vec3, direction: &Vec3) -> Option<Vec3> {
        let u = direction.0.normalize();
        let ce = origin.0 - self.mesh_properties.center.0;

        let a = 1.;
        let b = 2. * u.dot(&ce);

        let c = ce.dot(&ce) - self.radius * self.radius;

        let delta = b * b - 4. * a * c;

        if delta < 0.000001 {
            return None;
        }

        let d = if b > 0. {
            -b + delta.sqrt()
        } else {
            -b - delta.sqrt()
        };


        if d < 0.000001 {
            return None;
        }

        let best = d / (2. * a);

        let res = u * best + origin.0;

        Some(Vec3(res))
    }
}
