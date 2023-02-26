## rust-harfbuzz

[![Build Status](https://travis-ci.org/servo/rust-harfbuzz.svg)](https://travis-ci.org/servo/rust-harfbuzz)

[Documentation](https://docs.rs/harfbuzz-sys/)

Bindings to the Harfbuzz text shaping engine

### Building bindings from system library (under Windows)

#### Install needed libraries

```bash
vcpkg install --recurse glib fontconfig freetype cairo harfbuzz
```

#### Configure vcpkg

Configure environment variable `$env:RUSTFLAGS="-Ctarget-feature=+crt-static"`

#### Build

```bash
cargo build
```
