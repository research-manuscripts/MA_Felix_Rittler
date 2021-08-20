use image::ColorType;
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;
use image::io::Reader as ImageReader;

pub fn capture_screen(name: String, width: i32, height: i32) {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.

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

        // Flip the ARGB image into a RGBA image.
        let mut bitflipped = Vec::with_capacity(w * h * 4);
        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }

        // Save the image.
        image::save_buffer(name.clone(), &bitflipped, w as u32, h as u32, ColorType::Rgba8).unwrap();
        // let mut image = image::load_from_memory(&bitflipped).unwrap();
        let mut image = ImageReader::open(name.clone()).unwrap().decode().unwrap();
        let cropped= image::imageops::crop(&mut image, 0,0, width as u32, height as u32);
        cropped.to_image().save(name.clone()).unwrap();
        let message = format!("Image saved to {}.", name);
        println!("{}", message);
        break;
    }
}
