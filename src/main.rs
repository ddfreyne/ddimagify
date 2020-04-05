extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::ImageError;
use std::fs;
use std::io::prelude::*;

fn write(input_filename: &str, output_filename: &str) -> Result<(), ImageError> {
    let mut t = Vec::new();

    if input_filename == "-" {
        std::io::stdin().read_to_end(&mut t)?;
    } else {
        std::fs::File::open(input_filename)?.read_to_end(&mut t)?;
    };

    let pixels_count = (t.len() + 3) / 4;
    let extra = t.len() % 4;

    let mut buf = image::ImageBuffer::new(pixels_count as u32, 1);

    for (idx, _b) in t.iter().enumerate().step_by(4) {
        let x = idx / 4;
        let y = 0;

        let pixel = buf.get_pixel_mut(x as u32, y as u32);

        let diff = pixels_count * 4 - extra - idx;
        match diff {
            1 => *pixel = image::Rgba([t[idx], 0, 0, 0]),
            2 => *pixel = image::Rgba([t[idx], t[idx + 1], 0, 0]),
            3 => *pixel = image::Rgba([t[idx], t[idx + 1], t[idx + 2], 0]),
            _ => *pixel = image::Rgba([t[idx], t[idx + 1], t[idx + 2], t[idx + 3]]),
        }
    }

    buf.save(output_filename)?;

    Ok(())
}

fn read(input_filename: &str, output_filename: &str) -> Result<(), ImageError> {
    let mut buffer: Box<dyn Write> = if output_filename == "-" {
        Box::new(std::io::stdout())
    } else {
        Box::new(fs::File::create(output_filename)?)
    };

    let img = image::open(input_filename)?.to_rgba();

    for (_x, _y, pixel) in img.enumerate_pixels() {
        buffer.write_all(&pixel.0)?;
    }

    Ok(())
}

fn main() {
    let matches = App::new("ddnaughty")
        .version("0.1")
        .about("Pretend everything is an image")
        .author("Denis D.")
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .value_name("MODE")
                .help("Specify mode (read or write)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Read from this file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Write to this file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let mode = matches.value_of("mode").unwrap();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    match mode {
        "read" => read(input, output).unwrap(),
        "write" => write(input, output).unwrap(),
        _ => {
            println!("Error: Unknown mode (expected `read` or `write`)");
        }
    };
}
