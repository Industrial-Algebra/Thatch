// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! The Ω↔stratum dictionary: intersection classification for pairs of
//! Schubert positions.
//!
//! Two different geometries live behind the word "intersection", and the
//! dictionary's content is that they are *different edges*:
//!
//! 1. **Compatibility (the set-meet).** For Schubert varieties on a
//!    Grassmannian, `Ω_λ ∩ Ω_μ = Ω_{λ∨μ}` where `λ∨μ` is the componentwise
//!    join. The meet is nonempty iff the join fits the `k×m` box — and it
//!    is then again a Schubert variety. In particular a *dimensioned but
//!    empty* meet cannot occur for pairs: on Grassmannians the pairwise
//!    world has no geometric zeros.
//!
//! 2. **Composability (the product).** The cohomological product
//!    `[Ω_λ]·[Ω_μ]` is structurally zero whenever `|λ| + |μ|` exceeds the
//!    variety dimension `k·m` — the LR budget. Two positions can be
//!    perfectly compatible (their varieties meet) yet impossible to
//!    compose (their constraints overdraw the budget).
//!
//! The dictionary maps both readings onto the Ω lattice — the structured
//! emptiness values used across the ecosystem (mirrored here as
//! [`OmegaValue`] with zero dependencies; arrows point *away* from
//! Thatch):
//!
//! | Ω value | pair (set-meet) | composition (product) |
//! |---|---|---|
//! | `StructuralZero` | join outside the box | `\|λ\| + \|μ\| > k·m` |
//! | `GeometricZero` | **unreachable** (theorem) | **unreachable** on Grassmannian pairs |
//! | `Positive` | `Ω_{λ∨μ} ≠ ∅` | budget available |
//! | `Underdetermined` | **epistemic slot** — never returned; the dictionary is decidable | ditto |
//!
//! `GeometricZero` and `Underdetermined` earn their place in Ω only in
//! richer settings (flag conditions, multi-way products, budgeted
//! computation); the pairwise Grassmannian world is fully decided by the
//! stratum poset of [module][`mod@crate::strata`].
//!
//! # Examples
//!
//! ```
//! use thatch::{omega_composition, omega_pair, Grassmannian, Partition};
//!
//! // Gr(2,4): ambient 4, box 2x2.
//! let g = Grassmannian::new(2, 2)?;
//! let lambda = Partition::new(vec![2, 1])?;
//! let mu = Partition::new(vec![2, 1])?;
//!
//! // Compatible: the varieties meet (join (2,1) fits the box)...
//! assert_eq!(omega_pair(&g, &lambda, &mu)?, thatch::OmegaValue::Positive);
//! // ...yet impossible to compose: |λ|+|μ| = 6 > k·m = 4.
//! assert_eq!(
//!     omega_composition(&g, &lambda, &mu)?,
//!     thatch::OmegaValue::StructuralZero
//! );
//! # Ok::<(), thatch::ThatchError>(())
//! ```

use crate::error::ThatchError;
use crate::grassmannian::Grassmannian;
use crate::partition::Partition;

/// Where `μ` sits relative to `λ` in the closure poset of the [strata module][`mod@crate::strata`].
///
/// Both sides are compared zero-padded, so short forms are equal to their
/// padded forms: `[2]` is the same stratum as `[2, 0]`.
///
/// `Stricter` means `μ ≥ λ` componentwise (strictly): `X_μ ⊆ Ω_λ` —
/// deeper in the closure order. μ satisfies λ's condition and more.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pairing {
    /// Same stratum (equal after zero-padding).
    SameStratum,
    /// `μ > λ`: `μ` lies in λ's Schubert variety — deeper in the closure.
    Stricter,
    /// `λ > μ`: the mirror image; `λ` lies in μ's variety.
    Looser,
    /// Neither dominates: the strata are unrelated in the closure order.
    Incomparable,
}

/// The structured emptiness lattice Ω, mirrored dependency-free.
///
/// The variant names and semantics follow the ecosystem's
/// `IntersectionKind`; the correspondence table lives in the
/// [module docs][self].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OmegaValue {
    /// The constraint system overdrew its budget — emptiness by dimension.
    StructuralZero,
    /// Correctly dimensioned yet empty. Unreachable for Grassmannian pairs
    /// (Ω_λ ∩ Ω_μ = Ω_{λ∨μ} is itself a Schubert variety).
    GeometricZero,
    /// Nonempty, with known multiplicity.
    Positive,
    /// Epistemic slot: the computation could not resolve the answer. Never
    /// returned by this module — the dictionary is decidable.
    Underdetermined,
}

impl OmegaValue {
    /// True for either kind of empty intersection (mirrors Ω's `is_zero`).
    pub fn is_zero(&self) -> bool {
        matches!(self, Self::StructuralZero | Self::GeometricZero)
    }
}

/// Position of `μ` relative to `λ` in the closure poset.
///
/// # Examples
///
/// ```
/// use thatch::{Pairing, Partition, pairing};
///
/// let lambda = Partition::new(vec![1, 0])?;
/// let mu = Partition::new(vec![2, 1])?;
/// assert_eq!(pairing(&lambda, &mu), Pairing::Stricter);
/// // Short forms compare zero-padded: [2] is [2, 0].
/// assert_eq!(
///     pairing(&Partition::new(vec![2, 0])?, &Partition::new(vec![2])?),
///     Pairing::SameStratum
/// );
/// # Ok::<(), thatch::ThatchError>(())
/// ```
pub fn pairing(lambda: &Partition, mu: &Partition) -> Pairing {
    let le = lambda.le_componentwise(mu);
    let ge = mu.le_componentwise(lambda);
    match (le, ge) {
        (true, true) => Pairing::SameStratum,
        (true, false) => Pairing::Stricter,
        (false, true) => Pairing::Looser,
        (false, false) => Pairing::Incomparable,
    }
}

/// Classify the **compatibility** edge: the set-meet `Ω_λ ∩ Ω_μ`.
///
/// `StructuralZero` iff the componentwise join `λ∨μ` falls outside the
/// `k×m` box; `Positive` otherwise. `GeometricZero` is unreachable for
/// pairs — see the module docs — and `Underdetermined` is never returned.
///
/// # Errors
///
/// Returns [`ThatchError::NotInBox`] if `λ` or `μ` is not a valid
/// position on the Grassmannian.
pub fn omega_pair(
    g: &Grassmannian,
    lambda: &Partition,
    mu: &Partition,
) -> Result<OmegaValue, ThatchError> {
    for p in [lambda, mu] {
        if !g.contains(p) {
            return Err(ThatchError::NotInBox {
                partition: p.parts().to_vec(),
                k: g.k,
                m: g.m,
            });
        }
    }
    let join = lambda.join_componentwise(mu)?;
    Ok(if join.fits_in(g.k, g.m as u32) {
        OmegaValue::Positive
    } else {
        OmegaValue::StructuralZero
    })
}

/// Classify the **composability** edge: the cohomological product budget.
///
/// `StructuralZero` iff `|λ| + |μ| > k·m` (the LR budget is overdrawn);
/// `Positive` otherwise. Compatibility and composability are independent:
/// see the showcase test `composition_zero_while_meet_nonempty`.
///
/// # Errors
///
/// Returns [`ThatchError::NotInBox`] if `λ` or `μ` is not a valid
/// position on the Grassmannian.
pub fn omega_composition(
    g: &Grassmannian,
    lambda: &Partition,
    mu: &Partition,
) -> Result<OmegaValue, ThatchError> {
    for p in [lambda, mu] {
        if !g.contains(p) {
            return Err(ThatchError::NotInBox {
                partition: p.parts().to_vec(),
                k: g.k,
                m: g.m,
            });
        }
    }
    let budget = (g.k * g.m) as u64;
    Ok(if lambda.size() + mu.size() > budget {
        OmegaValue::StructuralZero
    } else {
        OmegaValue::Positive
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ThatchError;

    fn p(parts: &[u32]) -> Partition {
        Partition::new(parts.to_vec()).expect("valid partition")
    }

    #[test]
    fn join_identity_with_zero() {
        assert_eq!(p(&[2, 1]).join_componentwise(&p(&[0])).unwrap(), p(&[2, 1]));
    }

    #[test]
    fn join_idempotent() {
        assert_eq!(
            p(&[2, 1]).join_componentwise(&p(&[2, 1])).unwrap(),
            p(&[2, 1])
        );
    }

    #[test]
    fn join_pads_short_forms() {
        // componentwise max with zero-padding: [2] ∨ [1,1] = [2,1]
        assert_eq!(p(&[2]).join_componentwise(&p(&[1, 1])).unwrap(), p(&[2, 1]));
    }

    #[test]
    fn join_commutes() {
        assert_eq!(
            p(&[2]).join_componentwise(&p(&[1, 1])).unwrap(),
            p(&[1, 1]).join_componentwise(&p(&[2])).unwrap()
        );
    }

    #[test]
    fn pairing_same_stratum_short_form() {
        assert_eq!(pairing(&p(&[2, 0]), &p(&[2])), Pairing::SameStratum);
    }

    #[test]
    fn pairing_stricter_and_looser() {
        // (2,1) ≥ (1,0): X_(2,1) ⊆ Ω_(1,0)
        assert_eq!(pairing(&p(&[1, 0]), &p(&[2, 1])), Pairing::Stricter);
        assert_eq!(pairing(&p(&[2, 1]), &p(&[1, 0])), Pairing::Looser);
    }

    #[test]
    fn pairing_incomparable() {
        // 2 ≥ 1 but 0 < 1: neither dominates
        assert_eq!(pairing(&p(&[2, 0]), &p(&[1, 1])), Pairing::Incomparable);
    }

    #[test]
    fn pairing_agrees_with_strata_boundary() {
        // Stricter ⟺ μ appears in the boundary of strata(λ): spot-fixtured
        // on Gr(2,4) λ=(1,0), whose boundary is the 4 strata μ > (1,0).
        let g = Grassmannian::new(2, 2).expect("valid grassmannian");
        let lambda = p(&[1, 0]);
        let poset = crate::strata::strata(&lambda, &g).expect("valid strata");
        let expected_stricter = |mu: &Partition| {
            poset
                .boundary
                .iter()
                .any(|b| b.partition.parts() == mu.parts())
        };
        for mu in g.partitions() {
            let expected = expected_stricter(&mu);
            let got = pairing(&lambda, &mu);
            assert_eq!(
                got == Pairing::Stricter,
                expected,
                "λ=(1,0), μ={:?}: pairing says {:?}, boundary says {}",
                mu.parts(),
                got,
                expected
            );
        }
    }

    #[test]
    fn omega_pair_exhaustive_never_geometric_or_underdetermined() {
        // Theorem carrier: over ALL pairs on Gr(2,4), the set-meet is
        // Positive or StructuralZero — never GeometricZero (no dimensioned
        // emptiness for pairs) and never Underdetermined (decidable).
        let g = Grassmannian::new(2, 2).expect("valid grassmannian");
        for lambda in g.partitions() {
            for mu in g.partitions() {
                let v = omega_pair(&g, &lambda, &mu).expect("valid pair");
                let join = lambda
                    .join_componentwise(&mu)
                    .expect("valid join")
                    .fits_in(g.k, g.m as u32);
                assert_eq!(
                    v == OmegaValue::Positive,
                    join,
                    "λ={:?} μ={:?}",
                    lambda.parts(),
                    mu.parts()
                );
                assert!(
                    matches!(v, OmegaValue::Positive | OmegaValue::StructuralZero),
                    "unreachable Ω value for pair: {v:?}"
                );
            }
        }
    }

    #[test]
    fn composition_zero_while_meet_nonempty() {
        // The dictionary's headline: λ = μ = (2,1) on Gr(2,4).
        // Compatible (join (2,1) fits the box) yet incomposable
        // (|λ|+|μ| = 6 > k·m = 4). Compatibility ≠ composability.
        let g = Grassmannian::new(2, 2).expect("valid grassmannian");
        let lambda = p(&[2, 1]);
        assert_eq!(
            omega_pair(&g, &lambda, &lambda).unwrap(),
            OmegaValue::Positive
        );
        assert_eq!(
            omega_composition(&g, &lambda, &lambda).unwrap(),
            OmegaValue::StructuralZero
        );
    }

    #[test]
    fn omega_composition_positive_under_budget() {
        let g = Grassmannian::new(2, 2).expect("valid grassmannian");
        // |(1,0)| + |(1,0)| = 2 ≤ 4
        assert_eq!(
            omega_composition(&g, &p(&[1, 0]), &p(&[1, 0])).unwrap(),
            OmegaValue::Positive
        );
    }

    #[test]
    fn omega_pair_rejects_out_of_box() {
        let g = Grassmannian::new(2, 2).expect("valid grassmannian");
        let bad = p(&[3, 3]); // 3 > m = 2: outside the box
        assert!(matches!(
            omega_pair(&g, &bad, &p(&[0])),
            Err(ThatchError::NotInBox { .. })
        ));
    }

    #[test]
    fn is_zero_mirrors_omega() {
        assert!(OmegaValue::StructuralZero.is_zero());
        assert!(OmegaValue::GeometricZero.is_zero());
        assert!(!OmegaValue::Positive.is_zero());
        assert!(!OmegaValue::Underdetermined.is_zero());
    }
}
