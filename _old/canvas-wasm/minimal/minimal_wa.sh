#!/bin/bash
# Build Script
cd "$(dirname "${BASH_SOURCE[0]}")"  # cd current directory
cd ..                                # cd parent  directory
set -x

emcc     -O minimal/minimal_wa.c      -o minimal/minimal_wa.c.js -s EXPORTED_FUNCTIONS=_render
wasm2wat    minimal/minimal_wa.c.wasm -o minimal/minimal_wa.c.wat
js-beautify minimal/minimal_wa.c.js | sponge minimal/minimal_wa.c.js