mod img_proc;
mod util;

use image::{load_from_memory_with_format, GenericImageView, ImageBuffer, ImageFormat, Luma};
use img_proc::{read_image, sample};
use std::process::Command;
use std::{env, io::Write, path::PathBuf};
use util::{Args, OutputType};

const TXT_TEXTURE: &[u8] = " .;coPO?S#".as_bytes();
const TILE_SIZE: u32 = 8;
static ATLAS: &'static [u8] = include_bytes!("../atlas.png");

fn main() {
    let args = Args::parse();

    let pwd = PathBuf::from(env::current_dir().expect("Failed to locate $PWD"));
    let output_path = format!(
        "{}-ascii.{}",
        pwd.join(args.filepath.file_stem().expect("Failed to trim file stem"))
            .to_str()
            .expect("Failed to write path string"),
        args.output_type
    );

    match args.output_type {
        OutputType::Text => {
            let (luminance, width, height) =
                read_image(args.filepath.to_str().expect("Failed to write file path"));

            let (scale_x, scale_y) = (4, 8);
            let mut buf: Vec<u8> = vec![];

            for y in (0..(height - (height % scale_y))).step_by(scale_y as usize) {
                for x in (0..(width - (width % scale_x))).step_by(scale_x as usize) {
                    let avg = sample(&luminance, y, x, width, scale_y, scale_x);

                    let mut index = (avg as f32 / 32.0) as usize;

                    if args.inverted {
                        index = (7 - index) % 8;
                    };

                    let ascii_char = TXT_TEXTURE
                        .get(index)
                        .expect(&format!("Invalid luminance index: [{}, {}]", x, y))
                        .clone();

                    buf.push(ascii_char);
                }
                buf.push(10);
            }

            let mut file =
                std::fs::File::create(output_path.clone()).expect("Faile to create file");

            if let Err(err) = file.write_all(&buf) {
                panic!("Error writing file: {}", err);
            }
        }

        OutputType::Image => {
            let (luminance, width, height) = read_image(
                args.filepath
                    .to_str()
                    .expect("Failed to read input image data"),
            );

            let atlas = load_from_memory_with_format(ATLAS, ImageFormat::Png).unwrap();

            let mut canvas: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

            for y in (0..(height - (height % TILE_SIZE))).step_by(TILE_SIZE as usize) {
                for x in (0..(width - (width % TILE_SIZE))).step_by(TILE_SIZE as usize) {
                    let avg = sample(&luminance, y, x, width, TILE_SIZE, TILE_SIZE);
                    let mut index = avg / 32;

                    if args.inverted {
                        index = (7 - index) % 8;
                    };

                    for i in 0..TILE_SIZE {
                        for j in 0..TILE_SIZE {
                            let luma = (atlas.get_pixel(TILE_SIZE * index + j, i).0[0] as f32
                                * (index as f32 / 10.0))
                                as u8;
                            canvas.put_pixel(x + j, y + i, Luma([luma]));
                        }
                    }
                }
            }

            canvas
                .save_with_format(output_path.clone(), image::ImageFormat::Png)
                .expect("Failed to save output image");
        }

        OutputType::Video => {
            todo!("Implement video codec");
        }
    };

    if args.display {
        match std::env::consts::OS {
            "windows" => {
                _ = Command::new("cmd")
                    .arg("/C")
                    .arg("start")
                    .arg(output_path)
                    .output()
                    .expect("Failed to open photos");
            }
            "mac" => {
                match Command::new("open").arg(output_path).spawn() {
                    Ok(_) => (),
                    Err(err) => eprintln!("Failed to open file: {}", err),
                };
            }
            "linux" => {
                let programs = match args.output_type {
                    OutputType::Text => ["gedit", "kate", "leafpad", "nano", "cat"],
                    OutputType::Image => ["eog", "gthumb", "gwenview", "ristretto", "feh"],
                    OutputType::Video => ["vlc", "mpv", "smplayer", "mplayer", "ffplay"],
                };

                for viewer in programs {
                    if let Ok(o) = Command::new("which").arg(viewer).output() {
                        if o.status.success() {
                            _ = Command::new(viewer).arg(output_path.clone()).spawn();
                            return;
                        }
                    }
                }

                println!("Failed to display {}", output_path);
            }
            _ => println!(
                "Failed to find suitable application for opening the file. Output written to {}",
                output_path
            ),
        };
    }
}
