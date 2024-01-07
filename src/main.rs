use std::collections::LinkedList;
use std::fs::File;
use std::io::Read;
use crate::commons::{ConsoleImagePrinter, ImageParser, ImagePrinter};
use crate::pnm::PPMImageParser;
use colored::Colorize;

mod commons;
mod pnm;

fn main() {
    let mut buf: Vec<u8> = Vec::new();
    File::open("C:\\Users\\astrelets\\Documents\\p3_image.ppm").unwrap().read_to_end(&mut buf).unwrap();

    let viewer = PPMImageParser;
    let rgb_image = viewer.get_image(&buf);

    let image_printer = ConsoleImagePrinter;
    image_printer.print_image(&rgb_image);
}
