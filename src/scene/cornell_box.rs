use std::f32::consts::PI;

use nalgebra::distance;
use rand::Rng;

use crate::{
    algebra::Vec3,
    color::{BLACK, BLUE, GREEN, PINK, RED, WHITE},
    mesh::{cuboid::Cuboid, sphere::Sphere, Mesh},
};

use super::Scene;

const P: f32 = 0.2;

pub struct CornellBox {
    pub meshes: Vec<&'static dyn Mesh>,
}

pub fn new() -> CornellBox {
    let mut light = Sphere::new(0., 101., 0., 100., WHITE);
    light.mesh_properties.emission = Some(WHITE);

    let mut spec_ball = Sphere::new(0.6, -0.8, -0.3, 0.2, RED);
    // spec_ball.mesh_properties.reflectivity = Some(1.);

    CornellBox {
        meshes: vec![
            Box::leak(Box::new(spec_ball)),
            Box::leak(Box::new(Sphere::new(0., 0., 101., 100., GREEN))),
            Box::leak(Box::new(Sphere::new(-101., 0., 0., 100., RED))),
            Box::leak(Box::new(Sphere::new(101., 0., 0., 100., BLUE))),
            Box::leak(Box::new(light)),
            Box::leak(Box::new(Sphere::new(0., -101., 0., 100., WHITE))),
            Box::leak(Box::new(Cuboid::new(-0.6, -0.7, -0.6, 1.3, 1.4, 0.5, PINK))),
        ],
    }
}

impl Scene for CornellBox {
    fn get_meshes(&self) -> &Vec<&dyn Mesh> {
        &self.meshes
    }

    fn compute_color(
        &self,
        origin: &crate::algebra::Vec3,
        d: &crate::algebra::Vec3,
        rng: &mut rand::rngs::ThreadRng,
    ) -> crate::color::Color {
        let closest: Option<(_, Vec3, f32)> = self
            .get_meshes()
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
            .filter(|(_, _, d)| d > &0.001)
            .min_by(|(_, _, d1), (_, _, d2)| d1.total_cmp(d2));

        if closest.is_none() {
            return BLACK;
        }

        let (mesh, intersection, _) = closest.unwrap();

        if rng.gen::<f32>() < P {
            return mesh.get_properties().emission.unwrap_or_default();
        }

        let mut random_direction = Vec3::new(
            rng.gen_range((-1.)..(1.)),
            rng.gen_range((-1.)..(1.)),
            rng.gen_range((-1.)..(1.)),
        );

        let n = (intersection.0 - mesh.get_properties().center.0).normalize();

        while random_direction.0.magnitude() > 1. {
            random_direction.0.x = rng.gen_range((-1.)..(1.));
            random_direction.0.y = rng.gen_range((-1.)..(1.));
            random_direction.0.z = rng.gen_range((-1.)..(1.));
        }

        let mut random_direction = random_direction.0.normalize();

        if n.dot(&random_direction) < 0. {
            random_direction = -random_direction;
        }

        let next_emissions = self.compute_color(&intersection, &Vec3(random_direction), rng);
        let color = mesh.brdf(d, &Vec3(n), &Vec3(random_direction), next_emissions)
            * (n.dot(&random_direction) * ((2. * PI) / 1. - P))
            * next_emissions;

        let emission = mesh.get_properties().emission.unwrap_or_default();

        return emission + color;
    }
}
