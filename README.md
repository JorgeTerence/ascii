# Ascii

Turn any image (_and soon video_) into ascii art.

![Vagabond ascii art](./.github/assets/musashi-ascii.png)

[_Go to galery_](#galery)

## About

This project is inspired by a video from the graphics programmer and content creator [Acerola](https://www.youtube.com/@Acerola_t), who made a [video on ascii art](https://youtu.be/gg40RWiaHRY?si=-8QZkvO8Thm2zgVa).

In a nutshell, the RGB values of a pixel can converge into a luminance value. Sampling the luminance values of an image results in a grayscale image. What this application does is to take this luminance value and use it as an index for a texture map. Ascii characters are used based on how much space their glyphs fill. The image is also downscaled so that the characters are visible from the default scale.

![Luminance texture atlas](https://github.com/user-attachments/assets/52dee645-b5c9-491f-8cd5-85068f8a8590)

The function used for calculating the luminance value is `sample`:

```rust
fn sample(data: &Vec<u8>, y: u32, x: u32, width: u32, scale_y: u32, scale_x: u32) -> u32 {
    let mut sum = 0;

    for i in y..y + scale_y {
        for j in x..x + scale_x {
            sum += data[(i * width + j) as usize] as u32;
        }
    }

    sum / (scale_x * scale_y)
}
```

After sampling the luminance, it is used as an index for either the texture atlas or character array, wether the export options are set for text or image:

```rust
let avg = sample(&luminance, y, x, width, TILE_SIZE, TILE_SIZE);
let mut index = avg / 32;

for i in 0..TILE_SIZE {
    for j in 0..TILE_SIZE {
        let luma = (atlas.get_pixel(TILE_SIZE * index + j, i).0[0] as f32
            * (index as f32 / 10.0))
            as u8;
        canvas.put_pixel(x + j, y + i, Luma([luma]));
    }
}
```

## Installation

```sh
cargo install --git https://github.com/JorgeTerence/ascii
```

Or download the executable from the latest release.

## How to use

```sh
ascii path/to/media
```

### Options

- `-d` or `--display`: Automatically open the output in the system's default media viewer;
- `-i` or `--invert`: Invert the input media's luminance values;
- `-o` or `--output`: Set output format; accepts _img_, _txt_ or _video_ (not yet implemented)

## Future plans

- [ ] Edge detection
- [ ] Support for video formats
- [ ] Live video from camera stream

## Galery

![Temple ascii art](.github/assets/temple-ascii.png)

Photo by Kai-Chieh Chan: [https://www.pexels.com/photo/red-and-brown-temple-569893](https://www.pexels.com/photo/red-and-brown-temple-569893)

![In the Court of the Crimson King](.github/assets/court-of-the-crimson-king-ascii.png)

![Division Bell](.github/assets/division-bell-ascii.png)

![Cat biting a sock](.github/assets/nico-sock-ascii.png)

![Cat wearing glasses](.github/assets/nico-glasses-ascii.png)
