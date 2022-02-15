pub mod cpu;
use cpu::YARCH8;

fn main () {
    // Init CPU State (where pc, sp are ?)
    let mut yarch8 = YARCH8::new();
    
    // Read rom file into RAM (load program into memory)
    let rom_path = "ROM/ibm_logo.ch8";
    yarch8.load(rom_path);
    yarch8.ram_peek();

    // Start program
    yarch8.start();

    // TODO: Add loop here
    // Fetch
    let ins = yarch8.fetch();
    println!("{}",ins);
    
    // Decode
    // Execute
    yarch8.decode_execute(ins);
}