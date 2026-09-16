# Strata: Partitions and Schubert Varieties

The substrate of everything Thatch computes is the stratification of a
Grassmannian by Schubert varieties. This chapter is the mathematics,
stated as the crate uses it.

## Partitions as Codimension Data

A partition λ = (λ₁ ≥ λ₂ ≥ … ≥ λₖ ≥ 0) is *codimension data*: it indexes
how far a position sits from generic. The **size** |λ| = Σλᵢ is the
codimension of the Schubert variety Ω_λ.

In Thatch, partitions live in the `k × m` box: at most `k` parts, each at
most `m`. Short forms are identified with their zero-padded forms —
`[2]` and `[2, 0]` are the same position.

```rust
use thatch::Partition;

let lambda = Partition::new(vec![2, 1])?;   // weakly decreasing, or error
assert_eq!(lambda.size(), 3);               // |λ| = codim Ω_λ
assert!(lambda.fits_in(2, 2));              // inside the 2×2 box
```

## Grassmannians

`Grassmannian::new(k, m)` is the space of k-planes in (k+m)-space. The
crate convention: `m` is the *slack*, ambient dimension is `k + m`, and
the box of valid positions is `k × m`. `Gr(2,4)` of the literature is
`Grassmannian::new(2, 2)` — six positions, `[0,0]` through `[2,2]`.

The variety dimension is `k·m`, so the **open cell** of λ has dimension
`k·m − |λ|`.

## Schubert Varieties and the Closure Order

Ω_λ is a closed subvariety stratified by Schubert cells:

```
Ω_λ = ⨿_{μ ≥ λ} X_μ
```

where `μ ≥ λ` is the componentwise (zero-padded) order. Three
consequences the crate relies on:

1. **The boundary is the strict upset:** ∂Ω_λ = ⨿_{μ > λ} X_μ — all cells
   strictly above λ in the componentwise order.
2. **dim X_μ = k·m − |μ|** — deeper in the order, smaller the cell.
3. **The strata form a poset under closure:** X_μ ⊆ closure(X_ν) iff
   μ ≥ν componentwise.

## The `strata` Function

`strata(&lambda, &g)` returns the full poset of Ω_λ's boundary:

```rust
use thatch::{strata, Grassmannian, Partition};

let g = Grassmannian::new(2, 2)?;        // Gr(2,4)
let lambda = Partition::new(vec![1, 0])?;
let poset = strata(&lambda, &g)?;

assert_eq!(poset.open_cell_dimension, 3);  // k·m − |λ| = 4 − 1
// Boundary: the four strata μ > (1,0), ascending lexicographically.
assert_eq!(poset.boundary.len(), 4);
// Closure pairs: (i, j) with closure(boundary[i]) ⊇ boundary[j].
assert_eq!(poset.order.len(), 5);
```

Fixtures the test-suite pins (Gr(2,4)):

| λ | boundary strata | closure pairs |
|---|---|---|
| (0,0) | 5 | 9 |
| (1,0) | 4 | 5 |
| (1,1) | 2 | 1 |
| (2,0) | 2 | 1 |
| (2,1) | 1 | 0 |
| (2,2) | 0 | 0 |

Read the (2,1) row geometrically: Ω_(2,1) is a curve whose boundary is
the single point Ω_(2,2). Read the (2,2) row: Ω_(2,2) is itself a point —
no boundary at all.

## Why Combinatorial Suffices

On Grassmannians, everything above is decided by the componentwise order
on box partitions — no limits, no cohomology, no sheaves. That
decidability is what makes the [Ω↔stratum dictionary](./omega.md) exact.
