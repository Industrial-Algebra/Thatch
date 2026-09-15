// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Partitions as codimension data for Schubert varieties.

use crate::error::ThatchError;

/// A partition λ, stored as weakly decreasing non-negative parts.
///
/// In Thatch's convention λ is *codimension* data: the Schubert variety
/// Ω_λ in Gr(k, k+m) has codimension |λ| = Σ λᵢ.
///
/// # Examples
///
/// ```
/// use thatch::Partition;
///
/// let lambda = Partition::new(vec![2, 1]).expect("valid");
/// assert_eq!(lambda.parts(), &[2, 1]);
/// assert_eq!(lambda.size(), 3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Partition {
    parts: Vec<u32>,
}

impl Partition {
    /// Create a partition, validating that parts are weakly decreasing.
    ///
    /// # Errors
    ///
    /// Returns [`ThatchError::NotWeaklyDecreasing`] if any part is greater
    /// than its predecessor.
    pub fn new(parts: Vec<u32>) -> Result<Self, ThatchError> {
        if parts.windows(2).any(|w| w[0] < w[1]) {
            return Err(ThatchError::NotWeaklyDecreasing(parts));
        }
        Ok(Self { parts })
    }

    /// The parts, as given (no canonical trailing-zero trimming).
    pub fn parts(&self) -> &[u32] {
        &self.parts
    }

    /// Number of parts.
    pub fn len(&self) -> usize {
        self.parts.len()
    }

    /// Whether the partition has no parts.
    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    /// |λ| — the sum of the parts; the codimension of Ω_λ.
    pub fn size(&self) -> u64 {
        self.parts.iter().map(|&p| p as u64).sum()
    }

    /// Whether this is the zero partition.
    pub fn is_zero(&self) -> bool {
        self.parts.iter().all(|&p| p == 0)
    }

    /// Whether λ has at most `k` parts, each at most `m`.
    pub fn fits_in(&self, k: usize, m: u32) -> bool {
        self.parts.len() <= k && self.parts.iter().all(|&p| p <= m)
    }

    /// Componentwise comparison with zero padding: is `self ≤ other`,
    /// i.e. selfᵢ ≤ otherᵢ for every position (missing parts read as 0)?
    ///
    /// This is the closure order on cells: `X_μ ⊆ Ω_λ` iff `μ ≤ λ`.
    pub fn le_componentwise(&self, other: &Partition) -> bool {
        let n = self.parts.len().max(other.parts.len());
        (0..n).all(|i| self.at(i) <= other.at(i))
    }

    /// Componentwise join λ ∨ μ (zero-padded): the least upper bound in
    /// the componentwise order. For Schubert varieties on a Grassmannian,
    /// Ω_λ ∩ Ω_μ = Ω_{λ∨μ} — the join classifies the set-meet.
    ///
    /// # Examples
    ///
    /// ```
    /// use thatch::Partition;
    ///
    /// let join = Partition::new(vec![2])?
    ///     .join_componentwise(&Partition::new(vec![1, 1])?)?;
    /// assert_eq!(join, Partition::new(vec![2, 1])?);
    /// # Ok::<(), thatch::ThatchError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ThatchError::NotWeaklyDecreasing`] — unreachable for
    /// valid inputs (the max of weakly-decreasing sequences is
    /// weakly-decreasing); the `Result` follows the constructor
    /// convention.
    pub fn join_componentwise(&self, other: &Partition) -> Result<Partition, ThatchError> {
        let n = self.parts.len().max(other.parts.len());
        let joined: Vec<u32> = (0..n).map(|i| self.at(i).max(other.at(i))).collect();
        Partition::new(joined)
    }

    fn at(&self, i: usize) -> u32 {
        self.parts.get(i).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_accepts_weakly_decreasing() {
        let p = Partition::new(vec![3, 1, 1]).expect("valid");
        assert_eq!(p.parts(), &[3, 1, 1]);
    }

    #[test]
    fn new_rejects_increasing() {
        match Partition::new(vec![2, 3]) {
            Err(ThatchError::NotWeaklyDecreasing(_)) => {}
            other => panic!("expected NotWeaklyDecreasing, got {other:?}"),
        }
    }

    #[test]
    fn size_len_is_zero() {
        let p = Partition::new(vec![2, 1]).expect("valid");
        assert_eq!(p.size(), 3);
        assert_eq!(p.len(), 2);
        assert!(!p.is_zero());

        let z = Partition::new(vec![]).expect("valid");
        assert!(z.is_zero());
        assert!(z.is_empty());
        assert_eq!(z.size(), 0);
    }

    #[test]
    fn fits_in() {
        assert!(Partition::new(vec![2, 1]).expect("valid").fits_in(2, 2));
        assert!(!Partition::new(vec![3, 1]).expect("valid").fits_in(2, 2));
        assert!(!Partition::new(vec![2, 1, 0]).expect("valid").fits_in(2, 2));
    }

    #[test]
    fn le_componentwise_pads_zeros() {
        let short = Partition::new(vec![2]).expect("valid");
        let long = Partition::new(vec![2, 1]).expect("valid");
        assert!(short.le_componentwise(&long));
        assert!(!long.le_componentwise(&short));
        assert!(long.le_componentwise(&long));

        let a = Partition::new(vec![1, 0]).expect("valid");
        let b = Partition::new(vec![1, 1]).expect("valid");
        assert!(a.le_componentwise(&b));
    }
}
