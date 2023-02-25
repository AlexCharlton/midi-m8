use byteorder::{ByteOrder, BigEndian};
use midi_msg::*;


// Midi file handling
#[inline]
pub fn u32_to_bytes(x: u32) -> [u8; 4] {
    let mut buf = [0; 4];
    BigEndian::write_u32(&mut buf, x);
    buf
}

#[inline]
pub fn push_u32(x: u32, v: &mut Vec<u8>) {
    let [b1, b2, b3, b4] = u32_to_bytes(x);
    v.push(b1);
    v.push(b2);
    v.push(b3);
    v.push(b4);
}

#[inline]
pub fn u16_to_bytes(x: u16) -> [u8; 2] {
    let mut buf = [0; 2];
    BigEndian::write_u16(&mut buf, x);
    buf
}

#[inline]
pub fn push_u16(x: u16, v: &mut Vec<u8>) {
    let [b1, b2] = u16_to_bytes(x);
    v.push(b1);
    v.push(b2);
}


pub fn push_vari(x: u32, v: &mut Vec<u8>) {
    if x <        0x00000080 {
        v.push(x as u8 & 0b01111111);
    } else if x < 0x00004000 {
        v.push(((x >> 7) as u8 & 0b01111111) + 0b10000000);
        v.push(x as u8 & 0b01111111);
    } else if x < 0x00200000 {
        v.push(((x >> 14) as u8 & 0b01111111) + 0b10000000);
        v.push(((x >> 7) as u8 & 0b01111111) + 0b10000000);
        v.push(x as u8 & 0b01111111);
    } else if x <= 0x0FFFFFFF {
        v.push(((x >> 21) as u8 & 0b01111111) + 0b10000000);
        v.push(((x >> 14) as u8 & 0b01111111) + 0b10000000);
        v.push(((x >> 7) as u8 & 0b01111111) + 0b10000000);
        v.push(x as u8 & 0b01111111);
    } else {
        panic!("Cannot use such a large number as a variable quantity")
    }
}

#[derive(Debug)]
#[repr(u16)]
#[allow(dead_code)]
pub enum MidiFileFormat {
    SingleTrack = 0,
    SimultaniousTracks = 1,
    IndependantTracks = 2,
}

#[derive(Debug)]
pub struct MidiFile {
    pub ticks_per_quarter_note: u16,
    // TODO support subdivision-of-second delta-times
    pub format: MidiFileFormat,
    pub tracks: Vec<MidiFileTrack>
}
impl MidiFile {
    pub fn to_midi(&self) -> Vec<u8> {
        let mut r: Vec<u8> = vec![];
        self.extend_midi(&mut r);
        r
    }

    pub fn extend_midi(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(b"MThd");
        push_u32(6, v); // Length of header, always 6 bytes
        push_u16(self.format as u16, v);
        push_u16(self.tracks.len() as u16, v); // num tracks
        if self.ticks_per_quarter_note > 0x7FFF {
            panic!("Ticks per quarter note must be less than {}", 0x7FFF);
        }
        push_u16(self.ticks_per_quarter_note, v);
        for track in self.tracks.iter() {
            track.extend_midi(v);
        }
    }
}

#[derive(Debug)]
pub struct MidiFileTrack {
    /// A vector of tick/Midi event tuples.
    /// Unlike in the Midi file representation, the ticks in the tuple represent
    /// absolute time since the start of the track. Events must be in order.
    pub events: Vec<(u32, MidiMsg)>,
    /// Name of the track
    pub name: Option<String>,
    /// Length of the track in ticks
    pub n_ticks: u32,
}
#[allow(dead_code)]
impl MidiFileTrack {
    pub fn extend_midi(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(b"MTrk");
        let mut events: Vec<u8> = vec![];

        if let Some(name) = &self.name {
            let len = name.len().min(127);
            events.extend_from_slice(&[0x00, 0xFF, 0x03, len as u8]);
            events.extend_from_slice(&name.as_bytes()[..len]);
        }

        let mut last_tick = 0;
        for (ticks, msg) in self.events.iter() {
            push_vari(*ticks - last_tick, &mut events);
            msg.extend_midi(&mut events);
            last_tick = *ticks;
        }
        push_vari(self.n_ticks - last_tick + 1, &mut events);
        events.extend_from_slice(&[0xFF, 0x2F, 0x00]);
        push_u32(events.len() as u32, v);
        v.extend_from_slice(&events);
    }

    /// Sort events by tick time
    pub fn sort_events(&mut self) -> () {
        self.events.sort_by(|a, b| a.0.cmp(&b.0));
    }
}


#[cfg(test)]
mod tests {
    use crate::midi_file::*;

    #[test]
    fn test_u32_to_bytes() {
        let mut v: Vec<u8> = vec![];
        push_u32(6, &mut v);
        assert_eq!(v, vec![0, 0, 0, 6]);
    }

    fn validate_vari(n: u32, bytes: Vec<u8>) {
        let mut v: Vec<u8> = vec![];
        push_vari(n, &mut v);
        assert_eq!(v, bytes, "{:#02X} should have been {:#02X?} not {:#02X?}", n, &bytes, &v);
    }

    #[test]
    fn test_push_vari() {
        validate_vari(6, vec![6]);
        validate_vari(0x7F, vec![0x7F]);
        validate_vari(0x80, vec![0x81, 0x00]);
        validate_vari(0xE89, vec![0x9D, 0x09]);
        validate_vari(0x3C0, vec![0x87, 0x40]);
        validate_vari(0x2000, vec![0xC0, 0x00]);
        validate_vari(0x3FFF, vec![0xFF, 0x7F]);
        validate_vari(0x4000, vec![0x81, 0x80, 0x00]);
        validate_vari(0x1FFFFF, vec![0xFF, 0xFF, 0x7F]);
        validate_vari(0x0FFFFFFF, vec![0xFF, 0xFF, 0xFF, 0x7F]);

    }

    #[test]
    fn test_midi_file() {
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

        assert_eq!(&midi_file, &[0x4d, 0x54, 0x68, 0x64, 0x0, 0x0, 0x0, 0x6, 0x0, 0x1, 0x0, 0x2, 0x3, 0xc0, 0x4d, 0x54, 0x72, 0x6b, 0x0, 0x0, 0x0, 0x18, 0x0, 0xff, 0x3, 0x7, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x20, 0x31, 0x0, 0x90, 0x48, 0x64, 0x78, 0x80, 0x48, 0x64, 0x9d, 0x9, 0xff, 0x2f, 0x0, 0x4d, 0x54, 0x72, 0x6b, 0x0, 0x0, 0x0, 0x18, 0x0, 0xff, 0x3, 0x7, 0x54, 0x72, 0x61, 0x63, 0x6b, 0x20, 0x32, 0x0, 0x90, 0x48, 0x64, 0x78, 0x80, 0x48, 0x64, 0x9d, 0x9, 0xff, 0x2f, 0x0]);
    }
}
