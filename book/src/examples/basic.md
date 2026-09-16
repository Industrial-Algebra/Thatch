# Basic Usage

A walkthrough of the two questions Thatch answers today, on Gr(2,4).

## Setup

```rust
use thatch::{Grassmannian, Partition, ThatchError};

fn main() -> Result<(), ThatchError> {
    let g = Grassmannian::new(2, 2)?;   // Gr(2,4): the box is 2×2
    Ok(())
}
```

## Question 1 — What is the boundary of Ω_λ?

Take λ = (1, 0) — codimension 1, a hypersurface:

```rust
use thatch::{strata, Grassmannian, Partition};

# fn main() -> Result<(), thatch::ThatchError> {
let g = Grassmannian::new(2, 2)?;
let lambda = Partition::new(vec![1, 0])?;
let poset = strata(&lambda, &g)?;

println!("Ω_(1,0) has open cell dimension {}", poset.open_cell_dimension);

for b in &poset.boundary {
    println!(
        "  X_{:?}: dim {}, codim {} in the variety",
        b.partition.parts(), b.dimension, b.codimension_in_variety
    );
}
# Ok(())
# }
```

Output:

```text
Ω_(1,0) has open cell dimension 3
  X_[1, 1]: dim 2, codim 1 in the variety
  X_[2, 0]: dim 2, codim 1 in the variety
  X_[2, 1]: dim 1, codim 2 in the variety
  X_[2, 2]: dim 0, codim 3 in the variety
```

Four boundary strata, ascending lexicographically — the surface's curve
of degeneration, then its point.

## Question 2 — Do two positions compose?

The dictionary's two edges, on λ = μ = (2, 1):

```rust
use thatch::{
    omega_composition, omega_pair, Grassmannian, OmegaValue, Partition,
};

# fn main() -> Result<(), thatch::ThatchError> {
let g = Grassmannian::new(2, 2)?;
let lambda = Partition::new(vec![2, 1])?;

let meet = omega_pair(&g, &lambda, &lambda)?;
let product = omega_composition(&g, &lambda, &lambda)?;

println!("compatible:  {:?}", meet);      // Positive
println!("composable:  {:?}", product);   // StructuralZero
# assert_eq!(meet, OmegaValue::Positive);
# assert_eq!(product, OmegaValue::StructuralZero);
# Ok(())
# }
```

The two positions coexist — yet their composition
overdraws the LR budget (`|λ|+|μ| = 6 > k·m = 4`). Compatibility and
composability are independent edges; conflating them is the mistake the
dictionary exists to prevent.

## Where the Numbers Live

Every fixture above is pinned in the test-suite (boundary counts per λ,
the 36-pair exhaustive theorem check on Gr(2,4), the showcase pair) —
what you read here is what the CI verifies.
