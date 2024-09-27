mod img_proc;
mod util;

use image::{ImageBuffer, ImageReader, Luma};
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
        "{}.{}",
        pwd.join(file_path.file_stem().unwrap()).to_str().unwrap(),
        output_type.to_str()
    );

    match output_type {
        OutputType::Text => {
            let (scale_x, scale_y) = (4, 8);

            let mut buf: Vec<u8> = vec![];
            // let mut buf_luma: Vec<f32> = vec![];

            for y in (0..(height - (height % scale_y))).step_by(scale_y as usize) {
                for x in (0..(width - (width % scale_x))).step_by(scale_x as usize) {
                    let avg = sample(&luminance, y, x, width, scale_y, scale_x);

                    let index = avg as f32 / 32.0;

                    let ascii_char = match TXT_TEXTURE.get(index as usize) {
                        None => panic!("Invalid luminance index: [{}, {}] {}%", x, y, avg),
                        Some(v) => v.to_owned(),
                    };

                    buf.push(ascii_char);
                    // buf_luma.push(index);
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

            let mut canvas: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

            /*
            iterate over luma values, steping by TILE_SIZE
            for each TILE_SIZE x TILE_SIZE block, calculate the average luminance
            use the average luminance to index into the ascii texture
            draw the ascii character to the canvas
             */

            for y in (0..(height - (height % TILE_SIZE))).step_by(TILE_SIZE as usize) {
                for x in (0..(width - (width % TILE_SIZE))).step_by(TILE_SIZE as usize) {
                    let avg = sample(&luminance, y, x, width, TILE_SIZE, TILE_SIZE);

                    for i in 0..TILE_SIZE {
                        for j in 0..TILE_SIZE {
                            let index = avg as u32 / 32;
                            canvas.put_pixel(
                                x + j,
                                y + i,
                                Luma([atlas.get_pixel(TILE_SIZE * index + j, i).0[0]]),
                            );
                        }
                    }
                }
            }

            canvas.save_with_format(output_path, image::ImageFormat::Png).unwrap();
        }
    };
}

// make it interchangable between text buffer, image rect and video frame
// fn render_text(data: &Vec<u8>, width: u32, height: u32) {
//     let mut buf: Vec<u8> = vec![];

//     for y in (0..(height - (height % scale_y))).step_by(scale_y as usize) {
//         for x in (0..(width - (width % scale_x))).step_by(scale_x as usize) {
//             let avg = sample(&data, y, x, width, scale_y, scale_x);

//             let index = (avg as f32 / 32.0) as usize;

//             let ascii_char = match TEXTURE.get(index) {
//                 None => panic!("Invalid luminance index: [{}, {}] {}%", x, y, avg),
//                 Some(v) => v.to_owned(),
//             };

//             buf.push(ascii_char);
//         }
//         buf.push(10);
//     }
// }

// fn draw_image(data: &Vec<u8>, width: u32, height: u32) {
//     for y in 0..height {
//         for x in 0..width {
//             print!("{:3} ", data[(y * width + x) as usize]);
//         }
//         println!();
//     }
// }
