// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! # Thatch — Schubert strata on Grassmannians
//!
//! Combinatorial substrate for Schubert varieties: boundary-stratum
//! posets ([`strata()`]) and intersection classification for pairs of
//! positions ([`dict`]).
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

pub mod dict;
pub mod error;
pub mod grassmannian;
pub mod partition;
pub mod strata;

pub use dict::{omega_composition, omega_pair, pairing, OmegaValue, Pairing};
pub use error::ThatchError;
pub use grassmannian::Grassmannian;
pub use partition::Partition;
pub use strata::{strata, Stratum, StratumPoset};
