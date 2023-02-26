# midi-m8
A command line tool for turning Dirtywave M8 songs into Midi tracks.

[![Crates.io](https://img.shields.io/crates/v/midi-m8)](https://crates.io/crates/midi-m8) [![CI](https://github.com/AlexCharlton/midi-m8/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexCharlton/midi-m8/actions/workflows/ci.yml)

## Installation
Precompiled 64 bit binaries for Linux, Windows and OS X can be found in the [Releases](https://github.com/AlexCharlton/midi-m8/releases/latest). Download them and run them from the command line.

To compile your own version:

1. [install the Rust toolchain](https://rustup.rs/)
2. `$ cargo-install midi-m8`

You'll now have a binary in the `./target/release/` directory.

## Usage
```
Usage: midi-m8 [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Input (.m8s) file

Options:
  -t, --output <OUTPUT>
          Output file name [default: tracks.midi]
  -g, --global-transpose <GLOBAL_TRANSPOSE>
          How to map M8 note numbers to Midi Note numbers [default: 36]
  -n, --only-track <ONLY_TRACK_N>
          Only output track number (1-8)
  -m, --max-note-length <MAX_NOTE_LENGTH>
          Cap the maximum note length to this value in quarter notes
      --track-1-max-note-length <TRACK_1_MAX_NOTE_LEN>
          Cap the maximum note length for track 1 to this value in quarter notes
      --track-2-max-note-length <TRACK_2_MAX_NOTE_LEN>
          Cap the maximum note length for track 2 to this value in quarter notes
      --track-3-max-note-length <TRACK_3_MAX_NOTE_LEN>
          Cap the maximum note length for track 3 to this value in quarter notes
      --track-4-max-note-length <TRACK_4_MAX_NOTE_LEN>
          Cap the maximum note length for track 4 to this value in quarter notes
      --track-5-max-note-length <TRACK_5_MAX_NOTE_LEN>
          Cap the maximum note length for track 5 to this value in quarter notes
      --track-6-max-note-length <TRACK_6_MAX_NOTE_LEN>
          Cap the maximum note length for track 6 to this value in quarter notes
      --track-7-max-note-length <TRACK_7_MAX_NOTE_LEN>
          Cap the maximum note length for track 7 to this value in quarter notes
      --track-8-max-note-length <TRACK_8_MAX_NOTE_LEN>
          Cap the maximum note length for track 8 to this value in quarter notes
  -h, --help
          Print help
  -V, --version
          Print version
```

Or in other words, point the command at a `.m8s` file, and you'll get a multi-track Midi file in return. You should be able to drag these Midi files into your DAW.

## Possible features
That are not currently supported:
- Time signatures
- Tempo
- Table support
- Respect sequencer commands (other than GRV)
- Respect scales
- Instrument mode: output one track per instrument
- Configurable channels
- Instrument (program) changes
- CC support
