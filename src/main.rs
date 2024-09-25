mod img_proc;
mod util;

use image::Luma;
use img_proc::{read_image, sample};
use std::{env::current_dir, io::Write, path::PathBuf};
use util::{parse_args, OutputType};

const TXT_TEXTURE: &[u8] = " .;coPO?S#".as_bytes();
const ATLAS: &[u8] = include_bytes!("../atlas.png");
const TILE_SIZE: u32 = 8;

fn main() {
    let (output_type, file_path) = parse_args();

    let (luminance, width, height) = read_image(file_path.to_str().unwrap());
    let (scale_x, scale_y) = (4, 8);

    let mut buf: Vec<u8> = vec![];
    let mut buf_luma: Vec<f32> = vec![];

    for y in (0..(height - (height % scale_y))).step_by(scale_y as usize) {
        for x in (0..(width - (width % scale_x))).step_by(scale_x as usize) {
            let avg = sample(&luminance, y, x, width, scale_y, scale_x);

            let index = avg as f32 / 32.0;

            let ascii_char = match TXT_TEXTURE.get(index as usize) {
                None => panic!("Invalid luminance index: [{}, {}] {}%", x, y, avg),
                Some(v) => v.to_owned(),
            };

            buf.push(ascii_char);
            buf_luma.push(index);
        }
        buf.push(10);
    }

    let pwd = PathBuf::from(current_dir().unwrap());
    let output_path = format!(
        "{}.{}",
        pwd.join(file_path.file_stem().unwrap()).to_str().unwrap(),
        output_type.to_str()
    );

    let mut file = std::fs::File::create(output_path).unwrap();

    match output_type {
        OutputType::Text => {
            match file.write_all(&buf) {
                Err(err) => panic!("Error writing file: {}", err),
                Ok(_) => println!("File written successfully!"),
            };
        }

        OutputType::Image => {
            let mut canvas = image::ImageBuffer::new(width, height);
            canvas.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
                let index = (y * width + x) as usize;
                *pixel = Luma([ATLAS[index]]); // TODO: do some fancy math
            });
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
