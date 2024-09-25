use image::ImageReader;
use std::{io::Write, path::PathBuf};

const TEXTURE: &[u8] = " .;coPO?S#".as_bytes();

fn main() {
    let (output_type, file_path) = parse_args();

    let (luminance, width, height) = read_image(file_path.to_str().unwrap());
    let (scale_x, scale_y) = (4, 8);

    let mut buf: Vec<u8> = vec![];

    for y in (0..(height - (height % scale_y))).step_by(scale_y as usize) {
        for x in (0..(width - (width % scale_x))).step_by(scale_x as usize) {
            let avg = sample(&luminance, y, x, width, scale_y, scale_x);

            let index = (avg as f32 / 32.0) as usize;

            let ascii_char = match TEXTURE.get(index) {
                None => panic!("Invalid luminance index: [{}, {}] {}%", x, y, avg),
                Some(v) => v.to_owned(),
            };

            buf.push(ascii_char);
        }
        buf.push(10);
    }

    let pwd = PathBuf::from(std::env::current_dir().unwrap());
    let output_path = format!(
        "{}.{}",
        pwd.join(file_path.file_stem().unwrap()).to_str().unwrap(),
        output_type
    );

    let mut file = std::fs::File::create(output_path).unwrap();

    match file.write_all(&buf) {
        Err(err) => panic!("Error writing file: {}", err),
        Ok(_) => println!("File written successfully!"),
    };
}

fn sample(data: &Vec<u8>, y: u32, x: u32, width: u32, scale_y: u32, scale_x: u32) -> u32 {
    let mut sum = 0;

    for i in y..y + scale_y {
        for j in x..x + scale_x {
            sum += data[(i * width + j) as usize] as u32;
        }
    }

    sum / (scale_x * scale_y)
}

fn read_image(file_path: &str) -> (Vec<u8>, u32, u32) {
    match ImageReader::open(file_path) {
        Err(err) => panic!("Error reading file: {}", err),
        Ok(file) => {
            match file.decode() {
                Err(err) => panic!("Error decoding image: {}", err),
                Ok(img) => return (img.to_luma8().to_vec(), img.width(), img.height()),
            };
        }
    }
}

fn parse_args() -> (String, PathBuf) {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        panic!("Media path not provided: ascii <image_path>  --output_type");
    }

    let output_type: &str;
    let file_path: String;

    if args.len() < 3 {
        output_type = "txt";
        file_path = args[1].to_owned();
    } else {
        output_type = match &args[1][2..] as &str {
            "txt" => "txt",
            "img" => "png",
            _ => panic!(
                "Invalid output type: {}. Available types are 'txt' and 'img'",
                &args[1]
            ),
        };

        file_path = args[2].to_owned();
    };

    (output_type.to_string(), PathBuf::from(file_path))
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
