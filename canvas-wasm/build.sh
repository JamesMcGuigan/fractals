#!/bin/bash
cd "$(dirname "${BASH_SOURCE[0]}")"  # cd current directory
set -x

# Build Script
./minimal/minimal_wa.sh
./full/full_wa.sh