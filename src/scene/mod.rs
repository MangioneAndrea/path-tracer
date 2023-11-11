use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use crate::algebra::{Unit, Vec3};
use crate::camera::Camera;
use crate::color::{Color, ColorOps, BLACK};
use crate::PixelsBuffer;
use crate::mesh::Mesh;

pub(crate) mod cornell_box;

const ITERATIONS: usize = 64;

pub trait Scene: Sync + Send {
    fn compute_color(&self, camera: &Vec3, d: &Vec3, rng: &mut rand::rngs::ThreadRng) -> Color;
    fn get_meshes(&self) -> &Vec<&dyn Mesh>;
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
    let fov_scale = camera.fov_scale();

    let threads_n = std::thread::available_parallelism()
        .expect("Windows macos and linux know the amount of threads")
        .get();

    let work_per_thread = H / threads_n;

    for thd in 0..threads_n {
        let scene = scene.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            for row in ((thd * work_per_thread)..((thd + 1) * work_per_thread)).step_by(STEP) {
                let mut colors = [BLACK; ITERATIONS];
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
            }
        });
    }
}
