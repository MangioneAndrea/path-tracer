use std::env;

use scene::{cornell_box, Scene};

pub(crate) mod algebra;
pub(crate) mod camera;
pub(crate) mod color;
pub(crate) mod mesh;
pub(crate) mod scene;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let camera = camera::Camera::default();
    let target_scene = cornell_box::new();

    target_scene.get_pixels::<400, 400>(&camera);
}
