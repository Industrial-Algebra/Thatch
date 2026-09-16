# Getting Started

Thatch has **zero dependencies** — add it and use it.

## Install

```toml
[dependencies]
thatch = "0.1"
```

## First Boundary

Ask for the boundary of a Schubert variety:

```rust
use thatch::{strata, Grassmannian, Partition};

fn main() -> Result<(), thatch::ThatchError> {
    // Gr(2, 4): 2-planes in 4-space. The box is 2×2.
    let g = Grassmannian::new(2, 2)?;

    // λ = (2, 1): codimension 3. Ω_(2,1) is a curve.
    let lambda = Partition::new(vec![2, 1])?;
    let poset = strata(&lambda, &g)?;

    println!("open cell dimension: {}", poset.open_cell_dimension);
    for b in &poset.boundary {
        println!(
            "  μ = {:?}: dim {}, codim {}",
            b.partition.parts(), b.dimension, b.codimension_in_variety
        );
    }
    Ok(())
}
```

The boundary of the curve Ω_(2,1) is the single point Ω_(2,2) — the poset
says so, with dimensions attached.

## First Dictionary Call

Two positions on the Grassmannian have **two** different intersection
questions, and the dictionary separates them:

```rust
use thatch::{omega_composition, omega_pair, Grassmannian, Partition, OmegaValue};

fn main() -> Result<(), thatch::ThatchError> {
    let g = Grassmannian::new(2, 2)?;
    let lambda = Partition::new(vec![2, 1])?;

    // Compatibility: do the varieties meet? Yes — the meet is Ω_(2,1).
    assert_eq!(
        omega_pair(&g, &lambda, &lambda)?,
        OmegaValue::Positive
    );

    // Composability: does the LR budget allow the product? No — |λ|+|μ| = 6
    // exceeds k·m = 4.
    assert_eq!(
        omega_composition(&g, &lambda, &lambda)?,
        OmegaValue::StructuralZero
    );
    Ok(())
}
```

Compatible, yet incomposable. That independence is the dictionary's
content — see [the Ω↔stratum dictionary](./api/dict.md).

## Where to Go Next

- [Strata: the mathematics](./concepts/strata.md) — partitions, closure
  orders, and why the boundary is a strict upset.
- [Ω: structured emptiness](./concepts/omega.md) — the lattice and the
  pair theorem.
- [Basic usage walkthrough](./examples/basic.md).
