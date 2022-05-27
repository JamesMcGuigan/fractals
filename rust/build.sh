#!/bin/bash
# Build Script
cd "$(dirname "${BASH_SOURCE[0]}")"  # cd current directory
set -x

# --target wasm32-unknown-unknown implied by .cargo/config.toml | breaks: cargo test
cargo build --target wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

# wasm-bindgen ./target/wasm32-unknown-unknown/debug/canvas_wasm.wasm   --out-dir ./target/wasm32-unknown-unknown/debug/wasm-bindgen/
# wasm-bindgen ./target/wasm32-unknown-unknown/release/canvas_wasm.wasm --out-dir ./target/wasm32-unknown-unknown/release/wasm-bindgen/

wasm-pack build --release --target web        -d target/wasm-pack/web/
# wasm-pack build --release --target no-modules -d target/wasm-pack/no-modules/
# wasm-pack build --release --target nodejs     -d target/wasm-pack/nodejs/

find ./ -name '*.wasm'   | xargs -t -L1 -I{} wasm2wat {} -o {}.wat
find ./target/*/*/*.wasm | parallel -v -k -n 1 'wasm-bindgen {1} --out-dir {1//}/bindgen/'