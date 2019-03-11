pub const PROGRAM_START_ADDRESS: u16 = 0x200;
use controller::Controller;
use rand::prelude::*;

pub struct Cpu {
    vx: [u8; 16],
    i: u16,
    pc: u16,
    ret_stack: Vec<u16>
}

impl Cpu {
    pub fn new()  -> Cpu {
        Cpu {
            vx: [0;16],
            i: 0,
            pc: PROGRAM_START_ADDRESS,
            ret_stack: Vec::new()
        }
    }

    pub fn run_instruction(&mut self, controller: &mut Controller){
        let hi = controller.read_byte(self.pc as usize) as u16;
        let lo = controller.read_byte(self.pc as usize +1) as u16;
        let instruction = hi << 8 | lo;
        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n =  (instruction & 0x0F) as u8;
        let x = (instruction >> 8 & 0x0F) as u8;
        let y = (instruction >> 4 & 0x00F) as u8;
        let vx = self.get_vx(x as usize);
        let vy = self.get_vx(y as usize);

        //Debug
        //println!("hi: {:#x}, lo: {:#x} instruction {:#x}", hi, lo, instruction);
        //println!("nnn: {:#x}, nn: {:#x} n {:#x} x {:#x} y {:#x}", nnn, nn, n, x, y);
        //println!("{:#x}", instruction >> 12 & 0x000F);
        //panic!();

        match instruction >> 12 & 0x000F {
            0x0 => match nn {
                0xE0 => {
                    controller.clr_scr();
                    self.pc += 2;
                }
                0xEE => {
                    ////return from subroutine
                    self.pc = self.ret_stack.pop().unwrap();
                }
                _ => unreachable!()
            },
            0x1 => {
                self.pc = nnn;
            },
            0x2 => {
                self.ret_stack.push(self.pc + 2);
                self.pc = nnn;
            },
            0x3 => {
                if vx == nn {
                    self.pc+=4;
                } else {
                    self.pc+=2;
                }
            },
            0x4 => {
                if vx != nn {
                    self.pc+=4;
                } else {
                    self.pc+=2;
                }
            },
            0x5 => {
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6 => {
                self.set_vx(x as usize, nn);
                self.pc += 2;
            },
            0xA => {
                self.i = nnn;
                self.pc+=2;
            },
            0x7 => {
                self.set_vx(x as usize, vx.wrapping_add(nn));
                self.pc+=2;
            },
            0x8 => {
                match n {
                    0x0 => {
                        self.set_vx(x as usize, vy);
                    },
                    0x1 => {
                        self.set_vx(x as usize, vx | vy);
                    },
                    0x2 => {
                        self.set_vx(x as usize, vx & vy);
                    },
                    0x3 => {
                        self.set_vx(x as usize, vx ^ vy);
                    },
                    0x4 => {
                        let sum: u16 = vx as u16 + vy as u16;
                        self.set_vx(x as usize, sum as u8);
                        if sum > 0xFF {
                            self.set_vx(0xF, 1);
                        } else {
                            self.set_vx(0xF, 0);
                        }
                    },
                    0x5 => {
                        if vx > vy {
                            self.set_vx(x as usize, vx - vy);
                            self.set_vx(0xF, 1);
                        } else {
                            self.set_vx(x as usize, vy - vx);
                            self.set_vx(0xF, 0);
                        }
                    },
                    0x6 => {
                        self.set_vx(0xF, vx & 0x1);
                        self.set_vx(x as usize, vx >> 1);
                    },
                    0x7 => {
                        if vy > vx {
                            self.set_vx(x as usize, vy - vx);
                            self.set_vx(0xF, 1);
                        } else {
                            self.set_vx(x as usize, vx - vy);
                            self.set_vx(0xF, 0);
                        }
                    },
                    0xE => {
                        self.set_vx(0xF, (vx & 0x80) >> 7);
                        self.set_vx(x as usize, vx << 1);
                    }
                    _ => unreachable!()
                }
                self.pc += 2;
            }
            0x9 => {
                if vx!=vy {
                    self.pc += 4;
                } else {
                    self.pc+=2;
                }
            },
            0xB => {
                self.pc = nnn + self.get_vx(0) as u16;
            }
            0xD => {
                //draw(Vx,Vy,N)
                self.draw_sprite(controller, vx, vy, n);
                self.pc += 2;
            },
            0xC => {
                let mut rng = thread_rng();
                let gen: f64 = rng.gen(); // generates a float between 0 and 1
                let random_number: u8 = <f64>::round(gen*255.0) as u8;

                //println!("Random number: {}", random_number);
                self.set_vx(x as usize, random_number & nn);
                self.pc+=2;
            },
            0xE =>{
                match nn{
                    0x9E =>{
                        if !controller.is_key_pressed(vx) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        if controller.is_key_pressed(vx) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => unreachable!()
                }
            },
            0xF => {
                match nn {
                    0x1e => {
                        self.i+=vx as u16;
                        self.pc+=2;
                    },
                    0x0A => {
                        if let Some(val) = controller.get_key_pressed() {
                            self.set_vx(x as usize, val);
                            self.pc += 2;
                        }
                    }
                    0x07 => {
                        self.set_vx(x as usize, controller.get_delay_timer());
                        self.pc+=2;
                    }
                    0x15 => {
                        controller.set_delay_timer(vx);
                        self.pc+=2;
                    },
                    0x18 => {
                        //TODO sound timer
                        self.pc+=2;
                    },
                    0x29 => {
                        self.i = vx as u16 * 5 ;
                        self.pc+=2;
                    },
                    0x33 => {
                        let tens = (vx / 10) % 10;
                        let ones = vx % 10;
                        let hundreds = vx / 100;
                        //println! ("vx {} hundreds {}, tens {} ones {}", vx, hundreds, tens, ones);
                        controller.write_byte(self.i as usize, hundreds);
                        controller.write_byte((self.i+1) as usize, tens );
                        controller.write_byte((self.i+2) as usize, ones );
                        self.pc+=2;
                    },
                    0x55 => {
                        for index in 0..=x {
                            controller.write_byte((self.i + index as u16) as usize, self.get_vx(index as usize));
                        }
                        self.i+=x as u16 + 1;
                        self.pc+=2;
                    },
                    0x65 => {
                        let i = self.i;
                        for index in 0..=x {
                            self.set_vx(index as usize, controller.read_byte((i+index as u16) as usize));
                        }
                        self.i += x as u16 + 1;
                        self.pc+=2;
                    },
                    _ => unreachable!()
                }
            }

            _ => unreachable!()
        }
    }

    fn draw_sprite(&mut self, controller: &mut Controller, x: u8, y: u8, n: u8) {
        let mut set_vf = false;
        for sprite_y in 0..n {
            let byte = controller.read_byte((self.i + sprite_y as u16) as usize);
            if controller.draw_byte(byte, x, y + sprite_y) {
                set_vf = true;
            }
        }
        if set_vf {
            self.set_vx(0xF, 1);
        } else {
            self.set_vx(0xF, 0);
        }
    }

    fn set_vx(&mut self, index: usize, value: u8){
        self.vx[index]=value;
    }
    fn get_vx(&self, index: usize) -> u8 {
        self.vx[index]
    }
}