### Install target architecture:
RP2040 uses a Cortex-M0+ processor
```rust
rustup target add thumbv6m-none-eabi
```

### To compile:
```rust
cargo build --target thumbv6m-none-eabi
```
For flashing the RP2040, convert the compiled ELF file into a UF2 format, which the RP2040 bootloader can directly accept. Use elf2uf2-rs. After generating the UF2 file, drag and drop it onto the RPI-RP2 mass storage device that appears when the RP2040 is in bootloader mode.
