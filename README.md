# Gibberish

Turn any image (_and soon video_) into ascii art.

![Vagabond ascii art](./.github/assets/musashi-ascii.png)

See the galery.

## About

This project is inspired by a video from the graphics programmer and content creater [Acerola](https://www.youtube.com/@Acerola_t), who made a [video on ascii art](https://youtu.be/gg40RWiaHRY?si=-8QZkvO8Thm2zgVa).

<!-- ## Environment setup

Check out [https://scoop.sh](https://scoop.sh) for help installing `scoop`.

```sh
scoop install 7zip

$VCINSTALLDIR = $(& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -property installationPath)
Add-Content $env:GITHUB_ENV "LIBCLANG_PATH=${VCINSTALLDIR}\VC\Tools\LLVM\x64\bin`n"
Invoke-WebRequest "${env:FFMPEG_DOWNLOAD_URL}" -OutFile ffmpeg-release-full-shared.7z
7z x ffmpeg-release-full-shared.7z
mkdir ffmpeg
mv ffmpeg-*/* ffmpeg/
Add-Content $env:GITHUB_ENV "FFMPEG_DIR=${pwd}\ffmpeg`n"
Add-Content $env:GITHUB_PATH "${pwd}\ffmpeg\bin`n"
``` -->

## Installation

```sh
cargo install https://github.com/JorgeTerence/ascii
```

## How to use

```sh
ascii path/to/media
```

## Future plans

- [ ] Edge detection
- [ ] Support for video formats
- [ ] Live video from camera stream

## Galery

<!-- Photo by Kai-Chieh Chan: https://www.pexels.com/photo/red-and-brown-temple-569893/ -->
