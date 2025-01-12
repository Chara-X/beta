#!/usr/bin/bash

cargo install crate
cargo init
cargo test
cargo build -r
cargo doc
