extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use std::env;

use scene::cornell_box;

use crate::camera::Camera;
use crate::scene::Scene;

pub(crate) mod algebra;
pub(crate) mod camera;
pub(crate) mod color;
pub(crate) mod mesh;
pub(crate) mod scene;

const W: usize = 800;
const H: usize = 600;

fn main() -> Result<(), String> {
    println!("{:?}", env::var_os("RUST_BACKTRACE"));

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", W as u32, H as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    canvas.present();

    let mut target_scene = cornell_box::new();
    let camera = Camera::default();

    let mut event_pump = sdl_context.event_pump()?;
    target_scene.get_pixels::<W, H>(&camera, &mut canvas);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
