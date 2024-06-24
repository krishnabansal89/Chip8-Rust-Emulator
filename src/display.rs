use macroquad::prelude::*;

pub struct Display{
    screen : [[u8;64];32],
}

impl Display{
    pub fn new() -> Self{
        Self{
            screen : [[0;64];32],
        }
    }
    pub fn clear_screen(&mut self){
        for i in 0..32{
            for j in 0..64{
                self.screen[i][j] = 0;
            }
        }
    }
    pub fn draw_sprite(&mut self , x : u8 , y : u8 , sprite : &[u8]) -> bool{
        let mut collision = false;
        for i in 0..sprite.len(){
            for j in 0..8{
                if (sprite[i] & (0x80 >> j)) != 0{
                    let x = (x as usize + j) % 64;
                    let y = (y as usize + i) % 32;
                    if self.screen[y][x] == 1{
                        collision = true;
                    }
                    self.screen[y][x] ^= 1;
                }
            }
        }
        collision
    }
    pub fn get_screen(&self) -> &[[u8;64];32]{
        &self.screen
    }
    pub fn draw(&self, chip8_display: &[[u8;64];32]) {
        for y in 0..32 {
            for x in 0..64 {
                println!("Checking pixel at x : {} y : {} , value : {}",x,y,chip8_display[y][x]);
                if chip8_display[y][x] > 0{
                    // println!("Drawing pixel at x : {} y : {}",x,y);
                    draw_rectangle(
                        x as f32 * 10.0,
                        y as f32 * 10.0,
                        10.0,
                        10.0,
                        WHITE,
                    );
                }
            }
        }
    }

}