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

    fn get_image_p3(&self, content: &Vec<&str>) -> RGBImage {
        let width: i32 = content.get(1).expect(INCORRECT_FORMAT).to_string().parse().unwrap();
        let height: i32 = content.get(2).expect(INCORRECT_FORMAT).to_string().parse().unwrap();

        let image_info_start_index = 4;
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

    fn get_image_p6(&self, content: &Vec<&str>) -> RGBImage {
        todo!()
    }
}

impl ImageParser for PPMImageParser {
    fn get_image(&self, bytes: &Vec<u8>) -> RGBImage {
        let content_length = bytes.len();
        if (content_length < 10) {
            panic!("No content!")
        }

        let binding = String::from_utf8(bytes.to_vec()).unwrap();
        let content_lines: Vec<_> = binding
            .lines()
            .filter(|line| !line.starts_with("#"))
            .filter(|line| !line.is_empty())
            .collect();
        let binding = content_lines.join(" ").replace("  ", " ");
        let content: Vec<_> = binding
            .split(" ")
            .filter(|v| !v.is_empty())
            .collect();

        let format_type = content.get(0).expect(INCORRECT_FORMAT);
        if (*format_type == "P3") {
            self.get_image_p3(&content)
        } else if (*format_type == "P6") {
            self.get_image_p6(&content)
        } else {
            panic!("{}", INCORRECT_FORMAT)
        }
    }
}