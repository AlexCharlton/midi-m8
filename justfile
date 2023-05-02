target-host := `rustc -vV | grep host: | cut -d ' ' -f 2`
target := env_var_or_default("CARGO_BUILD_TARGET", target-host)
target-os := if target =~ "-windows-" { "windows"
    } else if target =~ "darwin" { "macos"
    } else if target =~ "linux" { "linux"
    } else { "unknown" }
target-arch := if target =~ "x86_64" { "x64"
    } else if target =~ "i[56]86" { "x86"
    } else if target =~ "aarch64" { "arm64"
    } else if target =~ "armv7" { "arm32"
    } else { "unknown" }

output-ext := if target-os == "windows" { ".exe" } else { "" }
output-filename := "midi-m8" + output-ext

build:
    cargo build --release

build_plugin:
    just _build_plugin

[macos]
[windows]
_build_plugin:
    cargo xtask bundle -p midi-m8-plugin --release
    cd plugin && cargo bundle --release

[linux]
_build_plugin:
    @echo "Plugin not (yet) supported on Linux"

package-dir:
    rm -rf packages/prep
    mkdir -p packages/prep

package: build build_plugin package-dir
    just _package `./target/release/{{output-filename}} --version | awk '{print $2}'`

[linux]
_package version:
    cp target/release/{{output-filename}} packages/prep
    cd packages/prep && tar cv * | gzip -9 > "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLI.tgz"
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLI.tgz"


[macos]
_package version:
    cp target/release/{{output-filename}} packages/prep
    cd packages/prep && zip -r -9  "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLI.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLI.zip"

    rm -r packages/prep/*
    cp -r target/bundled/*.clap packages/prep
    cd packages/prep && zip -r -9  "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLAP.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLAP.zip"

    rm -r packages/prep/*
    cp -r target/bundled/*.vst3 packages/prep
    cd packages/prep && zip -r -9  "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-VST3.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-VST3.zip"

    rm -r packages/prep/*
    cp -r target/release/bundle/osx/*.app packages/prep
    cd packages/prep && zip -r -9  "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-STANDALONE.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-STANDALONE.zip"


[windows]
_package version:
    cp target/release/{{output-filename}} packages/prep
    cd packages/prep && 7z a -mx9 "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLI.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLI.zip"

    rm -r packages/prep/*
    cp -r target/bundled/*.clap packages/prep
    cd packages/prep && 7z a -mx9 "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLAP.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-CLAP.zip"

    rm -r packages/prep/*
    cp -r target/bundled/*.vst3 packages/prep
    cd packages/prep && 7z a -mx9 "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-VST3.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-VST3.zip"

    rm -r packages/prep/*
    cp -r target/release/bundle/msi/*.msi packages/prep
    cd packages/prep && 7z a -mx9 "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}-STANDALONE.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}-STANDALONE.zip"
