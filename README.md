# esp-storage (WORK IN PROGRESS)

This implements `embedded-storage` traits to access ESP32 flash.

The implementation uses ROM functions. 
(In future not only ROM functions in order to make it work on ESP32-S2 and ESP32)

## Work in progress

This is work in progress! Don't use it!

Currently it's not supported to read/write anything that is not aligned to sector boundaries.
Writing something that is smaller than a sector currently will erase the remaining part of that sector.
Buffers needs to be sized as a multiple of four bytes.

## Run examples

`cargo "+esp" run --example demo --features esp32 --target xtensa-esp32-none-elf --release`

`cargo "+esp" run --example demo --features esp32s2 --target xtensa-esp32s2-none-elf`

`cargo "+esp" run --example demo --features esp32s3 --target xtensa-esp32s3-none-elf`

`cargo "+nightly" run --example demo --features esp32c3 --target riscv32imc-unknown-none-elf`

## Current Status

### ESP32C3
Seems to work

### ESP32
Every write operation garbages the first four bytes. Only works when compiled with `release` profile.

### ESP32S2
Doesn't work

### ESP32S3
Seems to work
