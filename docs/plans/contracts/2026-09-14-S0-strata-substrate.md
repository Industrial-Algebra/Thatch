# Plan Contract — Thatch S0: strata substrate (pure combinatorics)

**Repo:** Thatch · **Branch:** `feature/s0-strata-substrate` (already checked out)
**Depends on:** founding doc §7 S0 (rev 2) · **Followed by:** S2 (Ω↔stratum dictionary)
**Conventions locked:** partition λ is *codimension* data — codim(Ω_λ) = |λ|;
dim(open cell X_λ) = k·m − |λ|; Ω_λ = ⊔_{μ ≥ λ} X_μ (componentwise, μ in the
k×m box); boundary strata of Ω_λ = { μ in box : μ ≥ λ, μ ≠ λ }.

**TDD: write every test in the Tests section FIRST, confirm red, then
implement.**

## Files

Create: `src/error.rs`, `src/partition.rs`, `src/grassmannian.rs`,
`src/strata.rs`. Rewrite `src/lib.rs` with the full final content below.
Modify `Cargo.toml` only to add `thiserror = "2"` under `[dependencies]`.

## Contracts

### C1 — `src/error.rs` (full final content)

```rust
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
                write!(f, "invalid partition: parts must be weakly decreasing, got {parts:?}")
            }
            ThatchError::InvalidGrassmannian { k, m } => {
                write!(f, "invalid grassmannian: need k >= 1 and m >= 1, got k = {k}, m = {m}")
            }
            ThatchError::NotInBox { partition, k, m } => {
                write!(
                    f,
                    "partition {partition:?} does not fit in the {k}x{m} box"
                )
            }
        }
    }
}

impl std::error::Error for ThatchError {}
```

(No `thiserror` needed for three variants — plain impls keep the crate
dependency-free; do **not** add the dependency.)

### C2 — `src/partition.rs` (full final content)

```rust
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

    fn at(&self, i: usize) -> u32 {
        self.parts.get(i).copied().unwrap_or(0)
    }
}
```

### C3 — `src/grassmannian.rs` (full final content)

```rust
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
```

### C4 — `src/strata.rs` (full final content)

```rust
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
        .filter(|mu| mu != lambda && lambda.le_componentwise(mu))
        .map(|mu| Stratum {
            size: mu.size(),
            partition: mu,
        })
        .map(|s| Stratum {
            partition: s.partition,
            dimension: km - s.size,
            codimension_in_variety: s.size - l_size,
        })
        .collect();
    // partitions() is descending-lex; reverse gives ascending-lex.
    boundary.reverse();
    let order = (0..boundary.len())
        .flat_map(|i| {
            (i + 1..boundary.len())
                .filter(|&j| boundary[i].partition.le_componentwise(&boundary[j].partition))
                .map(move |j| (i, j))
        })
        .collect();
    Ok(StratumPoset {
        grassmannian: *g,
        variety: lambda.clone(),
        open_cell_dimension: km - l_size,
        boundary,
        order,
    })
}
```

Note: the double-`map` in `strata` is a scaffolding artifact — collapse it
to a single `map` computing `size` once (`let s = mu.size(); Stratum {
partition: mu, dimension: km - s, codimension_in_variety: s - l_size }`).
The `partition` field must be moved after computing `s`, so compute `s`
before the struct literal.

### C5 — `src/lib.rs` (full final content — replaces the ideation stub)

```rust
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
```

## Tests (write first; every assertion enumerated)

Unit tests live in each module's `#[cfg(test)]` block. Doc tests above
count toward completion.

1. `new_accepts_weakly_decreasing` — `Partition::new(vec![3, 1, 1])` is
   `Ok`; `parts() == &[3, 1, 1]`.
2. `new_rejects_increasing` — `Partition::new(vec![2, 3])` is
   `Err(ThatchError::NotWeaklyDecreasing(_))`.
3. `size_len_is_zero` — `Partition::new(vec![2, 1])`: `size() == 3`,
   `len() == 2`, `!is_zero()`; `Partition::new(vec![])`:
   `is_zero() && is_empty() && size() == 0`.
4. `fits_in` — `[2,1]` fits (2, 2); `[3,1]` does not (3 > 2); `[2,1,0]`
   does not (3 parts > k=2).
5. `le_componentwise_pads_zeros` — `[2] ≤ [2,1]` (pad to [2,0]); `[2,1] ≤
   [2]` is false; `[2,1] ≤ [2,1]` is true; `[1,0] ≤ [1,1]` is true.
6. `grassmannian_new_rejects_zero` — `Grassmannian::new(0, 2)` and
   `Grassmannian::new(2, 0)` both `Err(InvalidGrassmannian { .. })`.
7. `partitions_gr24_exact_order` — `Grassmannian::new(2, 2)`:
   partitions (as parts() slices, in order) ==
   `[[2,2], [2,1], [2,0], [1,1], [1,0], [0,0]]`.
8. `partitions_gr36_count` — `Grassmannian::new(3, 3)`: `len() == 20`;
   first `[3,3,3]`, last `[0,0,0]`.
9. `grassmannian_contains` — Gr(2,2): contains `[2,1]`; not `[3,3]`;
   not `[2,1,0]`.
10. `strata_rejects_out_of_box` — `strata(&Partition::new(vec![3,
    3]).unwrap(), &Grassmannian::new(2, 2).unwrap())` is
    `Err(ThatchError::NotInBox { .. })`.
11. `strata_point_variety_is_boundaryless` — λ=[2,2] in Gr(2,2):
    `boundary` empty, `order` empty, `open_cell_dimension == 0`.
12. `strata_curve_with_point_boundary` — λ=[2,1]: `boundary.len() == 1`;
    `boundary[0].partition == [2,2]`; `boundary[0].dimension == 0`;
    `boundary[0].codimension_in_variety == 1`; `open_cell_dimension == 1`.
13. `strata_full_variety` — λ=[0,0]: boundary partitions in order ==
    `[[1,0], [1,1], [2,0], [2,1], [2,2]]`; dimensions ==
    `[3, 2, 2, 1, 0]`; codimensions == `[1, 2, 2, 3, 4]`;
    `open_cell_dimension == 4`; `order.len() == 9`.
14. `strata_hyperplane_class` — λ=[1,0]: boundary ==
    `[[1,1], [2,0], [2,1], [2,2]]`; `order == [(0,2), (0,3), (1,2), (1,3), (2,3)]`;
    dimensions `[2, 2, 1, 0]`; codimensions `[1, 1, 2, 3]`;
    `open_cell_dimension == 3`.
15. `strata_lambda_1_1` — λ=[1,1]: boundary `[[2,1], [2,2]]`;
    `order == [(0, 1)]`; dimensions `[1, 0]`; codimensions `[1, 2]`;
    `open_cell_dimension == 2`.
16. `strata_lambda_2_0` — λ=[2,0]: boundary `[[2,1], [2,2]]`;
    `order == [(0, 1)]`; `open_cell_dimension == 2`.
17. `strata_gr36_point` — λ=[3,3,3] in Gr(3,3): boundary empty,
    `open_cell_dimension == 0`.
18. `strata_gr36_lambda_210` — λ=[2,1,0]: `boundary.len() == 14`;
    `open_cell_dimension == 6`.
19. `strata_gr36_lambda_300` — λ=[3,0,0]: `boundary.len() == 9`;
    `open_cell_dimension == 3`.

In tests, compare `Partition` values via `parts()` slices (e.g.
`assert_eq!(poset.boundary[0].partition.parts(), &[2, 2])`) — do not
construct expected `Partition` values when a slice comparison suffices.

## Constraints

- **No dependencies.** Do not add `thiserror` or anything else — C1 uses
  plain impls precisely to keep the crate dependency-free.
- License headers (two lines) on every file, exactly as in the Contracts.
- `cargo fmt --all` for formatting.
- No `unwrap`/`expect` outside tests and doc examples.

## Completion (filtered; all must be green / zero)

```sh
cargo test 2>&1 | grep -E 'test result|FAILED|^error' | tail -12
cargo clippy --all-targets -- -D warnings 2>&1 | tail -4
cargo clippy --all-targets --all-features -- -D warnings 2>&1 | tail -4
cargo fmt --all --check && echo FMT-OK
cargo doc --no-deps 2>&1 | grep -c "warning"
```

## Out of scope

- Smoothness/singularity of strata (Lakshmibai–Sandhya) — S0.2, later.
- S2 (Ω↔stratum dictionary), surreal thickenings, skeletons.
- Any `amari` dependency; any serialization; any CI files.
