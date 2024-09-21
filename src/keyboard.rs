use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};
use std::time::Duration;

use crate::key::Key;

#[derive(Debug)]
pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn wait_for_keypress(
        &self,
        /* event_pump: &mut EventPump */
        sdl_context: &Sdl,
    ) -> Result<Key, String> {
        let mut event_pump = sdl_context.event_pump().unwrap();

        'wait: loop {
            /* println!("POLLING EVENTS:"); */
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'wait,
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        println!("{}: {}", key, key.into_i32());
                    }
                    _ => {
                        println!("Ignoring event")
                    }
                }
            }

            // for event in event_pump.poll_iter() {
            //     println!("Event: {:?}", event);
            //     match event {
            //         Event::Quit { .. } => {
            //             break 'wait;
            //         }
            //         Event::KeyDown { keycode, .. } => match keycode {
            //             Some(key) => {
            //                 println!("Pressed {:?}", key);
            //                 return Key::try_from(key);
            //             }
            //             None => {
            //                 println!("No key pressed");
            //             }
            //         },
            //         _ => {
            //             println!("Ignoring event");
            //         }
            //     }
            // }

            /* ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); */
        }

        Err(String::from("how did we get here"))
    }
}
