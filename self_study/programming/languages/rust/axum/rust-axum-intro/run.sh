#!/bin/sh

# https://crates.io/crates/cargo-watch

# Cargo Watch watches over your project's source for changes, and runs Cargo commands when they occur.
if ! command -v cargo-watch &> /dev/null; then
    cargo install cargo-watch || exit 1
    sleep 1
fi

echo "1. Start server (hot reload on changes inside ./src)"
echo "2. Run tests (hot reload on changes inside ./tests)"

printf "\nChose number: "
read number

if [[ $number -eq "1" ]]; then
    cargo-watch -q -c -w src/ -x run
elif [[ $number -eq "2" ]]; then
    cargo-watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
fi
