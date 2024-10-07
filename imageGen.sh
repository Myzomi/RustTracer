#!/bin/sh
cargo build 
./target/debug/rust-tracer > ./images/imagev4.ppm
xdg-open ./images/imagev4.ppm