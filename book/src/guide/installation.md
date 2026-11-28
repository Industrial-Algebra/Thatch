# Installation

## Requirements

- Rust 1.75+ (the toolchain is pinned to nightly in-repo for development;
  the crate itself builds on stable)

`thatch` depends only on `thatch-core`, which has no dependencies — `cargo add thatch` brings
nothing else in.

## Adding to a Project

```toml
[dependencies]
thatch = "0.2"
```

Or:

```bash
cargo add thatch
```

## Building from Source

```bash
git clone https://github.com/Industrial-Algebra/Thatch
cd Thatch
cargo build --workspace   # default features, both crates
cargo test --workspace   # strata fixtures, dictionary theorems, workspace-floor tests
cargo doc --workspace --open   # full API documentation, both crates
```

## Feature Notes

The crate carries a single conventional feature, `std` (default on). See
[feature flags](./feature-flags.md).
