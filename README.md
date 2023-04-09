# SONIRAWWWR

NOTE: I don't reccomend using this just yet because, the colour demosaicing is obviously not working right -- the green channel's values are not being converted properly which leaves a soft pink hue over any images, which also has the apperance of softening focus.

SONIRAWWWR is a command-line interface (CLI) tool for converting RAW image files to various image formats such as PNG, JPG, and TIFF.

## Installation

You can install SONIRAWWWR by cloning the repository and running the following command in the project directory:

```bash
cargo install --path 
```

## Usage

SONIRAWWWR can be used by specifying the input file using the `-f` or `--filepath` option and the export format using the `-e` or `--export-format` option. Here's an example command for exporting a RAW image to PNG format:

```bash
sonirawwr -i /path/to/input/file.ARW -o outputpath.png #or, from _this_ directory:
cargo run --release -- -i /path/to/input/file.ARW -o outputpath.png 
```
