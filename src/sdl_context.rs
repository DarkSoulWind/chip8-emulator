use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::Sdl;

use crate::graphics::Graphics;
use crate::key::Key;
use crate::memory::Memory;

pub struct SdlContext {
    _sdl_context: Sdl,
    graphics: Graphics,
    waiting_for_keypress: bool,
}

impl SdlContext {
    pub fn new() -> Self {
        let _sdl_context = sdl2::init().expect("Unable to initialise sdl2");
        Self {
            graphics: Graphics::new(&_sdl_context),
            waiting_for_keypress: false,
            _sdl_context,
        }
    }

    pub fn render_graphics(&mut self, memory: &Memory) {
        self.graphics.render(memory);
    }

    pub fn wait_for_keypress(&mut self) {
        self.waiting_for_keypress = true;
    }

    pub fn handle_input(&mut self) -> Result<(), &str> {
        let mut event_pump = self._sdl_context.event_pump().unwrap();

        'waiting: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return Err("QUIT"),
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        if self.waiting_for_keypress {
                            let the_key = Key::try_from(key).unwrap();
                            println!("{}: {} ({:?})", key, key.into_i32(), the_key);
                            self.waiting_for_keypress = false;
                        }
                    }
                    _ => {}
                }
            }

            if !self.waiting_for_keypress {
                break 'waiting;
            }
        }

        Ok(())
    }
}
