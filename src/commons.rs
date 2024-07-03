use colored::{Color, Colorize};

#[derive(Debug)]
pub struct RGBPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct RGBLine {
    width: u32,
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
    width: u32,
    height: u32,
    rgb_lines: Vec<RGBLine>
}

impl RGBImage {
    pub fn new() -> RGBImage {
        RGBImage { width: 0, height: 0, rgb_lines: Vec::new() }
    }

    pub fn of(width: u32, height: u32) -> RGBImage {
        let mut rgb_lines: Vec<RGBLine> = Vec::new();
        for _ in (0..height) {
            let mut rgb_line = RGBLine::new();
            for _ in (0..width) {
                rgb_line.add_rgb_pixel(RGBPixel {
                    r: 0,
                    g: 0,
                    b: 0,
                });
            }
            rgb_lines.push(rgb_line);
        }

        RGBImage { width, height, rgb_lines }
    }

    pub fn add_rgb_line(&mut self, rgb_line: RGBLine) {
        self.height += 1;
        self.width = rgb_line.width;
        self.rgb_lines.push(rgb_line);
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &RGBPixel {
        self.rgb_lines.get(y - 1).unwrap().rgb_pixels.get(x - 1).unwrap()
    }
}

pub trait ImageParser {
    fn get_image(&self, image_path: &String) -> RGBImage;
}

pub trait ImagePrinter {
    fn print_image(&self, image: &RGBImage);
    fn print_image_scaled(&self, image: &RGBImage, scale: f32);
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

    fn print_image_scaled(&self, image: &RGBImage, scale: f32) {
        let original_width = image.width;
        let original_height = image.height;
        let new_width = (original_width as f32 * scale) as usize;
        let new_height = (original_height as f32 * scale) as usize;

        let mut new_image = RGBImage::new();

        for row in (0..new_height) {
            let mut rgb_line = RGBLine::new();
            for col in (0..new_width) {
                let original_pixel = image.get_pixel((col as f32 / scale) as usize + 1,
                                                     (row as f32 / scale) as usize + 1);
                rgb_line.add_rgb_pixel(RGBPixel {
                    r: original_pixel.r,
                    g: original_pixel.g,
                    b: original_pixel.b,
                })
            }
            new_image.add_rgb_line(rgb_line);
        }

        self.print_image(&new_image);
    }
}