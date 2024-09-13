use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use std::time::Duration;

use crate::memory::Memory;

pub struct Graphics {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    is_showing: bool,
}

impl Graphics {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("Unable to initialise sdl2");
        let video_subsystem = sdl_context
            .video()
            .expect("Unable to initialise video subsystem");

        let window = video_subsystem
            // screen must be in the ratio 64x32
            .window("Chip8 Emulator", 960, 480)
            .position_centered()
            .build()
            .expect("Unable to create window");

        let canvas = window
            .into_canvas()
            .build()
            .expect("Unable to create canvas");
        Self {
            sdl_context,
            canvas,
            is_showing: false,
        }
    }

    pub fn is_showing(&self) -> bool {
        self.is_showing
    }

    pub fn show(&mut self) {
        self.is_showing = true;
    }

    pub fn hide(&mut self) {
        self.is_showing = false;
    }

    pub fn get_event_pump(&self) -> Result<EventPump, String> {
        self.sdl_context.event_pump()
    }

    pub fn render(&mut self, memory: &Memory) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        let (window_width, window_height) = self.canvas.window().size();
        let square_size = window_width / 64;

        self.canvas.set_draw_color(Color::BLACK);

        // drawing the grid squares
        for y in 0..32 {
            for x in 0..64 {
                if memory.get8_framebuffer(x, y) == 1 {
                    self.canvas
                        .fill_rect(Rect::new(
                            (x as i32) * (square_size as i32),
                            (y as i32) * (square_size as i32),
                            square_size,
                            square_size,
                        ))
                        .unwrap();
                }
            }
        }

        // drawing the vertical grid lines
        for i in 1..64 {
            let x = (i as i32) * (window_width as i32 / 64);
            self.canvas
                .draw_line(Point::new(x, 0), Point::new(x, window_height as i32))
                .unwrap();
        }

        // drawing the horizontal grid lines
        for i in 1..32 {
            let y = (i as i32) * (window_height as i32 / 32);
            self.canvas
                .draw_line(Point::new(0, y), Point::new(window_width as i32, y))
                .unwrap();
        }

        if self.is_showing {
            self.canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
