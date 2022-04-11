# Canvas WASM
DOCS
- https://compile.fi/canvas-filled-three-ways-js-webassembly-and-webgl/
- https://developer.mozilla.org/en-US/docs/WebAssembly/Loading_and_running


## Install EMSDK
```
cd git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
echo 'source "/usr/local/src/emsdk/emsdk_env.sh"' >> $HOME/.bash_profile
emsdk --help
```

## Install wasm2wat
```
sudo pacman -S wabt
```

## Compile
```
emcc -Os minimal/minimal_wa.c -o minimal/minimal_wa.min.js  # generates: minimal_wa.wasm 
emcc -Os full/full_wa.c       -o full/full_wa.min.js        # generates: full_wa.wasm

wasm2wat minimal/minimal_wa.c.wasm -o minimal/minimal_wa.c.wat
wasm2wat full/full_wa.c.wasm       -o minimal/full_wa.c.wat
```