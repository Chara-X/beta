
#!/usr/bin/bash

cargo install crate
cargo init
cargo test -r --target triple --no-run -- --list --exact --show-output --test-threads 0 filter
cargo build -r --target triple
cargo doc --no-deps
cargo tree
