use std::error::Error;
use std::fs::File;
use std::env;

use m8_files::Song;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1])?;
    let song = Song::read(&mut f)?;

    dbg!(song);

    Ok(())
}
