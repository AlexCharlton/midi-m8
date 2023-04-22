use lemna_nih_plug::nih_plug::prelude::*;

use midi_m8_plugin::M8Plug;

fn main() {
    nih_export_standalone::<M8Plug>();
}
