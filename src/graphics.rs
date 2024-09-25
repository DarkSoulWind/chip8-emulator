use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;

use crate::memory::Memory;

pub struct Graphics {
    canvas: Canvas<Window>,
}

impl Graphics {
    pub fn new(sdl_context: &Sdl) -> Self {
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
        Self { canvas }
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

        self.canvas.present();
    }
}
