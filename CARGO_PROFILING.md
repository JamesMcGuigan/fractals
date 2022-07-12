# Cargo Build Profiling
- [YouTube: Making a Rust crate compile faster](https://www.youtube.com/watch?v=pMiqRM5ooNw&ab_channel=JonGjengset)
- https://fasterthanli.me/articles/why-is-my-rust-build-so-slow+

## Config
Custom build config
- .cargo/config.toml 
- ~/.cargo/config.toml

```
# .cargo/config.toml

[build]
# $ cargo clean; time cargo build --timings
# jobs = 8  # (default)  350.0s (5m 50.0s) | 407.4s (6m 47.4s) | 283.9s (4m 43.9s)
# jobs = 8  # --release  153.0s (2m 33.0s) | 158.9s (2m 38.9s)
# jobs = 1  # 1456.7s (24m 16.7s)

# mold is an alterative faster linker - https://github.com/rui314/mold
# doesn't really speed up builds significantly
# $ pacman -S mold
# $ readelf -p .comment target/release/fractals
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=--ld-path=/usr/bin/mold"]
```

Cargo.toml
```
[lib]
proc-macro = true
crate-type = ["cdylib"]

[patch.crates-io]
proc-macro2 = { git = "https://github.com/dtolnay/watt" }
```
- https://docs.rs/watt/latest/watt/
- watt precompiles macros into sandboxed WASM
- `$ cargo build --release --target wasm32-unknown-unknown`

## Tools

cargo-bloat - View symbol size per library
```
$ cargo install cargo-bloat
$ cargo bloat
```

llvm-lines - Linecount of LLVM IR generated for generic functions
```
$ cargo install cargo-llvm-lines
$ cargo llvm-lines | head
```

cargo-udeps - Linecount of LLVM IR generated for generic functions
```
$ cargo install cargo-udeps
$ cargo udeps
```


Cargo Build Profiler
- https://ui.perfetto.dev
```
$ cargo install flamegraph 
$ cargo install --git https://github.com/rust-lang/measureme --branch stable crox
$ cargo install --git https://github.com/rust-lang/measureme --branch stable summarize
$ cargo rustc --release --lib -- -Z self-profile -Z self-profile-events=default,args

$ summarize summarize fractals-*.mm_profdata 
$ crox fractals-*.mm_profdata  # generates chrome_profiler.json
     open in Chrome DevTools -> Performance
     open in https://ui.perfetto.dev
```