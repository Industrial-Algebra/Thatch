# Introduction

**Thatch** is a small Rust library of combinatorial tools for Schubert
varieties on Grassmannians: stratum posets and intersection
classification, with zero dependencies.

## What It Does

1. **Strata substrate** — partitions as codimension data, Grassmannian
   positions, and `strata()`: the boundary-stratum poset of Ω_λ, with
   per-stratum dimensions and closure pairs.
2. **Intersection classification** — two different intersection
   questions, kept deliberately separate:
   - `omega_pair` — the **set-meet**: Ω_λ ∩ Ω_μ = Ω_{λ∨μ}, nonempty iff
     the componentwise join fits the box.
   - `omega_composition` — the **product budget**: the cohomological
     product survives iff |λ| + |μ| ≤ k·m.

The two can disagree — positions can have a nonempty meet while
overdrawing the product budget — and the crate is precise about which
question it is answering.

## Design Notes

- **Zero dependencies.** Everything is decided combinatorially from the
  componentwise order on box partitions — no limits, no cohomology
  machinery, no sheaves.
- **No panics.** Fallible constructors return `Result`; lookups are
  total. Validated inputs only.
- **TDD throughout.** Every fixture (stratum counts, the exhaustive
  pair classification, the budget boundaries) is pinned by
  regression tests.

## License

Apache-2.0. Commercial licensing available — contact Industrial Algebra.
