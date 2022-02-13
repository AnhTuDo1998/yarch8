struct YARCH8 {
    pc: u16,
    i: u16,
    ram: [u8; 4096],
    v_regs: [u8; 16],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u8,
    disp_buff: [[bool; 64]; 32],
}

impl YARCH8 {
    fn new() -> Self {
        unimplemented!("Constructor not done!")
    }
}

fn main () {
    // Init CPU State (where pc, sp are ?)
    // Read rom file into RAM
    // Execute
}