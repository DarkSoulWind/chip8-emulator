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
        200: 600A // LD V0, 10
        202: 6105 // LD V1, 5
        204: A300 // LD I, 300
        206: 64FF // LD V4, 0xFF
        208: F415 // LD DT, V4
        20A: F007 // LD V0, DT
        20C: D015 // DRW V0, V1, 5
        20E: D015 // DRW V0, V1, 5
        210: 4000 // SNE V0, 0
        212: 1206 // JP 206
        214: 120A // JP 20A

        300: FF
        301: 81
        302: 81
        303: 81
        304: FF
        "#;

    let mut chip8 = Chip8::load_from_text(code).setup_sdl();
    chip8.run();
}
