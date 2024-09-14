use sdl2::EventPump;
use sdl2::Sdl;

use crate::graphics::Graphics;
use crate::keyboard::Keyboard;
use crate::memory::Memory;

pub struct SdlContext {
    _sdl_context: Sdl,
    graphics: Option<Graphics>,
    keyboard: Keyboard,
}

impl SdlContext {
    pub fn new() -> Self {
        let _sdl_context = sdl2::init().expect("Unable to initialise sdl2");
        Self {
            graphics: None,
            keyboard: Keyboard::new(),
            _sdl_context,
        }
    }

    pub fn get_event_pump(&self) -> Result<EventPump, String> {
        self._sdl_context.event_pump()
    }

    pub fn setup_graphics(&mut self) {
        match &mut self.graphics {
            Some(_) => {},
            None => {
                self.graphics = Some(Graphics::new(&self._sdl_context));
            },
        }
    }

    pub fn render_graphics(&mut self, memory: &Memory) {
        self.graphics.as_mut().expect("Graphics have not been initialised").render(memory);
    }

    pub fn show_window(&mut self) {
        self.graphics.as_mut().expect("Graphics have not been initialised").show();
    }

    pub fn hide_window(&mut self) {
        self.graphics.as_mut().expect("Graphics have not been initialised").hide();
    }
}
