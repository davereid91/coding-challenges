use std::{error::Error, ffi::OsStr, fs, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(name = "something")]
struct Args {
    #[arg(short = 'c', long = "bytes")]
    bytes: bool,

    #[arg(value_name = "FILE")]
    file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let filename = args.file.file_name().and_then(|s| s.to_str()).unwrap_or("");
    let contents = fs::read_to_string(&args.file)?;

    if args.bytes {
        println!("{} {}", contents.len(), filename)
    }

    Ok(())
}
