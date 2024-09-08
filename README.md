# Rust on Microbit

This repo contains some experiments to run Rust on the [BBC
Micro:bit](https://microbit.org/). Much of the setup is based on the guide
"Embedded Rust setup explained" (https://www.youtube.com/watch?v=TOAynddiu5M).
For other resources see below.

![micro:bit](/micro_bit.jpg "micro:bit")


## Documentation

* https://docs.rust-embedded.org/book/
* https://docs.rust-embedded.org/discovery/microbit/

## Hardware

https://tech.microbit.org/hardware/

## Host Toolchain

### Targets

Use `rustup target add <target-name>` to add support for targets in the build
tool chain. Check the `https://doc.rust-lang.org/nightly/rustc/platform-support.html` page for
details.

* Microbit: thumbv7em-none-eabihf

### LLVM Tools and Binutils

Install llvm-tools and cargo-binutils

```
rustup component add llvm-tools
cargo install cargo-binutils
```

Cargo binutils is a wrapper around llvm-tools for better ergonomics.

### Cargo Embed

Cargo embed is now part of probe-rs and is installed using

```
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

To list attached probes, use the command `probe-rs list`

See documentation at `https://probe.rs/docs/tools/cargo-embed/` and `https://probe.rs/`

### Commands

```
cargo size -- -Ax
```

### RTT

Real-Time Transfer (RTT) I/O protocol. Implements input and output via a debug
probe using in-memory ring buffers and polling.

Target side implementation of the protocol: `https://crates.io/crates/rtt-target`

The implementation requires a platform specific critical section
implementation. For cortex-m, this can be accomplished using the cortex-m crate
optional feature `critical-section-single-core`. Add this using:

`cargo add cortex-m --features critical-section-single-core`

See `https://docs.rs/cortex-m/0.7.7/cortex_m/#critical-section-single-core` for
details.

