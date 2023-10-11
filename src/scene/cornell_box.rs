use crate::{
    color::{BLACK, BLUE, GREEN, RED, WHITE},
    mesh::sphere::Sphere,
};

use super::Scene;

pub struct CornellBox {
    pub spheres: [Sphere; 7],
}

pub fn new() -> CornellBox {
    CornellBox {
        spheres: [
            Sphere::new(-0.6, -0.7, -0.6, 0.3, RED),
            Sphere::new(-0.6, -0.7, -0.6, 0.3, RED),
            Sphere::new(0., 0., 101., 100., GREEN),
            Sphere::new(-101., 0., 0., 100., RED),
            Sphere::new(101., 0., 0., 100., BLUE),
            Sphere::new(0., 101., 0., 100., WHITE),
            Sphere::new(0., -101., 0., 100., BLACK),
        ],
    }
}

impl Scene for CornellBox {
    fn compute_color(
        &self,
        camera: &crate::camera::Camera,
        d: crate::algebra::Vec3,
    ) -> crate::color::Color {
        GREEN
    }
}
