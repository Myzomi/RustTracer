#!/bin/sh
cargo build --release
./target/release/rust-tracer > ./images/imagev2.ppm
xdg-open ./images/imagev2.ppm