#!/bin/bash
set -e
dir=$(cd -P -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)
LLVM_PROFILE_FILE="$dir/%p-%m.profraw" RUSTFLAGS="-Zinstrument-coverage" cargo +nightly test --manifest-path="$dir/Cargo.toml"
grcov . -s "$dir" --binary-path "$dir/target/debug/" -t html --branch --ignore-not-existing -o "$dir/target/debug/coverage/"
rm "$dir/"*-*.profraw
open "$dir/target/debug/coverage/index.html"
