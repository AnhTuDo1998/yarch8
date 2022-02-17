use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

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
    canvas: WindowCanvas,
}

impl YARCH8 {
    pub fn new(canvas: WindowCanvas) -> Self {
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
            canvas: canvas,
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

    pub fn stall(&mut self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn fetch(&mut self) -> u16 {
        // Read 2B from the current PC address
        let fetch_address = self.pc as usize;
        self.pc += 2;
        ((self.ram[fetch_address] as u16) << 8) + (self.ram[fetch_address + 1] as u16)
    }

    pub fn decode_execute(&mut self, instruction: u16) {
        match instruction & 0xF000 {
            // Clear screen
            0x0000 => self.clear_screen(),
            // Jump
            0x1000 => self.pc = instruction & 0x0FFF,
            // Set VXNN
            0x6000 => {
                let target_reg: usize = (instruction & 0x0F00) as usize >> 16u8;
                self.v_regs[target_reg] = (instruction & 0x00FF) as u8;
            }
            // Add to Vx N
            0x7000 => {
                let target_reg: usize = (instruction & 0x0F00) as usize >> 16u8;
                self.v_regs[target_reg] += (instruction & 0x00FF) as u8;
            }
            // Set I NN
            0xA000 => self.i = instruction & 0x0FFF,
            // Draw
            0xD000 => {
                // Part 1: Update array of boolean
                let x_reg: usize = (instruction & 0x0F00) as usize >> 16u8;
                let y_reg: usize = (instruction & 0x00F0) as usize >> 8u8;
                let height_n = (instruction & 0x000F) as usize;
                let mut x = usize::try_from(self.v_regs[x_reg] & 63).unwrap();
                let mut y = usize::try_from(self.v_regs[y_reg] & 32).unwrap();

                // Clear flag register
                self.v_regs[15] = 0;

                for offset  in 0..=height_n {
                    // Do we hit the bottom edge of screen ?
                    if y >= 64 {
                        break;
                    }
                    // Get sprite row
                    let sprite_row = self.ram[self.i as usize + offset];
                    // Loop through bit location
                    for bit_idx in (0..8u32) {
                        // Hit right edge?
                        if x >= 32 {
                            break;
                        }

                        // Otw try to draw/undraw
                        let bit = (sprite_row & 0x80).checked_shr(8 - bit_idx).unwrap_or(0);
                        if bit == 0x1 {
                            if self.disp_buff[x][y] {
                                self.v_regs[15] = 1;
                                self.disp_buff[x][y] = false;
                            }
                            else {
                                self.disp_buff[x][y] = true;
                            }

                            x+=1;
                        }
                    }

                    y+=1;
                }
                

                // Part 2: Draw the array of boolean
                self.render_screen();
            }
            _ => unimplemented!(),
        }
    }

    pub fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn render_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // logic to display render from boolean matrix
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (row_idx, row) in self.disp_buff.iter().enumerate() {
            
            for (col_idx, pixel) in row.iter().enumerate() {
                // Draw a pixel if it is true
                if *pixel {
                    // For now assume no scaling, so width = height = 1
                    let x: i32 = i32::try_from(col_idx).unwrap();
                    let y: i32 = i32::try_from(row_idx).unwrap();
                    self.canvas.fill_rect(Rect::new(x,y,1,1)).unwrap();
                }
            }
        }

        self.canvas.present();
    }
}
