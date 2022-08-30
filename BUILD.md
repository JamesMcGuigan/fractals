# Build + Install + Run

Install
```bash
rustup install nightly  
rustup default nightly

cargo install --locked trunk
cargo install wasm-bindgen-cli
cargo install cargo-expand
cargo install cargo-generate cargo-web
cargo install grass

# BUGFIX: thread 'main' panicked at 'unknown name section chunk type: 7' | https://github.com/koute/cargo-web/issues/251
cargo install --git https://github.com/Badel2/cargo-web cargo-web
cargo install --path .
```

Validate
```bash
RUSTFLAGS="-Z macro-backtrace" cargo check
RUSTFLAGS="-Z macro-backtrace" cargo clippy
cargo expand --lib | cat 
cargo expand --bin yewcounter | cat
```

Build
```bash
## Use Trunk - https://yew.rs/docs/getting-started/project-setup/using-trunk
cargo install --path .
cargo update
cargo udeps
trunk clean
trunk serve --open  # Works - requires index.html
trunk build --release
http-server dist/               # Javascript webserver
see start -b 8080 -p dist/      # Rust webserver
```


Cargo Expand
```
cargo expand --lib
cargo expand --bin fractals
cargo expand --bin julia_image
```

CLI
```
cd fractals_cli;
cargo build; RUST_BACKTRACE=1 ./target/debug/julia_image

cargo build --release; 
./target/release/julia_image -c 0.5+0.5i -r 1 -o fractal.jpg
```
