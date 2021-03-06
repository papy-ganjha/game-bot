extern crate repng;

use enigo::*;
use scrap::{Capturer, Display};
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

fn system_inputs() {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(500, 200);
    enigo.mouse_down(MouseButton::Left);
    enigo.mouse_move_relative(100, 100);
    enigo.mouse_up(MouseButton::Left);
    enigo.key_sequence("gg");
}

fn screenshot() {
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());
    let one_sec = Duration::new(1, 0);
    let one_frame = one_sec / 60;
    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
        println!("Captured! Saving...");

        // Flip the ARGB image into a BGRA image.

        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
            }
        }

        // Save the image.

        repng::encode(
            File::create("screenshot.png").unwrap(),
            w as u32,
            h as u32,
            &bitflipped,
        )
        .unwrap();

        println!("Image saved to `screenshot.png`.");
        break;
    }
}

fn main() {
    println!("Hello, world!");
    screenshot();
    system_inputs();
}
