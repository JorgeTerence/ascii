use image::ImageReader;
use std::io::Write;

const TEXTURE: &[u8] = " .;coPO?S#".as_bytes();

fn main() {
    let (luminance, width, height) = read_image("nois.jpg");
    let (scale_x, scale_y) = (4, 8);

    let mut buf: Vec<u8> = vec![];

    for y in (0..(height - (height % scale_y))).step_by(scale_y as usize) {
        for x in (0..(width - (width % scale_x))).step_by(scale_x as usize) {
            let avg = sample(&luminance, y, x, width, scale_y, scale_x);
            let index = (avg as f32 / 64.0) as usize;

            let ascii_char = match TEXTURE.get(index) {
                None => panic!("Invalid luminance index: [{}, {}] {}%", x, y, avg),
                Some(v) => v.to_owned(),
            };

            buf.push(ascii_char);
        }
        buf.push(10);
    }

    let mut file = std::fs::File::create("ascii_art.txt").unwrap();
    match file.write_all(&buf) {
        Err(err) => panic!("Error writing file: {}", err),
        Ok(_) => println!("File written successfully!"),
    };
}

fn sample(
    data: &Vec<u8>,
    y: u32,
    x: u32,
    width: u32,
    scale_y: u32,
    scale_x: u32,
) -> u32 {
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
