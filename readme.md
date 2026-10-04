# NeoMir

A modern, high-performance educational IDE and virtual machine. Built as a fast, native Rust alternative to the legacy KuMir environment.

Designed for algorithmic training without the bloat, featuring a custom scripting engine, integrated robot simulation, and advanced keyboard-driven workflows.

## Key Features

* **Native & Fast:** Written entirely in Rust. Zero heavy web-framework overhead.
* **Custom VM & Language:** Implements a bespoke scripting language with its own lexer, parser, AST, and Virtual Machine (`src/lang/`).
* **Robot Simulator:** Built-in algorithmic robot execution environment (`src/robot.rs`)[cite: 2].
* **Vim Mode:** Native Vim keybindings for mouse-free, high-speed editing (`src/ui/vim_mode.rs`)[cite: 2].
* **IPC Plugins:** Extensible architecture using secure inter-process communication (`src/plugins/ipc.rs`)[cite: 2].
* **Linux-First Packaging:** Automated build scripts for Flatpak, DEB, and RPM (`packaging/`)[cite: 2].

## Build from Source

Requirements: standard Rust toolchain (cargo, rustc).

```
git clone [https://github.com/your-username/NeoMir.git](https://github.com/your-username/NeoMir.git)
cd NeoMir
cargo build --release
cargo run --release
```
System Packages

Ready-to-use build scripts are available in the packaging/ directory for building Flatpak, Debian/Ubuntu (.deb), and Fedora/RHEL (.rpm) packages.