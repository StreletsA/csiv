use std::env;
use std::ffi::OsStr;
use std::io::Read;
use std::path::Path;

use colored::Colorize;

use crate::commons::{ConsoleImagePrinter, ImageParser, ImagePrinter};
use crate::pnm::PPMImageParser;
use crate::default::DefaultImageParser;

mod commons;
mod pnm;
mod default;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path: &String = &args[1];
    let mut image_scale: f32 = 1f32;

    if args.len() > 2 {
        image_scale = args[2].parse().unwrap();
    }

    let rgb_image = define_image_parser(image_path).get_image(image_path);
    let image_printer = ConsoleImagePrinter;
    image_printer.print_image_scaled(&rgb_image, image_scale);
}

fn define_image_parser(image_path: &String) -> Box<dyn ImageParser> {
    let image_extension = Path::new(image_path)
        .extension()
        .and_then(OsStr::to_str)
        .expect("Image extension not found!");

    match image_extension {
        "ppm" => Box::new(PPMImageParser),
        _ => Box::new(DefaultImageParser)
    }
}
