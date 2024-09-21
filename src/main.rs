extern crate sdl2;
mod emulator;
mod graphics;
mod instruction;
mod key;
mod keyboard;
mod memory;
mod register;
mod sdl_context;

use std::time::Duration;

use emulator::Chip8;
use register::Register;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() {
    // let code = r#"
    //     200: 600A // SET V0, 10
    //     202: 6105 // SET V1, 5
    //     204: A300 // SET I, 300
    //     206: D015 // DRW V0, V1, 5 (draw at position (10, 5))
    //     208: 6219 // SET V2, 25
    //     20A: 631E // SET V3, 30
    //     20C: D232
    //     20E: 120E
    //
    //     300: FF
    //     301: 81
    //     302: 81
    //     303: 81
    //     304: FF
    //     "#;

    let code = r#"
        200: F10A // wait for keypress, store it in V1
        "#;

    let mut chip8 = Chip8::load_from_text(code);
    // chip8.setup_graphics();
    // chip8.show();
    chip8.run();
    // let key = chip8.get8(Register::v_register_from(1) as usize);
    // println!("You pressed key: {key}");
}
