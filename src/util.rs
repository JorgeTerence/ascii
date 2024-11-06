use clap::{value_parser, Arg, ArgAction, Command};
use core::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum OutputType {
    Text,
    Image,
    Video,
}

impl FromStr for OutputType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "txt" => Ok(Self::Text),
            "img" => Ok(Self::Image),
            "video" => Ok(Self::Video),
            _ => Err(format!(
                "Invalid output type: {}. Available types are 'txt', 'img' and 'video'",
                s
            )),
        }
    }
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

pub struct Args {
    pub filepath: PathBuf,
    pub output_type: OutputType,
    pub display: bool,
    pub inverted: bool,
    pub _edge_detection: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("ascii")
            .version("0.8")
            .arg_required_else_help(true)
            .arg(Arg::new("filepath").value_parser(value_parser!(PathBuf)))
            .arg(
                Arg::new("output_type")
                    .short('o')
                    .long("output")
                    .value_parser(value_parser!(OutputType))
                    .default_value("img"),
            )
            .arg(
                Arg::new("display")
                    .short('d')
                    .long("display")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("inverted")
                    .short('i')
                    .long("invert")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("edge_detection")
                    .short('e')
                    .long("edges")
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        Self {
            filepath: matches
                .get_one::<PathBuf>("filepath")
                .expect("required")
                .clone(),
            output_type: matches
                .get_one::<OutputType>("output_type")
                .unwrap()
                .clone(),
            display: matches.get_one::<bool>("display").unwrap().clone(),
            inverted: matches.get_one::<bool>("inverted").unwrap().clone(),
            _edge_detection: matches.get_one::<bool>("edge_detection").unwrap().clone(),
        }
    }
}
