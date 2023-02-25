# Midi m8
A command line tool for turning Dirtywave M8 songs into Midi tracks.

## Installation
Precompiled 64 bit binaries for Linux and Windows can be found in the [Releases](https://github.com/AlexCharlton/midi-m8/releases). Download them and run them from the command line.

To compile your own version:

1. [install the Rust toolchain](https://rustup.rs/)
2. `$ git clone https://github.com/AlexCharlton/midi-m8.git`
3. `$ cd midi-m8; cargo build -r`

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

## Possible features
That are not currently supported:
- Time signatures
- Tempo
- Table support
- Respect sequencer commands (other than GRV)
- Respect scales
- Configurable channels
- Instrument (program) changes
- CC support
