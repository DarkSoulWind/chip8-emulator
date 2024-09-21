use sdl2::keyboard::Keycode;

#[derive(Debug)]
pub enum Key {
    Code0,
    Code1,
    Code2,
    Code3,
    Code4,
    Code5,
    Code6,
    Code7,
    Code8,
    Code9,
    CodeA,
    CodeB,
    CodeC,
    CodeD,
    CodeE,
    CodeF,
}

impl TryFrom<Keycode> for Key {
    type Error = String;

    fn try_from(value: Keycode) -> Result<Self, Self::Error> {
        // Recommended QWERTY Mapping:
        // 1 2 3 4        -->    1 2 3 C
        // Q W E R        -->    4 5 6 D
        // A S D F        -->    7 8 9 E
        // Z X C V        -->    A 0 B F

        match value {
            Keycode::Num1 => Ok(Key::Code1),
            Keycode::Num2 => Ok(Key::Code2),
            Keycode::Num3 => Ok(Key::Code3),
            Keycode::Num4 => Ok(Key::CodeC),
            Keycode::Q => Ok(Key::Code4),
            Keycode::W => Ok(Key::Code5),
            Keycode::E => Ok(Key::Code6),
            Keycode::R => Ok(Key::CodeD),
            Keycode::A => Ok(Key::Code7),
            Keycode::S => Ok(Key::Code8),
            Keycode::D => Ok(Key::Code9),
            Keycode::F => Ok(Key::CodeE),
            Keycode::Z => Ok(Key::CodeA),
            Keycode::X => Ok(Key::Code0),
            Keycode::C => Ok(Key::CodeB),
            Keycode::V => Ok(Key::CodeF),
            _ => Err(format!("Invalid keycode {value}")),
        }
    }
}
