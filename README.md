# PPM-LAB

PPM-LAB reads a PPM P3 image, applies a selected image filter, and writes the processed image back to disk.

## Supported Filters

- [x] Grayscale filter
- [x] Invert filter
- [x] Box blur filter

Examples:

- Original image

![grayscale](./screenshots/original.png)

- Gray scale

```bash
cargo run -- assets/moon.ppm grayscale
```

![grayscale](./screenshots/grayscale.png)

- Invert

```bash
cargo run -- assets/moon.ppm invert
```

![invert](./screenshots/invert.png)

- box blur

```bash
cargo run -- assets/moon.ppm boxblur
```

![box-blur](./screenshots/box_blur.png)

## Installation

Clone the repository:

```bash
git clone https://github.com/l1l-01/ppm-lab
cd ppm-lab
```

Build the project:

```bash
cargo build --release
```

Or run it directly:

```bash
cargo run -- <image_path> <filter>
```

## Usage

```bash
cargo run -- image.ppm grayscale
```

```bash
cargo run -- image.ppm boxblur
```

## License

This project is licensed under the MIT License.
