use chip8::Chip8;
use display::Display;
use macroquad::prelude::*;
mod ram;
mod chip8;
mod cpu;
mod display;

use std::{fs::File, io::Read};

#[macroquad::main("INVADERS")]
async fn main() {
    let mut file = File::open("./data/INVADERS").expect("Couldn't open the file");
    let mut content = Vec::<u8>::new();
    file.read_to_end(&mut content).expect("Couldn't read the file");

    //Loading the data in CPU RAM
    let mut last_update = get_time();
    let update_interval = 1.0 / 15.0; 
    let mut chip8 = Chip8::new();
    println!("Loading the ROM {:X?}",&content);
    chip8.load_rom(&content);
    
    //Running the CPU
    loop{
        let current_time = get_time();
        if current_time - last_update >= update_interval {
            chip8.cpu.update_timers();
            last_update = current_time;
        }

        for _ in 0..50{
            chip8.run();
        }
        clear_background(macroquad::color::BLACK);
        chip8.cpu.update_keys();
        chip8.cpu.display.draw(&chip8.cpu.display.get_screen());
        // display.draw(display.get_screen());
        next_frame().await
    }

}
