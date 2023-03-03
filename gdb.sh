#!/usr/bin/env bash

project_root=$(dirname $(realpath $0))
pushd $project_root/openocd

aarch64-none-elf-gdb -q -nx -x .gdbinit test.elf

popd

