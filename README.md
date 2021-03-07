# FED

`fed` is a text editor using the [skia graphics library](https://skia.org/) and supporting the [language server protocol](https://microsoft.github.io/language-server-protocol/).

You can read about the project architecture in [ARCHITECTURE.md](./ARCHITECTURE.md).

## Building on Linux

Prerequisite: Have clang and python2 installed.

Do `cargo build --features skia-safe/x11` or `cargo build --features skia-safe/wayland`.
