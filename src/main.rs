use cpu::Cpu;

mod ram;
mod cpu;

use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./data/INVADERS").expect("Couldn't open the file");
    let mut content = Vec::<u8>::new();
    file.read_to_end(&mut content).expect("Couldn't read the file");

    //Loading the data in CPU RAM
    let mut cpu = Cpu::new();
    cpu.load_rom(&content);



}
