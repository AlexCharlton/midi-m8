use std::ops::Range;

use crate::midi_file::*;
use m8_files::*;
use midi_msg::*;

pub const TICKS_PER_QUARTER_NOTE: u32 = 24;

#[derive(Debug)]
pub struct Config {
    pub global_transpose: i16,
    pub max_note_length: [u32; 8],
    pub tracks: Range<usize>,
    pub start_from: u8,
}
impl Config {
    pub fn max_note_len(mut self, len_quarter: f32) -> Self {
        let len = (len_quarter * TICKS_PER_QUARTER_NOTE as f32) as u32;
        self.max_note_length = [len, len, len, len, len, len, len, len];
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            global_transpose: 36,
            max_note_length: [
                std::u32::MAX / 2,
                std::u32::MAX / 2,
                std::u32::MAX / 2,
                std::u32::MAX / 2,
                std::u32::MAX / 2,
                std::u32::MAX / 2,
                std::u32::MAX / 2,
                std::u32::MAX / 2,
            ],
            tracks: 1..9,
            start_from: 0,
        }
    }
}

#[derive(Debug)]
struct TrackCtx {
    /// Ticks elapsed
    ticks: u32,
    transpose: i16,
    global_transpose: i16,
    max_note_length: u32,
    groove: Groove,
    last_note: u8,
    last_note_tick: u32,
    events: Vec<(u32, MidiMsg)>,
}

impl TrackCtx {
    fn groove_ticks(&self, step: usize) -> u32 {
        let steps = &self.groove.active_steps();
        steps[step % steps.len()] as u32
    }

    fn change_groove(&mut self, step: &Step, song: &Song) {
        if step.fx1.command == FXCommand::GRV {
            self.groove = song.grooves[step.fx1.value as usize].clone();
        }
        if step.fx2.command == FXCommand::GRV {
            self.groove = song.grooves[step.fx2.value as usize].clone();
        }
        if step.fx3.command == FXCommand::GRV {
            self.groove = song.grooves[step.fx3.value as usize].clone();
        }
    }

    fn add_note_off(&mut self, at_tick: u32, channel: Channel, note: u8) {
        self.events.push((
            at_tick.min(self.last_note_tick + self.max_note_length),
            MidiMsg::ChannelVoice {
                channel,
                msg: ChannelVoiceMsg::NoteOff { note, velocity: 0 },
            },
        ));
    }

    fn add_note_on(&mut self, at_tick: u32, channel: Channel, note: u8, velocity: u8) {
        let actual_note = (note as i16 + self.transpose + self.global_transpose) as u8;
        self.last_note_tick = at_tick;
        self.last_note = actual_note;
        self.events.push((
            at_tick,
            MidiMsg::ChannelVoice {
                channel,
                msg: ChannelVoiceMsg::NoteOn {
                    note: actual_note,
                    velocity,
                },
            },
        ));
    }
}

pub fn song_to_midi(song: &Song, cfg: &Config) -> Vec<u8> {
    let f = MidiFile {
        format: MidiFileFormat::SimultaniousTracks,
        ticks_per_quarter_note: TICKS_PER_QUARTER_NOTE as u16,
        tracks: song_to_tracks(song, cfg),
    };
    // dbg!(&f);
    f.to_midi()
}

fn song_to_tracks(song: &Song, cfg: &Config) -> Vec<MidiFileTrack> {
    cfg.tracks
        .clone()
        .map(|x| collect_track_events(x - 1, song, cfg))
        .collect()
}

fn collect_track_events(track: usize, song: &Song, cfg: &Config) -> MidiFileTrack {
    let mut ctx = TrackCtx {
        ticks: 0,
        transpose: 0,
        global_transpose: cfg.global_transpose,
        max_note_length: cfg.max_note_length[track],
        groove: song.grooves[0].clone(),
        last_note: 255,
        last_note_tick: 0,
        events: vec![],
    };
    let mut song_step = cfg.start_from as usize;
    while song.song.steps[song_step * 8 + track] < 0xFF {
        let chain_num = song.song.steps[song_step * 8 + track];
        collect_chain_events(chain_num, song, &mut ctx);
        song_step += 1;
    }

    if ctx.last_note != 255 {
        ctx.add_note_off(ctx.ticks, Channel::Ch1, ctx.last_note);
    }

    MidiFileTrack {
        name: Some(format!("Track {}", track + 1)),
        events: ctx.events,
        n_ticks: ctx.ticks.max(TICKS_PER_QUARTER_NOTE * 4),
    }
}

fn collect_chain_events(chain_num: u8, song: &Song, ctx: &mut TrackCtx) {
    let chain = &song.chains[chain_num as usize];
    // dbg!(chain);
    let mut chain_step = 0;
    while chain.steps[chain_step].phrase < 0xFF {
        let cs = &chain.steps[chain_step];
        ctx.transpose = (cs.transpose as i8) as i16;
        collect_phrase_events(cs.phrase, song, ctx);
        chain_step += 1;
    }
}

fn collect_phrase_events(phrase_num: u8, song: &Song, ctx: &mut TrackCtx) {
    let phrase = &song.phrases[phrase_num as usize];
    for i in 0..16 {
        let step = &phrase.steps[i];
        // dbg!(step, ctx.ticks);
        if step.note.0 != 255 {
            if ctx.last_note != 255 {
                ctx.add_note_off(ctx.ticks, Channel::Ch1, ctx.last_note);
            }
            ctx.add_note_on(ctx.ticks, Channel::Ch1, step.note.0, step.velocity);
        }
        ctx.change_groove(step, song);
        ctx.ticks += ctx.groove_ticks(i);
    }
}
