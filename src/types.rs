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
    pub phrases: [Phrase;256]
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
            .field("phrases", &self.phrases[16]) // TODO
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
        let mut i:u16 = 0;
        let grooves: [Groove; 32] = arr![Groove::from_reader(reader, {i += 1; (i - 1) as u8})?; 32];
        let song = SongSteps::from_reader(reader)?;
        i = 0;
        let phrases: [Phrase; 256] = arr![Phrase::from_reader(reader, {i += 1; (i - 1) as u8})?; 256];

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
            phrases,
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

#[derive(PartialEq)]
pub struct Phrase {
    number: u8,
    steps: [Step; 16]
}
impl Phrase {
    pub fn print_screen(&self) -> String {
        (0..16).fold("   N   V  I  FX1   FX2   FX3  \n".to_string(),
                     |s, row| s + &self.steps[row].print(row as u8) + "\n"
        )
    }

    pub fn from_reader(reader: &Reader, number: u8) -> Result<Self> {
        Ok(Self {
            number,
            steps: arr![Step::from_reader(reader)?; 16],
        })
    }
}

impl fmt::Debug for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Phrase {:02x}\n\n{}", self.number, self.print_screen())
    }
}

#[derive(PartialEq, Debug)]
pub struct Step {
    note: Note,
    velocity: u8,
    instrument: u8,
    fx1: FX,
    fx2: FX,
    fx3: FX,
}
impl Step {
    pub fn print(&self, row: u8) -> String {
        let velocity = if self.velocity == 255 { format!("--") }
        else { format!("{:02x}", self.velocity)};
        let instrument = if self.instrument == 255 { format!("--") }
        else { format!("{:02x}", self.instrument)};
        format!("{:02x} {} {} {} {} {} {}", row, self.note, velocity, instrument,
                self.fx1, self.fx2, self.fx3)
    }

    pub fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            note: Note(reader.read()),
            velocity: reader.read(),
            instrument: reader.read(),
            fx1: FX::from_reader(reader)?,
            fx2: FX::from_reader(reader)?,
            fx3: FX::from_reader(reader)?,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Note(u8);

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 255 {
            write!(f, "---")
        } else {
            let oct = (self.0 / 12) + 1;
            let n = match self.0 % 12 {
                0 => "C-",
                1 => "C#",
                2 => "D-",
                3 => "D#",
                4 => "E-",
                5 => "F-",
                6 => "F#",
                7 => "G-",
                8 => "G#",
                9 => "A-",
                10 => "A#",
                11 => "B-",
                _ => panic!()
            };
            write!(f, "{}{:X}", n, oct)
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct FX {
    command: FXCommand,
    value: u8
}
impl FX {
    pub fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            command: unsafe { std::mem::transmute(reader.read())},
            value: reader.read(),
        })
    }
}

impl fmt::Display for FX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.command == FXCommand::NONE {
            write!(f, "---00")
        } else {
            write!(f, "{:?}{:02x}", self.command, self.value)
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum FXCommand {
    ARP =  0x00,
    CHA =  0x01,
    DEL =  0x02,
    GRV =  0x03,
    HOP =  0x04,
    KIL =  0x05,
    RAN =  0x06,
    RET =  0x07,
    REP =  0x08,
    NTH =  0x09,
    PSL =  0x0A,
    PSN =  0x0B,
    PVB =  0x0C,
    PVX =  0x0D,
    SCA =  0x0E,
    SCG =  0x0F,
    SED =  0x10,
    SNG =  0x11,
    TBL =  0x12,
    THO =  0x13,
    TIC =  0x14,
    TPO =  0x15,
    TSP =  0x16,
    VMV =  0x17,
    XCM =  0x18,
    XCF =  0x19,
    XCW =  0x1A,
    XCR =  0x1B,
    XDT =  0x1C,
    XDF =  0x1D,
    XDW =  0x1E,
    XDR =  0x1F,
    XRS =  0x20,
    XRD =  0x21,
    XRM =  0x22,
    XRF =  0x23,
    XRW =  0x24,
    XRZ =  0x25,
    VCH =  0x26,
    VCD =  0x27,
    VRE =  0x28,
    VT1 =  0x29,
    VT2 =  0x2A,
    VT3 =  0x2B,
    VT4 =  0x2C,
    VT5 =  0x2D,
    VT6 =  0x2E,
    VT7 =  0x2F,
    VT8 =  0x30,
    DJF =  0x31,
    IVO =  0x32,
    ICH =  0x33,
    IDE =  0x34,
    IRE =  0x35,
    IV2 =  0x36,
    IC2 =  0x37,
    ID2 =  0x38,
    IR2 =  0x39,
    USB =  0x3A,
    I00 =  0x80,
    I01 =  0x81,
    I02 =  0x82,
    I03 =  0x83,
    I04 =  0x84,
    I05 =  0x85,
    I06 =  0x86,
    I07 =  0x87,
    I08 =  0x88,
    I09 =  0x89,
    I0A =  0x8A,
    I0B =  0x8B,
    I0C =  0x8C,
    I0D =  0x8D,
    I0E =  0x8E,
    I8F =  0x8F,
    I90 =  0x90,
    I91 =  0x91,
    I92 =  0x92,
    I93 =  0x93,
    I94 =  0x94,
    I95 =  0x95,
    I96 =  0x96,
    I97 =  0x97,
    I98 =  0x98,
    I99 =  0x99,
    I9A =  0x9A,
    I9B =  0x9B,
    I9C =  0x9C,
    I9D =  0x9D,
    I9E =  0x9E,
    I9F =  0x9F,
    IA0 =  0xA0,
    IA1 =  0xA1,
    IA2 =  0xA2,
    NONE = 0xff
}
