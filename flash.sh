#!/bin/bash

set -e


if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "usage: $0 <path-to-binary.elf>" >&2
    exit 1
fi

if [ "$#" -lt 1 ]; then
    echo "$0: Expecting a .elf file" >&2
    exit 1
fi

elf_file="$1"
device="$2"

# Build project
rustup run nightly cargo build --release

# Flash into board
avrdude -q -patmega328p -carduino -P${device} -D "-Uflash:w:${elf_file}:e"

# Opens serial console
ravedude uno --open-console --baudrate 57600
