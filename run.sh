#!/bin/bash

set -ex

base_name=$1
ppm_file="images/$1.ppm"
png_file="images/$1.png"

cargo run > "${ppm_file}"
convert "${ppm_file}" "${png_file}"