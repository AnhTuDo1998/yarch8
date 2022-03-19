# Overview
YARCH8 (pronounced as "Zaa Kay") is short for "Yet Another Rust-based CHIP-8", a personal project for practicing Rust by building emulator. YARCH8 name is inspired from the Vietnamese dish Gia Cay.

# Build
<!-- TODO: Add command line for specifying the binary hex .ch8 to be loaded to run -->
YARCH8 makes use of Rust's SDL2 wrapper for rendering in drawing sprite. This crate is dependent on libsdl2-dev and maybe installed as:
```
sudo apt install libsdl2-dev
```

To build and run the binary
```
cargo run
```

# Modules
CPU - Mimic hardware of the system
Renderer - Logic to draw updated buffer

# References
- [Cowgod's chip-8 manual](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Guide by tobiasvl](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [Sunjay's Game programming in Rust guide](https://sunjay.dev/learn-game-dev/intro.html)
