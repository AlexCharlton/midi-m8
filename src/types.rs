use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use arr_macro::arr;
use byteorder::{ByteOrder, LittleEndian};

#[derive(PartialEq, Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", &self.0)
    }
}

impl std::error::Error for ParseError {}

type Result<T> = std::result::Result<T, ParseError>;

pub struct Reader {
    buffer: Vec<u8>,
    position: Rc<RefCell<usize>>
}

impl Reader {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            buffer, position: Rc::new(RefCell::new(0))
        }
    }

    pub fn read(&self) -> u8 {
        let p: usize = *self.position.borrow();
        let b = self.buffer[p];
        *self.position.borrow_mut() += 1;
        b
    }

    pub fn read_bytes(&self, n: usize) -> &[u8] {
        let p: usize = *self.position.borrow();
        let bs = &self.buffer[p..p+n];
        *self.position.borrow_mut() += n;
        bs
    }

    pub fn pos(&self) -> usize {
        *self.position.borrow()
    }
}

#[derive(PartialEq)]
pub struct Song {
    pub version: Version,
    pub directory: [u8;128],
    pub transpose: i8,
    pub tempo: f32,
    pub quantize: u8,
    pub name: [u8;12],
    pub midi_settings: MidiSettings,
    pub key: u8,
    pub mixer_settings: MixerSettings,
    pub grooves: [Groove;32],
    pub song: SongSteps,
}

impl fmt::Debug for Song {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Song")
            .field("version", &self.version)
            .field("directory", &self.directory_to_str())
            .field("transpose", &self.transpose)
            .field("tempo", &self.tempo)
            .field("quantize", &self.quantize)
            .field("name", &self.name_to_str())
            .field("key", &self.key)
            //.field("grooves", &self.grooves)
            .field("song", &self.song)
            .finish()
    }
}

impl Song {
    pub fn from_reader(reader: &Reader) -> Result<Self> {
        let version = Version::from_reader(reader)?;
        reader.read_bytes(2);
        let directory = reader.read_bytes(128);
        let transpose = reader.read() as i8;
        let tempo = LittleEndian::read_f32(reader.read_bytes(4));
        let quantize = reader.read();
        let name = reader.read_bytes(12);
        let midi_settings = MidiSettings::from_reader(reader)?;
        let key = reader.read();
        reader.read_bytes(18);
        let mixer_settings = MixerSettings::from_reader(reader)?;
        // println!("{:x}", reader.pos());
        let mut i = 0;
        let grooves: [Groove; 32] = arr![Groove::from_reader(reader, {i += 1; i - 1})?; 32];
        let song = SongSteps::from_reader(reader)?;

        Ok(Self{
            version,
            directory: directory.try_into().unwrap(),
            transpose,
            tempo,
            quantize,
            name: name.try_into().unwrap(),
            midi_settings,
            key,
            mixer_settings,
            grooves,
            song,
        })
    }

    pub fn directory_to_str(&self) -> &str {
        let end = self.directory.iter().position(|&x| x == 0).expect("expected end of directory name");
        std::str::from_utf8(&self.directory[0..end]).expect("invalid utf-8 sequence in directory")
    }

    pub fn name_to_str(&self) -> &str {
        let end = self.name.iter().position(|&x| x == 0).expect("expected end of song name");
        std::str::from_utf8(&self.name[0..end]).expect("invalid utf-8 sequence in song name")
    }

}

#[derive(PartialEq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn from_reader(reader: &Reader) -> Result<Self> {
        let _version_string = reader.read_bytes(10);
        let lsb = reader.read();
        let msb = reader.read();
        let major = msb & 0x0F;
        let minor = (lsb >> 4) & 0x0F;
        let patch = lsb & 0x0F;
        Ok(Self {
            major, minor, patch
        })
    }
}


#[derive(PartialEq, Debug)]
pub struct MidiSettings {
    bytes: [u8; 27] // TODO
}
impl MidiSettings {
    pub fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            bytes: reader.read_bytes(27).try_into().unwrap()
        })
    }
}


#[derive(PartialEq, Debug)]
pub struct MixerSettings {
    bytes: [u8; 32] // TODO
}
impl MixerSettings {
    pub fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            bytes: reader.read_bytes(32).try_into().unwrap()
        })
    }
}

#[derive(PartialEq)]
pub struct Groove {
    number: u8,
    steps: [u8; 16]
}
impl Groove {
    pub fn from_reader(reader: &Reader, number: u8) -> Result<Self> {
        Ok(Self {
            number,
            steps: reader.read_bytes(16).try_into().unwrap()
        })
    }

    pub fn active_steps(&self) -> &[u8] {
        let end = (&self.steps).iter().position(|&x| x == 255).unwrap_or(15);
        &self.steps[0..end]
    }
}

impl fmt::Debug for Groove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Groove {}:{:?}", self.number, self.active_steps())
    }
}



#[derive(PartialEq)]
pub struct SongSteps {
    steps: [u8; 2048]
}
impl SongSteps {
    pub fn print_screen(&self) -> String {self.print_screen_from(0) }

    pub fn print_screen_from(&self, start: u8) -> String {
        (start..start+16).fold("   1  2  3  4  5  6  7  8  \n".to_string(),
                              |s, row| s + &self.print_row(row) + "\n"
        )
    }

    pub fn print_row(&self, row: u8) -> String {
        let start = row as usize * 8;
        (start..start+8).fold(format!("{row:02x} "),
                              |s, b| -> String {
                                  let v = self.steps[b];
                                  let repr = if v == 255 { format!("-- ") }
                                  else { format!("{:02x} ", v) };
                                  s + &repr
                              }
        )
    }

    pub fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            steps: reader.read_bytes(2048).try_into().unwrap()
        })
    }
}

impl fmt::Debug for SongSteps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Song\n\n{}", self.print_screen())
    }
}
