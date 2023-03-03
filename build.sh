#!/usr/bin/env bash

project_root=$(dirname $(realpath $0))
pushd $project_root
cargo build -p test && cp ./target/aarch64-unknown-none/debug/test ./openocd/test.elf
popd

