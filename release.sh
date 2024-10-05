#!/bin/sh
cargo build --release
./target/release/rust-tracer > ./images/imagev1.ppm
xdg-open ./images/imagev1.ppm