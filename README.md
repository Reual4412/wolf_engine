# Wolf Engine

[![CI](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexiWolf/wolf_engine/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/l/wolf_engine)](https://github.com/AlexiWolf/wolf_engine#license)
[![Crates.io](https://img.shields.io/crates/v/wolf_engine)](https://crates.io/crates/wolf_engine)

A simple, flexible, and easy-to-use game framework written in Rust.

- TODO: Write a short description of the engine.  Answer what WE is and why it is built.

## Features

- TODO: Features.

## Platform Support 

Excellent cross-platform support is one of the main goals of Wolf Engine.  The entirety of the engine, with
`--all-features` enabled, should work on Desktop (Windows, Linux, MacOS), Mobile (Android, iOS), and WASM. Failure to 
build / run on these platforms is considered a bug.  Please create a bug report if you run into any problems.

### The Core Module 

The core module is intended to be a highly-portable subset of wolf engine enabling wider platform support, FFI, and 
support for no-std platforms.  The core module should theoretically run on any platform Rust itself can run on.  
However, for no-std platforms, you will very likely need to provide your own no-std-compatible `Context` data, and 
`EventLoop` implementation.

# Getting Started

### Installation

Start by adding Wolf Engine as a dependency in your `Cargo.toml` file.

```TOML
[dependencies]
wolf_engine = "*"
```

TODO: Provide a complete installation example.

### Usage

[The documentation](https://docs.rs/wolf_engine/latest/wolf_engine/) provides an overview of the engine, and its APIs.
[The examples folder](https://github.com/AlexiWolf/wolf_engine/tree/main/examples) offer practical, and more advanced 
usage examples.


# License

Wolf Engine is licensed under either:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

At your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as 
defined in the Apache-2.0 license, shall be dual licensed as above, without additional terms or conditions.

