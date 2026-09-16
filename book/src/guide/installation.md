# Installation

## Requirements

- Rust 1.75+ (the toolchain is pinned to nightly in-repo for development;
  the crate itself builds on stable)

Thatch has **zero runtime dependencies** — `cargo add thatch` brings
nothing else in.

## Adding to a Project

```toml
[dependencies]
thatch = "0.1"
```

Or:

```bash
cargo add thatch
```

## Building from Source

```bash
git clone https://github.com/Industrial-Algebra/Thatch
cd Thatch
cargo build          # default features
cargo test           # 40 tests: strata fixtures + dictionary theorems
cargo doc --open     # full API documentation
```

## Feature Notes

The crate carries a single conventional feature, `std` (default on). See
[feature flags](./feature-flags.md).
