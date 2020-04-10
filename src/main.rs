use clap::{App, Arg};
use image::ImageError;
use std::fs;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

fn divide_rounding_up(dividend: usize, divisor: usize) -> usize {
    (dividend + (divisor - 1)) / divisor
}

fn wrap<T: AsRef<Path>>(input_filename: &str, output_filename: T) -> Result<(), ImageError> {
    let mut t = Vec::new();

    if input_filename == "-" {
        std::io::stdin().read_to_end(&mut t)?;
    } else {
        std::fs::File::open(input_filename)?.read_to_end(&mut t)?;
    };

    // +4 to add length
    // +3 /4 to round up
    let pixels_count = 1 + divide_rounding_up(t.len(), 4);
    let extra = t.len() % 4;

    let width = (pixels_count as f64).sqrt() as usize;
    let height = divide_rounding_up(pixels_count, width);
    let mut buf = image::ImageBuffer::new(width as u32, height as u32);

    // Write length
    let pixel = buf.get_pixel_mut(0, 0);
    *pixel = image::Rgba((t.len() as u32).to_be_bytes());

    // Write data
    for (idx, _b) in t.iter().enumerate().step_by(4) {
        let x = (idx / 4 + 1) % width;
        let y = (idx / 4 + 1) / width;

        let diff = pixels_count * 4 - extra - (idx + 4);

        let pixel = buf.get_pixel_mut(x as u32, y as u32);

        match diff {
            3 => *pixel = image::Rgba([t[idx], 0, 0, 0]),
            2 => *pixel = image::Rgba([t[idx], t[idx + 1], 0, 0]),
            1 => *pixel = image::Rgba([t[idx], t[idx + 1], t[idx + 2], 0]),
            _ => *pixel = image::Rgba([t[idx], t[idx + 1], t[idx + 2], t[idx + 3]]),
        }
    }

    buf.save(output_filename)?;

    Ok(())
}

fn unwrap<T: AsRef<Path>>(input_filename: &str, output_filename: T) -> Result<(), ImageError> {
    let raw_writer: Box<dyn Write> = if output_filename.as_ref() == Path::new("-") {
        Box::new(std::io::stdout())
    } else {
        Box::new(fs::File::create(output_filename)?)
    };
    let mut writer = BufWriter::new(raw_writer);

    let img = image::open(input_filename)?.to_rgba();

    // Read length
    let pixel = img.get_pixel(0, 0);
    let length = u32::from_be_bytes(pixel.0);

    // Read data
    let mut written = 0;
    for (_x, _y, pixel) in img.enumerate_pixels().skip(1) {
        if length > written {
            let diff = (length - written) as usize;

            if diff >= 4 {
                writer.write_all(&pixel.0)?;
            } else {
                let data = &pixel.0[0..diff];
                writer.write_all(data)?;
            }
            written += 4;
        }
    }

    writer.flush()?;

    Ok(())
}

fn main() {
    let matches = App::new("png-smuggler")
        .version("0.1")
        .about("Smuggle things as PNGs")
        .author("Denis D.")
        .arg(
            Arg::with_name("MODE")
                .index(1)
                .help("Specify mode (wrap, unwrap)")
                .required(true),
        )
        .arg(
            Arg::with_name("FROM")
                .help("Read from this file")
                .index(2)
                .required(true),
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

    let mode = matches.value_of("MODE").unwrap();
    let from = matches.value_of("FROM").unwrap();
    let output = matches
        .value_of("output")
        .map(|o| o.to_owned())
        .unwrap_or_else(|| {
            let stem: Option<&str> = Path::new(from).file_stem().and_then(|o| o.to_str());
            let new: String = stem.unwrap_or(from).to_owned() + ".png";
            new
        });

    if Path::new(&output).exists() {
        println!("Error: Output file already exists ({})", output);
        std::process::exit(1);
    }

    println!("mode: {}", mode);
    println!("{} ~~> {}", from, output);

    match mode {
        "wrap" => wrap(from, output).unwrap(),
        "unwrap" => unwrap(from, output).unwrap(),
        _ => {
            println!("Error: Unknown mode (expected `wrap` or `unwrap`)");
            std::process::exit(1);
        }
    };
}
