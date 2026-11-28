// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! # Thatch — Schubert strata on Grassmannians
//!
//! Combinatorial substrate for Schubert varieties: boundary-stratum
//! posets ([`strata()`]) and intersection classification for pairs of
//! positions ([`dict`]).
//!
//! ## Layout
//!
//! `thatch` is an umbrella: the mathematics lives in
//! [`thatch_core`], which has **zero dependencies**, and this crate
//! re-exports its entire public surface — every item and every module
//! path. `use thatch::…` is the supported import; it has resolved to
//! the same names since 0.1.0.
//!
//! ## Features
//!
//! - `std` (default): Standard library support (forwarded to
//!   `thatch-core`)
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
//!
//! The downstream seam — the structured-emptiness value that
//! consumers classify against:
//!
//! ```
//! use thatch::OmegaValue;
//!
//! let value = OmegaValue::Positive;
//! assert_ne!(value, OmegaValue::StructuralZero);
//! ```

pub use thatch_core::dict::{omega_composition, omega_pair, pairing, OmegaValue, Pairing};
pub use thatch_core::error::ThatchError;
pub use thatch_core::grassmannian::Grassmannian;
pub use thatch_core::partition::Partition;
pub use thatch_core::strata::{Stratum, StratumPoset};
// Imports BOTH namespaces: the module `strata` and the function `strata`
// (thatch-core re-exports the function at its root, so `thatch_core::strata`
// resolves in the module and value namespaces). Importing the function again
// from `thatch_core::strata::strata` would be E0252.
pub use thatch_core::strata;
pub use thatch_core::{dict, error, grassmannian, partition};

#[cfg(test)]
mod paths {
    //! The 0.1.0 surface through both import styles. Two paths name the
    //! same *type* iff the same expression type-checks at both; the same
    //! *function* iff their zero-sized fn-item types share a `TypeId`
    //! (`type_name` is not unique — two same-named local fns print the
    //! same name). Proved by typing, not by comparing output.
    use crate::dict::{OmegaValue as OmegaValueViaModule, Pairing as PairingViaModule};
    use crate::error::ThatchError as ThatchErrorViaModule;
    use crate::grassmannian::Grassmannian as GrassmannianViaModule;
    use crate::partition::Partition as PartitionViaModule;
    use crate::strata::{Stratum as StratumViaModule, StratumPoset as StratumPosetViaModule};
    use crate::{
        omega_composition, omega_pair, pairing, strata, Grassmannian, OmegaValue, Pairing,
        Partition, Stratum, StratumPoset, ThatchError,
    };

    fn same_item<A: 'static, B: 'static>(_: A, _: B) -> bool {
        std::any::TypeId::of::<A>() == std::any::TypeId::of::<B>()
    }

    #[test]
    fn module_paths_and_root_reexports_name_the_same_items() {
        let gr: Grassmannian = Grassmannian::new(2, 2).expect("valid");
        let lambda: Partition = Partition::new(vec![1, 0]).expect("valid");
        let poset: StratumPoset = strata(&lambda, &gr).expect("in box");

        // Types: each expression type-checks at the root name AND the module name.
        let _: GrassmannianViaModule = Grassmannian::new(2, 2).expect("valid");
        let _: PartitionViaModule = Partition::new(vec![1, 0]).expect("valid");
        let _: StratumPosetViaModule = strata(&lambda, &gr).expect("in box");
        let _: &StratumViaModule = &poset.boundary[0];
        let _: &Stratum = &poset.boundary[0];
        let _: OmegaValue = omega_pair(&gr, &lambda, &lambda).expect("in box");
        let _: OmegaValueViaModule = omega_pair(&gr, &lambda, &lambda).expect("in box");
        let _: OmegaValue = omega_composition(&gr, &lambda, &lambda).expect("in box");
        let _: OmegaValueViaModule = omega_composition(&gr, &lambda, &lambda).expect("in box");
        let _: Pairing = pairing(&lambda, &lambda);
        let _: PairingViaModule = pairing(&lambda, &lambda);
        let _: ThatchError = Partition::new(vec![2, 3]).expect_err("increasing");
        let _: ThatchErrorViaModule = Partition::new(vec![2, 3]).expect_err("increasing");

        // Functions: the root name and the module path are the same item.
        assert!(same_item(strata, crate::strata::strata));
        assert!(same_item(omega_pair, crate::dict::omega_pair));
        assert!(same_item(omega_composition, crate::dict::omega_composition));
        assert!(same_item(pairing, crate::dict::pairing));
    }
}
