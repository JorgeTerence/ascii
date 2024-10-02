mod img_proc;
mod util;

use image::{ImageBuffer, ImageReader, Rgb};
use img_proc::{read_image, sample};
use std::{env::current_dir, io::Write, path::PathBuf};
use util::{parse_args, OutputType};

const TXT_TEXTURE: &[u8] = " .;coPO?S#".as_bytes();
const TILE_SIZE: u32 = 8;

fn main() {
    let (output_type, file_path) = parse_args();

    let (luminance, width, height) = read_image(file_path.to_str().unwrap());

    let pwd = PathBuf::from(current_dir().unwrap());
    let output_path = format!(
        "{}-ascii.{}",
        pwd.join(file_path.file_stem().unwrap()).to_str().unwrap(),
        output_type.to_str()
    );

    match output_type {
        OutputType::Text => {
            let (scale_x, scale_y) = (4, 8);
            let mut buf: Vec<u8> = vec![];

            for y in (0..(height - (height % scale_y))).step_by(scale_y as usize) {
                for x in (0..(width - (width % scale_x))).step_by(scale_x as usize) {
                    let avg = sample(&luminance, y, x, width, scale_y, scale_x);

                    let index = avg as f32 / 32.0;

                    let ascii_char = match TXT_TEXTURE.get(index as usize) {
                        None => panic!("Invalid luminance index: [{}, {}] {}%", x, y, avg),
                        Some(v) => v.to_owned(),
                    };

                    buf.push(ascii_char);
                }

                buf.push(10);
            }

            let mut file = std::fs::File::create(output_path).unwrap();

            match file.write_all(&buf) {
                Err(err) => panic!("Error writing file: {}", err),
                Ok(_) => println!("File written successfully!"),
            };
        }

        OutputType::Image => {
            let atlas = ImageReader::open("atlas.png")
                .unwrap()
                .decode()
                .unwrap()
                .to_luma8();

            let mut canvas: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

            for y in (0..(height - (height % TILE_SIZE))).step_by(TILE_SIZE as usize) {
                for x in (0..(width - (width % TILE_SIZE))).step_by(TILE_SIZE as usize) {
                    let avg = sample(&luminance, y, x, width, TILE_SIZE, TILE_SIZE);
                    let index = avg / 32;

                    for i in 0..TILE_SIZE {
                        for j in 0..TILE_SIZE {
                            let luma = (atlas.get_pixel(TILE_SIZE * index + j, i).0[0] as f32
                                * (index as f32 / 10.0))
                                as u8;
                            canvas.put_pixel(x + j, y + i, Rgb([luma, luma, luma]));
                        }
                    }
                }
            }

            canvas
                .save_with_format(output_path, image::ImageFormat::Png)
                .unwrap();
        }
    };
}
