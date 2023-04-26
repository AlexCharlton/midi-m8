use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;

use lemna::{self, widgets, *};
use lemna_nih_plug::nih_plug;
use lemna_nih_plug::nih_plug::{
    context::gui::GuiContext,
    params::{range::*, *},
};

pub type Renderer = lemna::render::wgpu::WGPURenderer;
type Node = lemna::Node<Renderer>;

const DARK_GRAY: Color = color!(0x16, 0x16, 0x16);
const MID_GRAY: Color = color!(0x5F, 0x5F, 0x5F);
const LIGHT_GRAY: Color = color!(0xDE, 0xDE, 0xDE);
const BLUE: Color = color!(0x00, 0xE5, 0xEE);

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
                lay!(size: size_pct!(100.0), wrap: true,
                     padding: rect!(10.0),
                     axis_alignment: Alignment::Center, cross_alignment: Alignment::Center)
            )
            // Fileselector
            // Parameters
            // TrackDraggers
            // Footer
            .push(node!(
                AllTracksDragSource {},
                lay!(size: size!(Auto, 100.0)),
                0
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

/// DragSource

#[derive(Debug)]
pub struct AllTracksDragSource {}

impl Component<Renderer> for AllTracksDragSource {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new().bg(BLUE).border(MID_GRAY, 2.0),
                lay!(
                    size: size_pct!(100.0),
                    margin: rect!(10.0),
                    padding: rect!(5.0),
                    cross_alignment: layout::Alignment::Center,
                    axis_alignment: layout::Alignment::Center
                ),
                0
            )
            .push(node!(widgets::Text::new(
                txt!("ALL TRACKS"),
                widgets::TextStyle {
                    h_alignment: HorizontalAlign::Center,
                    color: DARK_GRAY,
                    ..widgets::TextStyle::default()
                }
            ))),
        )
    }

    fn on_drag_start(&mut self, event: &mut Event<event::DragStart>) -> Vec<Message> {
        current_window()
            .unwrap()
            .start_drag(Data::Filepath("/test/file.txt".into()));
        event.stop_bubbling();
        vec![]
    }
}
