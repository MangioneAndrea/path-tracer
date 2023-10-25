extern crate sdl2;

use color::{Color, BLACK, WHITE};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;

use std::env;

use scene::cornell_box;

use crate::camera::Camera;
use crate::scene::cornell_box::CornellBox;
use crate::scene::get_pixels;

pub(crate) mod algebra;
pub(crate) mod camera;
pub(crate) mod color;
pub(crate) mod mesh;
pub(crate) mod scene;

const W: usize = 1024;
const H: usize = 768;
const STEP: usize = 32;

pub struct PixelsBuffer {
    row: usize,
    col: usize,
    w: usize,
    h: usize,
    pixels: [Color; STEP * STEP],
}

impl PixelsBuffer {
    pub fn new(row: usize, col: usize) -> PixelsBuffer {
        PixelsBuffer {
            row,
            col,
            pixels: [BLACK; STEP * STEP],
            w: STEP,
            h: STEP,
        }
    }
}

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

    let target_scene = Arc::new(Box::new(cornell_box::new()));
    let camera = Camera::default();

    let (tx, rx) = channel::<Box<PixelsBuffer>>();

    let mut event_pump = sdl_context.event_pump()?;
    get_pixels::<W, H, STEP, CornellBox>(target_scene, camera, tx);

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(None, W as u32, H as u32)
        .unwrap();

    println!("{:?}", texture.query());

    'running: loop {
        if let Ok(data) = rx.try_recv() {
            texture
                .with_lock(
                    sdl2::rect::Rect::new(
                        data.col as i32,
                        (data.row) as i32,
                        STEP as u32,
                        STEP as u32,
                    ),
                    |buffer: &mut [u8], _: usize| {
                        for y in 0..STEP {
                            for x in 0..STEP {
                                let global_idx = (y * STEP + x) * 4;
                                let color: sdl2::pixels::Color = data.pixels[y * STEP + x].into();
                                buffer[global_idx + 3] = 255;
                                buffer[global_idx + 2] = color.r;
                                buffer[global_idx + 1] = color.g;
                                buffer[global_idx + 0] = color.b;
                            }
                        }
                    },
                )
                .unwrap();
        }
        canvas.copy(&texture, None, None).unwrap();

        canvas.present();

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

        std::thread::sleep(Duration::from_micros(10));
    }

    Ok(())
}
