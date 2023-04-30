use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use lemna::{self, widgets, *};
use lemna_nih_plug::nih_plug;
use lemna_nih_plug::nih_plug::{
    context::gui::GuiContext,
    params::{range::*, *},
};
use m8_files::Song;
use midi_m8_core::midi_file::MidiFile;
use midi_m8_core::song_to_midi::{song_to_midi_file, Config, TICKS_PER_QUARTER_NOTE};
use temp_file::TempFile;

use crate::drag_sources::*;
use crate::file_selection::*;
use crate::parameters::*;

pub type Renderer = lemna::render::wgpu::WGPURenderer;
pub type Node = lemna::Node<Renderer>;

pub const DARK_GRAY: Color = color!(0x16, 0x16, 0x16);
pub const MID_GRAY: Color = color!(0x5F, 0x5F, 0x5F);
pub const LIGHT_GRAY: Color = color!(0xDE, 0xDE, 0xDE);
pub const BLUE: Color = color!(0x00, 0xE5, 0xEE);

#[derive(Params, Debug)]
pub struct M8Params {
    #[id = "start"]
    pub start: Arc<IntParam>,
    #[id = "max_len"]
    pub max_len: Arc<FloatParam>,
    #[id = "transpose"]
    pub transpose: Arc<IntParam>,
}

impl Default for M8Params {
    fn default() -> Self {
        Self {
            start: Arc::new(IntParam::new(
                "Start",
                0,
                IntRange::Linear { min: 0, max: 255 },
            )),
            max_len: Arc::new(
                FloatParam::new(
                    "Max Note Length",
                    0.0,
                    FloatRange::Linear { min: 0.0, max: 4.0 },
                )
                .with_value_to_string(Arc::new(|v| {
                    if v == 0.0 {
                        "--".into()
                    } else {
                        note_len_to_string(v)
                    }
                }))
                .with_step_size(1.0 / 16.0),
            ),
            transpose: Arc::new(IntParam::new(
                "Transpose",
                36,
                IntRange::Linear { min: 0, max: 72 },
            )),
        }
    }
}

fn note_len_to_string(v: f32) -> String {
    let whole_notes = v.floor();
    let fractional = v - whole_notes;
    let fract = if fractional < 0.0001 {
        ""
    } else if fractional <= 1.0 / 16.0 {
        "1/16"
    } else if fractional <= 2.0 / 16.0 {
        "1/8"
    } else if fractional <= 3.0 / 16.0 {
        "1/8."
    } else if fractional <= 4.0 / 16.0 {
        "1/4"
    } else if fractional <= 5.0 / 16.0 {
        "5/16"
    } else if fractional <= 6.0 / 16.0 {
        "1/4."
    } else if fractional <= 7.0 / 16.0 {
        "7/16"
    } else if fractional <= 8.0 / 16.0 {
        "1/2"
    } else if fractional <= 9.0 / 16.0 {
        "9/16"
    } else if fractional <= 10.0 / 16.0 {
        "5/8"
    } else if fractional <= 11.0 / 16.0 {
        "11/16"
    } else if fractional <= 12.0 / 16.0 {
        "3/4"
    } else if fractional <= 13.0 / 16.0 {
        "13/16"
    } else if fractional <= 14.0 / 16.0 {
        "7/8"
    } else if fractional <= 15.0 / 16.0 {
        "15/16"
    } else {
        ""
    };

    let whole_notes = whole_notes as u32;
    if whole_notes > 0 && fract != "" {
        format!("{whole_notes} {fract}")
    } else if whole_notes > 0 {
        format!("{whole_notes}")
    } else {
        format!("{fract}")
    }
}

#[derive(Debug)]
pub struct MidiTempFiles {
    pub all: TempFile,
    pub tracks: [Option<TempFile>; 8],
}

#[derive(Debug)]
pub enum AppMsg {
    FileSelected { selection: Option<PathBuf> },
    OpenSite,
}

#[derive(Default)]
pub struct AppState {
    pub params: Arc<M8Params>,
    pub gui_context: Option<Arc<dyn GuiContext>>,
    file: Option<PathBuf>,
    song: Option<Arc<MidiTempFiles>>,
    error: Option<String>, // TODO
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("file", &self.file)
            .finish()
    }
}

#[state_component(AppState)]
#[derive(Debug)]
pub struct M8PlugApp {}

#[state_component_impl(AppState)]
impl lemna::Component<Renderer> for M8PlugApp {
    fn init(&mut self) {
        self.state = Some(AppState::default())
    }

    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new().bg(DARK_GRAY),
                lay!(
                    size: size_pct!(100.0),
                    direction: Direction::Column,
                    axis_alignment: Alignment::Stretch,
                    cross_alignment: Alignment::Stretch
                )
            )
            .push(node!(
                FileSelection::new(self.state_ref().file.clone()),
                lay!(size: size!(Auto, 30.0)),
                0
            ))
            .push(node!(
                Parameters::new(self.state_ref().params.clone()),
                lay!(size: size!(Auto, 90.0)),
                1
            ))
            .push(node!(
                DragSources::new(self.state_ref().song.clone()),
                lay!(size: size!(Auto, 150.0)),
                2
            ))
            // Footer
            .push(
                node!(
                    widgets::Div::new(),
                    lay!(
                        size: size!(Auto, 30.0),
                        direction: Direction::Row,
                        padding: rect!(5.0),
                    ),
                    3
                )
                .push(node!(widgets::Button::new(
                    txt!("?"),
                    ButtonStyle {
                        text_color: BLUE,
                        background_color: DARK_GRAY,
                        highlight_color: DARK_GRAY,
                        active_color: DARK_GRAY,
                        border_color: BLUE,
                        radius: 16.0,
                        padding: 1.5,
                        tool_tip_style: ToolTipStyle {
                            text_color: LIGHT_GRAY,
                            background_color: MID_GRAY,
                            border_color: DARK_GRAY,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ).tool_tip("Select or drag a M8 song file. Then drag the MIDI data from the desired track or all tracks.\n\nYou can adjust the max note length, the starting song position, and the amount by which to transpose M8 note numbers to turn them into MIDI note numbers (default is 36).".into())))
                .push(node!(
                    widgets::Button::new(
                        txt!(format!("MIDI-M8 V{}", env!("CARGO_PKG_VERSION"))),
                        ButtonStyle {
                            text_color: BLUE,
                            background_color: DARK_GRAY,
                            highlight_color: DARK_GRAY,
                            active_color: DARK_GRAY,
                            border_width: 0.0,
                            padding: 1.5,
                            ..Default::default()
                        },
                    ).on_click(Box::new(|| msg!(AppMsg::OpenSite))),
                    lay!(
                        position_type: PositionType::Absolute,
                        position: rect!(Auto, Auto, Auto, 0.0)
                    )
                )),
            ),
        )
    }

    fn on_drag_drop(&mut self, event: &mut Event<event::DragDrop>) -> Vec<Message> {
        match &event.input.0 {
            Data::Filepath(p) if p.extension().map(|e| e == "m8s").unwrap_or(false) => {
                self.state_mut().file = Some(p.clone());
                match self.update_song() {
                    Err(e) => self.state_mut().error = Some(e.to_string()),
                    Ok(_) => (),
                }
                event.dirty();
            }
            _ => (),
        }
        vec![]
    }

    fn on_drag_enter(&mut self, event: &mut Event<event::DragEnter>) -> Vec<Message> {
        let contains_m8_song = event.input.0.iter().any(|f| match f {
            Data::Filepath(p) => p.extension().map(|e| e == "m8s").unwrap_or(false),
            _ => false,
        });
        current_window()
            .unwrap()
            .set_drop_target_valid(contains_m8_song);
        vec![]
    }

    fn update(&mut self, message: Message) -> Vec<Message> {
        match message.downcast_ref::<AppMsg>() {
            Some(AppMsg::FileSelected { selection: f }) => {
                self.state_mut().file = f.clone();
                match self.update_song() {
                    Err(e) => self.state_mut().error = Some(e.to_string()),
                    Ok(_) => (),
                }
            }
            Some(AppMsg::OpenSite) => match open::that("https://github.com/AlexCharlton/midi-m8") {
                _ => (),
            },
            None => (),
        }
        vec![]
    }
}

impl M8PlugApp {
    fn update_song(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.state_ref().file {
            let mut f = File::open(p)?;
            let song = Song::read(&mut f)?;
            let mut config = Config {
                start_from: self.state_ref().params.start.value() as u8,
                global_transpose: self.state_ref().params.transpose.value() as i16,
                ..Config::default()
            };

            let max_len = self.state_ref().params.max_len.value();
            if max_len > 0.0 {
                for i in 0..8 {
                    config.max_note_length[i] = (self.state_ref().params.max_len.value()
                        * TICKS_PER_QUARTER_NOTE as f32)
                        as u32;
                }
            }

            let midi_file = song_to_midi_file(&song, &config);
            self.state_mut().song = Some(Arc::new(Self::midi_file_to_paths(midi_file)?));
        } else {
            self.state_mut().song = None;
        }
        Ok(())
    }

    fn midi_file_to_paths(midi_file: MidiFile) -> Result<MidiTempFiles, Box<dyn Error>> {
        let all = midi_file.to_midi();
        let mut f = MidiTempFiles {
            all: TempFile::with_suffix(".midi")?.with_contents(&all[..])?,
            tracks: Default::default(),
        };

        for (i, track) in midi_file.tracks.iter().enumerate() {
            if track.events.is_empty() {
                continue;
            }
            let t = midi_file.track_to_midi(i);
            f.tracks[i] = Some(TempFile::with_suffix(".midi")?.with_contents(&t[..])?);
        }
        Ok(f)
    }
}

impl lemna::App<Renderer> for M8PlugApp {
    fn new() -> Self {
        Self { state: None }
    }
}
