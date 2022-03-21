use rand;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

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
    keys: [bool; 16], // 16 keys pressed or not pressed
    delay_time_start: Instant,
    sound_time_start: Instant,
    timer_req_duration: Duration,
    cycle_req_duration: Duration,
}

impl YARCH8 {
    pub fn new(timer_freq: u32, cycle_freq:u32) -> Self {
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
            keys: [false; 16],
            delay_time_start: Instant::now(),
            sound_time_start: Instant::now(),
            timer_req_duration: Duration::new(0, 1_000_000_000u32/timer_freq),
            cycle_req_duration: Duration::new(0, 1_000_000_000u32/cycle_freq),
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
        self.store_font();
        self.pc = 0x200;
    }

    pub fn stall(&self) {
        ::std::thread::sleep(self.cycle_req_duration);
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
                let (wrapped_sum, _) = self.v_regs[vx].overflowing_add(nn);
                self.v_regs[vx] = wrapped_sum;
            }
            // Arithmetics...
            0x8000 => match n {
                0 => self.v_regs[vx] = self.v_regs[vy],
                1 => self.v_regs[vx] |= self.v_regs[vy],
                2 => self.v_regs[vx] &= self.v_regs[vy],
                3 => self.v_regs[vx] ^= self.v_regs[vy],
                4 => {
                    let (wrapped_sum, is_overflow) =
                        self.v_regs[vx].overflowing_add(self.v_regs[vy]);
                    self.v_regs[vx] = wrapped_sum;
                    if is_overflow {
                        self.v_regs[15] = 0x1;
                    } else {
                        self.v_regs[15] = 0x0;
                    }
                }
                5 => {
                    // VX = VX - VY
                    let (result, is_overflow) = self.v_regs[vx].overflowing_sub(self.v_regs[vy]);
                    self.v_regs[vx] = result;
                    if is_overflow {
                        self.v_regs[15] = 0x0;
                    } else {
                        self.v_regs[15] = 0x1;
                    }
                }
                6 => {
                    // TODO: Config to handle ambiguity
                    // Default to modern
                    // Right shift
                    //self.v_regs[vx] = self.v_regs[vy];
                    self.v_regs[15] = self.v_regs[vx] & 0x01;
                    self.v_regs[vx] = self.v_regs[vx] >> 1;
                }
                7 => {
                    // VX = VY - VX
                    let (result, is_overflow) = self.v_regs[vy].overflowing_sub(self.v_regs[vx]);
                    self.v_regs[vx] = result;
                    if is_overflow {
                        self.v_regs[15] = 0x0;
                    } else {
                        self.v_regs[15] = 0x1;
                    }
                }
                0xE => {
                    // TODO: Config to handle ambiguity
                    // Default to modern
                    // Left shift
                    //self.v_regs[vx] = self.v_regs[vy];
                    self.v_regs[15] = (self.v_regs[vx] & 0x80) >> 7;
                    self.v_regs[vx] = self.v_regs[vx] << 1;
                }
                _ => unimplemented!(),
            },
            0x9000 => {
                if self.v_regs[vx] != self.v_regs[vy] {
                    self.pc += 2;
                }
            }
            // Set I NN
            0xA000 => self.i = nnn,
            0xB000 => {
                // TODO: Adding a check if we are using OG chip-8 or not to allow
                // multiple behaviour support

                // Assume that CHIP-8 OG is used for now
                // Jump to NNN + V0 content
                self.pc = nnn + u16::from(self.v_regs[0]);
            }
            0xC000 => {
                // Gen random number, AND with NN and store in VX
                let nonce: u8 = rand::random();
                self.v_regs[vx] = nonce & nn;
            }
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
            0xE000 => match nn {
                0x9E => {
                    // if VX 's value key is pressed, skip (PC +2)
                    if self.keys[self.v_regs[vx] as usize] {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    // if VX 's value key is not pressed, skip (PC +2)
                    if !self.keys[self.v_regs[vx] as usize] {
                        self.pc += 2;
                    }
                }
                _ => unimplemented!(),
            },
            0xF000 => match nn {
                0x07 => {
                    // Set VX to delay_timer value
                    self.v_regs[vx] = self.delay_timer;
                }
                0x0A => {
                    // Get key pressed, otherwise skip
                    // Deviate from original behaviour
                    // Just get the first one pressed from the list
                    if self.keys.iter().any(|&k| k) {
                        let (keypressed_idx, _) = *self
                            .keys
                            .iter()
                            .enumerate()
                            .filter(|(_, &k)| k)
                            .collect::<Vec<_>>()
                            .first()
                            .unwrap();
                        self.v_regs[vx] = keypressed_idx as u8;
                    } else {
                        // Revert value of PC to basically blocking...
                        self.pc -= 2;
                    }
                }
                0x15 => {
                    // Set delay timer to vx
                    self.delay_timer = self.v_regs[vx];
                    self.delay_time_start = Instant::now();
                }
                0x18 => {
                    // Set sound timer to vx
                    self.sound_timer = self.v_regs[vx];
                    self.sound_time_start = Instant::now();
                }
                0x1E => {
                    //add to idx
                    // Here we do not care about setting VF.
                    // TODO: handle VF if overflow...
                    self.i += u16::from(self.v_regs[vx]);
                }
                0x29 => {
                    // Font char
                    // Take lower of vx reg as char
                    let font_base = 0x50;
                    let offset = self.v_regs[vx] * 5;
                    // Set index reg to the address = font_base + offset
                    self.i = u16::from(font_base + offset);
                }
                0x33 => {
                    // Take digits in VX and write in I, I + 1, ...
                    let mut num_digit = self.v_regs[vx].to_string().len();
                    let mut num = self.v_regs[vx];
                    while num != 0 {
                        let digit = num.rem_euclid(10);
                        self.ram[usize::from(self.i) + num_digit - 1] = digit;
                        num_digit -= 1;
                        num = num.div_euclid(10);
                    }
                }
                0x55 => {
                    // TODO: Ambiguous instruction
                    // Load
                    for idx in 0..=vx {
                        self.ram[self.i as usize + idx] = self.v_regs[idx];
                    }
                }
                0x65 => {
                    // TODO: Ambiguous instruction
                    // Store
                    for idx in 0..=vx {
                        self.v_regs[idx] = self.ram[self.i as usize + idx];
                    }
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    /* Keys Related
     */
    pub fn key_press(&mut self, key: u8) {
        self.keys[usize::from(key)] = true;
    }

    pub fn key_released(&mut self, key: u8) {
        self.keys[usize::from(key)] = false;
    }

    /* Fonts
     */
    fn store_font(&mut self) {
        let font_base = 0x50;
        let fonts = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (idx, byte) in fonts.iter().enumerate() {
            self.ram[font_base + idx] = *byte;
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

    pub fn to_decrease_delay_timer(&self) -> bool {
        let elapsed_time = self.delay_time_start.elapsed().as_nanos();
        self.delay_timer > 0 && elapsed_time > self.timer_req_duration.as_nanos()
    }

    pub fn to_decrease_sound_timer(&self) -> bool {
        let elapsed_time = self.delay_time_start.elapsed().as_nanos();
        self.sound_timer > 0 && elapsed_time > self.timer_req_duration.as_nanos()
    }

    pub fn decrease_delay_timer(&mut self){
        self.delay_timer -= 1;
        self.delay_time_start = Instant::now();
    }

    pub fn decrease_sound_timer(&mut self){
        self.sound_timer -= 1;
        self.sound_time_start = Instant::now();
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
