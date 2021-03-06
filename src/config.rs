use std::path::{Path, PathBuf};

use clap::{App, Arg, ArgMatches, Values};

use image::ImageFormat;

use BoxResult;

pub fn parse_args() -> BoxResult<Config> {
    let matches = App::new("Red Mountain Resize")
        .version(crate_version!())
        .author("Brad Glaser <bpglaser@gmail.com>")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .takes_value(true)
                .validator(validate_dist)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .takes_value(true)
                .validator(validate_dist)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("dimensions")
                .short("d")
                .long("dimensions")
                .conflicts_with_all(&["width", "height"])
                .required_unless_one(&["width", "height"])
                .value_name("WIDTHxHEIGHT")
                .takes_value(true)
                .number_of_values(2)
                .validator(validate_dimension)
                .value_delimiter("x"),
        )
        .arg(
            Arg::with_name("debug_path")
                .long("debug")
                .value_name("DEBUG_PATH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input_path")
                .required(true)
                .value_name("INPUT_PATH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output_path")
                .required(false)
                .value_name("OUTPUT_PATH")
                .takes_value(true)
                .validator(validate_extension),
        )
        .arg(Arg::with_name("time").short("t").long("time"))
        .get_matches();

    Config::try_from(matches)
}

fn validate_dist(s: String) -> Result<(), String> {
    match s.parse::<isize>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Invalid distance".to_owned()),
    }
}

fn validate_dimension(s: String) -> Result<(), String> {
    match s.parse::<isize>() {
        Ok(n) => {
            if n <= 0 {
                Err("Dimension must be greater than zero".to_owned())
            } else {
                Ok(())
            }
        }
        Err(_) => Err("Invalid dimension".to_owned()),
    }
}

fn validate_extension(s: String) -> Result<(), String> {
    let path = Path::new(&s);
    get_format(path).map(|_| ())
}

#[derive(Debug)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    pub width: Option<isize>,
    pub height: Option<isize>,
    pub dimensions: Option<(usize, usize)>,
    pub debug_path: Option<PathBuf>,
    pub time: bool,
}

impl Config {
    pub fn get_output_path(&mut self) -> &Path {
        match self.output_path {
            Some(ref output_path) => output_path,
            None => self.get_default_path(),
        }
    }

    fn get_default_path(&mut self) -> &Path {
        let mut output_path = self.input_path.clone();

        let mut stem = self.input_path.file_stem().unwrap().to_owned();
        stem.push("-resized");
        output_path.set_file_name(stem);

        let extension = self.input_path.extension().unwrap();
        output_path.set_extension(extension);

        self.output_path = Some(output_path);
        self.output_path.as_ref().unwrap()
    }

    fn try_from(matches: ArgMatches) -> BoxResult<Self> {
        let input_path = matches
            .value_of("input_path")
            .expect("the input path")
            .into();

        let output_path = matches.value_of("output_path").map(|s| s.into());
        let width = matches.value_of("width").and_then(|s| s.parse().ok());
        let height = matches.value_of("height").and_then(|s| s.parse().ok());

        let dimensions = matches.values_of("dimensions").map(
            Config::parse_dimensions,
        );

        let debug_path = matches.value_of("debug_path").map(|s| s.into());

        let time = matches.is_present("time");

        Ok(Config {
            input_path,
            output_path,
            width,
            height,
            dimensions,
            debug_path,
            time,
        })
    }

    fn parse_dimensions(mut values: Values) -> (usize, usize) {
        let x = values.next().and_then(|s| s.parse().ok()).expect("x value");
        let y = values.next().and_then(|s| s.parse().ok()).expect("y value");
        (x, y)
    }
}

pub fn get_format<P: AsRef<Path>>(path: P) -> Result<ImageFormat, String> {
    match get_extension(&path) {
        Some(extension) => {
            match extension.as_str() {
                "png" => Ok(ImageFormat::PNG),
                "jpg" | "jpeg" => Ok(ImageFormat::JPEG),
                "gif" => Ok(ImageFormat::GIF),
                "webp" => Ok(ImageFormat::WEBP),
                "ppm" => Ok(ImageFormat::PPM),
                "tif" | "tiff" => Ok(ImageFormat::TIFF),
                "tga" => Ok(ImageFormat::TGA),
                "bmp" => Ok(ImageFormat::BMP),
                "ico" => Ok(ImageFormat::ICO),
                "hdr" => Ok(ImageFormat::HDR),
                _ => Err("Invalid file extension".to_owned()),
            }
        }
        None => Err("No file extension given.".to_owned()),
    }
}

fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref().extension().and_then(|s| s.to_str()).map(
        |s| {
            s.to_lowercase()
        },
    )
}
