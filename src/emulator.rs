use std::time::Duration;

use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::register::Register;
use crate::sdl_context::SdlContext;

pub struct Chip8 {
    memory: Memory,
    sdl_context: Option<SdlContext>,
    delay_timer: u32,
}

impl Chip8 {
    pub fn load_from_file(filepath: &str) -> Result<Self, &str> {
        let contents = std::fs::read_to_string(filepath);
        match contents {
            Ok(data) => Ok(Chip8::load_from_text(&data)),
            Err(_) => Err("Unable to read file contents"),
        }
    }

    pub fn load_from_text(data: &str) -> Self {
        let mut memory = Memory::new();
        memory.set16(Register::PC as usize, 0x200);

        let mut chip8 = Chip8 {
            memory,
            sdl_context: None,
            delay_timer: 0,
        };
        chip8.read_data(&data);

        chip8
    }

    pub fn setup_sdl(mut self) -> Self {
        self.sdl_context = Some(SdlContext::new());
        self
    }

    fn read_data(&mut self, data: &str) {
        let data = data.to_owned();
        // for debugging purposes
        let mut line_number = 0;
        for line in data.split("\n") {
            // remove leading and trailing whitespace
            let line = line.trim();
            line_number += 1;

            // skip if the line is a comment
            if line.starts_with("//") {
                continue;
            }

            // skip if the line is empty
            if line == "" {
                continue;
            }

            /* println!("line: {:?}", &line); */
            self.read_line(line, line_number);
            /* println!("{}", format!("address: {} instruction: {:X}", address, instruction)); */
        }
    }

    fn read_line(&mut self, line: &str, line_number: i32) {
        let mut words = line.split_whitespace(); // split the line into words

        let mut address_text = words
            .next()
            .expect(&format!("{line_number}: Could not read address text"))
            .to_owned();
        address_text.pop(); // remove colon at the end of address
        let address = usize::from_str_radix(&address_text, 16).expect(&format!(
            "{line_number}: Could not convert address to usize"
        ));

        let instruction_text = words
            .next()
            .expect(&format!("{line_number}: Could not read instruction text"));

        // checking whether the instruction is 8 or 16 bits long
        match instruction_text.len() {
            2 => {
                let instruction = u8::from_str_radix(instruction_text, 16).expect(&format!(
                    "{line_number}: Could not convert instruction to u8"
                ));
                self.memory.set8(address, instruction);
            }
            4 => {
                let instruction = u16::from_str_radix(instruction_text, 16).expect(&format!(
                    "{line_number}: Could not convert instruction to u16"
                ));
                self.memory.set16(address, instruction);
            }
            _ => {
                panic!("{}", &format!("{line_number}: FUUUUUUUCK"));
            }
        };
    }

    pub fn cycle(&mut self) -> i8 {
        let next_instruction = self.fetch();
        if next_instruction == 0 {
            return -1;
        }
        let instruction = Instruction::decode(next_instruction);
        self.execute(instruction);

        0
    }

    pub fn run(&mut self) {
        let timer_frequency: f32 = 1.0 / 60.0;
        let mut elapsed_time: f32 = 0.0;

        'fde: loop {
            let delta_time = self.sdl_context.as_mut().unwrap().get_delta_time();
            /* println!("Delta time: {delta_time}s"); */

            elapsed_time += delta_time;

            if elapsed_time >= timer_frequency {
                if self.delay_timer > 0 {
                    println!("Delay timer: {}", self.delay_timer);
                    self.delay_timer -= 1;
                }

                elapsed_time -= timer_frequency;
            }

            self.sdl_context
                .as_mut()
                .expect("SDL context not initialised")
                .render_graphics(&self.memory);
            self.sdl_context
                .as_mut()
                .expect("SDL context not initialised")
                .handle_input()
                .unwrap();

            if self.cycle() == -1 {
                break 'fde;
            }

            // sleep for 1/60th of a second
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    // run the emulator without requiring SDL context
    // used only for testing purposes
    pub fn test_run(&mut self) {
        'fde: loop {
            if self.cycle() == -1 {
                break 'fde;
            }

            // sleep for 1/60th of a second
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    pub fn fetch(&mut self) -> u16 {
        // fetch instruction at PC and add 2 to PC
        let next_instruction_address = self.memory.get16(Register::PC as usize);
        self.memory
            .set16(Register::PC as usize, next_instruction_address + 2);
        let next_instruction = self.memory.get16(next_instruction_address as usize);
        next_instruction
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::CLS => {
                self.memory.clear_framebuffer();
            }
            Instruction::JP(nnn) => {
                self.memory.set16(Register::PC as usize, nnn);
            }
            Instruction::LDImm(vx, value) => {
                self.memory.set8(vx as usize, value);
            }
            Instruction::ADDImm(vx, value) => {
                let new_value = self.memory.get8(vx.clone() as usize) + value;
                self.memory.set8(vx as usize, new_value);
            }
            Instruction::LDDir(vx, vy) => {
                self.memory.set8(vx as usize, self.get8(vy as usize));
            }
            Instruction::OR(vx, vy) => {
                let vx_value = self.memory.get8(vx.clone() as usize);
                let vy_value = self.memory.get8(vy as usize);
                self.memory.set8(vx as usize, vx_value | vy_value);
            }
            Instruction::AND(vx, vy) => {
                let vx_value = self.memory.get8(vx.clone() as usize);
                let vy_value = self.memory.get8(vy as usize);
                self.memory.set8(vx as usize, vx_value & vy_value);
            }
            Instruction::XOR(vx, vy) => {
                let vx_value = self.memory.get8(vx.clone() as usize);
                let vy_value = self.memory.get8(vy as usize);
                self.memory.set8(vx as usize, vx_value ^ vy_value);
            }
            Instruction::ADDDir(vx, vy) => {
                let vx_value = self.memory.get8(vx.clone() as usize);
                let vy_value = self.memory.get8(vy as usize);
                self.memory.set8(vx as usize, vx_value + vy_value);
            }
            Instruction::SUB(vx, vy) => {
                let vx_value = self.memory.get8(vx.clone() as usize);
                let vy_value = self.memory.get8(vy as usize);
                self.memory
                    .set8(vx as usize, vx_value.wrapping_sub(vy_value));
                // if borrow occured (Vx < Vy) then set VF to 0
                // otherwise set VF to 1
                self.memory.set8(
                    Register::v_register_from(0xF) as usize,
                    if vx_value < vy_value { 0 } else { 1 },
                );
            }
            Instruction::LDI(location) => {
                self.memory.set16(Register::IR as usize, location);
            }
            Instruction::JPOff(offset) => {
                let v0_value = self.memory.get8(Register::v_register_from(0) as usize) as u16;
                self.memory.set16(Register::PC as usize, v0_value + offset);
            }
            Instruction::DRW(vx, vy, height) => {
                self.draw_update(Instruction::DRW(vx, vy, height));
            }
            Instruction::LDK(vx) => {
                println!("Waiting for keypress...");
                self.sdl_context.as_mut().unwrap().wait_for_keypress();
                /* self.memory.set8(vx as usize, key as u8); */
            }
            Instruction::SEImm(vx, nn) => {
                if self.memory.get8(vx as usize) == nn {
                    self.fetch();
                }
            }
            Instruction::SNE(vx, nn) => {
                if self.memory.get8(vx as usize) != nn {
                    self.fetch();
                }
            }
            Instruction::SEDir(vx, vy) => {
                if self.memory.get8(vx as usize) != self.memory.get8(vy as usize) {
                    self.fetch();
                }
            }
            Instruction::LDDT(vx) => {
                // set the delay timer to the value of vx
                let vx_value = self.memory.get8(vx as usize);
                self.delay_timer = vx_value as u32;
            }
            Instruction::LDVDT(vx) => {
                // set the value of vx to the delay timer
                self.memory.set8(vx as usize, self.delay_timer as u8);
            }
        }
    }

    pub fn draw_update(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::DRW(vx, vy, height) => {
                let x_position = self.memory.get8(vx as usize);
                let y_position = self.memory.get8(vy as usize);
                let index_location = self.memory.get16(Register::IR as usize);
                for i in 0..height {
                    let new_byte_data = self.memory.get8((index_location as usize) + (i as usize));
                    for j in 0..8 {
                        let x_position_wrapped = x_position.wrapping_add(j) % 64;
                        let y_position_wrapped = y_position.wrapping_add(i) % 32;
                        let old_bit_data =
                            self.memory.get8_framebuffer(x_position_wrapped, y_position_wrapped);
                        let new_bit_data = (new_byte_data >> j) & 1;
                        let xored = new_bit_data ^ old_bit_data;

                        // check if the pixel will be "unset"
                        // set VF to 1 if true
                        self.memory.set8(
                            Register::v_register_from(0xF) as usize,
                            (old_bit_data > xored) as u8,
                        );

                        self.memory
                            .set8_framebuffer(x_position_wrapped, y_position_wrapped, xored);
                    }
                }
            }
            _ => {
                panic!("Not a draw call")
            }
        }
    }

    // helper functions for testing
    pub fn get16(&self, index: usize) -> u16 {
        self.memory.get16(index)
    }

    pub fn get8(&self, index: usize) -> u8 {
        self.memory.get8(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let chip8 = Chip8::load_from_file("./example_code.txt").unwrap();
        assert_eq!(chip8.get16(0x206), 0xD015);
    }

    #[test]
    fn test_read_text() {
        let code = r#"
    200: A202
    202: 6000
    204: 6100
    206: D015
    208: F0
    209: 90
    "#;
        let chip8 = Chip8::load_from_text(code);
        assert_eq!(chip8.get16(0x206), 0xD015);
    }

    #[test]
    fn test_fetch() {
        let mut chip8 = Chip8::load_from_file("./example_code.txt").unwrap();
        // PC should always be set to 0x200 initially
        assert_eq!(chip8.fetch(), 0xA202);
    }

    #[test]
    fn test_fetch_increment() {
        let mut chip8 = Chip8::load_from_file("./example_code.txt").unwrap();
        chip8.fetch();
        assert_eq!(chip8.get16(Register::PC as usize), 0x202);
    }

    #[test]
    fn test_execute_add_imm() {
        let code = r#"
    200: 6001
    202: 7001
    "#;
        let mut chip8 = Chip8::load_from_text(code);
        chip8.test_run();
        assert_eq!(chip8.get8(Register::v_register_from(0) as usize), 0x02)
    }

    #[test]
    fn test_execute_ld_dir() {
        let code = r#"
    200: 6000
    202: 6101
    204: 8010 // LD v0, v1
    "#;
        let mut chip8 = Chip8::load_from_text(code);
        chip8.test_run();
        assert_eq!(chip8.get8(Register::v_register_from(0) as usize), 0x01)
    }

    #[test]
    fn test_execute_or() {
        let code = r#"
    200: 6000
    202: 6101
    204: 8011 // OR v0, v1
    "#;
        let mut chip8 = Chip8::load_from_text(code);
        chip8.test_run();
        assert_eq!(chip8.get8(Register::v_register_from(0) as usize), 0x01)
    }

    #[test]
    fn test_execute_and() {
        let code = r#"
    200: 6000
    202: 6101
    204: 8012 // AND v0, v1
    "#;
        let mut chip8 = Chip8::load_from_text(code);
        chip8.test_run();
        assert_eq!(chip8.get8(Register::v_register_from(0) as usize), 0x00)
    }

    #[test]
    fn test_execute_xor() {
        let code = r#"
    200: 6007
    202: 6101
    204: 8013 // XOR v0, v1
    "#;
        let mut chip8 = Chip8::load_from_text(code);
        chip8.test_run();
        assert_eq!(chip8.get8(Register::v_register_from(0) as usize), 0x06)
    }

    #[test]
    fn test_execute_add_dir() {
        let code = r#"
    200: 6001
    202: 6101
    204: 8014 // ADD v0, v1
    "#;
        let mut chip8 = Chip8::load_from_text(code);
        chip8.test_run();
        assert_eq!(chip8.get8(Register::v_register_from(0) as usize), 0x02)
    }

    #[test]
    fn test_execute_sub() {
        // no borrowing
        {
            let code = r#"
    200: 6009
    202: 6103
    204: 8015 // SUB v0, v1
    "#;
            let mut chip8 = Chip8::load_from_text(code);
            chip8.test_run();
            assert_eq!(chip8.get8(Register::v_register_from(0) as usize), 0x06);
            assert_eq!(chip8.get8(Register::v_register_from(0xF) as usize), 1);
        }
        // borrowing
        {
            let code = r#"
    200: 6003
    202: 6105
    204: 8015 // SUB v0, v1
    "#;
            let mut chip8 = Chip8::load_from_text(code);
            chip8.test_run();
            assert_eq!(chip8.get8(Register::v_register_from(0) as usize), 0xFE);
            assert_eq!(chip8.get8(Register::v_register_from(0xF) as usize), 0);
        }
    }

    #[test]
    fn test_execute_ldi() {
        let code = r#"
    200: A300 // SET IR, 0x300
    "#;
        let mut chip8 = Chip8::load_from_text(code);
        chip8.test_run();
        assert_eq!(chip8.get16(Register::IR as usize), 0x300)
    }
}
