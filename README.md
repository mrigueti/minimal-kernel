# Minimal Kernel

A bare-metal x86_64 kernel written in Rust that demonstrates basic OS development concepts.

## Overview

This project is a minimalist operating system kernel that:
- Runs directly on x86_64 hardware without an underlying OS
- Uses Rust's strong type system and memory safety features
- Displays a simple message on the screen using VGA text mode

## Features

- **Bare Metal:** Runs on bare hardware with no underlying operating system
- **No Standard Library:** Uses `#![no_std]` to avoid dependencies on the Rust standard library
- **Custom Entry Point:** Implements a custom `_start` function as the entry point
- **VGA Text Output:** Writes directly to VGA memory to display text on screen
- **Panic Handler:** Includes a simple panic handler as required for `no_std` environments

## Requirements

- Rust (nightly)
- `cargo-bootimage` tool
- QEMU or other virtualization software for testing
- Optional: real hardware for physical testing

## Building

To build the kernel:

```bash
cargo +nightly bootimage
```

This will compile the kernel and create a bootable disk image.

## Running

### Using QEMU

The simplest way to run the kernel is with QEMU:

```bash
cargo +nightly bootimage
```

This will automatically launch QEMU with the appropriate parameters.

### On Real Hardware

1. Build the bootable image with `cargo +nightly bootimage`
2. Write the generated `.bin` file to a USB drive using a tool like Rufus or Etcher
3. Boot a computer from this USB drive

### Using VirtualBox

See the documentation for instructions on creating a VMDK that points to the bootable image.

## Project Structure

- `src/main.rs`: Contains the kernel entry point and VGA text mode code
- `x86_64-minimal_os.json`: Custom target specification for the x86_64 architecture
- `.cargo/config.toml`: Cargo configuration including build settings

## How It Works

The kernel starts at the `_start` function, which calls `print_message()` to write "Hello, new kernel!" to the screen using VGA memory at address 0xb8000. It then enters an infinite loop.

## Learning Resources

This project draws inspiration from resources like:
- [Writing an OS in Rust](https://os.phil-opp.com/) by Philipp Oppermann
- [OSDev Wiki](https://wiki.osdev.org/)

## License

This project is open source and available under the MIT License.
