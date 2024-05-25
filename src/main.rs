extern crate flate2;
extern crate zip;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, copy, BufReader, Read, Write};
use std::path::Path;
use std::time::Instant;
use zip::write::FileOptions;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("\nCompresses the source file to the target file: \n");
        eprintln!("Usage:              {} <compression-type> <source> <target>", args[0]);
        println!("Compression types:  gzip, zip \n");

        println!("Ex:                 {} gzip source.txt source.txt.gz", args[0]);
        println!("Ex:                 {} zip source.txt source.zip \n", args[0]);

        println!("gzip                {} this compresses the source file to the target file using the gzip format", args[0]);
        println!("zip                 {} this compresses the source file to the target file using the zip format\n", args[0]);

        std::process::exit(1);
    }

    let compression_type = &args[1];
    let source_path = &args[2];
    let target_path = &args[3];

    if compression_type == "--help" {
        println!("\nCompresses the source file to the target file: \n");
        eprintln!("Usage:              {} <compression-type> <source> <target>", args[0]);
        println!("Compression types:  gzip, zip \n");

        println!("Ex:                 {} gzip source.txt source.txt.gz", args[0]);
        println!("Ex:                 {} zip source.txt source.zip \n", args[0]);

        println!("gzip                {} this compresses the source file to the target file using the gzip format", args[0]);
        println!("zip                 {} this compresses the source file to the target file using the zip format\n", args[0]);

        std::process::exit(0);
    }

    let input_file = File::open(source_path).unwrap_or_else(|err| {
        eprintln!("Failed to open source file {}: {}", source_path, err);
        std::process::exit(1);
    });

    let output_file = File::create(target_path).unwrap_or_else(|err| {
        eprintln!("Failed to create target file {}: {}", target_path, err);
        std::process::exit(1);
    });

    let start = Instant::now();

    match compression_type.as_str() {
        "gzip" => compress_gzip(input_file, output_file)?,
        "zip" => compress_zip(input_file, target_path)?,
        _ => {
            eprintln!("Unsupported compression type: {}", compression_type);
            std::process::exit(1);
        }
    }

    let elapsed = start.elapsed();
    let elapsed_secs = elapsed.as_secs();
    let elapsed_millis = elapsed.subsec_millis();

    if elapsed_secs > 60 {
        println!("Time: {} minutes", elapsed_secs / 60);
    } else {
        println!(
            "Time: {} seconds and {} milliseconds",
            elapsed_secs, elapsed_millis
        );
    }

    Ok(())
}

fn compress_gzip(input: File, output: File) -> io::Result<()> {
    let mut input = BufReader::new(input);
    let mut encoder = GzEncoder::new(output, Compression::default());

    copy(&mut input, &mut encoder).unwrap_or_else(|err| {
        eprintln!("Failed to compress file: {}", err);
        std::process::exit(1);
    });
    let output = encoder.finish().unwrap_or_else(|err| {
        eprintln!("Failed to finalize compression: {}", err);
        std::process::exit(1);
    });
    let source_len = input.get_ref().metadata().unwrap().len();
    let target_len = output.metadata().unwrap().len();

    println!("Source length: {}", format_size(source_len));
    println!("Target length: {}", format_size(target_len));

    Ok(())
}

fn compress_zip(mut input: File, target_path: &str) -> io::Result<()> {
    let mut zip_writer = zip::ZipWriter::new(File::create(target_path)?);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    zip_writer.start_file(
        Path::new(target_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        options,
    )?;

    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;
    zip_writer.write_all(&buffer)?;

    zip_writer.finish()?;

    let source_len = buffer.len() as u64;
    let target_len = File::open(target_path)?.metadata()?.len();

    println!("Source length: {}", format_size(source_len));
    println!("Target length: {}", format_size(target_len));

    Ok(())
}

fn format_size(size: u64) -> String {
    const KILOBYTE: u64 = 1024;
    const MEGABYTE: u64 = KILOBYTE * 1024;
    const GIGABYTE: u64 = MEGABYTE * 1024;

    if size >= GIGABYTE {
        return format!("{:.2} GB", size as f64 / GIGABYTE as f64);
    }
    if size >= MEGABYTE {
        return format!("{:.2} MB", size as f64 / MEGABYTE as f64);
    }
    if size >= KILOBYTE {
        return format!("{:.2} KB", size as f64 / KILOBYTE as f64);
    }
    return format!("{} bytes", size);
}
