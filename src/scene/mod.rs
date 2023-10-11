use crate::algebra::{Unit, Vec3};
use crate::camera::Camera;
use crate::color::{Color, BLACK};

pub(crate) mod cornell_box;

pub trait Scene {
    fn compute_color(&self, camera: &Camera, d: Vec3) -> Color;

    fn get_pixels<const W: usize, const H: usize>(&self, camera: &Camera) -> [[Color; W]; H] {
        let mut res = [[BLACK; W]; H];
        let aspect_ratio: Unit = W as f32 / W as f32;

        let direction = camera.direction();
        let r = camera.r();
        let u = camera.u();

        for y in 0..H {
            let adj_y: Unit = (y as Unit) / ((H as Unit) / 2. - 1.);
            for x in 0..W {
                let adj_x: Unit = ((x as Unit) / ((W as Unit) / 2. - 1.)) * aspect_ratio;

                let d = Vec3(
                    direction.0
                        + (r.0 * nalgebra::Vector1::new(camera.fov_scale() * adj_x))
                        + (u.0 * (-camera.fov_scale() * adj_y)),
                );

                res[y][x] = self.compute_color(camera, d);
            }
        }

        return res;
    }
}
