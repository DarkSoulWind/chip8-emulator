extern crate sdl2;
mod emulator;
mod graphics;
mod instruction;
mod key;
mod memory;
mod register;
mod sdl_context;

use emulator::Chip8;

pub fn main() {
    let code = r#"
        200: 600A // SET V0, 10
        202: 6105 // SET V1, 5
        204: A300 // SET I, 300
        206: D015 // DRW V0, V1, 5 (draw at position (10, 5))
        208: 6219 // SET V2, 25
        20A: 631E // SET V3, 30
        20C: D232
        20E: 00E0
        210: 120E

        300: FF
        301: 81
        302: 81
        303: 81
        304: FF
        "#;

    // let code = r#"
    //     200: F10A // wait for keypress, store it in V1
    //     "#;

    let mut chip8 = Chip8::load_from_text(code);
    chip8.run();
}
