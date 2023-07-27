# Cassowary
Inventory controller

## Rust Toolchain
Requires a rust toolchain to build. This can be obtained at [rustup.rs](https://rustup.rs).
Specifically, the nightly toolchain is needed due to the use of the unstable `LazyCell` API.

```
<install rustup>
rustup toolchain install nightly
```

## Build
```
cargo build --release --workspace
```
## Install

There are two ways to do this
1) [Copy from the build directory](#1-copy-from-build)
2) [Using `cargo install`](#2-using-cargo-install)

### 1) copy from build
Copy the executable(s) from `target/release/` to wherever it needs to be
```
cargo build --release
cp target/release/cassowary.exe <path to install binary to>
```

### 2) using `cargo install`
Supply your installation directory with the `--root` option. The binary will be put a `bin` directory in this path.
```
cargo install --path . --root <installation path>
```

If you do not specify the `--root` option, the binary will be installed to `~/.cargo/bin`.
