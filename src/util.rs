use std::path::PathBuf;

pub enum OutputType {
    Text,
    Image,
    Video,
}

impl OutputType {
    pub fn to_str(&self) -> &str {
        match self {
            OutputType::Text => "txt",
            OutputType::Image => "png",
            OutputType::Video => "mp4",
        }
    }
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
            _ => panic!(
                "Invalid output type: {}. Available types are 'txt' and 'img'",
                &args[1]
            ),
        };

        file_path = args[2].to_owned();
    };

    (output_type, PathBuf::from(file_path))
}