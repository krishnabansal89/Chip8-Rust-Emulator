use crate::display::Display;
use crate::ram::Ram;

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
            // 0x8000 => {
            //     match opcode & 0x000F{
            //         0x0000 => self.load_reg(x,y),
            //         0x0001 => self.or(x,y),
            //         0x0002 => self.and(x,y),
            //         0x0003 => self.xor(x,y),
            //         0x0004 => self.add_reg(x,y),
            //         0x0005 => self.sub(x,y),
            //         0x0006 => self.shr(x),
            //         0x0007 => self.subn(x,y),
            //         0x000E => self.shl(x),
            //         _ => panic!("Unknown opcode {:#x}",opcode)
            //     }
            // },
            0x9000 => {
                if self.vx[x] != self.vx[y]{
                    self.pc +=2;
                }
            },
            0xA000 => {
                self.i = nnn;
            },
            // 0xB000 => self.jump_v0(nnn),
            // 0xC000 => self.rand(x,kk),
            0xD000 => {
                self.display.draw_sprite(self.vx[x],self.vx[y],&ram.mem[self.i as usize..self.i as usize + n as usize]);
            },
            // 0xE000 => {
            //     match opcode & 0x00FF{
            //         0x009E => self.skip_key(x),
            //         0x00A1 => self.skip_nkey(x),
            //         _ => panic!("Unknown opcode {:#x}",opcode)
            //     }
            // },
            // 0xF000 => {
            //     match opcode & 0x00FF{
            //         0x0007 => self.load_dt(x),
            //         0x000A => self.wait_key(x),
            //         0x0015 => self.set_dt(x),
            //         0x0018 => self.set_st(x),
            //         0x001E => self.add_i(x),
            //         0x0029 => self.load_font(x),
            //         0x0033 => self.bcd(x),
            //         0x0055 => self.store(x),
            //         0x0065 => self.load_store(x),
            //         _ => panic!("Unknown opcode {:#x}",opcode)
            //     }
            // },
            _ => panic!("Unknown opcode {:#x}",opcode)
        };}
}