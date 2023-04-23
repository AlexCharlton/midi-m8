## Running

Standalone
```
$ cargo xtask bundle -p midi-m8-plugin && NIH_LOG=./plugin.log ./target/debug/midi-m8-plugin[.exe]
$ cargo xtask bundle -p midi-m8-plugin
```

Plugin
```
$ cargo xtask bundle -p midi-m8-plugin && cp ./target/bundled/midi-m8-plugin.clap /c/Program\ Files/Common\ Files/CLAP/
```
