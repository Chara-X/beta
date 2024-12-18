#!/usr/bin/bash

cargo install crate
cargo new path --lib
cargo test expr -- --ignored
cargo build -r
cargo doc
