use crate::commons::{ImageParser, RGBImage, RGBLine, RGBPixel};

const INCORRECT_FORMAT: &str = "Incorrect format!";
const UTF8_P: u8 = 80;
const UTF8_3: u8 = 51;
const UTF8_6: u8 = 54;

pub struct PPMImageParser;
impl PPMImageParser {
    pub fn new() -> PPMImageParser {
        return PPMImageParser;
    }

    fn get_image_p3(&self, bytes: &Vec<u8>) -> RGBImage {
        let binding = String::from_utf8(bytes.to_vec()).unwrap();
        let content_lines: Vec<_> = binding
            .lines()
            .filter(|line| !line.starts_with("#"))
            .filter(|line| !line.is_empty())
            .skip(1)
            .collect();

        let width_and_height: Vec<_> = content_lines.get(0)
            .expect(INCORRECT_FORMAT)
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();
        let width_and_height_len = width_and_height.len();
        if (width_and_height_len < 2) {
            panic!("{}", INCORRECT_FORMAT)
        }

        let width: i32 = width_and_height.get(0).expect(INCORRECT_FORMAT).to_string().parse().unwrap();
        let height: i32 = width_and_height.get(1).expect(INCORRECT_FORMAT).to_string().parse().unwrap();

        let image_info_start_line_index = 2;
        let mut created_pixels_count = 0;

        let mut rgb_image = RGBImage::new();

        for _ in (0..height) {
            let mut rgb_line = RGBLine::new();

            for _ in (0..width) {
                let line_index = image_info_start_line_index + created_pixels_count;
                let line = content_lines.get(line_index).expect(INCORRECT_FORMAT);
                let pixel_line_info: Vec<_> = line.split(" ").collect();

                let rgb_pixel = RGBPixel {
                    r: self.parse_u8_value(&pixel_line_info, 0),
                    g: self.parse_u8_value(&pixel_line_info, 1),
                    b: self.parse_u8_value(&pixel_line_info, 2)
                };

                rgb_line.add_rgb_pixel(rgb_pixel);

                created_pixels_count += 1;
            }


            rgb_image.add_rgb_line(rgb_line);
        }

        rgb_image
    }

    fn parse_u8_value(&self, pixel_line_info: &Vec<&str>, index: usize) -> u8 {
        let value: u16 = pixel_line_info.get(index)
            .expect(INCORRECT_FORMAT)
            .to_string()
            .parse()
            .unwrap();

        if (value == 255) {
            return 255u8;
        }

        return value as u8;
    }

    fn get_image_p6(&self, bytes: &Vec<u8>) -> RGBImage {
        todo!()
    }
}

impl ImageParser for PPMImageParser {
    fn get_image(&self, bytes: &Vec<u8>) -> RGBImage {
        let content_length = bytes.len();
        if (content_length < 10) {
            panic!("No content!")
        }

        // First byte must be P symbol
        bytes.get(0).filter(|byte| **byte == UTF8_P).expect(INCORRECT_FORMAT);
        // Second byte must be 3 or 6
        let format_type = bytes.get(1).expect(INCORRECT_FORMAT);
        if (*format_type == UTF8_3) {
            self.get_image_p3(bytes)
        } else if (*format_type == UTF8_6) {
            self.get_image_p6(bytes)
        } else {
            panic!("{}", INCORRECT_FORMAT)
        }
    }
}