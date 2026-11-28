// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! The 0.1.0 surface, seen from **outside** the crate. An integration
//! test is a separate crate: it sees only the public re-exports, so a
//! module path that became private, or an item that was dropped, fails
//! here even when the in-crate `paths` test (which resolves `crate::`
//! privates) still passes. Every 0.1.0 path is exercised and shown to
//! name the very same item as `thatch_core`.

use std::any::TypeId;

use thatch::{
    dict, error, grassmannian, omega_composition, omega_pair, pairing, partition, strata,
    Grassmannian, OmegaValue, Pairing, Partition, Stratum, StratumPoset, ThatchError,
};

fn same_type<A: 'static, B: 'static>() -> bool {
    TypeId::of::<A>() == TypeId::of::<B>()
}

fn same_item<A: 'static, B: 'static>(_: A, _: B) -> bool {
    TypeId::of::<A>() == TypeId::of::<B>()
}

#[test]
fn every_0_1_0_path_is_public_and_names_the_thatch_core_item() {
    let gr = Grassmannian::new(2, 2).expect("valid");
    let lambda = Partition::new(vec![1, 0]).expect("valid");

    // Module paths: each must be `pub` through the umbrella (a private
    // `use error;` instead of `pub use` would be E0603 right here).
    let _: thatch::dict::OmegaValue = OmegaValue::Positive;
    let _: thatch::error::ThatchError = Partition::new(vec![2, 3]).expect_err("increasing");
    let _: thatch::grassmannian::Grassmannian = Grassmannian::new(2, 2).expect("valid");
    let _: thatch::partition::Partition = Partition::new(vec![1, 0]).expect("valid");
    let _: thatch::strata::StratumPoset = strata(&lambda, &gr).expect("in box");
    let _: &thatch::strata::Stratum = &strata(&lambda, &gr).expect("in box").boundary[0];
    let _: fn(&Partition, &Partition) -> Pairing = thatch::dict::pairing;
    let _: fn(&Partition, &Grassmannian) -> Result<StratumPoset, ThatchError> =
        thatch::strata::strata;
    let _: fn(&Grassmannian, &Partition, &Partition) -> Result<OmegaValue, ThatchError> =
        thatch::dict::omega_pair;
    let _: fn(&Grassmannian, &Partition, &Partition) -> Result<OmegaValue, ThatchError> =
        thatch::dict::omega_composition;

    // Root paths — identity against thatch_core directly.
    assert!(same_type::<Grassmannian, thatch_core::Grassmannian>());
    assert!(same_type::<Partition, thatch_core::Partition>());
    assert!(same_type::<StratumPoset, thatch_core::StratumPoset>());
    assert!(same_type::<Stratum, thatch_core::Stratum>());
    assert!(same_type::<OmegaValue, thatch_core::OmegaValue>());
    assert!(same_type::<Pairing, thatch_core::Pairing>());
    assert!(same_type::<ThatchError, thatch_core::ThatchError>());
    assert!(same_item(strata, thatch_core::strata));
    assert!(same_item(omega_pair, thatch_core::omega_pair));
    assert!(same_item(omega_composition, thatch_core::omega_composition));
    assert!(same_item(pairing, thatch_core::pairing));

    // Module paths — EXHAUSTIVE over the 0.1.0 module surface: every item
    // thatch::<module>:: exposed, each proven public (a private re-export is
    // E0603 here) AND identical to thatch_core's (TypeId). Types by
    // same_type, functions by same_item — a same-signature wrapper has the
    // same fn-pointer type but a different fn-item type.
    assert!(same_type::<dict::OmegaValue, thatch_core::dict::OmegaValue>());
    assert!(same_type::<dict::Pairing, thatch_core::dict::Pairing>());
    assert!(same_type::<
        error::ThatchError,
        thatch_core::error::ThatchError,
    >());
    assert!(same_type::<
        grassmannian::Grassmannian,
        thatch_core::grassmannian::Grassmannian,
    >());
    assert!(same_type::<
        partition::Partition,
        thatch_core::partition::Partition,
    >());
    assert!(same_type::<
        strata::StratumPoset,
        thatch_core::strata::StratumPoset,
    >());
    assert!(same_type::<strata::Stratum, thatch_core::strata::Stratum>());
    assert!(same_item(dict::omega_pair, thatch_core::dict::omega_pair));
    assert!(same_item(
        dict::omega_composition,
        thatch_core::dict::omega_composition
    ));
    assert!(same_item(dict::pairing, thatch_core::dict::pairing));
    assert!(same_item(strata::strata, thatch_core::strata::strata));
}
