# Interactive Fractal Explorer

## Features
- Browser Canvas + CLI rendering
- Implemented in Rust Yew

## Hosted
- Vercel: https://fractals.jamesmcguigan.com/
- ICP:    https://nwb7c-dqaaa-aaaak-qatfa-cai.ic0.app/

## Documentation
- [BUILD.md](BUILD.md) - rustup + cargo build + CLI
- [DEPLOY.md](DEPLOY.md) - Vercel + ICP hosting config
- [CARGO_PROFILING.md](CARGO_PROFILING.md) - Notes on optimizing cargo build
- [LINKS.md](LINKS.md) - Rust Cargo library ecosystem

## CLI
![](./fractals_cli/fractal.png)
```
### Build
cargo clean
cargo build --release

### Run
cargo run   --release --bin julia_image -q     # builds, runs, outputs: fractal.zoom.png
./target/release/julia_image -c 0.5+0.5i -r 1 -o fractal.zoom.png
ls -la fractal.*
```
