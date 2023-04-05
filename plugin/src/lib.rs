use lemna::{self, widgets, *};
use lemna_nih_plug::nih_plug;
use nih_plug::prelude::*;
use std::sync::Arc;

type Renderer = lemna::render::wgpu::WGPURenderer;
type Node = lemna::Node<Renderer>;

#[derive(Debug)]
pub struct HelloApp {}

impl lemna::Component<Renderer> for HelloApp {
    fn view(&self) -> Option<Node> {
        Some(
            node!(
                widgets::Div::new(),
                lay!(size: size_pct!(100.0), wrap: true,
                     padding: rect!(10.0),
                     axis_alignment: Alignment::Center, cross_alignment: Alignment::Center)
            )
            .push(node!(
                widgets::Div::new().bg(Color::rgb(1.0, 0.0, 0.0)),
                lay!(size: size!(200.0, 100.0), margin: rect!(5.0)),
                0
            ))
            .push(node!(
                widgets::Div::new().bg(Color::rgb(0.0, 1.0, 0.0)),
                lay!(size: size!(100.0), margin: rect!(5.0)),
                1
            ))
            .push(node!(
                widgets::RoundedRect {
                    background_color: [0.0, 0.0, 1.0].into(),
                    border_width: 1.0,
                    ..Default::default()
                }
                .radius(5.0),
                lay!(size: size!(100.0), margin: rect!(5.0)),
                2
            )),
        )
    }
}

impl lemna::App<Renderer> for HelloApp {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Default)]
struct M8Plug {
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
        lemna_nih_plug::create_lemna_editor::<Renderer, HelloApp>("Midi M8", 200, 200, vec![])
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
