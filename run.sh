#!/bin/bash

set -ex

base_name=$1
ppm_file="images/$1.ppm"
png_file="images/$1.png"

RUSTFLAGS="-C opt-level=3" cargo run --release > "${ppm_file}"
convert "${ppm_file}" "${png_file}"