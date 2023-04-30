use lemna_nih_plug::nih_plug;
use nih_plug::prelude::*;
use std::sync::Arc;

mod drag_sources;
mod file_selection;
mod parameters;

mod app;
use app::*;

nih_export_clap!(M8Plug);
nih_export_vst3!(M8Plug);

#[derive(Default)]
pub struct M8Plug {
    params: Arc<M8Params>,
}

impl Plugin for M8Plug {
    const NAME: &'static str = "MIDI-M8";
    const VENDOR: &'static str = "ANC";
    const URL: &'static str = "https://github.com/AlexCharlton/midi-m8";
    const EMAIL: &'static str = "alex.n.charlton@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];
    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

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
        let app_params = self.params.clone();
        lemna_nih_plug::create_lemna_editor::<Renderer, M8PlugApp, _>(
            "Midi M8",
            400,
            300,
            vec![(
                "Roboto".into(),
                include_bytes!("../include/RobotoMono-Regular.ttf"),
            )],
            move |ctx, ui| {
                ui.with_app_state::<AppState, _>(|s| {
                    s.gui_context = Some(ctx.clone());
                    s.params = app_params.clone()
                });
            },
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
    const VST3_CLASS_ID: [u8; 16] = *b"ANC-MIDI-M8-Plug";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}
