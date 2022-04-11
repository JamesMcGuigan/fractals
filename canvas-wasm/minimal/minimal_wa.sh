#!/bin/bash
# Build Script
cd "$(dirname "${BASH_SOURCE[0]}")"  # cd current directory
cd ..                                # cd parent  directory
set -x

emcc -Os minimal/minimal_wa.c      -o minimal/minimal_wa.c.js  # generates: minimal_wa.wasm
wasm2wat minimal/minimal_wa.c.wasm -o minimal/minimal_wa.c.wat
