use crate::ram::Ram;

pub struct Cpu {
    ram : Ram,
}

impl Cpu{
    pub fn new() -> Cpu
    {
        Cpu{
            ram : Ram::new()
        }
    }
    pub fn load_rom(&mut self , data : &Vec<u8>)
    {
        let offset = 0x200;
        for i in 0..data.len()
        {
            self.ram.write_byte((i + offset) as u16 , data[i]);
        }
    }
}