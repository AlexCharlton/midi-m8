use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::env;

use m8_files::Song;
mod midi_file;

mod song_to_midi;
use song_to_midi::*;

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

    // dbg!(song);

    let mut f_out = File::create("out.mid")?;
    f_out.write(&song_to_midi(&song, &Config::default().max_note_len(4.0)))?;

    Ok(())
}
