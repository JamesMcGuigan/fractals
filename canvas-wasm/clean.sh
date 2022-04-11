#!/bin/bash
cd "$(dirname "${BASH_SOURCE[0]}")"  # cd current directory
set -x

# Clean Script
rm -vf */*.c.js
rm -vf */*.c.wasm
rm -vf */*.c.wat