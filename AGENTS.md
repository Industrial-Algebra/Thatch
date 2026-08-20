# Thatch — Agent Operating Map

> **Geometry of the edge** — the infinitesimal boundary between something
> and emptiness. Foundation crate in the IA "Rich Toolbox": Thatch depends
> on **nothing** in the IA ecosystem; everyone may draw from Thatch.

Founded from `docs/ideation-edge-geometry.md` (the founding document — read
it first). The mathematics is at the ideation stage: **no implementation
code before a probe survives**, and no public API before the first surviving
probe produces one. TDD per `/skill:ia-coding-standards` applies from the
first line of real code.

## Hard rules

- **Gitflow** (`/skill:ia-gitflow`): `develop`/`main` receive changes only
  via merged PRs; every release is followed by a `main → develop` backmerge
  **merge commit**. Until a remote exists, the founding commit on `main` is
  the bootstrap; branch discipline starts with the first feature.
- **Dependency direction**: no IA-ecosystem dependencies in `[dependencies]`
  without an explicit decision recorded against the founding doc's §5
  principles. Dev-dependencies for probes are fine.
- **License**: Apache-2.0 + two-line header on every `.rs` file.

## Verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo doc --no-deps
```

## Read-first routing

| Task | Read |
|---|---|
| Why this crate exists; the math it draws on | `docs/ideation-edge-geometry.md` |
| What to build next | probes S0–S3 (founding doc §7) |
| Conventions | `/skill:ia-coding-standards` |
| Release mechanics (someday) | `/skill:ia-gitflow`, `/skill:ia-release-polish` |
