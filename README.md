## rust-harfbuzz

[![Build Status](https://travis-ci.org/servo/rust-harfbuzz.svg)](https://travis-ci.org/servo/rust-harfbuzz)

[Documentation](https://docs.rs/harfbuzz-sys/)

Bindings to the Harfbuzz text shaping engine

### Building bindings from system library (under Windows)

#### Install harfbuzz statically linked library

```bash
vcpkg install harfbuzz --triplet x64-windows-static
```

#### Configure vcpkg for cargo

Set environment variable `RUSTFLAGS="-Ctarget-feature=+crt-static"`

#### Build

```bash
cargo build
```
