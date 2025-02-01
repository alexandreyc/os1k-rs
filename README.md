# os1k-rs

An implementation of [Operating System in 1,000 Lines](https://operating-system-in-1000-lines.vercel.app/en/)
in Rust.

## Development environment on macOS

With Homebrew install the following packages:

```shell
brew install qemu llvm lld
```

From the root of this repository, download the OpenSBI firmware:

```shell
curl -LO https://github.com/qemu/qemu/raw/v9.2.0/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin
```

You might also need to install an additional target triple for your Rust compiler:

```shell
rustup target add riscv32i-unknown-none-elf
```

## Notes

### Chapter 4

We need to figure out how to use the Rust compiler to buid a freestanding binary
with the correct memory layout and that does not link the standard library.

The article [A Freestanding Rust Binary](https://os.phil-opp.com/freestanding-rust-binary/)
by Philipp Oppermann is a must read on this topic.

In summary, we have to:

- Configure the compiler and linker in [`.cargo/config.toml`](.cargo/config.toml)
  to produce binary for our target architecture and with the correct memory layout.
- Use `#![no_std]` and `#![no_main]` to instruct the Rust compiler to build a
  freestanding binary.
- Define a `#[panic_handler]` and disable stack unwinding during panic in
  [`Cargo.toml`](./Cargo.toml).
