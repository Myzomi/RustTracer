#!/bin/sh
cargo build --release
./target/release/rust-tracer > ./images/imagev4.ppm
xdg-open ./images/imagev4.ppm