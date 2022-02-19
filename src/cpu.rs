use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Duration;

pub struct YARCH8 {
    pc: u16, // only 12 bit = 4096 address possible
    i: u16,  // same
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

    pub fn get_disp_buff(&self) -> &[[bool; 64]; 32] {
        &self.disp_buff
    }

    pub fn ram_peek(&self) {
        println!("{:?}", self.ram);
    }

    pub fn buffer_peek(&self) {
        println!("{:?}", self.disp_buff);
    }

    pub fn start(&mut self) {
        self.pc = 0x200;
    }

    pub fn stall(&mut self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }

    pub fn fetch(&mut self) -> u16 {
        // Read 2B from the current PC address
        let fetch_address = self.pc as usize;
        self.pc += 2;
        ((self.ram[fetch_address] as u16) << 8) + (self.ram[fetch_address + 1] as u16)
    }

    pub fn reg_peek(&mut self) {
        println!("{:?}", self.v_regs);
    }

    pub fn decode_execute(&mut self, instruction: u16) {
        match instruction & 0xF000 {
            // Clear screen
            0x0000 => self.disp_buff = [[false; 64]; 32],
            // Jump
            0x1000 => self.pc = instruction & 0x0FFF,
            // Set VXNN
            0x6000 => {
                let target_reg: usize = (instruction & 0x0F00) as usize >> 8u8;
                self.v_regs[target_reg] = (instruction & 0x00FF) as u8;
            }
            // Add to Vx N
            0x7000 => {
                let target_reg: usize = (instruction & 0x0F00) as usize >> 8u8;
                self.v_regs[target_reg] += (instruction & 0x00FF) as u8;
            }
            // Set I NN
            0xA000 => self.i = instruction & 0x0FFF,
            // Draw
            0xD000 => {
                // Update display buffer
                let x_reg: usize = (instruction & 0x0F00) as usize >> 8u8;
                let y_reg: usize = (instruction & 0x00F0) as usize >> 4u8;
                let n = (instruction & 0x000F) as usize;

                // Set an init value and restart from here every new line of sprite
                // If we increment by 1 for every sprite, the image is skewed and hit edge...
                let x_init = usize::try_from(self.v_regs[x_reg]).unwrap() % 64;
                let y_init = usize::try_from(self.v_regs[y_reg]).unwrap() % 32;

                // Clear flag register
                self.v_regs[15] = 0;

                for layer in 0..n {
                    let y = y_init + layer;
                    if y >= 32 {
                        // hit bottom so break!
                        break;
                    }

                    // Otw
                    let sprite: u8 = self.ram[usize::from(self.i) + layer];

                    for bit_pos in 0..8 {
                        let x = x_init + bit_pos;
                        if x >= 64 {
                            break;
                        }
                        // If apply bit mask = 0 => the pixel is off, no need shift
                        let b = (sprite & (1 << (7 - bit_pos))) != 0;
                        let prev_pixel = self.disp_buff[y][x];
                        self.disp_buff[y][x] = b ^ prev_pixel;
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}
