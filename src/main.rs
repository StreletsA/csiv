use std::ffi::OsStr;
use std::io::Read;
use std::path::Path;

use colored::Colorize;

use crate::commons::{ConsoleImagePrinter, ImageParser, ImagePrinter};
use crate::default::DefaultImageParser;
use crate::pnm::PPMImageParser;

mod commons;
mod pnm;
mod default;
mod cli;

fn main() {
    let csiv_params = cli::parse_params();
    let rgb_image = define_image_parser(&csiv_params.file_path).get_image(&csiv_params.file_path);
    let image_printer = ConsoleImagePrinter;
    image_printer.print_image_scaled(&rgb_image, csiv_params.scale);
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
