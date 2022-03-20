pub mod cpu;
pub mod renderer;

use cpu::YARCH8;
use renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use clap::Parser;

fn main() {
    let args = Args::parse();

    // SDL2 init
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // For scale up original screen size
    let scale = 20;

    let window = video_subsystem
        .window("YARCH8", 64 * scale, 32 * scale)
        .position_centered()
        .build()
        .unwrap();

    // canvas is our screen where we draw sprite
    let canvas = window.into_canvas().build().unwrap();
    let mut renderer = Renderer::new(canvas, scale);

    // Init CPU State (where pc, sp are ?)
    let mut yarch8 = YARCH8::new();

    // Read rom file into RAM (load program into memory)
    let rom_path = args.rom_file_path;
    yarch8.load(&rom_path);

    // Start program
    yarch8.start();

    // TODO: Add loop here
    'running: loop {
        // Handle keys events
        // TODO: use scancode over keycode in the future
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key_index) = get_keys_index(keycode) {
                        yarch8.key_press(key_index);
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key_index) = get_keys_index(keycode) {
                        yarch8.key_released(key_index);
                    }
                }
                _ => {}
            }
        }
        // Fetch
        let ins = yarch8.fetch();
        println!("Fetched instruction: {:#04x}", ins);

        // Decode
        // Execute
        yarch8.decode_execute(ins);
        renderer.render_screen(yarch8.get_disp_buff());

        // Debug
        //yarch8.stats_peek();

        // Time management
        yarch8.stall();
    }
}

/// Yet Another Chip-8 Emulator written in Rust
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to ROM file
    #[clap(short, long)]
    rom_file_path: String,
}

fn get_keys_index(k: Keycode) -> Option<u8> {
    return match k {
        Keycode::Num1 => Some(0),
        Keycode::Num2 => Some(1),
        Keycode::Num3 => Some(2),
        Keycode::Num4 => Some(0xC),

        Keycode::Q => Some(4),
        Keycode::W => Some(5),
        Keycode::E => Some(6),
        Keycode::R => Some(0xD),

        Keycode::A => Some(7),
        Keycode::S => Some(8),
        Keycode::D => Some(9),
        Keycode::F => Some(0xE),

        Keycode::Z => Some(0xA),
        Keycode::X => Some(0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    };
}
