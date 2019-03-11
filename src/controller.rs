use ram::Ram;
use keyboard::Keyboard;
use display::Display;
use std::time;

pub struct Controller {
    ram: Ram,
    keyboard: Keyboard,
    display: Display,
    delay_timer: u8,
    delay_timer_set_time: time::Instant,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            keyboard: Keyboard::new(),
            ram: Ram::new(),
            display: Display::new(),
            delay_timer: 0,
            delay_timer_set_time: time::Instant::now()
        }
    }
    pub fn draw_byte(&mut self,  byte: u8, x: u8, y: u8) -> bool {
        self.display.draw_byte(byte, x, y)
    }
    pub fn write_byte(&mut self, position: usize, value: u8) {
        self.ram.write_byte(position, value);
    }

    pub fn read_byte(&self, position: usize) -> u8 {
        self.ram.read_byte(position)
    }

    pub fn clr_scr(&mut self) {
        self.display.clear();
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer_set_time = time::Instant::now();
        self.delay_timer = value;
    }

    pub fn get_delay_timer(&self) -> u8 {
        let diff = time::Instant::now() - self.delay_timer_set_time;
        let ms = diff.get_millis();
        let ticks = ms / 16;
        if ticks >= self.delay_timer as u64 {
            0
        } else {
            self.delay_timer - ticks as u8
        }
    }
    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.keyboard.set_key_pressed(key);
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.keyboard.get_key_pressed()
    }

    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        self.keyboard.is_key_pressed(key_code)
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        self.display.get_display_buffer()
    }
}

trait Milliseconds {
    fn get_millis( & self ) -> u64;
}

impl Milliseconds for time::Duration {
    fn get_millis( & self ) -> u64 {
        let nanos = self.subsec_nanos() as u64;
        let ms = (1000 * 1000 * 1000 * self.as_secs() + nanos) /(1000 * 1000);
        ms
    }
}