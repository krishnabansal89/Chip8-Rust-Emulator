use chip8::Chip8;
use display::Display;
use macroquad::prelude::*;
mod ram;
mod chip8;
mod cpu;
mod display;

use std::{fs::File, io::Read};

#[macroquad::main("Chip8")]
async fn main() {
    let mut file = File::open("./data/IBM.ch8").expect("Couldn't open the file");
    let mut content = Vec::<u8>::new();
    file.read_to_end(&mut content).expect("Couldn't read the file");

    //Loading the data in CPU RAM
    let display = Display::new();
    let mut chip8 = Chip8::new();
    println!("Loading the ROM {:X?}",&content);
    chip8.load_rom(&content);
    
    //Running the CPU
    loop{
        clear_background(macroquad::color::BLACK);

        chip8.run();
        chip8.cpu.display.draw(&chip8.cpu.display.get_screen());
        // display.draw(display.get_screen());
        next_frame().await;
    }

}
