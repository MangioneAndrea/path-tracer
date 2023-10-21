use sdl2::render::Canvas;

use crate::algebra::{Unit, Vec3};
use crate::camera::Camera;
use crate::color::{Color, ColorOps, BLACK};

pub(crate) mod cornell_box;

const ITERATIONS: usize = 128;

pub trait Scene {
    fn compute_color(&mut self, camera: &Vec3, d: &Vec3) -> Color;

    fn get_pixels<const W: usize, const H: usize>(
        &mut self,
        camera: &Camera,
        canvas: &mut Canvas<sdl2::video::Window>,
    ) {
        let aspect_ratio: Unit = (W as Unit) / (H as Unit);
        println!("Aspect ratio {}, {}: {}", W, H, aspect_ratio);

        let direction = camera.direction();
        let r = camera.r();
        let u = camera.u();

        for y in 0..H {
            let adj_y: Unit = (y as Unit) / ((H as Unit) / 2.) - 1.;

            for x in 0..W {
                let adj_x: Unit = ((x as Unit) / ((W as Unit) / 2.) - 1.) * aspect_ratio;

                let d = Vec3(
                    direction.0
                        + (r.0 * nalgebra::Vector1::new(camera.fov_scale() * adj_x))
                        + (u.0 * (-camera.fov_scale() * adj_y)),
                );

                /*j
                let thd_n = std::thread::available_parallelism().unwrap().get();

                let mut col = [BLACK; ITERATIONS];

                for slice in col.chunks_mut(ITERATIONS / thd_n) {
                    std::thread::spawn(move || {
                        for elem in slice {
                            *elem = self.compute_color(&camera.origin, &d);
                        }
                    });
                }
                */

                let mut colors = Vec::with_capacity(ITERATIONS);
                for _ in 0..ITERATIONS {
                    colors.push(self.compute_color(&camera.origin, &d));
                }
                canvas.set_draw_color(colors.avg());
                canvas.draw_point((x as i32, y as i32)).unwrap();
            }
            canvas.present();
        }
        println!("hey");
    }
}
