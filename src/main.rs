use clap::Parser;
use image::codecs::tiff::TiffEncoder;
use image::{ColorType, ImageEncoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    /// Export extension
    #[arg(short, long)]
    output: String,
}

fn main() {
    let cli = Cli::parse();

    // Convert the RAW image to a decoded image using imagepipe
    let decoded = match imagepipe::simple_decode_8bit(cli.input, 0, 0) {
        Ok(img) => img,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Open a file for writing the PNG image
    let output_file = match File::create(cli.output) {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Create a buffered writer for the output file
    let mut output_writer = BufWriter::new(output_file);

    // match image::codecs::png::PngEncoder::new(&mut output_writer).write_image(
    match TiffEncoder::new(&mut output_writer).write_image(
        &decoded.data,
        decoded.width as u32,
        decoded.height as u32,
        ColorType::Rgb8,
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
