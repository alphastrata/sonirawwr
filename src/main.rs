use anyhow::Result;
use clap::Parser;
use image::codecs::tiff::TiffEncoder;
use image::{ColorType, ImageEncoder};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// path of file to import and decode
    #[arg(short, long)]
    input: PathBuf,

    /// path to export re-encoded file to
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<()> {
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
    let output_file = match File::create(&cli.output) {
        Ok(val) => val,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let mut output_writer = BufWriter::new(output_file);

    if cli.output.contains("tif") {
        TiffEncoder::new(&mut output_writer).write_image(
            &decoded.data,
            decoded.width as u32,
            decoded.height as u32,
            ColorType::Rgb8,
        )?;
    } else if cli.output.contains("png") {
        image::codecs::png::PngEncoder::new(&mut output_writer).write_image(
            &decoded.data,
            decoded.width as u32,
            decoded.height as u32,
            ColorType::Rgb8,
        )?;
    } else if cli.output.contains("jpeg") || cli.output.contains("jpg") {
        image::codecs::jpeg::JpegEncoder::new(&mut output_writer).write_image(
            &decoded.data,
            decoded.width as u32,
            decoded.height as u32,
            ColorType::Rgb8,
        )?;
    } else {
        panic!()
    }

    Ok(())
}
