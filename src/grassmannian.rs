// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Grassmannian boxes Gr(k, k+m) and their partitions.

use crate::error::ThatchError;
use crate::partition::Partition;

/// A Grassmannian Gr(k, k+m) — the k x m box of partitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Grassmannian {
    /// Subspace dimension k.
    pub k: usize,
    /// Codimension slack m = n - k.
    pub m: usize,
}

impl Grassmannian {
    /// Create a Grassmannian box.
    ///
    /// # Errors
    ///
    /// Returns [`ThatchError::InvalidGrassmannian`] if k = 0 or m = 0.
    pub fn new(k: usize, m: usize) -> Result<Self, ThatchError> {
        if k == 0 || m == 0 {
            return Err(ThatchError::InvalidGrassmannian { k, m });
        }
        Ok(Self { k, m })
    }

    /// All partitions in the box, each exactly k parts (trailing zeros
    /// included), sorted descending lexicographically.
    ///
    /// # Examples
    ///
    /// ```
    /// use thatch::Grassmannian;
    ///
    /// let gr = Grassmannian::new(2, 2).expect("valid");
    /// assert_eq!(gr.partitions().len(), 6);
    /// ```
    pub fn partitions(&self) -> Vec<Partition> {
        let mut out = Vec::new();
        let k = self.k;
        let m = self.m as u32;
        let mut current = vec![m; k];
        loop {
            if let Ok(p) = Partition::new(current.clone()) {
                out.push(p);
            }
            // Advance to the previous partition in descending lex order:
            // find the rightmost part that can decrease given its right
            // neighbour is weakly smaller, decrement it, and fill the
            // rest with as-large-as-allowed weakly decreasing parts.
            let mut idx = None;
            for i in (0..k).rev() {
                let floor = if i + 1 < k { current[i + 1] } else { 0 };
                if current[i] > floor {
                    idx = Some(i);
                    break;
                }
            }
            let Some(i) = idx else { break };
            current[i] -= 1;
            for j in i + 1..k {
                current[j] = current[j - 1].min(m);
            }
        }
        out
    }

    /// Whether λ fits in this box.
    pub fn contains(&self, lambda: &Partition) -> bool {
        lambda.fits_in(self.k, self.m as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grassmannian_new_rejects_zero() {
        match Grassmannian::new(0, 2) {
            Err(ThatchError::InvalidGrassmannian { .. }) => {}
            other => panic!("expected InvalidGrassmannian, got {other:?}"),
        }
        match Grassmannian::new(2, 0) {
            Err(ThatchError::InvalidGrassmannian { .. }) => {}
            other => panic!("expected InvalidGrassmannian, got {other:?}"),
        }
    }

    #[test]
    fn partitions_gr24_exact_order() {
        let gr = Grassmannian::new(2, 2).expect("valid");
        let ps = gr.partitions();
        let got: Vec<&[u32]> = ps.iter().map(|p| p.parts()).collect();
        assert_eq!(
            got,
            vec![&[2, 2][..], &[2, 1], &[2, 0], &[1, 1], &[1, 0], &[0, 0]]
        );
    }

    #[test]
    fn partitions_gr36_count() {
        let gr = Grassmannian::new(3, 3).expect("valid");
        let ps = gr.partitions();
        assert_eq!(ps.len(), 20);
        assert_eq!(ps.first().map(|p| p.parts()), Some(&[3, 3, 3][..]));
        assert_eq!(ps.last().map(|p| p.parts()), Some(&[0, 0, 0][..]));
    }

    #[test]
    fn grassmannian_contains() {
        let gr = Grassmannian::new(2, 2).expect("valid");
        assert!(gr.contains(&Partition::new(vec![2, 1]).expect("valid")));
        assert!(!gr.contains(&Partition::new(vec![3, 3]).expect("valid")));
        assert!(!gr.contains(&Partition::new(vec![2, 1, 0]).expect("valid")));
    }
}
