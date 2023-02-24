use std::error::Error;
use std::fs::File;
use std::env;

use byteorder::{ByteOrder, BigEndian};
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

    // dbg!(song);

    Ok(())
}

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

pub fn push_vari(x: u32, v: &mut Vec<u8>) {
    if        x < 0x00000080 {
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

struct MidiFileHeader {
}
impl MidiFileHeader {
    pub fn extend_midi(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(b"MThd");
        push_u32(6, v);
    }
}

struct MidiFileTrack {
}
impl MidiFileTrack {
    pub fn extend_midi(&self, v: &mut Vec<u8>) {
        v.extend_from_slice(b"MTrk");
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_u32_to_bytes() {
        let mut v: Vec<u8> = vec![];
        push_u32(6, &mut v);
        assert_eq!(v, vec![0, 0, 0, 6]);
    }

    fn validate_vari(n: u32, bytes: Vec<u8>) {
        let mut v: Vec<u8> = vec![];
        push_vari(n, &mut v);
        assert_eq!(v, bytes, "{:x} should have been {:?} not {:?}", n, &bytes, &v);
    }

    #[test]
    fn test_push_vari() {
        validate_vari(6, vec![6]);
        validate_vari(0x7F, vec![0x7F]);
        validate_vari(0x80, vec![0x81, 0x00]);
        validate_vari(0x2000, vec![0xC0, 0x00]);
        validate_vari(0x3FFF, vec![0xFF, 0x7F]);
        validate_vari(0x4000, vec![0x81, 0x80, 0x00]);
        validate_vari(0x1FFFFF, vec![0xFF, 0xFF, 0x7F]);
        validate_vari(0x0FFFFFFF, vec![0xFF, 0xFF, 0xFF, 0x7F]);

    }
}
