#!/bin/bash
# Clean Script
cd "$(dirname "${BASH_SOURCE[0]}")"  # cd current directory
set -x
rm -rvf ./target/
