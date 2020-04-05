extern crate clap;
extern crate image;

use clap::{App, Arg};
use std::fs;

fn write(s: &str, filename: &str) {
    let chars_count = s.chars().count();

    let mut buf = image::ImageBuffer::new(chars_count as u32, 1);

    for (idx, ch) in s.chars().enumerate() {
        let x = idx;
        let y = 0;

        let pixel = buf.get_pixel_mut(x as u32, y as u32);
        *pixel = image::Rgba((ch as u32).to_be_bytes());
    }

    buf.save(filename).unwrap();
}

fn read(filename: &str) {
    let img = image::open(filename).unwrap().to_rgba();

    for (_x, _y, pixel) in img.enumerate_pixels() {
        let ch = std::char::from_u32(u32::from_be_bytes(pixel.0));
        print!("{}", ch.unwrap());
    }
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
                .takes_value(true),
        )
        .get_matches();

    let mode = matches.value_of("mode").unwrap();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output");

    match mode {
        "read" => read(input),
        "write" => {
            let s = fs::read_to_string(input).unwrap();

            match output {
                Some(f) => write(s.as_str(), f),
                None => println!("Error: Mode set to `write`, but no output file specified"),
            }
        }
        _ => {
            println!("Error: Unknown mode (expected `read` or `write`)");
        }
    };
}
