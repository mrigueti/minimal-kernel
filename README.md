# Minimal Kernel

A bare-metal x86_64 kernel written in Rust. This project demonstrates basic OS development concepts by booting a minimal kernel that displays a message and a simple ASCII image directly via VGA text mode.

## Requirements

To build and run this kernel in QEMU, you will need:

-   **Rust (nightly toolchain):** The project uses features only available on the nightly Rust release. Install it using `rustup toolchain install nightly`.
-   **`cargo-bootimage` tool:** This Cargo subcommand helps build the kernel and create a bootable disk image. Install it with `cargo install bootimage`.
-   **QEMU:** A processor emulator. Make sure it's installed and available in your system's PATH.

## Building and Running with QEMU

The `cargo-bootimage` tool simplifies the process of building the kernel and running it in QEMU.

1.  **Navigate to the project directory** in your terminal.
2.  **Run the following command:**

    ```bash
    cargo +nightly bootimage
    ```

This single command will:
-   Compile the kernel using the nightly Rust toolchain (`+nightly`).
-   Create a bootable disk image using `bootimage`.
-   Automatically launch the generated image in QEMU.

You should see QEMU start up and display the "Hello, New Kernel!" message along with an ASCII art owl in the VGA text mode window.

## Project Overview

-   **Bare Metal:** Runs directly on x86_64 hardware (or QEMU) without an underlying OS.
-   **`#![no_std]`:** Does not rely on the Rust standard library.
-   **Custom Entry Point:** Uses a `_start` function defined in `src/main.rs`.
-   **VGA Output:** Writes directly to VGA memory (address `0xb8000`) for screen output.
-   **Target Configuration:** Uses a custom target definition (`x86_64-minimal_os.json`) and Cargo configuration (`.cargo/config.toml`).

## License

This project is open source and available under the MIT License.
