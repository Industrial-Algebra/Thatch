# The Ω↔Stratum Dictionary

The S2 surface: the structured emptiness lattice and its dictionary to
the stratum poset. See [Ω: structured emptiness](../concepts/omega.md)
for the mathematics.

## `OmegaValue`

```rust
use thatch::OmegaValue;

assert!(OmegaValue::StructuralZero.is_zero());
assert!(OmegaValue::GeometricZero.is_zero());
assert!(!OmegaValue::Positive.is_zero());
assert!(!OmegaValue::Underdetermined.is_zero());
```

## `pairing` — closure-poset position

Where does μ sit relative to λ?

```rust
use thatch::{pairing, Pairing, Partition};

let lambda = Partition::new(vec![1, 0])?;
let mu = Partition::new(vec![2, 1])?;

assert_eq!(pairing(&lambda, &mu), Pairing::Stricter);   // μ ≥ λ: X_μ ⊆ Ω_λ
assert_eq!(pairing(&mu, &lambda), Pairing::Looser);
assert_eq!(
    pairing(&Partition::new(vec![2, 0])?, &Partition::new(vec![2])?),
    Pairing::SameStratum                                       // zero-padded
);
assert_eq!(
    pairing(&Partition::new(vec![2, 0])?, &Partition::new(vec![1, 1])?),
    Pairing::Incomparable
);
```

`Stricter` is the access-control reading: a principal positioned at μ
satisfies λ's constraint *and more* — over-qualified for λ.

The function agrees exhaustively with `strata`: μ appears in the boundary
of `strata(λ)` exactly when `pairing(λ, μ) = Stricter`.

## `omega_pair` — the compatibility edge

```rust
use thatch::{omega_pair, Grassmannian, Partition, OmegaValue};

let g = Grassmannian::new(2, 2)?;
assert_eq!(
    omega_pair(&g, &Partition::new(vec![2, 1])?, &Partition::new(vec![2, 1])?)?,
    OmegaValue::Positive          // join (2,1) fits the box
);
assert_eq!(
    omega_pair(&g, &Partition::new(vec![2, 2])?, &Partition::new(vec![2, 1])?)?,
    OmegaValue::Positive          // join (2,2) still fits
);
```

`StructuralZero` would require the join to leave the box. `GeometricZero`
is unreachable for pairs (the meet is always a Schubert variety);
`Underdetermined` is never returned — the dictionary is decidable.

## `omega_composition` — the composability edge

```rust
use thatch::{omega_composition, Grassmannian, Partition, OmegaValue};

let g = Grassmannian::new(2, 2)?;
// Budget k·m = 4: |(1,0)| + |(1,0)| = 2 — fine.
assert_eq!(
    omega_composition(&g, &Partition::new(vec![1, 0])?, &Partition::new(vec![1, 0])?)?,
    OmegaValue::Positive
);
// |(2,1)| + |(2,1)| = 6 > 4 — overdrawn.
assert_eq!(
    omega_composition(&g, &Partition::new(vec![2, 1])?, &Partition::new(vec![2, 1])?)?,
    OmegaValue::StructuralZero
);
```

## Errors

Both `omega_*` functions validate their inputs against the box and return
`ThatchError::NotInBox` (with the offending partition) otherwise.
