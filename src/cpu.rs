use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct YARCH8 {
    pc: u16, // only 12 bit = 4096 address possible
    i: u16, // same
    ram: [u8; 4096],
    v_regs: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u8,
    disp_buff: [[bool; 64]; 32],
}

impl YARCH8 {
    pub fn new() -> Self {
        YARCH8 {
            pc: 0x0,
            i: 0x0,
            ram: [0x0; 4096],
            v_regs: [0x0; 16],
            delay_timer: 0x0,
            sound_timer: 0x0,
            stack: [0x0; 16],
            sp: 0x0,
            disp_buff: [[false; 64]; 32],    
        }
    }

    pub fn load(&mut self, rom_path: &str) {
        // Read from rom file and write into memory
        let rom_file = File::open(rom_path).expect("Loading ROM error!");
        
        for (idx, byte) in rom_file.bytes().enumerate() {
            // Memory start from 0x200 as original platform
            self.ram[0x200 + idx] = byte.expect("Byte error in loading ROM!");
        }
    }

    pub fn ram_peek(&self) {
        println!("{:?}", self.ram);
    }

    pub fn start(&mut self) {
        self.pc = 0x200;
    }

    pub fn fetch(&mut self) -> u16 {
        // Read 2B from the current PC address
        let fetch_address = self.pc as usize;
        self.pc += 2;
        ((self.ram[fetch_address] as u16) << 8) + (self.ram[fetch_address+1] as u16)
    }

    pub fn decode_execute(&self, instruction: u16) {
        match instruction & 0x8000 {
            _ => unimplemented!()
        }
    }
}