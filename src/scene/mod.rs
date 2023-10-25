use std::sync::mpsc::Sender;
use std::thread;
use std::time::Instant;

use crate::algebra::{Unit, Vec3};
use crate::camera::Camera;
use crate::color::{Color, ColorOps, BLACK};
use crate::PixelsBuffer;

pub(crate) mod cornell_box;

const ITERATIONS: usize = 1;

pub trait Scene {
    fn compute_color(&mut self, camera: &Vec3, d: &Vec3) -> Color;

    fn get_pixels<const W: usize, const H: usize, const STEP: usize>(
        &mut self,
        camera: &Camera,
        tx: Sender<Box<PixelsBuffer>>,
    ) {
        let aspect_ratio: Unit = (W as Unit) / (H as Unit);

        let direction = camera.direction();
        let r = camera.r();
        let u = camera.u();
        let start = Instant::now();
        for row in (0..H).step_by(STEP) {
            for col in (0..W).step_by(STEP) {
                let mut pb = Box::new(PixelsBuffer::new(row, col));
                for y in row..(row + STEP) {
                    for x in col..(col + STEP) {
                        let adj_y: Unit = (y as Unit) / ((H as Unit) / 2.) - 1.;
                        let adj_x: Unit = ((x as Unit) / ((W as Unit) / 2.) - 1.) * aspect_ratio;

                        let d = Vec3(
                            direction.0
                                + (r.0 * nalgebra::Vector1::new(camera.fov_scale() * adj_x))
                                + (u.0 * (-camera.fov_scale() * adj_y)),
                        );

                        let mut colors = [BLACK; ITERATIONS];
                        for chunk in colors.chunks_mut(8) {
                            for color in chunk {
                                *color = self.compute_color(&camera.origin, &d);
                            }
                        }

                        let c: sdl2::pixels::Color = colors.to_vec().avg().into();
                        pb.pixels[(y - row) * STEP + (x - col) * 4] = c.r;
                        pb.pixels[((y - row) * STEP + (x - col) * 4) + 1] = c.g;
                        pb.pixels[((y - row) * STEP + (x - col) * 4) + 2] = c.b;
                        pb.pixels[((y - row) * STEP + (x - col) * 4) + 3] = c.a;
                    }
                }
                tx.send(pb).unwrap();
            }
        }

        let duration = start.elapsed();

        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
