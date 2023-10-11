use crate::algebra::{Unit, Vec3};

pub struct Camera {
    origin: Vec3,
    target: Vec3,
    fov: Unit,
    up: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vec3::new(0., 0., -4.),
            target: Vec3::new(0., 0., 6.),
            up: Vec3::new(0., 1., 0.),
            fov: 0.7,
        }
    }
}

impl Camera {
    pub fn direction(&self) -> Vec3 {
        Vec3((self.target.0 - self.origin.0).normalize())
    }
    pub fn r(&self) -> Vec3 {
        Vec3((self.up.0.cross(&self.direction().0)).normalize())
    }
    pub fn u(&self) -> Vec3 {
        Vec3((self.direction().0.cross(&self.r().0)).normalize())
    }
    pub fn fov_scale(&self) -> Unit {
        (self.fov / 2.).tan()
    }
}
