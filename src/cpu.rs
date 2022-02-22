use std::fs::File;
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
    sp: usize,
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
        // Read from rom file and write into memory, from 0x200 onwards
        let rom_file = File::open(rom_path).expect("Loading ROM error!");
        for (idx, byte) in rom_file.bytes().enumerate() {
            self.ram[0x200 + idx] = byte.expect("Byte error in loading ROM!");
        }
    }

    pub fn start(&mut self) {
        self.pc = 0x200;
    }

    pub fn stall(&self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }

    pub fn fetch(&mut self) -> u16 {
        // Read 2B from the current PC address
        let fetch_address = self.pc as usize;
        self.pc += 2;
        ((self.ram[fetch_address] as u16) << 8) + (self.ram[fetch_address + 1] as u16)
    }

    pub fn decode_execute(&mut self, instruction: u16) {
        // Extract
        let vx: usize = self.get_vx(instruction);
        let vy: usize = self.get_vy(instruction);
        let nnn: u16 = self.get_nnn(instruction);
        let nn: u8 = self.get_nn(instruction);
        let n: u8 = self.get_n(instruction);

        // Decode
        match instruction & 0xF000 {
            0x0000 => {
                if nn == 0xE0 {
                    // Clear screen
                    self.disp_buff = [[false; 64]; 32];
                } else if nn == 0xEE {
                    // Return from routine
                    if self.sp == 0 {
                        panic!("Return failed. No return address in stack!");
                    }
                    self.pc = self.stack[self.sp - 1];
                    // Better clear stack
                    self.stack[self.sp - 1] = 0x0000;
                    self.sp -= 1;
                } else {
                    // Throw error
                    panic!()
                }
            }
            // Jump
            0x1000 => self.pc = nnn,
            // Call routine
            0x2000 => {
                if self.sp >= 16 {
                    // Stack overflow
                    panic!("Call routine failed as stack overflow...");
                }
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            // Skips or Nops
            0x3000 => {
                if self.v_regs[vx] == nn {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.v_regs[vx] != nn {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.v_regs[vx] == self.v_regs[vy] {
                    self.pc += 2;
                }
            }
            // Set VXNN
            0x6000 => {
                self.v_regs[vx] = nn;
            }
            // Add to Vx NN
            0x7000 => {
                self.v_regs[vx] += nn;
            }
            0x9000 => {
                if self.v_regs[vx] != self.v_regs[vy] {
                    self.pc += 2;
                }
            }
            // Set I NN
            0xA000 => self.i = nnn,
            // Draw
            0xD000 => {
                // Set an init value and restart from here every new line of sprite
                // If we increment by 1 for every sprite, the image is skewed and hit edge...
                let x_init = usize::try_from(self.v_regs[vx]).unwrap() % 64;
                let y_init = usize::try_from(self.v_regs[vy]).unwrap() % 32;

                // Clear flag register
                self.v_regs[15] = 0;

                for layer in 0..(n as usize) {
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
                        // Set VF if needed
                        if b && prev_pixel {
                            self.v_regs[15] = 1;
                        }
                        self.disp_buff[y][x] = b ^ prev_pixel;
                    }
                }
            }
            _ => unimplemented!(),
        }
    }

    /* UTIL FUNCTIONS:
    - Get some register from instructions, etc
    */
    fn get_vx(&self, instruction: u16) -> usize {
        (instruction & 0x0F00) as usize >> 8u8
    }

    fn get_vy(&self, instruction: u16) -> usize {
        (instruction & 0x00F0) as usize >> 4u8
    }

    fn get_n(&self, instruction: u16) -> u8 {
        (instruction & 0x000F) as u8
    }

    fn get_nn(&self, instruction: u16) -> u8 {
        (instruction & 0x00FF) as u8
    }

    fn get_nnn(&self, instruction: u16) -> u16 {
        instruction & 0x0FFF
    }

    /* DEBUG FUNCTIONS:
        Print out stuffs for debugging
    */
    pub fn get_disp_buff(&self) -> &[[bool; 64]; 32] {
        &self.disp_buff
    }

    pub fn ram_peek(&self) {
        println!("{:?}", self.ram);
    }

    pub fn buffer_peek(&self) {
        println!("{:?}", self.disp_buff);
    }

    pub fn reg_peek(&self) {
        println!("{:?}", self.v_regs);
    }
}
