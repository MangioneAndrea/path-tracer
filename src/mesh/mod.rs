use std::f32::consts::PI;

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

/*
private Vec3f diffuseBRDF(HitRecord hit) {
    return hit.obj().color(hit).multiplyMutable((float) (1 / Math.PI));
}

private Vec3f specularBRDF(HitRecord hit, ImmutableVec3f incomingDirection, ImmutableVec3f outgoingDirection) {
    final ImmutableVec3f normal = hit.point().subtract(hit.obj().center()).normalize();
    final ImmutableVec3f reflection = incomingDirection.reflect(normal).normalize();

    if (outgoingDirection.dot(reflection) > 1 - 0.01) {
        return diffuseBRDF(hit).add(specularColor(hit).multiply(10));
    }

    return diffuseBRDF(hit);
}

    return closestHit.obj().emission(closestHit).mutableCopy()
            .add(BRDF(closestHit, ray.direction(), randomDirection.immutableCopy())
                    .multiply((float) ((2 * Math.PI) / (1 - RECURSION_BREAK_PROBABILITY)) * randomDirection.dot(surfaceNormal))
                    .multiply(computeColor(rng, spheres, new Ray(closestHit.point(), randomDirection.immutableCopy()))));

                    auto cameraReflection = direction - (glm::dot(direction * glm::dvec3(2, 2, 2), normal)) / (nl * nl) * normal;
                    if (reflectivity && glm::dot(glm::normalize(cameraReflection), w) > (1 - theta)) {
                    return (this->color + (specular * micro)) * (1. / M_PI);
                    }
*/
pub trait Mesh: Sync {
    fn closest_intersection(&self, from: &Vec3, to: &Vec3) -> Option<Vec3>;

    fn get_properties(&self) -> &MeshProperties;

    fn normal_at(&self, at: &Vec3) -> Vec3; 

    fn brdf(
        &self,
        incoming_direction: &Vec3,
        normal: &Vec3,
        random_direction: &Vec3,
        next_emission: Color,
    ) -> Color {
        match self.get_properties().reflectivity {
            None => self.get_properties().color * (1. / PI),
            Some(_) => {
                let nl = normal.0.magnitude();
                let camera_reflection = incoming_direction.0
                    - (incoming_direction.0/* * Vec3::new(2.,2.,2.)*/).dot(&normal.0) / (nl * nl)
                        * normal.0;

                if random_direction.0.dot(&camera_reflection) > 0.99 {}
                return next_emission * 1. / PI;
            }
        }
    }
}
