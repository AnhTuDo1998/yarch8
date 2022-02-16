pub mod cpu;
use cpu::YARCH8;

fn main() {
    // SDL2 init
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("YARCH8", 64, 32)
        .position_centered()
        .build()
        .unwrap();

    // canvas is our screen where we draw sprite
    let mut canvas = window.into_canvas().build().unwrap();

    // Init CPU State (where pc, sp are ?)
    let mut yarch8 = YARCH8::new(canvas);

    // Read rom file into RAM (load program into memory)
    let rom_path = "ROM/ibm_logo.ch8";
    yarch8.load(rom_path);
    yarch8.ram_peek();

    // Start program
    yarch8.start();

    // TODO: Add loop here
    loop {
        // Fetch
        let ins = yarch8.fetch();
        println!("{}", ins);

        // Decode
        // Execute
        yarch8.decode_execute(ins);

        // Debug
        //yarch8.stats_peek();

        // Time management
        yarch8.stall();
    }
}
