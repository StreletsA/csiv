use std::str::FromStr;
use clap::{command, Arg};

const IMG_FILE_PATH: &str = "image";
const IMG_SCALE: &str = "scale";

pub struct CSIVParams {
    // Image file path
    pub file_path: String,
    // Image scale
    pub scale: f32,
}

pub fn parse_params() -> CSIVParams {
    let arg_matches = command!()
        .arg(Arg::new(IMG_FILE_PATH).short('i').long(IMG_FILE_PATH).help("Image filepath").required(true))
        .arg(Arg::new(IMG_SCALE).short('s').long(IMG_SCALE).help("Image scale").default_value("1").required(false))
        .get_matches();

    let file_path_arg_value = arg_matches.get_one::<String>(IMG_FILE_PATH).unwrap().as_str();
    let scale_arg_value = f32::from_str(
        arg_matches.get_one::<String>(IMG_SCALE).unwrap().as_str()
    ).expect("Image scale must be a float!");

    CSIVParams {
        file_path: file_path_arg_value.to_string(),
        scale: scale_arg_value
    }
}
