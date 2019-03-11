use minifb::{Key};

pub struct Keyboard{
    key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard{
        Keyboard {
            key_pressed: None
        }
    }

    pub fn get_chip8_keycode(key: Option<Key>) -> Option<u8> {
        match key {
            Some(Key::Key1) => Some(0x1),
            Some(Key::Key2) => Some(0x2),
            Some(Key::Key3) => Some(0x3),
            Some(Key::Key4) => Some(0xC),

            Some(Key::Q) => Some(0x4),
            Some(Key::W) => Some(0x5),
            Some(Key::E) => Some(0x6),
            Some(Key::R) => Some(0xD),

            Some(Key::A) => Some(0x7),
            Some(Key::S) => Some(0x8),
            Some(Key::D) => Some(0x9),
            Some(Key::F) => Some(0xE),

            Some(Key::Z) => Some(0xA),
            Some(Key::X) => Some(0x0),
            Some(Key::C) => Some(0xB),
            Some(Key::V) => Some(0xF),
            _ => None,
        }
    }

    //Todo implement proper key handling
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        if let Some(key) = self.key_pressed {
            key == key_code
        } else {
            false
        }
    }


    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }

    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }
}