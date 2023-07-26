#!/usr/bin/env bash

### Build
cargo clean
cargo build --release

### Run
cargo run   --release --bin julia_image -q     # builds, runs, outputs: fractal.zoom.png
./target/release/julia_image -c 0.5+0.5i -r 1 -o fractal.zoom.png
ls -la fractal.*
