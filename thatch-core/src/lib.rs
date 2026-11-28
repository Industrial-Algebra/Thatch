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
//! use thatch_core::{strata, Grassmannian, Partition};
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

#[cfg(test)]
mod floor {
    //! The workspace floor, enforced rather than stated. See AGENTS.md
    //! "Dependency direction". Cargo itself parses the manifest — the
    //! resolved dependency graph under ALL features and across ALL
    //! targets is the authority, not a line scan of TOML. (`--all-features`
    //! is required: the nested command does not inherit the outer
    //! invocation's feature selection, and an optional dependency is
    //! absent from the resolved graph until its feature is on. No
    //! `--offline`: `cargo tree` resolves the whole graph for ALL
    //! targets and features, so any *allowed* dev/build declaration must
    //! be fetchable — a BROADER fetch than this platform's `cargo test`
    //! needs (a target-specific build-dep is never built here but is
    //! resolved by `--target all`). With network this is invisible;
    //! strictly offline with such a declaration uncached, this test
    //! fails for a reason that is not the invariant. CI's `cargo
    //! metadata --no-deps | jq` step is the declaration-authoritative
    //! check; this test is the local, in-`cargo test` regression net.)

    #[test]
    fn has_no_normal_dependencies() {
        let cargo = std::env::var("CARGO").expect("cargo sets CARGO for test binaries");
        let manifest = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        let out = std::process::Command::new(cargo)
            .args([
                "tree",
                "--all-features",
                "--target",
                "all",
                "-e",
                "normal",
                "--prefix",
                "none",
                "--format",
                "{p}",
                "--manifest-path",
                manifest,
            ])
            .output()
            .expect("cargo tree runs");
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            out.status.success(),
            "cargo tree failed:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(
            stdout.lines().count(),
            1,
            "thatch-core must have zero normal dependencies; cargo tree printed:\n{stdout}"
        );
    }
}
