# Feature Flags

Thatch follows the ecosystem doctrine: **features are additive only** —
`#[cfg(feature)]` adds capability, never removes API.

| Feature | Default | Effect |
|---|---|---|
| `std` | ✓ | Conventional ecosystem flag; currently inert — the crate uses `std` unconditionally |

## The `std` Story

The scaffold once declared `#![cfg_attr(not(feature = "std"), no_std)]` —
a promise the implementation did not keep (the error types use
`std::fmt`, the storage is `Vec`). The promise was removed rather than
falsely kept, and CI asserts that `--no-default-features` builds cleanly.
If a genuine `no_std`/`alloc` story is ever needed, it will arrive as an
additive change with its own gate — not as a resurrection of the false
attribute.

## Verification

CI (`.github/workflows/ci.yml`) runs, on every PR and push to
`develop`/`main`:

- `cargo fmt --all --check`
- `cargo clippy --all-targets` with `-D warnings`, in both the default
  and all-features configurations
- `cargo test` in the default and all-features configurations
- `cargo check --no-default-features` (the honest-build assertion)
- `cargo doc --all-features --no-deps` with `RUSTDOCFLAGS=-D warnings`
