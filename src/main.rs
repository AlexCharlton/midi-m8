use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::env;

use m8_files::Song;
use midi_msg::*;
mod midi_file;
use midi_file::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1])?;
    let _song = Song::read(&mut f)?;

    // dbg!(song);

    let quarter_note: u32 = 960;

    let midi_file = MidiFile {
        format: MidiFileFormat::SimultaniousTracks,
        ticks_per_quarter_note: quarter_note as u16,
        tracks: vec![
            MidiFileTrack {
                name: Some("Track 1".to_string()),
                events: vec![
                    (0, MidiMsg::ChannelVoice {
                        channel: Channel::Ch1,
                        msg: ChannelVoiceMsg::NoteOn {note: 72, velocity: 100}
                    }),
                    (quarter_note / 8,
                     MidiMsg::ChannelVoice{
                         channel: Channel::Ch1,
                         msg: ChannelVoiceMsg::NoteOff {note: 72, velocity: 100}
                     })
                ],
                n_ticks: quarter_note * 4,
            },
            MidiFileTrack {
                name: Some("Track 2".to_string()),
                events: vec![
                    (0, MidiMsg::ChannelVoice {
                        channel: Channel::Ch1,
                        msg: ChannelVoiceMsg::NoteOn {note: 72, velocity: 100}
                    }),
                    (quarter_note / 8,
                     MidiMsg::ChannelVoice{
                         channel: Channel::Ch1,
                         msg: ChannelVoiceMsg::NoteOff {note: 72, velocity: 100}
                     })
                ],
                n_ticks: quarter_note * 4,
            },
        ]
    }.to_midi();

    let mut f_out = File::create("out.mid")?;
    f_out.write(&midi_file)?;

    Ok(())
}
