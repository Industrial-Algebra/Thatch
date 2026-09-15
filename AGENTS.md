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
  **merge commit**. The bootstrap is closed (founding commit `54b8611` is
  pushed to `origin/main`); every change lands via branch + PR. Forgejo
  mirror pending (`/skill:ia-forgejo-mirror`).
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

CI (`.github/workflows/ci.yml`) mirrors this matrix on `develop`/`main`
plus a `--no-default-features` build check. Toolchain: nightly, pinned in
`rust-toolchain.toml`.

## Read-first routing

| Task | Read |
|---|---|
| Why this crate exists; the math it draws on | `docs/ideation-edge-geometry.md` |
| The founding dive (verified cross-refs; S4; four-sides reading) | IA-documents `RESEARCH_REPORTS/RABBIT_HOLE_2026-08-20_Thatch.md` |
| What to build next | probes S0–S4 (founding doc §7) |
| Conventions | `/skill:ia-coding-standards` |
| Release mechanics (someday) | `/skill:ia-gitflow`, `/skill:ia-release-polish` |
