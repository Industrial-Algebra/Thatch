# Thatch — Agent Operating Map

> A small zero-dependency Rust crate: combinatorial tools for Schubert
> varieties on Grassmannians — stratum posets and intersection
> classification. Thatch depends on **nothing** in the IA ecosystem.

The public repo stays **matter-of-fact**: it documents what the crate
computes, not where it is going. Research direction, design rationale,
and the founding documents live in private storage (IA-documents); do not
surface them here without an explicit operator decision.

## Hard rules

- **Gitflow** (`/skill:ia-gitflow`): `develop`/`main` receive changes only
  via merged PRs; every release is followed by a `main → develop` backmerge
  **merge commit**. Every change lands via branch + PR.
- **Merge actor**: the **operator** merges every PR — feature, release, and
  backmerge alike. Agents author, verify, and hand off at the merge
  (ia-gitflow Rule 4, standing since 2026-09-16).
- **Dependency direction**: no IA-ecosystem dependencies in `[dependencies]`
  — ever. Dev-dependencies are fine.
- **License**: Apache-2.0 + two-line header on every `.rs` file.
- **TDD** (`/skill:ia-coding-standards`): failing test first; every fixture
  pinned by regression tests; no panics in library code.

## Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

CI (`.github/workflows/ci.yml`) mirrors this matrix on `develop`/`main`
plus a `--no-default-features` build check. Toolchain: nightly, pinned in
`rust-toolchain.toml`.

## Read-first routing

| Task | Read |
|---|---|
| What the crate computes | `src/strata.rs`, `src/dict.rs`, `book/src/` |
| Conventions | `/skill:ia-coding-standards` |
| Release mechanics | `/skill:ia-gitflow`, `/skill:ia-release-polish` |
| Research direction (private) | IA-documents `Thatch/` |
