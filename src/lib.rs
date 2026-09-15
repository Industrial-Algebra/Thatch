// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! # Thatch — geometry of the edge
//!
//! The infinitesimal boundary between something and emptiness (or a
//! singularity), treated as geometry in its own right. Founding document:
//! `docs/ideation-edge-geometry.md` in the repository.
//!
//! Probe **S0** ships the substrate: the boundary-stratum poset of a
//! Schubert variety Ω_λ in Gr(k, k+m), computed from partitions alone.
//!
//! ## Features
//!
//! - `std` (default): Standard library support
//!
//! ## Usage
//!
//! ```
//! use thatch::{strata, Grassmannian, Partition};
//!
//! let gr = Grassmannian::new(2, 2).expect("valid"); // Gr(2,4)
//! let lambda = Partition::new(vec![1, 0]).expect("valid");
//! let poset = strata(&lambda, &gr).expect("in box");
//! assert_eq!(poset.open_cell_dimension, 3);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub mod error;
pub mod grassmannian;
pub mod partition;
pub mod strata;

pub use error::ThatchError;
pub use grassmannian::Grassmannian;
pub use partition::Partition;
pub use strata::{strata, Stratum, StratumPoset};
