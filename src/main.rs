struct YARCH8 {
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
    fn new() -> Self {
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
}

fn main () {
    // Init CPU State (where pc, sp are ?)
    let cpu = YARCH8::new();
    
    // Read rom file into RAM (load program into memory)

    // Loop

    // Fetch
    // Decode
    // Execute
}