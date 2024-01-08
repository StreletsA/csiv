use std::io::Read;
use crate::commons::{ImageParser, RGBImage, RGBLine, RGBPixel};

const INCORRECT_FORMAT: &str = "Incorrect format!";
const COMMENT_PREFIX: &str = "#";
const UTF8_P: u8 = 80;
const UTF8_3: u8 = 51;
const UTF8_6: u8 = 54;
const UTF8_SPACE: u8 = 32;

struct PPMImageInfo {
    format: String,
    width: usize,
    height: usize,

    content: String
}

pub struct PPMImageParser;
impl PPMImageParser {
    pub fn new() -> PPMImageParser {
        return PPMImageParser;
    }

    fn get_image_p3(&self, bytes: &Vec<u8>) -> RGBImage {
        let binding = String::from_utf8(bytes.to_vec()).unwrap();
        let content_lines: Vec<_> = binding
            .lines()
            .filter(|line| !line.starts_with(COMMENT_PREFIX))
            .filter(|line| !line.is_empty())
            .collect();
        let binding = content_lines.join(" ").replace("  ", " ");
        let content: Vec<_> = binding
            .split(" ")
            .filter(|v| !v.is_empty())
            .skip(1)
            .collect();

        let width: i32 = content.get(0).expect(INCORRECT_FORMAT).to_string().parse().unwrap();
        let height: i32 = content.get(1).expect(INCORRECT_FORMAT).to_string().parse().unwrap();

        let image_info_start_index = 3;
        let mut used_values_count = 0;

        let mut rgb_image = RGBImage::new();

        for _ in (0..height) {
            let mut rgb_line = RGBLine::new();

            for _ in (0..width) {
                let index = image_info_start_index + used_values_count;
                let rgb_pixel = RGBPixel {
                    r: self.parse_u8_value(content.get(index).unwrap()),
                    g: self.parse_u8_value(content.get(index + 1).unwrap()),
                    b: self.parse_u8_value(content.get(index + 2).unwrap()),
                };
                rgb_line.add_rgb_pixel(rgb_pixel);

                used_values_count += 3;
            }

            rgb_image.add_rgb_line(rgb_line);
        }

        rgb_image
    }

    fn parse_u8_value(&self, u8_string: &str) -> u8 {
        let value: u16 = u8_string
            .to_string()
            .parse()
            .unwrap();

        if (value == 255) {
            return 255u8;
        }

        return value as u8;
    }

    fn get_image_p6(&self, bytes: &Vec<u8>) -> RGBImage {
        let mut index = 0;
        let mut spaces_count = 0;
        let mut header_without_format: Vec<u8> = Vec::new();

        // Skip format
        while *bytes.get(index).unwrap() != '\n' as u8 {
            if *bytes.get(index).unwrap() == ' ' as u8 {
                spaces_count += 1;
                if spaces_count > 0 {
                    header_without_format.push(*bytes.get(index).unwrap());
                }
            } else if spaces_count > 0 {
                header_without_format.push(*bytes.get(index).unwrap());
            }

            index += 1;
        }
        index += 1;

        // Skip comments
        while *bytes.get(index).unwrap() == '#' as u8 {
            while *bytes.get(index).unwrap() != '\n' as u8 {
                index += 1;
            }
            index += 1;
        }

        if header_without_format.is_empty() {
            spaces_count = 0;
            while *bytes.get(index).unwrap() != '\n' as u8 {
                if *bytes.get(index).unwrap() == ' ' as u8 {
                    spaces_count += 1;
                }

                header_without_format.push(*bytes.get(index).unwrap());

                index += 1;
            }
            index += 1;

            if (spaces_count == 1) {
                header_without_format.push(' ' as u8);
                while *bytes.get(index).unwrap() != '\n' as u8 {
                    header_without_format.push(*bytes.get(index).unwrap());
                    index += 1;
                }
                index += 1;
            }
        }

        let mut width_vec: Vec<u8> = Vec::new();
        let mut height_vec: Vec<u8> = Vec::new();
        let mut pixel_max_vec: Vec<u8> = Vec::new();

        let mut header_index = 0;
        while *header_without_format.get(header_index).unwrap() != ' ' as u8 {
            width_vec.push(*header_without_format.get(header_index).unwrap());
            header_index += 1;
        }
        header_index += 1;
        while *header_without_format.get(header_index).unwrap() != ' ' as u8 {
            height_vec.push(*header_without_format.get(header_index).unwrap());
            header_index += 1;
        }
        header_index += 1;
        while header_index < header_without_format.len() {
            pixel_max_vec.push(*header_without_format.get(header_index).unwrap());
            header_index += 1;
        }

        let width: i32 = String::from_utf8(width_vec).unwrap().parse().unwrap();
        let height: i32 = String::from_utf8(height_vec).unwrap().parse().unwrap();
        let pixel_max: i32 = String::from_utf8(pixel_max_vec).unwrap().parse().unwrap();

        let mut rgb_image = RGBImage::new();

        for _ in (0..height) {
            let mut rgb_line = RGBLine::new();

            for _ in (0..width) {

                let rgb_pixel = RGBPixel {
                    r: *bytes.get(index).unwrap(),
                    g: *bytes.get(index + 1).unwrap(),
                    b: *bytes.get(index + 2).unwrap(),
                };
                rgb_line.add_rgb_pixel(rgb_pixel);

                index += 3;
            }

            rgb_image.add_rgb_line(rgb_line);
        }

        rgb_image
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