
use std::u16;

use crate::display::Display;
use crate::ram::Ram;
use ::rand::Rng;
use macroquad::prelude::*;
pub struct Cpu {
    vx: [u8; 16],
    i: u16,
    pc: u16,
    stack: Vec<u16>,
    dt: u8,
    st: u8,
    pub display: Display,
    key_state: [bool; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            i: 0,
            pc: 0x200,
            stack: Vec::<u16>::new(),
            dt: 0,
            st: 0,
            display: Display::new(),
            key_state: [false; 16],
        }
    }
    pub fn update_keys(&mut self) {
        let keys = [
            KeyCode::X, KeyCode::Key1, KeyCode::Key2, KeyCode::Key3,
            KeyCode::Q, KeyCode::W, KeyCode::E, KeyCode::A,
            KeyCode::S, KeyCode::D, KeyCode::Z, KeyCode::C,
            KeyCode::Key4, KeyCode::R, KeyCode::F, KeyCode::V,
        ];
        for (i, &key) in keys.iter().enumerate() {
            self.key_state[i] = is_key_down(key);
            if self.key_state[i] {
                println!("Key pressed: {:?}", key);
            }
        }
    }
    
    

    pub fn run_instrutions(&mut self, ram: &mut Ram) {
        let opcode = self.fetch_opcode(ram);
        self.execute_opcode(opcode, ram);
    }

    fn fetch_opcode(&mut self, ram: &mut Ram) -> u16 {
        let opcode = (ram.read_byte(self.pc) as u16) << 8 | ram.read_byte(self.pc + 1) as u16;
        self.pc += 2;
        opcode
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        // Do the same for sound timer if you have one
    }
    

    fn execute_opcode(&mut self, opcode: u16, ram: &mut Ram) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let _x = self.vx[x];
        let _y = self.vx[y];
        println!("Executing opcode {:#x}",opcode);

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x00FF {
                    0xE0 => self.display.clear_screen(), // to do later
                    0xEE => {
                        self.pc = self.stack.pop().unwrap();
                    }
                    _ => panic!("Unknown opcode {:#x}", opcode),
                }
            }
            0x1000 => {
                self.pc = nnn;
            }
            0x2000 => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            0x3000 => {
                if self.vx[x] == nn {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.vx[x] != nn {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.vx[x] == self.vx[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                self.vx[x] = nn;
            }
            0x7000 => {
                self.vx[x] = self.vx[x].wrapping_add(nn);
            }
            0x8000 => match opcode & 0x000F {
                0x0000 => {
                    self.vx[x] = self.vx[y];
                }
                0x0001 => {
                    self.vx[x] = self.vx[x] | self.vx[y];
                }
                0x0002 => {
                    self.vx[x] = self.vx[x] & self.vx[y];
                }
                0x0003 => {
                    self.vx[x] = self.vx[x] ^ self.vx[y];
                }
                0x0004 => {
                    let _ = self.vx[x].wrapping_add(self.vx[y]);
                    if self.vx[x] as u16 + self.vx[y] as u16 > 0xFF {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                }
                0x0005 => {
                    let _ = self.vx[x].wrapping_sub(self.vx[y]);
                    if self.vx[x] > self.vx[y] {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                }
                0x0006 => {
                    self.vx[x] = self.vx[x] >> 1;
                    self.vx[0xF] = self.vx[x] & 0x1;
                }
                0x0007 => {
                    self.vx[x] = self.vx[y] - self.vx[x];
                    if self.vx[y] > self.vx[x] {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                }
                0x000E => {
                    self.vx[x] = self.vx[x] << 1;
                    self.vx[0xF] = self.vx[x] & 0x10;
                }
                _ => panic!("Unknown opcode {:#x}", opcode),
            },
            0x9000 => {
                if self.vx[x] != self.vx[y] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.i = nnn;
            }
            0xB000 => {
                self.pc = nnn + self.vx[0] as u16;
            }
            0xC000 => {
                let random_number: u8 = ::rand::thread_rng().gen();
                self.vx[x] = random_number & nn;
            }
            0xD000 => {
                self.vx[0xF] = 0; 
                let collison = self.display.draw_sprite(
                    self.vx[x],
                    self.vx[y],
                    &ram.mem[self.i as usize..self.i as usize + n as usize],
                );
                self.vx[0xF] = collison as u8;
            }
            0xE000 => {
                println!("Opcode {:#x}", opcode);
                match opcode & 0x00FF {
                0x009E => {
                    println!("Opcode Ex9E: Checking if key {} is pressed", self.vx[x]);
                    if self.key_state[self.vx[x] as usize] {
                        println!("Key is pressed, skipping next instruction");
                        self.pc += 2;
                    }
                }
            
                0x00A1 => {
                    println!("Opcode ExA1: Checking if key {} is not pressed", self.vx[x]);
                    if !self.key_state[self.vx[x] as usize] {
                        println!("Key is not pressed, skipping next instruction");
                        self.pc += 2;
                    }
                }
            
                            
                _ => panic!("Unknown opcode {:#x}", opcode),
            }},

            0xF000 => match opcode & 0x00FF {
                0x0007 => {
                    self.vx[x] = self.dt;
                }
                0x000A => {
                    let key_pressed = self.key_state.iter().position(|&k| k);
                    if let Some(key) = key_pressed {
                        println!("Key pressed from aaa {:#x}", key as u8);
                        self.vx[x] = key as u8;
                    } else {
                        self.pc -= 2; // Repeat this instruction
                    }
                }

                0x0015 => {
                    self.dt = self.vx[x];
                }
                0x0018 => {
                    self.st = self.vx[x];
                }
                0x001E => {
                    self.i += self.vx[x] as u16;
                    if self.i > 0xFFF {
                        self.vx[0xF] = 1;
                    }
                }
                0x0029 => {
                    self.i = self.vx[x] as u16 * 5;
                }
                0x0033 => {
                    &ram.write_byte(self.i, self.vx[x] / 100);
                    &ram.write_byte(self.i + 1, (self.vx[x] / 10) % 10);
                    &ram.write_byte(self.i + 2, self.vx[x] % 10);
                }
                0x0055 => {
                    for i in 0..x + 1 {
                        &ram.write_byte(self.i + i as u16, self.vx[i]);
                    }
                }
                0x0065 => {
                    for i in 0..x + 1 {
                        self.vx[i] = ram.read_byte(self.i + i as u16);
                    }
                }
                _ => panic!("Unknown opcode {:#x}", opcode),
            },
            _ => panic!("Unknown opcode {:#x}", opcode),
        }
    }
}
