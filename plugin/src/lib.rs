use lemna::{self, widgets, *};
use lemna_nih_plug::nih_plug;
use nih_plug::prelude::*;
use std::sync::Arc;

type Renderer = lemna::render::wgpu::WGPURenderer;
type Node = lemna::Node<Renderer>;

#[derive(Debug, Default)]
pub struct DropTargetState {
    active: bool,
}

#[state_component(DropTargetState)]
#[derive(Debug)]
pub struct DropTarget {}

impl DropTarget {
    fn new() -> Self {
        Self {
            state: Some(DropTargetState::default()),
        }
    }
}

#[state_component_impl(DropTargetState)]
impl Component<Renderer> for DropTarget {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new()
                    .bg(if self.state_ref().active {
                        Color::rgb(1.0, 0.5, 0.5)
                    } else {
                        Color::rgb(0.5, 1.0, 0.5)
                    })
                    .border(Color::BLACK, 2.0),
                lay!(
                    size: size_pct!(100.0),
                    margin: rect!(10.0),
                    padding: rect!(5.0),
                    cross_alignment: crate::layout::Alignment::Center,
                    axis_alignment: crate::layout::Alignment::Center
                ),
                0
            )
            .push(node!(widgets::Text::new(
                txt!("Drag something onto me"),
                widgets::TextStyle {
                    h_alignment: HorizontalAlign::Center,
                    ..widgets::TextStyle::default()
                }
            ))),
        )
    }

    fn on_drag_drop(&mut self, event: &mut Event<event::DragDrop>) -> Vec<Message> {
        println!("Got {:?}", event.input.0);
        self.state_mut().active = false;
        event.dirty();
        vec![]
    }

    fn on_drag_enter(&mut self, event: &mut Event<event::DragEnter>) -> Vec<Message> {
        self.state_mut().active = true;
        current_window().unwrap().set_drop_target_valid(true);
        event.dirty();
        vec![]
    }

    fn on_drag_leave(&mut self, event: &mut Event<event::DragLeave>) -> Vec<Message> {
        self.state_mut().active = false;
        current_window().unwrap().set_drop_target_valid(false);
        event.dirty();
        vec![]
    }

    fn on_drag_target(&mut self, event: &mut Event<event::DragTarget>) -> Vec<Message> {
        event.stop_bubbling();
        vec![]
    }
}

#[derive(Debug)]
pub struct DragSource {}

impl Component<Renderer> for DragSource {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new()
                    .bg(Color::rgb(0.5, 0.5, 1.0))
                    .border(Color::BLACK, 2.0),
                lay!(
                    size: size_pct!(100.0),
                    margin: rect!(10.0),
                    padding: rect!(5.0),
                    cross_alignment: crate::layout::Alignment::Center,
                    axis_alignment: crate::layout::Alignment::Center
                ),
                0
            )
            .push(node!(widgets::Text::new(
                txt!("Drag from me"),
                widgets::TextStyle {
                    h_alignment: HorizontalAlign::Center,
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

/// App

#[derive(Debug)]
pub struct M8PlugApp {}

impl lemna::Component<Renderer> for M8PlugApp {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new(),
                lay!(size: size_pct!(100.0), wrap: true,
                     padding: rect!(10.0),
                     axis_alignment: Alignment::Center, cross_alignment: Alignment::Center)
            )
            .push(node!(DropTarget::new(), lay!(size: size!(100.0)), 0))
            .push(node!(DragSource {}, lay!(size: size!(100.0)), 0)),
        )
    }

    fn on_drag_drop(&mut self, event: &mut Event<event::DragDrop>) -> Vec<Message> {
        // This will never print, because this is not a valid target per `on_drag_target`
        println!("Oops, you missed the target. Got {:?}", event.input.0);
        vec![]
    }

    fn on_drag_target(&mut self, _event: &mut Event<event::DragTarget>) -> Vec<Message> {
        current_window().unwrap().set_drop_target_valid(false);
        vec![]
    }
}

impl lemna::App<Renderer> for M8PlugApp {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Default)]
pub struct M8Plug {
    params: Arc<M8Params>,
}

#[derive(Params, Default)]
struct M8Params {}

impl Plugin for M8Plug {
    const NAME: &'static str = "MidiM8";
    const VENDOR: &'static str = "ANC";
    const URL: &'static str = "https://github.com/AlexCharlton/midi-m8";
    const EMAIL: &'static str = "alex.n.charlton@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];
    const MIDI_INPUT: MidiConfig = MidiConfig::None;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }

    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        lemna_nih_plug::create_lemna_editor::<Renderer, M8PlugApp>(
            "Midi M8",
            400,
            300,
            vec![("noto sans regular".to_string(), ttf_noto_sans::REGULAR)],
        )
    }
}

impl ClapPlugin for M8Plug {
    const CLAP_ID: &'static str = "anc.midi-m8";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Dirtywave M8 song files to Midi tracks");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility];
}

impl Vst3Plugin for M8Plug {
    const VST3_CLASS_ID: [u8; 16] = *b"ANC-Midi-M8-Plug";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}

nih_export_clap!(M8Plug);
nih_export_vst3!(M8Plug);
