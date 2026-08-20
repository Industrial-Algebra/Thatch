// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! # Thatch — geometry of the edge
//!
//! The infinitesimal boundary between something and emptiness (or a
//! singularity), treated as geometry in its own right: what lives in the
//! ε-neighborhood of an edge, what its strata are, and how it composes when
//! two somethings share a boundary.
//!
//! **Status:** ideation. The mathematics is not yet substantiated — the
//! founding document (`docs/ideation-edge-geometry.md` in the repository)
//! states the established results it will draw on (logarithmic geometry,
//! formal neighborhoods, KS wall-crossing, Schubert singular strata,
//! skeletons, surreal infinitesimals) and the falsifiable probes that must
//! earn any implementation. No public API exists yet, deliberately.
//!
//! ## Placement
//!
//! Thatch is a foundation crate in the Industrial Algebra "Rich Toolbox": it
//! depends on **nothing** in the IA ecosystem; Schubert (wall-crossing
//! dynamics), Karpal (structured emptiness, the Ω lattice), Amari (Schubert
//! calculus, surreal ε-quantities), and Minuet (holographic memory edges)
//! may all draw from it. The dependency arrow only ever points away from
//! Thatch.
//!
//! ## Features
//!
//! - `std` (default): Standard library support
//!
//! ## Usage
//!
//! ```rust
//! // Nothing to use yet — this crate is at the ideation stage. The first
//! // public API will follow the first surviving probe in the founding doc.
//! ```
