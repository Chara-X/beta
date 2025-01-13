#!/usr/bin/bash

cargo install crate
cargo init
cargo test -- --show-output --test-threads 0
cargo build -r
cargo doc
