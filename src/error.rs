// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Error type for Thatch.

use std::fmt;

/// Errors returned by Thatch.
#[derive(Debug, Clone, PartialEq)]
pub enum ThatchError {
    /// Parts were not weakly decreasing.
    NotWeaklyDecreasing(Vec<u32>),
    /// A Grassmannian was constructed with k = 0 or m = 0.
    InvalidGrassmannian {
        /// Subspace dimension k.
        k: usize,
        /// Codimension slack m = n - k.
        m: usize,
    },
    /// A partition does not fit in the k x m box.
    NotInBox {
        /// The offending parts.
        partition: Vec<u32>,
        /// Subspace dimension k.
        k: usize,
        /// Codimension slack m = n - k.
        m: usize,
    },
}

impl fmt::Display for ThatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThatchError::NotWeaklyDecreasing(parts) => {
                write!(
                    f,
                    "invalid partition: parts must be weakly decreasing, got {parts:?}"
                )
            }
            ThatchError::InvalidGrassmannian { k, m } => {
                write!(
                    f,
                    "invalid grassmannian: need k >= 1 and m >= 1, got k = {k}, m = {m}"
                )
            }
            ThatchError::NotInBox { partition, k, m } => {
                write!(f, "partition {partition:?} does not fit in the {k}x{m} box")
            }
        }
    }
}

impl std::error::Error for ThatchError {}
