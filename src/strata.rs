// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Boundary strata of Schubert varieties (probe S0).

use crate::error::ThatchError;
use crate::grassmannian::Grassmannian;
use crate::partition::Partition;

/// One boundary stratum of a Schubert variety.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stratum {
    /// The stratum's cell index μ (Ω_λ = ⊔_{μ ≥ λ} X_μ).
    pub partition: Partition,
    /// dim X_μ = k·m − |μ|.
    pub dimension: u64,
    /// Codimension of X_μ inside Ω_λ = |μ| − |λ|.
    pub codimension_in_variety: u64,
}

/// The boundary stratum poset of a Schubert variety Ω_λ.
#[derive(Debug, Clone, PartialEq)]
pub struct StratumPoset {
    /// The ambient box.
    pub grassmannian: Grassmannian,
    /// The variety's partition λ.
    pub variety: Partition,
    /// dim X_λ = k·m − |λ| (the open cell).
    pub open_cell_dimension: u64,
    /// Boundary strata, sorted ascending lexicographically by partition.
    pub boundary: Vec<Stratum>,
    /// Comparable pairs `(i, j)`, i < j, with `boundary[i] ≤ boundary[j]`
    /// componentwise — i.e. the closure of stratum i contains stratum j.
    pub order: Vec<(usize, usize)>,
}

/// Boundary strata of Ω_λ in `g`: all μ in the box with μ ≥ λ (μ ≠ λ),
/// with dimensions and closure pairs.
///
/// # Examples
///
/// ```
/// use thatch::{strata, Grassmannian, Partition};
///
/// let gr = Grassmannian::new(2, 2).expect("valid"); // Gr(2,4)
/// let lambda = Partition::new(vec![2, 1]).expect("valid");
/// let poset = strata(&lambda, &gr).expect("in box");
/// // Ω_(2,1) is a curve whose boundary is the single point Ω_(2,2).
/// assert_eq!(poset.boundary.len(), 1);
/// assert_eq!(poset.boundary[0].partition, Partition::new(vec![2, 2]).unwrap());
/// ```
///
/// # Errors
///
/// Returns [`ThatchError::NotInBox`] if λ does not fit in `g`'s box.
pub fn strata(lambda: &Partition, g: &Grassmannian) -> Result<StratumPoset, ThatchError> {
    if !g.contains(lambda) {
        return Err(ThatchError::NotInBox {
            partition: lambda.parts().to_vec(),
            k: g.k,
            m: g.m,
        });
    }
    let km = (g.k * g.m) as u64;
    let l_size = lambda.size();
    let mut boundary: Vec<Stratum> = g
        .partitions()
        .into_iter()
        .filter(|mu| {
            // Exclude λ itself up to padded equality (e.g. [2] ≡ [2, 0]):
            // structural equality would leak the variety's own cell into
            // the boundary when λ is given in short form.
            let is_lambda = mu.le_componentwise(lambda) && lambda.le_componentwise(mu);
            !is_lambda && lambda.le_componentwise(mu)
        })
        .map(|mu| {
            let s = mu.size();
            Stratum {
                partition: mu,
                dimension: km - s,
                codimension_in_variety: s - l_size,
            }
        })
        .collect();
    // partitions() is descending-lex; reverse gives ascending-lex.
    boundary.reverse();
    // Closure-based flat_map cannot express this borrow pattern without
    // capturing `boundary` by value; an explicit double loop yields the
    // same ascending pair order.
    let mut order = Vec::new();
    for (i, si) in boundary.iter().enumerate() {
        for (j, sj) in boundary.iter().enumerate().skip(i + 1) {
            if si.partition.le_componentwise(&sj.partition) {
                order.push((i, j));
            }
        }
    }
    Ok(StratumPoset {
        grassmannian: *g,
        variety: lambda.clone(),
        open_cell_dimension: km - l_size,
        boundary,
        order,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gr2() -> Grassmannian {
        Grassmannian::new(2, 2).expect("valid")
    }

    #[test]
    fn strata_rejects_out_of_box() {
        let lambda = Partition::new(vec![3, 3]).expect("valid");
        match strata(&lambda, &gr2()) {
            Err(ThatchError::NotInBox { .. }) => {}
            other => panic!("expected NotInBox, got {other:?}"),
        }
    }

    #[test]
    fn strata_short_form_lambda_excludes_own_cell() {
        // λ = [2] is the same partition as [2, 0]; its own cell X_(2,0)
        // must not appear in the boundary.
        let lambda = Partition::new(vec![2]).expect("valid");
        let p = strata(&lambda, &gr2()).expect("in box");
        let parts: Vec<&[u32]> = p.boundary.iter().map(|s| s.partition.parts()).collect();
        assert_eq!(parts, vec![&[2, 1][..], &[2, 2]]);
        assert_eq!(p.open_cell_dimension, 2);
    }

    #[test]
    fn strata_point_variety_is_boundaryless() {
        let lambda = Partition::new(vec![2, 2]).expect("valid");
        let p = strata(&lambda, &gr2()).expect("in box");
        assert!(p.boundary.is_empty());
        assert!(p.order.is_empty());
        assert_eq!(p.open_cell_dimension, 0);
    }

    #[test]
    fn strata_curve_with_point_boundary() {
        let lambda = Partition::new(vec![2, 1]).expect("valid");
        let p = strata(&lambda, &gr2()).expect("in box");
        assert_eq!(p.boundary.len(), 1);
        assert_eq!(p.boundary[0].partition.parts(), &[2, 2]);
        assert_eq!(p.boundary[0].dimension, 0);
        assert_eq!(p.boundary[0].codimension_in_variety, 1);
        assert_eq!(p.open_cell_dimension, 1);
    }

    #[test]
    fn strata_full_variety() {
        let lambda = Partition::new(vec![0, 0]).expect("valid");
        let p = strata(&lambda, &gr2()).expect("in box");
        let parts: Vec<&[u32]> = p.boundary.iter().map(|s| s.partition.parts()).collect();
        assert_eq!(parts, vec![&[1, 0][..], &[1, 1], &[2, 0], &[2, 1], &[2, 2]]);
        let dims: Vec<u64> = p.boundary.iter().map(|s| s.dimension).collect();
        assert_eq!(dims, vec![3, 2, 2, 1, 0]);
        let codims: Vec<u64> = p
            .boundary
            .iter()
            .map(|s| s.codimension_in_variety)
            .collect();
        assert_eq!(codims, vec![1, 2, 2, 3, 4]);
        assert_eq!(p.open_cell_dimension, 4);
        assert_eq!(p.order.len(), 9);
    }

    #[test]
    fn strata_hyperplane_class() {
        let lambda = Partition::new(vec![1, 0]).expect("valid");
        let p = strata(&lambda, &gr2()).expect("in box");
        let parts: Vec<&[u32]> = p.boundary.iter().map(|s| s.partition.parts()).collect();
        assert_eq!(parts, vec![&[1, 1][..], &[2, 0], &[2, 1], &[2, 2]]);
        assert_eq!(p.order, vec![(0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
        let dims: Vec<u64> = p.boundary.iter().map(|s| s.dimension).collect();
        assert_eq!(dims, vec![2, 2, 1, 0]);
        let codims: Vec<u64> = p
            .boundary
            .iter()
            .map(|s| s.codimension_in_variety)
            .collect();
        assert_eq!(codims, vec![1, 1, 2, 3]);
        assert_eq!(p.open_cell_dimension, 3);
    }

    #[test]
    fn strata_lambda_1_1() {
        let lambda = Partition::new(vec![1, 1]).expect("valid");
        let p = strata(&lambda, &gr2()).expect("in box");
        let parts: Vec<&[u32]> = p.boundary.iter().map(|s| s.partition.parts()).collect();
        assert_eq!(parts, vec![&[2, 1][..], &[2, 2]]);
        assert_eq!(p.order, vec![(0, 1)]);
        let dims: Vec<u64> = p.boundary.iter().map(|s| s.dimension).collect();
        assert_eq!(dims, vec![1, 0]);
        let codims: Vec<u64> = p
            .boundary
            .iter()
            .map(|s| s.codimension_in_variety)
            .collect();
        assert_eq!(codims, vec![1, 2]);
        assert_eq!(p.open_cell_dimension, 2);
    }

    #[test]
    fn strata_lambda_2_0() {
        let lambda = Partition::new(vec![2, 0]).expect("valid");
        let p = strata(&lambda, &gr2()).expect("in box");
        let parts: Vec<&[u32]> = p.boundary.iter().map(|s| s.partition.parts()).collect();
        assert_eq!(parts, vec![&[2, 1][..], &[2, 2]]);
        assert_eq!(p.order, vec![(0, 1)]);
        assert_eq!(p.open_cell_dimension, 2);
    }

    #[test]
    fn strata_gr36_point() {
        let gr = Grassmannian::new(3, 3).expect("valid");
        let lambda = Partition::new(vec![3, 3, 3]).expect("valid");
        let p = strata(&lambda, &gr).expect("in box");
        assert!(p.boundary.is_empty());
        assert_eq!(p.open_cell_dimension, 0);
    }

    #[test]
    fn strata_gr36_lambda_210() {
        let gr = Grassmannian::new(3, 3).expect("valid");
        let lambda = Partition::new(vec![2, 1, 0]).expect("valid");
        let p = strata(&lambda, &gr).expect("in box");
        assert_eq!(p.boundary.len(), 13);
        assert_eq!(p.open_cell_dimension, 6);
    }

    #[test]
    fn strata_gr36_lambda_300() {
        let gr = Grassmannian::new(3, 3).expect("valid");
        let lambda = Partition::new(vec![3, 0, 0]).expect("valid");
        let p = strata(&lambda, &gr).expect("in box");
        assert_eq!(p.boundary.len(), 9);
        // km = 3*3 = 9, |lambda| = 3, so dim X_lambda = 6.
        assert_eq!(p.open_cell_dimension, 6);
    }
}
