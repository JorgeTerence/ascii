use clap::Parser;
use core::fmt;
use std::path::PathBuf;

pub enum OutputType {
    Text,
    Image,
    Video,
}

impl fmt::Display for OutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputType::Text => write!(f, "txt"),
            OutputType::Image => write!(f, "png"),
            OutputType::Video => write!(f, "mp4"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    input: Option<PathBuf>,

    #[arg(long, default_value = OutputType::Image)]
    output: OutputType,

    #[arg(short, default_value = false)]
    invert: bool,
    #[arg(short, default_value = false)]
    edge_detection: bool,
    #[arg(short, default_value = false)]
    display: bool,
}

pub fn parse_args() -> (OutputType, PathBuf) {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("Media path not provided: ascii <image_path>  --output_type");
    }

    let output_type: OutputType;
    let file_path: String;

    if args.len() < 3 {
        output_type = OutputType::Text;
        file_path = args[1].to_owned();
    } else {
        output_type = match &args[1][2..] as &str {
            "txt" => OutputType::Text,
            "img" => OutputType::Image,
            "video" => OutputType::Video,
            _ => panic!(
                "Invalid output type: {}. Available types are 'txt', 'img' and 'video'",
                &args[1]
            ),
        };

        file_path = args[2].to_owned();
    };

    (output_type, PathBuf::from(file_path))
}
