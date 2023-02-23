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

struct Reader {
    buffer: Vec<u8>,
    position: Rc<RefCell<usize>>
}

#[allow(dead_code)]
impl Reader {
    fn new(buffer: Vec<u8>) -> Self {
        Self {
            buffer, position: Rc::new(RefCell::new(0))
        }
    }

    fn read(&self) -> u8 {
        let p: usize = *self.position.borrow();
        let b = self.buffer[p];
        *self.position.borrow_mut() += 1;
        b
    }

    fn read_bytes(&self, n: usize) -> &[u8] {
        let p: usize = *self.position.borrow();
        let bs = &self.buffer[p..p+n];
        *self.position.borrow_mut() += n;
        bs
    }

    fn read_bool(&self) -> bool {
        self.read() == 1
    }

    fn read_string(&self, n: usize) -> String {
        let b = self.read_bytes(n);
        let end = b.iter().position(|&x| x == 0).unwrap_or(0);
        std::str::from_utf8(&b[0..end]).expect("invalid utf-8 sequence in string").to_string()
    }

    fn pos(&self) -> usize {
        *self.position.borrow()
    }

    fn set_pos(&self, n: usize) {
        *self.position.borrow_mut() = n;
    }
}

#[derive(PartialEq)]
pub struct Song {
    pub version: Version,
    pub directory: String,
    pub transpose: u8,
    pub tempo: f32,
    pub quantize: u8,
    pub name: String,
    pub key: u8,

    pub song: SongSteps,
    pub phrases: [Phrase;255],
    pub chains: [Chain;255],
    pub instruments: [Instrument;128],
    pub tables: [Table;256],
    pub grooves: [Groove;32],
    pub scales: [Scale;16],

    pub mixer_settings: MixerSettings,
    pub effects_settings: EffectsSettings,
    pub midi_settings: MidiSettings,
    pub midi_mappings: [MidiMapping;128],
}

impl fmt::Debug for Song {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Song")
            .field("version", &self.version)
            .field("directory", &self.directory)
            .field("name", &self.name)
            .field("tempo", &self.tempo)
            .field("transpose", &self.transpose)
            .field("quantize", &self.quantize)
            .field("key", &self.key)
            .field("song", &self.song)
            .field("chains", &self.chains[0])
            .field("phrases", &self.phrases[16]) // TODO
            .field("instruments", &self.instruments[0])
            .field("tables", &self.tables[0])
            //.field("grooves", &self.grooves[0])
            // .field("scales", &self.scales)
            .finish()
    }
}

impl Song {
    pub fn read(reader: &mut impl std::io::Read) -> Result<Self> {
        let mut buf: Vec<u8> = vec!();
        reader.read_to_end(&mut buf).unwrap();
        // TODO check that buffer is long enough
        let reader = Reader::new(buf);
        Self::from_reader(&reader)
    }

    fn from_reader(reader: &Reader) -> Result<Self> {
        let version = Version::from_reader(reader)?;
        reader.read_bytes(2); // Skip
        let directory = reader.read_string(128);
        let transpose = reader.read();
        let tempo = LittleEndian::read_f32(reader.read_bytes(4));
        let quantize = reader.read();
        let name = reader.read_string(12);
        let midi_settings = MidiSettings::from_reader(reader)?;
        let key = reader.read();
        reader.read_bytes(18); // Skip
        let mixer_settings = MixerSettings::from_reader(reader)?;
        // println!("{:x}", reader.pos());

        let mut i:u16 = 0;
        let grooves: [Groove; 32] = arr![Groove::from_reader(reader, {i += 1; (i - 1) as u8})?; 32];
        let song = SongSteps::from_reader(reader)?;
        i = 0;
        let phrases: [Phrase; 255] = arr![Phrase::from_reader(reader, {i += 1; (i - 1) as u8})?; 255];
        i = 0;
        let chains: [Chain; 255] = arr![Chain::from_reader(reader, {i += 1; (i - 1) as u8})?; 255];
        i = 0;
        let tables: [Table; 256] = arr![Table::from_reader(reader, {i += 1; (i - 1) as u8})?; 256];
        i = 0;
        let instruments: [Instrument; 128] = arr![Instrument::from_reader(reader, {i += 1; (i - 1) as u8}, version)?; 128];

        reader.read_bytes(3); // Skip
        let effects_settings = EffectsSettings::from_reader(reader)?;
        reader.set_pos(0x1A5FE);
        let midi_mappings: [MidiMapping; 128] = arr![MidiMapping::from_reader(reader)?; 128];

        i = 0;
        let scales: [Scale; 16] = if version.at_least(2, 5) {
            reader.set_pos(0x1AA7E);
            arr![Scale::from_reader(reader, {i += 1; (i - 1) as u8})?; 16]
        } else {
            arr![{let mut s = Scale::default(); s.number = i as u8; i+=1; s}; 16]
        };

        Ok(Self{
            version,
            directory,
            transpose,
            tempo,
            quantize,
            name,
            midi_settings,
            key,
            mixer_settings,
            grooves,
            song,
            phrases,
            chains,
            tables,
            instruments,
            scales,
            effects_settings,
            midi_mappings,
        })
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

impl Version {
    fn from_reader(reader: &Reader) -> Result<Self> {
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

    fn at_least(&self, major: u8, minor: u8) -> bool {
        self.major > major ||
            (self.major == major && self.minor >= minor)
    }
}

#[derive(PartialEq)]
pub struct SongSteps {
    pub steps: [u8; 2048]
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

    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            steps: reader.read_bytes(2048).try_into().unwrap()
        })
    }
}

impl fmt::Display for SongSteps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SONG\n\n{}", self.print_screen())
    }
}
impl fmt::Debug for SongSteps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

#[derive(PartialEq)]
pub struct Chain {
    pub number: u8,
    pub steps: [ChainStep; 16]
}
impl Chain {
    pub fn print_screen(&self) -> String {
        (0..16).fold("  PH TSP\n".to_string(),
                     |s, row| s + &self.steps[row].print(row as u8) + "\n"
        )
    }

    fn from_reader(reader: &Reader, number: u8) -> Result<Self> {
        Ok(Self {
            number,
            steps: arr![ChainStep::from_reader(reader)?; 16],
        })
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CHAIN {:02x}\n\n{}", self.number, self.print_screen())
    }
}
impl fmt::Debug for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

#[derive(PartialEq, Debug)]
pub struct ChainStep {
    pub phrase: u8,
    pub transpose: u8,
}
impl ChainStep {
    pub fn print(&self, row: u8) -> String {
        if self.phrase == 255 {
            format!("{:x} -- 00", row)
        } else {
            format!("{:x} {:02x} {:02x}", row, self.phrase, self.transpose)
        }
    }

    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            phrase: reader.read(),
            transpose: reader.read(),
        })
    }
}

#[derive(PartialEq)]
pub struct Phrase {
    pub number: u8,
    pub steps: [Step; 16]
}
impl Phrase {
    pub fn print_screen(&self) -> String {
        (0..16).fold("  N   V  I  FX1   FX2   FX3  \n".to_string(),
                     |s, row| s + &self.steps[row].print(row as u8) + "\n"
        )
    }

    fn from_reader(reader: &Reader, number: u8) -> Result<Self> {
        Ok(Self {
            number,
            steps: arr![Step::from_reader(reader)?; 16],
        })
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PHRASE {:02x}\n\n{}", self.number, self.print_screen())
    }
}
impl fmt::Debug for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

#[derive(PartialEq, Debug)]
pub struct Step {
    pub note: Note,
    pub velocity: u8,
    pub instrument: u8,
    pub fx1: FX,
    pub fx2: FX,
    pub fx3: FX,
}
impl Step {
    pub fn print(&self, row: u8) -> String {
        let velocity = if self.velocity == 255 { format!("--") }
        else { format!("{:02x}", self.velocity)};
        let instrument = if self.instrument == 255 { format!("--") }
        else { format!("{:02x}", self.instrument)};
        format!("{:x} {} {} {} {} {} {}", row, self.note, velocity, instrument,
                self.fx1, self.fx2, self.fx3)
    }

    fn from_reader(reader: &Reader) -> Result<Self> {
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

#[derive(PartialEq)]
pub struct Table {
    pub number: u8,
    pub steps: [TableStep; 16]
}
impl Table {
    pub fn print_screen(&self) -> String {
        (0..16).fold("  N  V  FX1   FX2   FX3  \n".to_string(),
                     |s, row| s + &self.steps[row].print(row as u8) + "\n"
        )
    }

    fn from_reader(reader: &Reader, number: u8) -> Result<Self> {
        Ok(Self {
            number,
            steps: arr![TableStep::from_reader(reader)?; 16],
        })
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TABLE {:02x}\n\n{}", self.number, self.print_screen())
    }
}
impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}


#[derive(PartialEq, Debug)]
pub struct TableStep {
    pub transpose: u8,
    pub velocity: u8,
    pub fx1: FX,
    pub fx2: FX,
    pub fx3: FX,
}
impl TableStep {
    pub fn print(&self, row: u8) -> String {
        let transpose = if self.transpose == 255 { format!("--") }
        else { format!("{:02x}", self.transpose)};
        let velocity = if self.velocity == 255 { format!("--") }
        else { format!("{:02x}", self.velocity)};
        format!("{:x} {} {} {} {} {}", row, transpose, velocity,
                self.fx1, self.fx2, self.fx3)
    }

    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            transpose: reader.read(),
            velocity: reader.read(),
            fx1: FX::from_reader(reader)?,
            fx2: FX::from_reader(reader)?,
            fx3: FX::from_reader(reader)?,
        })
    }
}


#[derive(PartialEq, Debug)]
pub struct FX {
    pub command: FXCommand,
    pub value: u8
}
impl FX {
    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            command: FXCommand::from_u8(reader.read()),
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
pub enum FXCommand {
    // Sequencer commands
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
    // FX + mixer commands
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
    // Instrument commands
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
    // No command
    NONE = 0xff
}
impl FXCommand  {
    fn from_u8(u: u8) -> Self {
        unsafe { std::mem::transmute(u)}
    }

    #[allow(dead_code)]
    fn to_ui(self) -> u8 {
        unsafe { std::mem::transmute(self)}
    }
}

#[derive(PartialEq, Debug)]
pub enum Instrument {
    WavSynth(WavSynth),
    MacroSynth(MacroSynth),
    Sampler(Sampler),
    MIDIOut(MIDIOut),
    FMSynth(FMSynth),
    None,
}
impl Instrument {
    fn from_reader(reader: &Reader, number: u8, version: Version) -> Result<Self> {
        let start_pos = reader.pos();
        let kind = reader.read();
        let name = reader.read_string(12);
        let transpose = reader.read_bool();
        let table_tick = reader.read();
        let (volume, pitch, fine_tune) = if kind != 3 {
            (reader.read(), reader.read(), reader.read())
        } else { (0, 0, 0) };

        let finalize = || -> () { reader.set_pos(start_pos + 215); };

        Ok(match kind {
            0x00 => {
                let shape = reader.read();
                let size = reader.read();
                let mult = reader.read();
                let warp = reader.read();
                let mirror = reader.read();
                let synth_params = SynthParams::from_reader(reader, volume, pitch, fine_tune)?;
                finalize();
                Self::WavSynth(WavSynth {
                    number,
                    name,
                    transpose,
                    table_tick,
                    synth_params,

                    shape,
                    size,
                    mult,
                    warp,
                    mirror,
                })
            }
            0x01 => {
                let shape = reader.read();
                let timbre = reader.read();
                let color = reader.read();
                let degrade = reader.read();
                let redux = reader.read();
                let synth_params = SynthParams::from_reader(reader, volume, pitch, fine_tune)?;
                finalize();
                Self::MacroSynth(MacroSynth {
                    number,
                    name,
                    transpose,
                    table_tick,
                    synth_params,

                    shape,
                    timbre,
                    color,
                    degrade,
                    redux,
                })
            }
            0x02 => {
                let play_mode = reader.read();
                let slice = reader.read();
                let start = reader.read();
                let loop_start = reader.read();
                let length = reader.read();
                let degrade = reader.read();
                let synth_params = SynthParams::from_reader(reader, volume, pitch, fine_tune)?;
                reader.set_pos(start_pos + 0x57);
                let sample_path = reader.read_string(128);
                Self::Sampler(Sampler {
                    number,
                    name,
                    transpose,
                    table_tick,
                    synth_params,

                    sample_path,
                    play_mode,
                    slice,
                    start,
                    loop_start,
                    length,
                    degrade,
                })
            }
            0x03 => {
                let port = reader.read();
                let channel = reader.read();
                let bank_select = reader.read();
                let program_change = reader.read();
                reader.read_bytes(3); // discard
                let custom_cc: [ControlChange; 8] = arr![ControlChange::from_reader(reader)?; 8];
                Self::MIDIOut(MIDIOut {
                    number,
                    name,
                    transpose,
                    table_tick,

                    port,
                    channel,
                    bank_select,
                    program_change,
                    custom_cc,
                })
            }
            0x04 => {
                let algo = reader.read();
                let mut operators: [Operator; 4] = arr![Operator::default(); 4];
                if version.at_least(1, 4) {
                    for i in 0..4 {
                        operators[i].shape = reader.read();
                    }
                }
                for i in 0..4 {
                    operators[i].ratio = reader.read();
                    operators[i].ratio_fine = reader.read();
                }
                for i in 0..4 {
                    operators[i].level = reader.read();
                    operators[i].feedback = reader.read();
                }
                for i in 0..4 {
                    operators[i].mod_a = reader.read();
                }
                for i in 0..4 {
                    operators[i].mod_b = reader.read();
                }
                let mod1 = reader.read();
                let mod2 = reader.read();
                let mod3 = reader.read();
                let mod4 = reader.read();
                let synth_params = SynthParams::from_reader(reader, volume, pitch, fine_tune)?;
                finalize();

                Self::FMSynth(FMSynth {
                    number,
                    name,
                    transpose,
                    table_tick,
                    synth_params,

                    algo,
                    operators,
                    mod1,
                    mod2,
                    mod3,
                    mod4,
                })
            }
            0xFF => Self::None,
            _ => panic!("Instrument type {} not supported", kind)
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct WavSynth {
    pub number: u8,
    pub name: String,
    pub transpose: bool,
    pub table_tick: u8,
    pub synth_params: SynthParams,

    pub shape: u8,
    pub size: u8,
    pub mult: u8,
    pub warp: u8,
    pub mirror: u8,
}

#[derive(PartialEq, Debug)]
pub struct MacroSynth {
    pub number: u8,
    pub name: String,
    pub transpose: bool,
    pub table_tick: u8,
    pub synth_params: SynthParams,

    pub shape: u8,
    pub timbre: u8,
    pub color: u8,
    pub degrade: u8,
    pub redux: u8,
}

#[derive(PartialEq, Debug)]
pub struct Sampler {
    pub number: u8,
    pub name: String,
    pub transpose: bool,
    pub table_tick: u8,
    pub synth_params: SynthParams,

    pub sample_path: String,
    pub play_mode: u8,
    pub slice: u8,
    pub start: u8,
    pub loop_start: u8,
    pub length: u8,
    pub degrade: u8,
}

#[derive(PartialEq, Debug)]
pub struct FMSynth {
    pub number: u8,
    pub name: String,
    pub transpose: bool,
    pub table_tick: u8,
    pub synth_params: SynthParams,

    pub algo: u8,
    pub operators: [Operator; 4],
    pub mod1: u8,
    pub mod2: u8,
    pub mod3: u8,
    pub mod4: u8,
}

#[derive(PartialEq, Debug)]
pub struct MIDIOut {
    pub number: u8,
    pub name: String,
    pub transpose: bool,
    pub table_tick: u8,

    pub port: u8,
    pub channel: u8,
    pub bank_select: u8,
    pub program_change: u8,
    pub custom_cc: [ControlChange; 8],
}


#[derive(PartialEq, Debug)]
pub struct SynthParams {
    pub volume: u8,
    pub pitch: u8,
    pub fine_tune: u8,

    pub filter_type: u8,
    pub filter_cutoff: u8,
    pub filter_res: u8,

    pub amp: u8,
    pub limit: u8,

    pub mixer_pan: u8,
    pub mixer_dry: u8,
    pub mixer_chorus: u8,
    pub mixer_delay: u8,
    pub mixer_reverb: u8,

    pub envelopes: [Envelope; 2],
    pub lfos: [LFO; 2],
}
impl SynthParams {
    fn from_reader(reader: &Reader, volume: u8, pitch:u8, fine_tune: u8) -> Result<Self> {
        Ok(Self {
            volume,
            pitch,
            fine_tune,

            filter_type: reader.read(),
            filter_cutoff: reader.read(),
            filter_res: reader.read(),

            amp: reader.read(),
            limit: reader.read(),

            mixer_pan: reader.read(),
            mixer_dry: reader.read(),
            mixer_chorus: reader.read(),
            mixer_delay: reader.read(),
            mixer_reverb: reader.read(),

            envelopes: arr![Envelope::from_reader(reader)?; 2],
            lfos: arr![LFO::from_reader(reader)?; 2],
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct Envelope {
    pub dest: u8,
    pub amount: u8,
    pub attack: u8,
    pub hold: u8,
    pub decay: u8,
    pub retrigger: u8,
}
impl Envelope {
    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            dest: reader.read(),
            amount: reader.read(),
            attack: reader.read(),
            hold: reader.read(),
            decay: reader.read(),
            retrigger: reader.read(),
        })
    }
}


#[derive(PartialEq, Debug)]
pub struct LFO {
    pub shape: u8,
    pub dest: u8,
    pub trigger_mode: u8,
    pub freq: u8,
    pub amount: u8,
    pub retrigger: u8,
}
impl LFO {
    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            shape: reader.read(),
            dest: reader.read(),
            trigger_mode: reader.read(),
            freq: reader.read(),
            amount: reader.read(),
            retrigger: reader.read(),
        })
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Operator {
    pub shape: u8,
    pub ratio: u8,
    pub ratio_fine: u8,
    pub level: u8,
    pub feedback: u8,
    pub retrigger: u8,
    pub mod_a: u8,
    pub mod_b: u8,
}

#[derive(PartialEq, Debug)]
pub struct ControlChange {
    pub number: u8,
    pub default_value: u8,
}
impl ControlChange {
    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self{
            number: reader.read(),
            default_value: reader.read(),
        })
    }
}


#[derive(PartialEq)]
pub struct Groove {
    pub number: u8,
    pub steps: [u8; 16]
}
impl Groove {
    fn from_reader(reader: &Reader, number: u8) -> Result<Self> {
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

impl fmt::Display for Groove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Groove {}:{:?}", self.number, self.active_steps())
    }
}
impl fmt::Debug for Groove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

#[derive(PartialEq)]
pub struct Scale {
    pub number: u8,
    pub name: String,
    pub notes: [NoteOffset; 12] // Offsets for notes C-B
}
impl Scale {
    fn from_reader(reader: &Reader, number: u8) -> Result<Self> {
        let map = LittleEndian::read_u16(reader.read_bytes(2));
        let mut notes = arr![NoteOffset::default(); 12];

        for (i, note) in notes.iter_mut().enumerate() {
            note.enabled = ((map >> i) & 0x1) == 1;
            let offset = f32::from(reader.read()) + (f32::from(reader.read()) / 100.0);
            note.semitones = offset;
        }
        let name = reader.read_string(16);
        Ok(Self {
            number,
            name,
            notes,
        })
    }

    fn default() -> Self {
        Self {
            number: 0,
            name: "CHROMATIC".to_string(),
            notes: arr![NoteOffset::default(); 12]
        }
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let notes = vec!["C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B "];
        let offsets = self.notes.iter().zip(notes.iter()).map(|(offset, note)| -> String {
            let s = if offset.enabled {
                let sign = if offset.semitones < 0.0 { "-" } else { " " };
                format!(" ON{}{:02.2}", sign, offset.semitones.abs())
            } else {
                " -- -- --".to_string()
            };
            note.to_string() + &s
        }).collect::<Vec<String>>()
            .join("\n");

        write!(f, "Scale {}\nKEY   C\n\n   EN OFFSET\n{}\n\nNAME  {}",
               self.number, offsets, &self.name)
    }
}
impl fmt::Debug for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

#[derive(PartialEq, Debug)]
pub struct NoteOffset {
    pub enabled: bool,
    pub semitones: f32, // Semitones.cents: -24.0-24.0
}
impl NoteOffset {
    fn default() -> Self {
        Self { enabled: true, semitones: 0.0 }
    }
}

#[derive(PartialEq, Debug)]
pub struct MidiSettings {
    pub bytes: [u8; 27] // TODO
}
impl MidiSettings {
    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            bytes: reader.read_bytes(27).try_into().unwrap()
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct MixerSettings {
    pub bytes: [u8; 32] // TODO
}
impl MixerSettings {
    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            bytes: reader.read_bytes(32).try_into().unwrap()
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct EffectsSettings {
    pub bytes: [u8; 22] // TODO
}
impl EffectsSettings {
    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            bytes: reader.read_bytes(22).try_into().unwrap()
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct MidiMapping {
    pub bytes: [u8; 7] // TODO
}
impl MidiMapping {
    fn from_reader(reader: &Reader) -> Result<Self> {
        Ok(Self {
            bytes: reader.read_bytes(7).try_into().unwrap()
        })
    }
}
