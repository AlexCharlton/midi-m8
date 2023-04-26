use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use lemna::{self, widgets, *};
use lemna_nih_plug::nih_plug;
use lemna_nih_plug::nih_plug::{
    context::gui::GuiContext,
    params::{range::*, *},
};
use m8_files::Song;

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

#[derive(Default)]
pub struct AppState {
    pub params: Arc<M8Params>,
    pub gui_context: Option<Arc<dyn GuiContext>>,
    file: Option<PathBuf>,
    song: Option<Arc<Song>>,
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
            .push(node!(
                widgets::Div::new().bg([0.0, 0.5, 0.5]),
                lay!(size: size!(Auto, 30.0)),
                3
            )),
        )
    }

    fn on_drag_drop(&mut self, event: &mut Event<event::DragDrop>) -> Vec<Message> {
        // TODO
        vec![]
    }

    fn on_drag_target(&mut self, _event: &mut Event<event::DragTarget>) -> Vec<Message> {
        // TODO Evaluate file type
        current_window().unwrap().set_drop_target_valid(true);
        vec![]
    }
}

impl lemna::App<Renderer> for M8PlugApp {
    fn new() -> Self {
        Self { state: None }
    }
}
