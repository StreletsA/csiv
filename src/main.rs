use std::fs::File;
use std::io::Read;

use colored::Colorize;

use crate::commons::{ConsoleImagePrinter, ImageParser, ImagePrinter};
use crate::pnm::PPMImageParser;

mod commons;
mod pnm;

fn main() {
    let mut buf: Vec<u8> = Vec::new();
    File::open("C:\\Users\\astrelets\\Downloads\\sample_1920×1280.ppm").unwrap().read_to_end(&mut buf).unwrap();

    let viewer = PPMImageParser;
    let rgb_image = viewer.get_image(&buf);

    let image_printer = ConsoleImagePrinter;
    image_printer.print_image_scaled(&rgb_image, 0.2);
}
