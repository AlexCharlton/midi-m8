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
use midi_m8_core::song_to_midi::*;

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
    pub start: IntParam,
    #[id = "max_len"]
    pub max_len: FloatParam,
    #[id = "transpose"]
    pub transpose: IntParam,
}

impl Default for M8Params {
    fn default() -> Self {
        Self {
            start: IntParam::new("Start", 0, IntRange::Linear { min: 0, max: 255 }),
            // TODO with_value_to_string
            max_len: FloatParam::new(
                "Max Note Length",
                0.0,
                FloatRange::Linear { min: 0.0, max: 4.0 },
            )
            .with_step_size(1.0 / 16.0),
            transpose: IntParam::new("Transpose", 36, IntRange::Linear { min: 0, max: 72 }),
        }
    }
}

#[derive(Debug)]
pub enum AppMsg {
    FileSelected { selection: Option<PathBuf> },
}

#[derive(Default)]
pub struct AppState {
    pub params: Arc<M8Params>,
    pub gui_context: Option<Arc<dyn GuiContext>>,
    file: Option<PathBuf>,
    song: Option<Arc<Song>>,
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
                    widgets::Text::new(
                        txt!(format!("MIDIM8 V{}", env!("CARGO_PKG_VERSION"))),
                        TextStyle {
                            color: BLUE,
                            ..Default::default()
                        }
                    ),
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
                self.update_song()
                    .map_err(|e| self.state_mut().error = Some(e.to_string()));
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
                self.update_song()
                    .map_err(|e| self.state_mut().error = Some(e.to_string()));
            }
            _ => (),
        }
        vec![]
    }
}

impl M8PlugApp {
    fn update_song(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(p) = &self.state_ref().file {
            let mut f = File::open(p)?;
            let song = Song::read(&mut f)?;
            self.state_mut().song = Some(Arc::new(song));
        } else {
            self.state_mut().song = None;
        }
        // TODO create midi data
        Ok(())
    }
}

impl lemna::App<Renderer> for M8PlugApp {
    fn new() -> Self {
        Self { state: None }
    }
}
