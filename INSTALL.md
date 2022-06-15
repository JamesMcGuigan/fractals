# Installation and Setup

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
cargo expand --lib | cat 
cargo expand --bin yewcounter | cat 
```

Build
```bash
## Use Trunk - https://yew.rs/docs/getting-started/project-setup/using-trunk
trunk clean
trunk serve --open  # Works - requires index.html
trunk build --release
http-server dist/               # Javascript webserver
see start -b 8080 -p dist/      # Rust webserver
```
