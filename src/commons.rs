use std::thread::sleep;
use std::time::Duration;
use colored::{ColoredString, Colorize, Color};

#[derive(Debug)]
pub struct RGBPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct RGBLine {
    width: usize,
    rgb_pixels: Vec<RGBPixel>
}

impl RGBLine {
    pub fn new() -> RGBLine {
        RGBLine { width: 0, rgb_pixels: Vec::new() }
    }

    pub fn add_rgb_pixel(&mut self, rgb_pixel: RGBPixel) {
        self.width += 1;
        self.rgb_pixels.push(rgb_pixel);
    }
}

pub struct RGBImage {
    width: usize,
    height: usize,
    rgb_lines: Vec<RGBLine>
}

impl RGBImage {
    pub fn new() -> RGBImage {
        RGBImage { width: 0, height: 0, rgb_lines: Vec::new() }
    }

    pub fn add_rgb_line(&mut self, rgb_line: RGBLine) {
        self.height += 1;
        self.width = rgb_line.width;
        self.rgb_lines.push(rgb_line);
    }
}

pub enum ImageExtension {
    PPM
}

pub trait ImageParser {
    fn get_image(&self, bytes: &Vec<u8>) -> RGBImage;
}

pub trait ImagePrinter {
    fn print_image(&self, image: &RGBImage);
}

const PIXEL: &str = " ";

pub struct ConsoleImagePrinter;
impl ImagePrinter for ConsoleImagePrinter {
    fn print_image(&self, image: &RGBImage) {
        for rgb_line in &image.rgb_lines {
            for rgb_pixel in &rgb_line.rgb_pixels {
                let pixel_color = Color::TrueColor {
                    r: rgb_pixel.r,
                    g: rgb_pixel.g,
                    b: rgb_pixel.b,
                };
                print!("{}", PIXEL.on_color(pixel_color));
            }
            println!()
        }
    }
}