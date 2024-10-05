#!/bin/sh
cargo build 
./target/debug/rust-tracer > ./images/imagev2.ppm
xdg-open ./images/imagev2.ppm