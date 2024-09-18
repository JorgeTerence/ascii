use image::ImageReader;

const TEXTURE: &[u8] = " .;coPO?S#".as_bytes();

fn main() {
    let (luminance, width, height) = read_image("think.jpg");
    let (scale_x, scale_y) = (8, 4);

    let mut buff: Vec<char> = vec![];

    for y in 0..(height - (height % scale_y)) {
        for x in 0..(width - (width % scale_x)) {
            let ascii_char =
                match TEXTURE.get(sample(&luminance, y, x, width, scale_y, scale_x)) {
                    None => panic!("Invalid luminance index"),
                    Some(v) => v.to_owned(),
                };

            buff.push(ascii_char as char);
        }
    }
}

fn sample(data: &Vec<u8>, y: u32, x: u32, w: u32, scale_y: u32, scale_x: u32) -> usize {
    let start = y * w + x;
    let end = (start + scale_y * w) as usize;
    let delta = (w - scale_x) as usize;

    return data[(start as usize)..end].iter().step_by(delta).sum::<u8>() as usize;
}

fn read_image(file_path: &str) -> (Vec<u8>, u32, u32) {
    match ImageReader::open(file_path) {
        Err(err) => panic!("Error reading file: {}", err),
        Ok(file) => {
            match file.decode() {
                Err(err) => panic!("Error decoding image: {}", err),
                Ok(img) => {
                    return (img.to_luma8().to_vec(), img.width(), img.height());
                }
            };
        }
    }
}
