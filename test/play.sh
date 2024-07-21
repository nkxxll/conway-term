#!/usr/bin/env bash

cargo run -- -r 50
cargo run -- -r 50 -s
cargo run -- -s
echo "endless ..."
sleep 2
cargo run
