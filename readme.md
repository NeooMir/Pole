# Pole

A modern, high-performance educational IDE and virtual machine. Built as a fast, native Rust alternative to the legacy KuMir environment.

Designed for algorithmic training without the bloat, featuring a custom scripting engine, integrated robot simulation, and advanced keyboard-driven workflows.

## Key Features

* **Native & Fast:** Written entirely in Rust. Zero heavy web-framework overhead.
* **Custom VM & Language:** Implements a bespoke scripting language with its own lexer, parser, AST, and Virtual Machine (`src/lang/`).
* **Robot & Turtle Performers:** Built-in algorithmic robot execution environment and integrated turtle performer.
* **Marginalia & Real-time Debugging:** Live gutter values for variables, conditions, and statements during step-by-step and continuous execution.
* **Vim Mode:** Native Vim keybindings for mouse-free, high-speed editing (`src/ui/vim_mode.rs`).
* **IPC Plugins:** Extensible architecture using secure inter-process communication (`src/plugins/ipc.rs`).
* **Linux-First Packaging:** Primary Meson build system with automated packaging for Flatpak, DEB, and RPM (`packaging/`).

## Build from Source

### Primary Build System (Meson & Ninja)

Requirements: `meson` (>= 0.59), `ninja`, `rustc`, `cargo`, `gtk4-devel`, `libadwaita-devel`.

```bash
# Setup build directory
meson setup _build --buildtype=release

# Compile binary and resources
meson compile -C _build

# Run validation tests
meson test -C _build

# Run binary
./_build/src/pole

# Install to system (optional)
sudo meson install -C _build
```

### Direct Cargo Build

```bash
cargo build --release
cargo run --release
```

### Windows Build (Clang / LLVM)

The project is configured to always build and link for Windows targets using **Clang** and **LLD** (`.cargo/config.toml` & `packaging/cross-windows-clang.meson`).

```bash
# Cross-compile or native build for Windows via Cargo
cargo build --release --target x86_64-pc-windows-gnu

# Or run the automated packaging script (builds .exe & creates zip)
./packaging/build-windows.sh
```

## System Packages

Ready-to-use build scripts are available in the `packaging/` directory for building Flatpak, Debian/Ubuntu (.deb), Fedora/RHEL (.rpm), and Windows (.zip with .exe) packages.
