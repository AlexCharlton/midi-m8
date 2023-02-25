use m8_files::*;
use midi_msg::*;
use crate::midi_file::*;

pub const TICKS_PER_QUARTER_NOTE: u32 = 24;

pub fn song_to_midi(song: &Song) -> Vec<u8> {
    let f = MidiFile {
        format: MidiFileFormat::SimultaniousTracks,
        ticks_per_quarter_note: TICKS_PER_QUARTER_NOTE as u16,
        tracks: song_to_tracks(song)
    };
    // dbg!(&f);
    f.to_midi()
}

fn song_to_tracks(song: &Song) -> Vec<MidiFileTrack> {
    (0..8).map(|x| collect_track_events(x, song)).collect()
}

#[derive(Debug)]
struct TrackCtx {
    /// Ticks elapsed
    ticks: u32,
    transpose: i16,
    global_transpose: i16,
    groove: Groove,
    last_note: u8,
    events: Vec<(u32, MidiMsg)>,
}

impl TrackCtx {
    fn groove_ticks(&self, step: usize) -> u32 {
        let steps = &self.groove.active_steps();
        steps[step % steps.len()] as u32
    }
}

fn collect_track_events(track: usize, song: &Song) -> MidiFileTrack {
    let mut ctx = TrackCtx {
        ticks: 0,
        transpose: 0,
        global_transpose: 36,
        groove: song.grooves[0].clone(),
        last_note: 255,
        events: vec![],
    };
    let mut song_step = 0;
    while song.song.steps[song_step * 8 + track] < 0xFF {
        let chain_num = song.song.steps[song_step * 8 + track];
        collect_chain_events(chain_num, song, &mut ctx);
        song_step += 1;
    }

    if ctx.last_note != 255 {
        ctx.events.push((
            ctx.ticks,
            MidiMsg::ChannelVoice{
                channel: Channel::Ch1,
                msg: ChannelVoiceMsg::NoteOff {note: ctx.last_note, velocity: 0}
            }));
    }

    MidiFileTrack {
        name: Some(format!("Track {}", track+1)),
        events: ctx.events,
        n_ticks: ctx.ticks.max(TICKS_PER_QUARTER_NOTE * 4),
    }
}

fn collect_chain_events(chain_num: u8, song: &Song, ctx: &mut TrackCtx) -> () {
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

fn collect_phrase_events(phrase_num: u8, song: &Song, ctx: &mut TrackCtx) -> () {
    let phrase = &song.phrases[phrase_num as usize];
    for i in 0..16 {
        let step = &phrase.steps[i];
        // dbg!(step, ctx.ticks);
        if step.note.0 != 255 {
            if ctx.last_note != 255 {
                ctx.events.push((
                    ctx.ticks,
                    MidiMsg::ChannelVoice{
                        channel: Channel::Ch1,
                        msg: ChannelVoiceMsg::NoteOff {note: ctx.last_note, velocity: 0}
                    }));
            }
            ctx.last_note = (step.note.0 as i16 + ctx.transpose + ctx.global_transpose) as u8;
            ctx.events.push((
                ctx.ticks,
                MidiMsg::ChannelVoice {
                    channel: Channel::Ch1,
                    msg: ChannelVoiceMsg::NoteOn {note: ctx.last_note, velocity: step.velocity}
                }));
        }
        ctx.ticks += ctx.groove_ticks(i);
    }
}
