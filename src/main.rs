use std::error::Error;
use std::fs::File;
use std::io::Write;

use clap::Parser;

use m8_files::Song;
mod midi_file;

mod song_to_midi;
use song_to_midi::*;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Input (.m8s) file
    #[arg()]
    input_file: String,

    /// Output file name
    #[arg(short='t', long, default_value = "tracks.midi")]
    output: String,

    /// How to map M8 note numbers to Midi Note numbers
    #[arg(short, long, default_value_t = 36)]
    global_transpose: i16,

    /// Only output track number (1-8)
    #[arg(long, short='n', id="ONLY_TRACK_N")]
    only_track: Option<usize>,

    /// Cap the maximum note length to this value in quarter notes
    #[arg(short, long)]
    max_note_length: Option<f32>,
    /// Cap the maximum note length for track 1 to this value in quarter notes
    #[arg(long, id="TRACK_1_MAX_NOTE_LEN")]
    track_1_max_note_length: Option<f32>,
    /// Cap the maximum note length for track 2 to this value in quarter notes
    #[arg(long, id="TRACK_2_MAX_NOTE_LEN")]
    track_2_max_note_length: Option<f32>,
    /// Cap the maximum note length for track 3 to this value in quarter notes
    #[arg(long, id="TRACK_3_MAX_NOTE_LEN")]
    track_3_max_note_length: Option<f32>,
    /// Cap the maximum note length for track 4 to this value in quarter notes
    #[arg(long, id="TRACK_4_MAX_NOTE_LEN")]
    track_4_max_note_length: Option<f32>,
    /// Cap the maximum note length for track 5 to this value in quarter notes
    #[arg(long, id="TRACK_5_MAX_NOTE_LEN")]
    track_5_max_note_length: Option<f32>,
    /// Cap the maximum note length for track 6 to this value in quarter notes
    #[arg(long, id="TRACK_6_MAX_NOTE_LEN")]
    track_6_max_note_length: Option<f32>,
    /// Cap the maximum note length for track 7 to this value in quarter notes
    #[arg(long, id="TRACK_7_MAX_NOTE_LEN")]
    track_7_max_note_length: Option<f32>,
    /// Cap the maximum note length for track 8 to this value in quarter notes
    #[arg(long, id="TRACK_8_MAX_NOTE_LEN")]
    track_8_max_note_length: Option<f32>,
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut config = Config{global_transpose: args.global_transpose,
                            ..Config::default()};

    if let Some(track) = args.only_track {
        if track > 0 && track < 9 {
            config.tracks = track..(track+1);
        } else {
            println!("Warning: selected invalid track number {}. Defaulting to all tracks", track);
        }
    }

    let max_note_lengths: [Option<f32>; 8] = [
        args.track_1_max_note_length,
        args.track_2_max_note_length,
        args.track_3_max_note_length,
        args.track_4_max_note_length,
        args.track_5_max_note_length,
        args.track_6_max_note_length,
        args.track_7_max_note_length,
        args.track_8_max_note_length,
    ];

    for i in 0..8 {
        if let Some(l) = max_note_lengths[i].or(args.max_note_length) {
            config.max_note_length[i] = (l * TICKS_PER_QUARTER_NOTE as f32) as u32;
        }
    }

    let mut f = File::open(args.input_file)?;
    let song = Song::read(&mut f)?;
    // dbg!(song);

    let mut f_out = File::create(args.output)?;
    f_out.write(&song_to_midi(&song, &config))?;

    Ok(())
}
