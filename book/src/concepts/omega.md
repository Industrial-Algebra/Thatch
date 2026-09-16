# Ω: Structured Emptiness

"Empty" is not one thing. The Ω lattice types the kinds of emptiness:

| Value | Meaning | Provenance |
|---|---|---|
| `StructuralZero` | the budget was overdrawn — emptiness by dimension | the constraint system itself |
| `GeometricZero` | correctly dimensioned, yet empty | the geometry refused |
| `Positive` | nonempty, with known multiplicity | exists |
| `Underdetermined` | the computation could not resolve it | epistemic — the *asker*, not the world |

Thatch mirrors this lattice dependency-free as `OmegaValue`, keeping the
variant names and semantics in correspondence with the ecosystem
convention without a dependency.

## Two Intersections, Not One

The dictionary's central observation: behind the word "intersection" hide
two different questions with different geometries.

### 1. Compatibility — the set-meet

Do Ω_λ and Ω_μ *meet*? For Schubert varieties on a Grassmannian:

```
Ω_λ ∩ Ω_μ = Ω_{λ∨μ}
```

where λ∨μ is the componentwise join. The meet is itself a Schubert
variety, nonempty iff the join fits the box. `omega_pair(g, λ, μ)`
computes exactly this.

**The pair theorem.** Because the meet is always again a Schubert
variety, a *dimensioned but empty* meet cannot occur for pairs:
`GeometricZero` is unreachable in the pair world. The exhaustive test
over all of Gr(2,4)'s 36 pairs pins this; the general argument is the
displayed formula itself.

### 2. Composability — the product

Does the cohomological product `[Ω_λ] · [Ω_μ]` survive the budget? The
LR budget is the variety dimension `k·m`: the product is structurally
zero whenever `|λ| + |μ| > k·m`. `omega_composition(g, λ, μ)` computes
this.

## The Headline: Independence

The two questions are independent. On Gr(2,4):

- λ = μ = (2,1): the meet is Ω_(2,1) ≠ ∅ — **compatible**;
- but |λ| + |μ| = 6 > k·m = 4 — **incomposable**.

```rust
use thatch::{omega_composition, omega_pair, Grassmannian, Partition, OmegaValue};

let g = Grassmannian::new(2, 2)?;
let lambda = Partition::new(vec![2, 1])?;
assert_eq!(omega_pair(&g, &lambda, &lambda)?, OmegaValue::Positive);
assert_eq!(
    omega_composition(&g, &lambda, &lambda)?,
    OmegaValue::StructuralZero
);
```

Read order-theoretically: two conditions can hold simultaneously — their
meet exists (compatibility) — while their combined codimension demand
exceeds what the space affords (composability). "Both hold" and "both
compose" are different claims, with different proofs.

## Where Ω Earns Its Extra Values

For pairs on Grassmannians, `StructuralZero` and `Positive` decide
everything. The lattice's richer values earn their keep only when the
questions grow:

- **`GeometricZero`** — beyond pairs: flag conditions and general
  intersections can be dimensioned yet empty.
- **`Underdetermined`** — always epistemic: budgeted computation,
  incomplete data. It types the *asker*, never the world, and Thatch
  never returns it.

This is where Ω earns its extra values: multi-way products — three or
more constraints — leave the pair world, and there the full lattice is
needed: deeper intersections can be dimensioned yet empty
(`GeometricZero` becomes genuinely reachable), and budgeted computation
re-introduces the epistemic slot (`Underdetermined`).
