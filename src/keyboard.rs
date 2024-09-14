// Recommended QWERTY Mapping:
// 1 2 3 4        -->    1 2 3 C
// Q W E R        -->    4 5 6 D
// A S D F        -->    7 8 9 E
// Z X C V        -->    A 0 B F

#[derive(Debug)]
pub struct Keyboard {
    keys: [bool; 16]
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn wait_for_keypress(&self) -> u8 {
        todo!();
    }
}
