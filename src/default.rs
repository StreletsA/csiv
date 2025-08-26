use image::GenericImageView;
use image::ImageReader;

use crate::commons::{ImageParser, RGBImage, RGBLine, RGBPixel};

pub struct DefaultImageParser;
impl ImageParser for DefaultImageParser {
    fn get_image(&self, image_path: &String) -> RGBImage {
        let image = ImageReader::open(image_path).unwrap().decode().unwrap();
        let width = image.width();
        let height = image.height();

        let mut rgb_image = RGBImage::of(width, height);

        for h in (0..height) {
            let mut rgb_line = RGBLine::new();

            for w in (0..width) {
                let original_pixel = image.get_pixel(w, h);
                let rgb_pixel = RGBPixel {
                    r: original_pixel.0[0],
                    g: original_pixel.0[1],
                    b: original_pixel.0[2],
                };
                rgb_line.add_rgb_pixel(rgb_pixel);
            }

            rgb_image.add_rgb_line(rgb_line);
        }

        rgb_image
    }
}