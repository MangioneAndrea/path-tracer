use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use crate::algebra::{Unit, Vec3};
use crate::camera::Camera;
use crate::color::{Color, ColorOps, BLACK};
use crate::PixelsBuffer;

pub(crate) mod cornell_box;

const ITERATIONS: usize = 4096;

pub trait Scene: Sync + Send {
    fn compute_color(&self, camera: &Vec3, d: &Vec3, rng: &mut rand::rngs::ThreadRng) -> Color;
}

pub fn get_pixels<const W: usize, const H: usize, const STEP: usize, S>(
    scene: Arc<Box<S>>,
    camera: Camera,
    tx: Sender<Box<PixelsBuffer>>,
) where
    S: Scene + 'static,
{
    let aspect_ratio: Unit = (W as Unit) / (H as Unit);

    let direction = camera.direction();
    let r = camera.r();
    let u = camera.u();
    let start = Instant::now();
    let fov_scale = camera.fov_scale();

    let _: Vec<_> = (0..H)
        .step_by(STEP)
        .map(|row| {
            let mut colors = [BLACK; ITERATIONS];
            let scene = scene.clone();
            let tx = tx.clone();
            thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for col in (0..W).step_by(STEP) {
                    let mut pb = Box::new(PixelsBuffer::new(row, col));
                    for y in row..(row + STEP) {
                        for x in col..(col + STEP) {
                            let adj_y: Unit = (y as Unit) / ((H as Unit) / 2.) - 1.;
                            let adj_x: Unit =
                                ((x as Unit) / ((W as Unit) / 2.) - 1.) * aspect_ratio;

                            let d = Vec3(
                                direction.0
                                    + (r.0 * nalgebra::Vector1::new(fov_scale * adj_x))
                                    + (u.0 * (-fov_scale * adj_y)),
                            );

                            for color in &mut colors {
                                *color = scene.compute_color(&camera.origin, &d, &mut rng);
                            }

                            let c = colors.to_vec().avg().into();
                            pb.pixels[(y - row) * STEP + (x - col)] = c;
                        }
                    }
                    tx.send(pb).unwrap();
                }
            })
        })
        .collect();

    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
