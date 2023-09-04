use std::io;
use std::io::Error;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::fs::write;

mod args;
pub use args::Args;

mod png;
pub use png::Png;

pub fn run(args: Args) -> Result<(), Error> {

    let in_file = File::open(args.in_file_path)?;
    let mut reader = BufReader::new(in_file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let png = Png::build(&buffer).unwrap();
    println!("{} chunks", png.chunks.len());
    for chunk in png.chunks {
        println!("{}", chunk.chunk_type);
    }

    //write(args.out_file_path, buffer)?;

    Ok(())
}