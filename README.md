# PPM-LAB

PPM-LAB reads a PPM P3 image, applies a selected image filter, and writes the processed image back to disk.

## Usage

```bash
cargo run -- <img-path> <filter>
```

### Available Filters

| Filter          | Argument    |
| :-------------- | :---------- |
| Grayscale       | `grayscale` |
| Invert          | `invert`    |
| Box Blur        | `boxblur`   |
| Contrast        | `contrast`  |
| Red             | `r`         |
| Green           | `g`         |
| Blue            | `b`         |
| Magenta         | `m`         |
| Yellow          | `y`         |
| Cyan            | `c`         |
| Blue and Green  | `bg`        |
| Green and Blue  | `gb`        |
| Red and Blue    | `rb`        |
| Magenta and Red | `mr`        |
| Red and Green   | `rg`        |

## Image Credits

The sample images in the `examples/` directory were converted to the PPM P3 format from photographs on Unsplash.

- **Bailey Zindel** — https://unsplash.com/photos/body-of-water-surrounded-by-trees-NRQV-hBF10M

These images are used under the Unsplash License:
https://unsplash.com/license

Examples:

- Invert

```bash
cargo run -- examples/bailey-zindel-unsplash.ppm invert
```

![invert](/examples/invert.png)

- Yellow

```bash
cargo run -- examples/bailey-zindel-unsplash.ppm y
```

![yellow](/examples/yellow.png)

- box blur

```bash
cargo run -- examples/bailey-zindel-unsplash.ppm boxblur
```

![box-blur](/examples/box_blur.png)

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

## License

This project is licensed under the MIT License.
