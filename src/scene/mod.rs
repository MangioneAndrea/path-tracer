use std::time::Instant;

use rand::thread_rng;
use sdl2::render::Canvas;

use crate::algebra::{Unit, Vec3};
use crate::camera::Camera;
use crate::color::{Color, ColorOps, BLACK};

pub(crate) mod cornell_box;

const ITERATIONS: usize = 64;
const STEP: usize = 32;

pub trait Scene {
    fn compute_color(&self, camera: &Vec3, d: &Vec3, rng: &mut rand::rngs::ThreadRng) -> Color;

    fn get_pixels<const W: usize, const H: usize>(
        &self,
        camera: &Camera,
        canvas: &mut Canvas<sdl2::video::Window>,
    ) {
        let aspect_ratio: Unit = (W as Unit) / (H as Unit);
        println!("Aspect ratio {}, {}: {}", W, H, aspect_ratio);

        let direction = camera.direction();
        let r = camera.r();
        let u = camera.u();
        let start = Instant::now();
        for row in (0..H).step_by(STEP) {
            for col in (0..W).step_by(STEP) {
                let pixels = self.compute_square::<W, H>(
                    row,
                    col,
                    aspect_ratio,
                    &direction,
                    &r,
                    &u,
                    &camera,
                );
                for y in row..(row + STEP) {
                    for x in col..(col + STEP) {
                        canvas.set_draw_color(pixels[y - row][x - col]);
                        canvas.draw_point((x as i32, y as i32)).unwrap();
                    }
                }
                canvas.present();
            }
        }

        let duration = start.elapsed();

        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }

    fn compute_square<const W: usize, const H: usize>(
        &self,
        row: usize,
        col: usize,
        aspect_ratio: f32,
        direction: &Vec3,
        r: &Vec3,
        u: &Vec3,
        camera: &Camera,
    ) -> [[Color; STEP]; STEP] {
        let mut pixels = [[BLACK; STEP]; STEP];
        for y in row..(row + STEP) {
            for x in col..(col + STEP) {
                let adj_y: Unit = (y as Unit) / ((H as Unit) / 2.) - 1.;
                let adj_x: Unit = ((x as Unit) / ((W as Unit) / 2.) - 1.) * aspect_ratio;

                let d = Vec3(
                    direction.0
                        + (r.0 * nalgebra::Vector1::new(camera.fov_scale() * adj_x))
                        + (u.0 * (-camera.fov_scale() * adj_y)),
                );
                let mut rng = thread_rng();

                let mut colors = [BLACK; ITERATIONS];
                for chunk in colors.chunks_mut(8) {
                    for color in chunk {
                        *color = self.compute_color(&camera.origin, &d, &mut rng);
                    }
                }

                pixels[y - row][x - col] = colors.to_vec().avg();
            }
        }

        return pixels;
    }
}
