# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — Unreleased

First published surface: the strata substrate (probe S0) and the
Ω↔stratum dictionary (probe S2) of the founding document
(`docs/ideation-edge-geometry.md`). Thatch is the geometry of the edge —
the infinitesimal boundary between something and emptiness — with zero
dependencies; arrows point away from Thatch.

### Added

- **Strata substrate (S0)** — boundary-stratum posets of Schubert
  varieties on Grassmannians:
  - `Partition` — codimension data with validation (`new`, `size`,
    `fits_in`, `le_componentwise` zero-padded comparison,
    `join_componentwise` λ∨μ added with S2).
  - `Grassmannian` — `Gr(k, k+m)` positions: `new(k, m)` validating
    constructor, `partitions()` (descending lex, k-part vectors),
    `contains`.
  - `strata(λ, g)` — the `StratumPoset` of Ω_λ: `boundary` strata with
    per-stratum dimension (k·m − |μ|) and codimension in the variety
    (|μ| − |λ|), plus closure `order` pairs. Conventions: codim(Ω_λ)=|λ|,
    Ω_λ = ⊔_{μ≥λ} X_μ, boundary = strict upset, short-form λ treated
    zero-padded.
- **Ω↔stratum dictionary (S2)** — the structured emptiness lattice
  mirrored dependency-free and adjudicated against the poset:
  - `OmegaValue` (`StructuralZero | GeometricZero | Positive |
    Underdetermined`) with `is_zero`.
  - `pairing(λ, μ)` — closure-poset position (`SameStratum | Stricter |
    Looser | Incomparable`); agrees exhaustively with `strata` boundaries.
  - `omega_pair(g, λ, μ)` — the compatibility edge: Ω_λ ∩ Ω_μ = Ω_{λ∨μ},
    decided by join-in-box. `GeometricZero` is unreachable for pairs
    (exhaustively verified on Gr(2,4)); `Underdetermined` is an epistemic
    slot, never returned.
  - `omega_composition(g, λ, μ)` — the composability edge: structural
    zero iff |λ| + |μ| > k·m (the Littlewood–Richardson budget).
  - **Headline**: compatibility and composability are independent edges —
    λ = μ = (2,1) on Gr(2,4) has nonempty meet yet overdraws the budget.
- **CI** — GitHub Actions matrix on `develop`/`main` (fmt, clippy both
  configs, tests default + all-features, `--no-default-features` build
  check, docs with `-D warnings`); nightly toolchain pinned via
  `rust-toolchain.toml`.

### Fixed

- Removed the false `#![cfg_attr(not(feature = "std"), no_std)]` promise —
  the crate uses `std::fmt` and `Vec` storage; `--no-default-features`
  now builds cleanly (and CI keeps it honest).

[0.1.0]: https://github.com/Industrial-Algebra/Thatch/releases/tag/v0.1.0
