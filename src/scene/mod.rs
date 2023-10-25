use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use crate::algebra::{Unit, Vec3};
use crate::camera::Camera;
use crate::color::{Color, ColorOps, BLACK};
use crate::PixelsBuffer;

pub(crate) mod cornell_box;

const ITERATIONS: usize = 32;

pub trait Scene {
    fn compute_color(&self, camera: &Vec3, d: &Vec3, rng: &mut rand::rngs::ThreadRng) -> Color;

    fn get_pixels<const W: usize, const H: usize, const STEP: usize>(
        &mut self,
        camera: Camera,
        tx: Sender<Box<PixelsBuffer>>,
    ) {
        let aspect_ratio: Unit = (W as Unit) / (H as Unit);

        let direction = camera.direction();
        let r = camera.r();
        let u = camera.u();
        let start = Instant::now();

        let scene = Arc::new(self);

        let handles = (0..H).map(|row| {
            let scene = scene.clone();
            thread::spawn(|| {
                for row in (0..H).step_by(STEP) {
                    for col in (0..W).step_by(STEP) {
                        let mut rng = rand::thread_rng();
                        let mut pb = Box::new(PixelsBuffer::new(row, col));
                        for y in row..(row + STEP) {
                            for x in col..(col + STEP) {
                                let adj_y: Unit = (y as Unit) / ((H as Unit) / 2.) - 1.;
                                let adj_x: Unit =
                                    ((x as Unit) / ((W as Unit) / 2.) - 1.) * aspect_ratio;

                                let d = Vec3(
                                    direction.0
                                        + (r.0
                                            * nalgebra::Vector1::new(camera.fov_scale() * adj_x))
                                        + (u.0 * (-camera.fov_scale() * adj_y)),
                                );

                                let mut colors = [BLACK; ITERATIONS];
                                for chunk in colors.chunks_mut(8) {
                                    for color in chunk {
                                        *color = scene.compute_color(&camera.origin, &d, &mut rng);
                                    }
                                }

                                let c = colors.to_vec().avg().into();
                                pb.pixels[(y - row) * STEP + (x - col)] = c;
                            }
                        }
                        tx.send(pb).unwrap();
                    }
                }
            })
        });

        for handle in handles {
            handle.join();
        }

        let duration = start.elapsed();

        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}
