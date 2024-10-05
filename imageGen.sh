#!/bin/sh
cargo build 
./target/debug/rust-tracer > ./images/imagev1.ppm
xdg-open ./images/imagev1.ppm