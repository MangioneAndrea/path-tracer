extern crate sdl2;

use color::{Color, BLACK, WHITE};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::sync::mpsc::channel;
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

    let mut target_scene = cornell_box::new();
    let camera = Camera::default();

    let (tx, rx) = channel::<Box<PixelsBuffer>>();

    let mut event_pump = sdl_context.event_pump()?;
    std::thread::spawn(move || target_scene.get_pixels::<W, H, STEP>(camera, tx));

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(None, W as u32, H as u32)
        .unwrap();

    println!("{:?}", texture.query());

    'running: loop {
        while let Ok(data) = rx.try_recv() {
            texture
                .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    let row_padding = data.row;
                    let col_padding = data.col;
                    for y in 0..STEP {
                        for x in 0..STEP {
                            let global_idx = (y + row_padding) * W + x + col_padding;
                            let color: sdl2::pixels::Color = data.pixels[y * STEP + x].into();
                            buffer[global_idx * 4 + 0] = color.b;
                            buffer[global_idx * 4 + 1] = color.g;
                            buffer[global_idx * 4 + 2] = color.r;
                            buffer[global_idx * 4 + 3] = 255;
                        }
                    }
                })
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

        std::thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
