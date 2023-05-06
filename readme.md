# midi-m8
A tool for turning Dirtywave M8 songs into Midi tracks. Available for CLI or as a standalone GUI app, a VST3 or a CLAP plugin for Windows and OSX.

[![Crates.io](https://img.shields.io/crates/v/midi-m8)](https://crates.io/crates/midi-m8)
[![CI](https://github.com/AlexCharlton/midi-m8/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexCharlton/midi-m8/actions/workflows/ci.yml)

![Plugin in action](https://raw.githubusercontent.com/AlexCharlton/midi-m8/master/plugin/MIDI-M8.gif)

## Installation
Precompiled 64 bit binaries for Linux, Windows and OS X can be found in the [Releases](https://github.com/AlexCharlton/midi-m8/releases/latest).

## Usage
```
Usage: midi-m8 [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Input (.m8s) file

Options:
  -o, --output <OUTPUT>
          Output file name [default: tracks.midi]
  -g, --global-transpose <GLOBAL_TRANSPOSE>
          How to map M8 note numbers to Midi Note numbers [default: 36]
  -t, --only-track <ONLY_TRACK_N>
          Only output track number (1-8)
  -s, --start-from <START_FROM>
          Start from this song position (hex: 00-FF)
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

### Examples
**Basic**
```
$ midi-m8 Songs/Demos/DEMO1.m8s
```
This will create a file `tracks.midi`.

**Choose output file name**
```
$ midi-m8 Songs/Demos/DEMO1.m8s -o output.mid
```
This will create a file `output.mid`.

**Limit note length**
```
$ midi-m8 -m 2 Songs/Demos/DEMO1.m8s
```
This caps the note length to 2 quarter notes.

**Single track**
```
$ midi-m8 --only-track 5 Songs/Demos/DEMO1.m8s
```
This will output only track 5 to `track-5.midi`.

**Starting position**
```
$ midi-m8 -s 02 Songs/Demos/DEMO1.m8s
```
This will render Midi starting from the position `02` in the song.


## Possible features
That are not currently supported:
- Time signatures
- Tempo
- Table support
- Respect sequencer commands (other than GRV, which is already supported)
- Instrument mode: output one track per instrument
- Configurable channels
- Instrument (program) changes
- CC support

## Compiling
To compile your own version, you'll first need to [install the Rust toolchain](https://rustup.rs/).

Then, the easiest way to wind up with midi-m8 is to `$ cargo-install midi-m8`.

Alternately you could:
1. `$ git clone https://github.com/AlexCharlton/midi-m8.git && cd midi-m8`
2. `cargo build --release`
You'll now have a binary in the `./target/release/` directory.

## Changelog
### v1.2
- V3 support
