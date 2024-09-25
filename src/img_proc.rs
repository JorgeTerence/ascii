use image::ImageReader;

pub fn sample(data: &Vec<u8>, y: u32, x: u32, width: u32, scale_y: u32, scale_x: u32) -> u32 {
    let mut sum = 0;

    for i in y..y + scale_y {
        for j in x..x + scale_x {
            sum += data[(i * width + j) as usize] as u32;
        }
    }

    sum / (scale_x * scale_y)
}

pub fn read_image(file_path: &str) -> (Vec<u8>, u32, u32) {
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