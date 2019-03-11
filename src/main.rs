extern crate rand;
extern crate minifb;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};
use display::Display;
mod ram;
mod chip8;
mod cpu;
mod controller;
mod keyboard;
mod display;

use keyboard::Keyboard;
use chip8::Chip8;
use std::io::stdin;

fn read_file() -> std::io::Result<Vec<u8>> {
    let mut file_name = String::new();
    stdin().read_line(& mut file_name);
    let mut file = File::open(file_name.trim())?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}


fn main() {
    let data;
    println!("Game path: ");
    loop {
        if let Ok(t) = read_file(){
            data = t;
            break;
        };
        println!("Wrong path!");
        println!("Game path: ");
    }
    println!("{:?}",data.len());
    let mut cp8 =  Chip8::new();
    cp8.load_rom(&data);

    let width = 1280;
    let height = 640;

    //ARGB buffer
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Chip8 Emulator",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Wdindow creation failed: {:?}", e);
    });

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes);
        let key = match keys_pressed {
            Some(keys) => if !keys.is_empty() {
                Some(keys[0])
            } else {
                None
            },
            None => None,
        };

        let chip8_key = Keyboard::get_chip8_keycode(key);
        if chip8_key.is_some()
            || Instant::now() - last_key_update_time >= Duration::from_millis(100)
        {
            last_key_update_time = Instant::now();
            cp8.set_key_pressed(chip8_key);
        }

        if Instant::now() - last_instruction_run_time > Duration::from_millis(1) {
            cp8.execute_instruction();
            last_instruction_run_time = Instant::now();
        }

        if Instant::now() - last_display_time > Duration::from_millis(10) {
            let chip8_buffer = cp8.get_display_buffer();

            for y in 0..height {
                let y_coord = y / 20;
                let offset = y * width;
                for x in 0..width {
                    let index = Display::get_index_from_coords(x / 20, y_coord);
                    let pixel = chip8_buffer[index];
                    let color_pixel = match pixel {
                        0 => 0xFFFFFF,
                        1 => 0x009900,
                        _ => unreachable!(),
                    };
                    buffer[offset + x] = color_pixel;
                }
            }


            window.update_with_buffer(&buffer);
            last_display_time = Instant::now();
        }
    }
}