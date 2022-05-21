#!/bin/bash
# Build Script
cd "$(dirname "${BASH_SOURCE[0]}")"  # cd current directory
cd ..                                # cd parent  directory
set -x

emcc     -O full/full_wa.c      -o full/full_wa.c.js -s EXPORTED_FUNCTIONS=_render
wasm2wat    full/full_wa.c.wasm -o full/full_wa.c.wat
js-beautify full/full_wa.c.js | sponge full/full_wa.c.js