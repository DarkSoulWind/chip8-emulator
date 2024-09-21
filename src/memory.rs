#[derive(Debug)]
pub struct Memory {
    data: [u8; 4096],
    framebuffer: [u8; 64 * 32],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; 4096],
            framebuffer: [0; 64 * 32],
        }
    }

    pub fn get8(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn get16(&self, index: usize) -> u16 {
        let n1 = self.data[index] as u16;
        let n2 = self.data[index + 1] as u16;
        (n1 << 8) + n2
    }

    pub fn set8(&mut self, index: usize, value: u8) {
        self.data[index] = value;
    }

    pub fn set16(&mut self, index: usize, value: u16) {
        let n1 = (value >> 8) as u8;
        let n2 = (value & 0xFF) as u8;
        self.data[index] = n1;
        self.data[index + 1] = n2;
    }

    pub fn set8_framebuffer(&mut self, x: u8, y: u8, value: u8) {
        self.framebuffer[self.get_framebuffer_location(x as usize, y as usize)] = value;
    }

    pub fn get8_framebuffer(&self, x: u8, y: u8) -> u8 {
        self.framebuffer[self.get_framebuffer_location(x as usize, y as usize)]
    }

    pub fn get_framebuffer_location(&self, x: usize, y: usize) -> usize {
        (y * 64) + x
    }

    pub fn clear_framebuffer(&mut self) {
        for i in 0..(64 * 32) {
            self.framebuffer[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Register;

    #[test]
    fn test_set8() {
        let mut memory = Memory::new();
        memory.set8(Register::V0 as usize, 0x12);
        assert_eq!(memory.get8(Register::V0 as usize), 0x12);
    }

    #[test]
    fn test_set16_1() {
        let mut memory = Memory::new();
        memory.set16(Register::PC as usize, 0x200);
        assert_eq!(memory.get16(Register::PC as usize), 0x200);
    }

    #[test]
    fn test_set16_2() {
        let mut memory = Memory::new();
        memory.set16(0x206, 0xD015);
        assert_eq!(memory.get16(0x206), 0xD015);
    }

    #[test]
    fn test_get_framebuffer_value() {
        let mut memory = Memory::new();
        memory.set8_framebuffer(10, 10, 0b10011001);
        assert_eq!(memory.get8_framebuffer(10, 10), 0b10011001);
    }
}
