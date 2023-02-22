use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::env;

pub mod types;
use crate::types::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1])?;
    let mut buf: Vec<u8> = vec!();
    f.read_to_end(&mut buf)?;

    dbg!(&buf[..13]);

    let reader = Reader::new(buf);
    let song = Song::from_reader(&reader)?;

    dbg!(song);

    Ok(())
}
