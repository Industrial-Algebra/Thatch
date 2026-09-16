# Strata Substrate

The S0 surface: partitions, Grassmannians, and boundary-stratum posets.

## `Partition`

Codimension data with validation on construction.

```rust
use thatch::Partition;

let lambda = Partition::new(vec![2, 1])?;   // Err if not weakly decreasing
assert_eq!(lambda.parts(), &[2, 1]);
assert_eq!(lambda.len(), 2);
assert_eq!(lambda.size(), 3);               // |λ| — the codimension
assert!(lambda.is_zero() == false);

let zero = Partition::new(vec![0])?;
assert!(zero.is_zero());

assert!(lambda.fits_in(2, 2));              // valid position on Gr(2,4)
assert!(!lambda.fits_in(2, 1));             // part 2 > m = 1

// Zero-padded comparison: [2] ≡ [2, 0].
let short = Partition::new(vec![2])?;
assert!(short.le_componentwise(&lambda));
assert!(lambda.le_componentwise(&short));
```

### `join_componentwise`

The componentwise join λ∨μ (zero-padded) — the least upper bound in the
componentwise order. Under the hood of the dictionary: Ω_λ ∩ Ω_μ =
Ω_{λ∨μ}.

```rust
let join = Partition::new(vec![2])?
    .join_componentwise(&Partition::new(vec![1, 1])?)?;
assert_eq!(join, Partition::new(vec![2, 1])?);
```

## `Grassmannian`

`Gr(k, k+m)` — the space of k-planes in (k+m)-space. The constructor
validates `k ≥ 1, m ≥ 1`.

```rust
use thatch::Grassmannian;

let g = Grassmannian::new(2, 2)?;           // Gr(2,4) in the literature
assert_eq!(g.partitions().len(), 6);        // all positions, descending lex
assert!(g.contains(&Partition::new(vec![2, 2])?));
assert!(!g.contains(&Partition::new(vec![3])?));
```

Each partition returned by `partitions()` is a full k-part vector.

## `strata`

The boundary poset of Ω_λ:

```rust
use thatch::{strata, Grassmannian, Partition};

let g = Grassmannian::new(2, 2)?;
let poset = strata(&Partition::new(vec![2, 1])?, &g)?;

assert_eq!(poset.open_cell_dimension, 1);   // k·m − |λ| = 4 − 3
assert_eq!(poset.boundary.len(), 1);        // the point Ω_(2,2)
assert_eq!(poset.boundary[0].dimension, 0);
assert_eq!(poset.boundary[0].codimension_in_variety, 1); // |μ| − |λ|
assert!(poset.order.is_empty());            // one stratum: no pairs
```

### `StratumPoset` fields

| Field | Meaning |
|---|---|
| `grassmannian` | the ambient box |
| `variety` | λ itself |
| `open_cell_dimension` | `k·m − |λ|` |
| `boundary` | `Vec<Stratum>` — μ > λ ascending lexicographically |
| `order` | closure pairs `(i, j)` with `boundary[i] ≤ boundary[j]` |

### Errors

`strata` returns `ThatchError::NotInBox` if λ is not a valid position on
`g`. `Partition::new` returns `NotWeaklyDecreasing` for invalid input.
Both are the only fallible constructors in the substrate — no panics
anywhere in the crate.
