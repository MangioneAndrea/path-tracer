use nalgebra::distance;

use crate::{
    color::{BLACK, BLUE, GREEN, RED, WHITE},
    mesh::sphere::Sphere,
};

use super::Scene;

pub struct CornellBox {
    pub spheres: [Sphere; 2],
}

pub fn new() -> CornellBox {
    CornellBox {
        spheres: [
            Sphere::new(-0.6, -0.7, -0.6, 0.3, RED),
            Sphere::new(-0.6, -0.7, -0.6, 0.3, RED),
            /*
            Sphere::new(0., 0., 101., 100., GREEN),
            Sphere::new(-101., 0., 0., 100., RED),
            Sphere::new(101., 0., 0., 100., BLUE),
            Sphere::new(0., 101., 0., 100., BLUE),
            Sphere::new(0., -101., 0., 100., WHITE),
            */
        ],
    }
}

impl Scene for CornellBox {
    fn compute_color(
        &self,
        camera: &crate::camera::Camera,
        d: crate::algebra::Vec3,
    ) -> crate::color::Color {
        let closest: Option<(&Sphere, f32)> = self
            .spheres
            .iter()
            .map(|s| (s, s.closest_intersection(camera.origin, d)))
            .filter(|(_, intersection)| intersection.is_some())
            .map(|(s, intersection)| (s, distance(&camera.origin.0.into(), &intersection.unwrap().0.into())))
            .filter(|(_, d)| d > &0.)
            .min_by(|(_, d1), (_, d2)| d1.total_cmp(d2));

        return closest.map_or(BLACK, |c| c.0.mesh_properties.color);
    }
}
