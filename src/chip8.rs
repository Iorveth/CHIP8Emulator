use cpu;
use cpu::Cpu;
use controller::Controller;


pub struct Chip8 {
    controller: Controller,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            controller: Controller::new(),
            cpu: Cpu::new()
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.controller.write_byte(cpu::PROGRAM_START_ADDRESS as usize + i, data[i]);
        }
    }

    pub fn execute_instruction(&mut self){
        self.cpu.run_instruction(&mut self.controller);
    }
    pub fn get_display_buffer(&self) -> &[u8] {
        self.controller.get_display_buffer()
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.controller.set_key_pressed(key);
    }
}