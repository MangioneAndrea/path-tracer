use std::f32::consts::PI;

use nalgebra::{distance, Vector1};
use rand::{thread_rng, Rng};

use crate::{
    algebra::Vec3,
    color::{BLACK, BLUE, GREEN, PINK, RED, WHITE},
    mesh::{sphere::Sphere, Mesh},
};

use super::Scene;

const P: f32 = 0.1;

pub struct CornellBox {
    pub spheres: [Sphere; 7],
}

pub fn new() -> CornellBox {
    let mut light = Sphere::new(0., 101., 0., 100., WHITE);
    light.mesh_properties.emission = Some(WHITE);
    CornellBox {
        spheres: [
            Sphere::new(-0.6, -0.7, -0.6, 0.3, PINK),
            Sphere::new(-0.6, -0.4, -0.3, 0.3, RED),
            Sphere::new(0., 0., 101., 100., GREEN),
            Sphere::new(-101., 0., 0., 100., RED),
            Sphere::new(101., 0., 0., 100., BLUE),
            light,
            Sphere::new(0., -101., 0., 100., WHITE),
        ],
    }
}

impl Scene for CornellBox {
    fn compute_color(
        &self,
        origin: &crate::algebra::Vec3,
        d: &crate::algebra::Vec3,
        rng: &mut rand::rngs::ThreadRng,
    ) -> crate::color::Color {
        let closest: Option<(&Sphere, Vec3, f32)> = self
            .spheres
            .iter()
            .map(|s| (s, s.closest_intersection(&origin, &d)))
            .filter(|(_, intersection)| intersection.is_some())
            .map(|(s, intersection)| {
                (
                    s,
                    intersection.unwrap(),
                    distance(&origin.0.into(), &intersection.unwrap().0.into()),
                )
            })
            .filter(|(_, _, d)| d > &0.)
            .min_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2));

        if closest.is_none() {
            return BLACK;
        }

        let (sphere, intersection, _) = closest.unwrap();

        let rnd: usize = rng.gen();

        if (rnd as f32) < (P * (usize::MAX as f32)) {
            return sphere.mesh_properties.emission.unwrap_or_default();
        }

        let mut random_direction = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        let n = (intersection.0 - sphere.mesh_properties.center.0).normalize();

        while random_direction.0.magnitude() > 1. {
            random_direction.0.x = rng.gen();
            random_direction.0.y = rng.gen();
            random_direction.0.z = rng.gen();
        }

        let mut random_direction = random_direction.0.normalize();

        if random_direction.dot(&n) < 0. {
            random_direction = random_direction * Vector1::new(-1.);
        }

        let color = sphere.brdf(d, &Vec3(n), &Vec3(random_direction))
            * (n.dot(&random_direction) * ((2. * PI) / 1. - P));
        let emission = sphere.mesh_properties.emission.unwrap_or_default();
        let next_emissions = self.compute_color(&intersection, &Vec3(random_direction), rng);

        return emission + next_emissions * color;
    }
}
