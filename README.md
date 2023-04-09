# SONIRAWWWR

SONIRAWWWR is a command-line interface (CLI) tool for converting RAW image files to various image formats such as PNG, JPG, and TIFF.

## Installation

You can install SONIRAWWWR by cloning the repository and running the following command in the project directory:

```bash
cargo install --path . #or,
```

## Usage

SONIRAWWWR can be used by specifying the input file using the `-f` or `--filepath` option and the export format using the `-e` or `--export-format` option. Here's an example command for exporting a RAW image to PNG format:

```bash
sonirawwr -f /path/to/input/file.ARW -e png #or, from _this_ directory:
cargo run --release -- -f /path/to/input/file.ARW -e png
```
