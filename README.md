# Thatch

Combinatorial tools for Schubert varieties on Grassmannians: stratum
posets and intersection classification.

## What It Provides

- **Strata substrate** — `Partition` (validated codimension data),
  `Grassmannian` positions, and `strata()`: the boundary-stratum poset of
  Ω_λ with per-stratum dimensions and closure pairs.
- **Intersection classification** — `omega_pair` (does the set-meet
  Ω_λ ∩ Ω_μ = Ω_{λ∨μ} exist?) and `omega_composition` (does the product
  budget |λ| + |μ| ≤ k·m hold?), plus `pairing` for the closure-poset
  relation between two positions.

Zero dependencies. TDD throughout — every fixture is regression-tested.

## Quick Start

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
assert_eq!(omega_pair(&g, &lambda, &lambda)?, OmegaValue::Positive);              // meet exists
assert_eq!(omega_composition(&g, &lambda, &lambda)?, OmegaValue::StructuralZero); // budget overdrawn
```

## Documentation

[![Docs](https://img.shields.io/badge/docs-thatch.industrial--algebra.com-blue)](https://thatch.industrial-algebra.com)

The book covers the mathematics, the API, and worked examples:
**https://thatch.industrial-algebra.com**

## License

Apache-2.0. Commercial licensing available — contact Industrial Algebra.
