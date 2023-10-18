## ASCII GIF Player in Rust

This Rust program plays GIF animations in the terminal by converting each frame into ASCII art representations.

### Dependencies

rustCopy code

`extern crate image;
extern crate gif;` 

The program uses the `image` crate for image processing tasks and the `gif` crate to handle GIF animations.

### Imports

rustCopy code

`use image::RgbImage;
use std::io::{ self, Write };
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;` 

Here, various modules and functions are imported to handle image processing, I/O operations, and time-related tasks.

### ASCII Conversion Function

rustCopy code

`fn get_str_ascii(intent: u8) -> &'static str {
    let ascii = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@", "8", "&"];
    let index = (((intent as u16) * ((ascii.len() - 1) as u16)) / 255) as u8;
    return ascii[index as usize];
}` 

This function converts an intensity value (0-255) into an ASCII character. The choice of characters is designed to represent varying levels of shading, from light to dark.

### Main GIF Playback Function

rustCopy code

`fn play_gif_in_terminal(dir: &str, scale: u32) {
    //...
}` 

This function takes in the path to a GIF file and a scaling factor to adjust the resolution of the ASCII representation. It continuously plays the GIF in the terminal in an infinite loop.

Within the function:

-   The GIF file is opened and set up for RGBA decoding.
-   Each frame of the GIF is read and processed:
    -   The terminal is cleared.
    -   Pixel data of the frame is converted into an `RgbImage` for easier processing.
    -   Each pixel's intensity is calculated and converted into an ASCII character which is then printed to the terminal.
    -   After printing each frame, the program waits for the duration of the frame before moving to the next one.

### Main Entry Point

rustCopy code

`fn main() {
    play_gif_in_terminal("skellie.gif", 4);
}` 

The `main` function serves as the entry point of the program. It calls the `play_gif_in_terminal` function with a hardcoded GIF file and scale factor.