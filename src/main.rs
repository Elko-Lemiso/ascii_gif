extern crate image;
extern crate gif;

use image::RgbImage;
use std::io::{ self, Write };
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;

fn get_str_ascii(intent: u8) -> &'static str {
    let ascii = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@", "8", "&"];
    let index = (((intent as u16) * ((ascii.len() - 1) as u16)) / 255) as u8;
    return ascii[index as usize];
}

fn play_gif_in_terminal(dir: &str, scale: u32) {
    loop {
        // Start of the infinite loop
        // Open the file
        let input = File::open(dir).unwrap();
        // Configure the decoder such that it will expand the image to RGBA.
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        // Read the file header
        let mut decoder = options.read_info(input).unwrap();

        while let Some(frame) = decoder.read_next_frame().unwrap() {
            // Clear the terminal
            print!("\x1B[2J");
            io::stdout().flush().unwrap();

            let buffer = &frame.buffer;
            let img = RgbImage::from_fn(frame.width as u32, frame.height as u32, |x, y| {
                let index = y
                    .checked_mul(frame.width as u32)
                    .and_then(|result| result.checked_add(x))
                    .and_then(|result| result.checked_mul(4))
                    .expect("Error calculating index") as usize;
                image::Rgb([buffer[index], buffer[index + 1], buffer[index + 2]])
            });

            for y in 0..img.height() {
                for x in 0..img.width() {
                    if y % (scale * 2) == 0 && x % scale == 0 {
                        let pix = img.get_pixel(x, y);
                        let intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                        print!("{}", get_str_ascii(intent));
                    }
                }
                if y % (scale * 2) == 0 {
                    println!();
                }
            }

            // Wait for the duration of the frame
            let delay = (frame.delay as u64) * 10; // Convert to milliseconds
            sleep(Duration::from_millis(delay));
        }
    } // End of the infinite loop
}

fn main() {
    play_gif_in_terminal("skellie.gif", 4);
}
