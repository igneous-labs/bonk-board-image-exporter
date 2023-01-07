use clap::Parser;
use image::io::Reader as ImageReader;
use log::info;
use std::{fs::File, io::Write, path::PathBuf};

mod types;

use types::*;

const COLOR_SIZE: usize = 3; // RGB 24bit (3 * 8 bit)

#[derive(Parser, Debug)]
#[clap(version)]
struct BonkboardImageExporterArgs {
    /// Path to image file convert to bonk board user data
    #[arg(short, long)]
    input_file: PathBuf,

    /// Path to json output file
    #[arg(short, long)]
    output_file: PathBuf,

    /// Maximum number of pixels to pack into each transaction
    #[arg(short, long, default_value_t = 100)]
    pixels_per_tx: usize,

    /// Coordinate of the top left corner to place the image at, in the format of "x,y"
    #[arg(short, long, default_value = "0,0")]
    top_left_coord: String,

    /// RGB 24bit value to be used to exclude pixels from the image, in the format of "r,g,b"
    #[arg(short, long, default_value = "0,0,0")]
    color_to_ignore: String,
}

fn main() {
    flexi_logger::Logger::try_with_env_or_str("info")
        .unwrap()
        .start()
        .unwrap();

    let args = BonkboardImageExporterArgs::parse();
    let mut output = File::create(&args.output_file).expect("Failed to open output file");
    info!("Reading image file: {}", args.input_file.to_str().unwrap());
    let img = ImageReader::open(&args.input_file)
        .expect("Failed to open image file")
        .decode()
        .expect("Failed to decode image file");
    let img_width = img.width() as usize;
    let top_left_coord: Coord = args
        .top_left_coord
        .try_into()
        .expect("Failed to parse given top left coordinate");
    let color_to_ignore: Color = args
        .color_to_ignore
        .try_into()
        .expect("Failed to parse given color");

    let pixels = img.as_bytes().chunks(COLOR_SIZE);
    info!(
        "Read total {} pixels (image: {}x{})",
        pixels.len(),
        img_width,
        img.height()
    );

    let pixels: Vec<_> = pixels
        .enumerate()
        .filter_map(|(i, c)| {
            let color: Color = c.try_into().expect("Wrong size color information was given, make sure the image file is parsable to RGB 24bit color format");
            if color == color_to_ignore {
                None
            } else {
                let coord = Coord {
                    x: (i % img_width) as u16,
                    y: (i / img_width) as u16,
                } + top_left_coord;
                Some(Pixel {
                    coord,
                    color,
                })
            }
        })
        .collect();
    info!(
        "Excluding the given color {:?}, prepared {} pixels to write",
        color_to_ignore,
        pixels.len()
    );

    let pixels = &pixels
        .chunks(args.pixels_per_tx)
        .map(|pixels_chunk| pixels_chunk.iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    info!(
        "With the given maxium {} pixels per tx, writing an array of length {} to file",
        args.pixels_per_tx,
        pixels.len()
    );

    let json = serde_json::to_string_pretty(pixels).expect("Failed to parse json");
    output
        .write_all(json.as_bytes())
        .expect("Failed to write to output file");
    info!("Done");
}
