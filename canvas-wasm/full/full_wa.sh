#!/bin/bash
# Build Script
cd "$(dirname "${BASH_SOURCE[0]}")"  # cd current directory
cd ..                                # cd parent  directory
set -x

emcc -Os full/full_wa.c      -o full/full_wa.c.js  # generates: full_wa.c.wasm
wasm2wat full/full_wa.c.wasm -o full/full_wa.c.wat