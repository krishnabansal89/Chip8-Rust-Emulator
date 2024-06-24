use std::u16;

use crate::display::Display;
use crate::ram::Ram;
use macroquad::prelude::*;
use ::rand::Rng;
pub struct Cpu{
    vx : [u8;16],
    i : u16,
    pc : u16,
    stack : Vec::<u16>,
    dt : u8,
    st : u8,
    pub display : Display,
}

impl Cpu{
    pub fn new() -> Cpu{
        Cpu{
            vx : [0;16],
            i : 0,
            pc : 0x200,
            stack : Vec::<u16>::new(),
            dt : 0,
            st : 0,
            display : Display::new(),
        }
    }
    pub fn run_instrutions(&mut self , ram : &mut Ram){
        let opcode = self.fetch_opcode( ram);
        self.execute_opcode(opcode , &ram);
    }

    fn fetch_opcode(&mut self ,  ram : &mut Ram) -> u16{
        let opcode = (ram.read_byte(self.pc) as u16) << 8 | ram.read_byte(self.pc + 1) as u16;
        self.pc += 2;
        opcode
    }
    
    fn execute_opcode(&mut self, opcode : u16 , ram: &Ram){
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let _x = self.vx[x];
        let _y = self.vx[y];
        // println!("Executing opcode {:#x}",opcode);
        match opcode & 0xF000{
            0x0000 => {
                match opcode & 0x00FF{
                    0xE0 => self.display.clear_screen(), // to do later
                    0xEE => {
                        self.pc = self.stack.pop().unwrap();
                    },
                    _ => panic!("Unknown opcode {:#x}",opcode)
                }
            },
            0x1000 => {
                self.pc = nnn;
            },
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = nnn;
            },
            0x3000 => {
                if self.vx[x] == nn{
                    self.pc +=2;
                }
            },
            0x4000 => {
                if self.vx[x] != nn{
                    self.pc +=2;
                }
            },
            0x5000 => {
                if self.vx[x] == self.vx[y]{
                    self.pc +=2;
                }
            },
            0x6000 => {self.vx[x] = nn;},
            0x7000 => {
                self.vx[x] = self.vx[x] + nn;
            },
            0x8000 => {
                match opcode & 0x000F{
                    0x0000 => {
                        self.vx[x] = self.vx[y];
                    },
                    0x0001 => {
                        self.vx[x] = self.vx[x] | self.vx[y];
                    },
                    0x0002 => {
                        self.vx[x] = self.vx[x] & self.vx[y];
                    },
                    0x0003 => {
                        self.vx[x] = self.vx[x] ^ self.vx[y];
                    },
                    0x0004 => {
                        self.vx[x] = self.vx[x] + self.vx[y] as u8;
                        if(self.vx[x] as u16 + self.vx[y] as u16) > 0xFF{
                            self.vx[0xF] = 1;
                        }
                },
                    0x0005 => {
                        self.vx[x] = self.vx[x] - self.vx[y];
                        if self.vx[x] > self.vx[y]{
                            self.vx[0xF] = 1;
                        }
                        else {
                            self.vx[0xF] = 0;
                        }
                    },
                    0x0006 => {
                        self.vx[x] = self.vx[x] >> 1;
                        self.vx[0xF] = self.vx[x] & 0x1;
                    },
                    0x0007 =>  {
                        self.vx[x] = self.vx[y] - self.vx[x];
                        if self.vx[y] > self.vx[x]{
                            self.vx[0xF] = 1;
                        }
                        else {
                            self.vx[0xF] = 0;
                        }
                    },
                    0x000E => {
                        self.vx[x] = self.vx[x] << 1;
                        self.vx[0xF] = self.vx[x] & 0x10;
                    },
                    _ => panic!("Unknown opcode {:#x}",opcode)
                }
            },
            0x9000 => {
                if self.vx[x] != self.vx[y]{
                    self.pc +=2;
                }
            },
            0xA000 => {
                self.i = nnn;
            },
            0xB000 => {
                self.pc = nnn + self.vx[0] as u16;
            },
            0xC000 => {
                let random_number: u8 = ::rand::thread_rng().gen();
                self.vx[x] = random_number & nn;
            },
            0xD000 => {
                self.display.draw_sprite(self.vx[x],self.vx[y],&ram.mem[self.i as usize..self.i as usize + n as usize]);
            },
            0xE000 => {
                match opcode & 0x00FF{
                    0x009E => {
                        if is_key_down(KeyCode::Key1){
                            if self.vx[0] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::Key2){
                            if self.vx[1] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::Key3){
                            if self.vx[2] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::Key4){
                            if self.vx[3] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::Q){
                            if self.vx[4] == 1 
                                {self.pc += 2;}
                            
                        }   
                        if is_key_down(KeyCode::W){
                            if self.vx[5] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::E){
                            if self.vx[6] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::R){
                            if self.vx[7] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::A){
                            if self.vx[8] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::S){
                            if self.vx[9] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::D){
                            if self.vx[10] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::F){
                            if self.vx[11] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::Z){
                            if self.vx[12] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::X){
                            if self.vx[13] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::C){
                            if self.vx[14] == 1 
                                {self.pc += 2;}
                            
                        }
                        if is_key_down(KeyCode::V){
                            if self.vx[15] == 1 
                                {self.pc += 2;}
                            
                        }
                    },
                    0x00A1 => {
                        if !is_key_down(KeyCode::Key1){
                            if self.vx[0] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::Key2){
                            if self.vx[1] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::Key3){
                            if self.vx[2] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::Key4){
                            if self.vx[3] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::Q){
                            if self.vx[4] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::W){
                            if self.vx[5] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::E){
                            if self.vx[6] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::R){
                            if self.vx[7] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::A){
                            if self.vx[8] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::S){
                            if self.vx[9] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::D){
                            if self.vx[10] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::F){
                            if self.vx[11] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::Z){
                            if self.vx[12] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::X){
                            if self.vx[13] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::C){
                            if self.vx[14] == 0 
                                {self.pc += 2;}
                            
                        }
                        if !is_key_down(KeyCode::V){
                            if self.vx[15] == 0 
                                {self.pc += 2;}
                            
                        }

                    },
                    _ => panic!("Unknown opcode {:#x}",opcode)
                }
            },
            0xF000 => {
                match opcode & 0x00FF{
                    0x0007 => {
                        self.vx[x] = self.dt;
                    },
                    0x000A => 
                    {
                        let mut key = 0;
                        if is_key_down(KeyCode::Key1){
                            key = 1;
                        }
                        if is_key_down(KeyCode::Key2){
                            key = 2;
                        }
                        if is_key_down(KeyCode::Key3){
                            key = 3;
                        }
                        if is_key_down(KeyCode::Key4){
                            key = 4;
                        }
                        if is_key_down(KeyCode::Q){
                            key = 5;
                        }
                        if is_key_down(KeyCode::W){
                            key = 6;
                        }
                        if is_key_down(KeyCode::E){
                            key = 7;
                        }
                        if is_key_down(KeyCode::R){
                            key = 8;
                        }
                        if is_key_down(KeyCode::A){
                            key = 9;
                        }
                        if is_key_down(KeyCode::S){
                            key = 10;
                        }
                        if is_key_down(KeyCode::D){
                            key = 11;
                        }
                        if is_key_down(KeyCode::F){
                            key = 12;
                        }
                        if is_key_down(KeyCode::Z){
                            key = 13;
                        }
                        if is_key_down(KeyCode::X){
                            key = 14;
                        }
                        if is_key_down(KeyCode::C){
                            key = 15;
                        }
                        if is_key_down(KeyCode::V){
                            key = 16;
                        }
                        if key != 0{
                            self.vx[x] = key;
                        }
                        else{
                            self.pc -= 2;
                        }
                    },
                    0x0015 => {
                        self.dt = self.vx[x];
                    },
                    0x0018 => {
                        self.st = self.vx[x];
                    },
                    0x001E => {
                        self.i += self.vx[x] as u16;
                        if self.i > 0xFFF{
                            self.vx[0xF] = 1;
                        }
                    },
                    0x0029 => 
                    {
                        self.i = self.vx[x] as u16 * 5;
                    },
                    0x0033 => 
                {
                    ram.write_byte(self.i , self.vx[x] / 100);
                    ram.write_byte(self.i + 1 , (self.vx[x] / 10) % 10);
                    ram.write_byte(self.i + 2 , self.vx[x] % 10);
                },  0x0055 => self.store(x),
                    0x0065 => self.load_store(x),
                    _ => panic!("Unknown opcode {:#x}",opcode)
                }
            },
            _ => panic!("Unknown opcode {:#x}",opcode)
        };}
}