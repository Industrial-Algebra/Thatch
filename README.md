# Thatch

Geometry of the edge — the infinitesimal boundary between something and
emptiness.

> Named for Edward Teach/Thatch — Blackbeard. The pirate's domain was the
> edge: the shoreline between land and open sea, the map's boundary where
> the known meets the unmarked. This crate studies that boundary as
> mathematics.

**The thesis:** the boundary is itself a geometric object. It has strata,
it carries structure, and it composes when two somethings share an edge.
Karpal types the *kinds* of emptiness (Ω); Schubert computes the
*dynamics* of crossing (wall-crossing); Thatch studies the *interface
itself*. The founding document is
[`docs/ideation-edge-geometry.md`](docs/ideation-edge-geometry.md).

## Status — probes S0, S2, S4 landed

- **Strata substrate (S0)** — boundary-stratum posets of Schubert
  varieties on Grassmannians: partitions as codimension data, closure
  orders, per-stratum dimensions.
- **Ω↔stratum dictionary (S2)** — the structured-emptiness lattice
  mirrored dependency-free, mapped onto the poset. Findings: `GeometricZero`
  is unreachable for pairs (the set-meet is always again a Schubert
  variety), and **compatibility ≠ composability** — λ=μ=(2,1) on Gr(2,4)
  has a nonempty meet yet overdraws the LR budget (|λ|+|μ| = 6 > k·m = 4).
- **Heterogeneous walls (S4)** — measured in Schubert (PR #57): the
  trust×time product of walls is *additive* under the current engine,
  baseline pinned by regression tests.

Zero dependencies. TDD throughout — every fixture above is
regression-tested.

## Quick start

```rust
use thatch::{strata, Grassmannian, Partition};

let g = Grassmannian::new(2, 2)?;              // Gr(2,4)
let lambda = Partition::new(vec![2, 1])?;      // Ω_(2,1) is a curve
let poset = strata(&lambda, &g)?;
assert_eq!(poset.boundary.len(), 1);           // ...whose boundary is a point
```

Two intersection questions, two answers:

```rust
use thatch::{omega_composition, omega_pair, Grassmannian, OmegaValue, Partition};

let g = Grassmannian::new(2, 2)?;
let lambda = Partition::new(vec![2, 1])?;
assert_eq!(omega_pair(&g, &lambda, &lambda)?, OmegaValue::Positive);          // compatible
assert_eq!(omega_composition(&g, &lambda, &lambda)?, OmegaValue::StructuralZero); // …yet incomposable
```

## Documentation

[![Docs](https://img.shields.io/badge/docs-thatch.industrial--algebra.com-blue)](https://thatch.industrial-algebra.com)

The book covers the thesis, the mathematics, the API, and the probe
program: **https://thatch.industrial-algebra.com**

## Placement

Thatch is a foundation crate in the Industrial Algebra "Rich Toolbox": it
depends on nothing in the IA ecosystem; Schubert, Karpal, Amari, and
Minuet may all draw from it.

## License

Apache-2.0. Commercial licensing available — contact Industrial Algebra.
