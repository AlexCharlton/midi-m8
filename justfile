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

package-dir:
    rm -rf packages/prep
    mkdir -p packages/prep

package: build package-dir
    just _package `./target/release/{{output-filename}} --version | awk '{print $2}'`

[linux]
_package version:
    cp target/release/{{output-filename}} packages/prep
    cd packages/prep && tar cv * | gzip -9 > "../midi-m8-{{version}}-{{target-os}}_{{target-arch}}.tgz"
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}.tgz"

[macos]
_package version:
    cp target/release/{{output-filename}} packages/prep
    cd packages/prep && zip -r -9  "../midi-m8-{{version}}{{target-os}}_{{target-arch}}.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}.zip"

[windows]
_package version:
    cp target/release/{{output-filename}} packages/prep
    cd packages/prep && 7z a -mx9 "../midi-m8-{{version}}{{target-os}}_{{target-arch}}.zip" *
    @echo "Created ./packages/midi-m8-{{version}}-{{target-os}}_{{target-arch}}.zip"
