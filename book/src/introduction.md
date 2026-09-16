# Introduction

**Thatch** is the geometry of the edge — the infinitesimal boundary between
something and emptiness, studied as a mathematical object in its own right.

> Named for Edward Teach/Thatch — Blackbeard. The pirate's domain was the
> edge: the shoreline between land and open sea, the map's boundary where
> the known meets the unmarked. This crate studies that boundary as
> mathematics.

## The Thesis

Across the Industrial Algebra stack, two systems approach the same
boundary from opposite sides:

- **Schubert (wall-crossing):** a *wall* is the parameter value at which a
  capability crosses from stable to unstable — from something to nothing.
  The phase diagram counts what exists; the wall is the existence boundary.
- **Karpal (structured emptiness):** the Ω lattice types the *kinds* of
  emptiness, insisting that "empty" is not one thing and that the
  provenance of a zero is load-bearing.

Both stop at the boundary. Neither says what the boundary *is*.

**The thesis of Thatch: the boundary is itself a geometric object.** It has
strata, it carries structure, and it composes when two somethings share an
edge.

| Layer | Question it answers | Home |
|---|---|---|
| Kinds | *What sort* of emptiness is this? | Karpal (Ω) |
| Dynamics | *When/how* does something cross into it? | Schubert (wall-crossing) |
| **Geometry** | *What is the interface itself?* | **Thatch** |

## What Is Implemented

Thatch 0.1 ships two verified pieces of the picture:

1. **The strata substrate** (probe S0) — boundary-stratum posets of
   Schubert varieties on Grassmannians: partitions as codimension data,
   the closure order, per-stratum dimensions.
2. **The Ω↔stratum dictionary** (probe S2) — the structured emptiness
   lattice mirrored dependency-free, mapped onto the poset, with the
   theorem that the *pair* world has no geometric zeros, and the headline
   that **compatibility and composability are independent edges**.

The founding document
([`docs/ideation-edge-geometry.md`](https://github.com/Industrial-Algebra/Thatch/blob/develop/docs/ideation-edge-geometry.md))
records the full program: the established fields Thatch draws on, the
falsifiable probes, and what this crate deliberately is not.

## Placement

Thatch is a foundation crate in the Industrial Algebra "Rich Toolbox": it
depends on nothing in the IA ecosystem. Schubert, Karpal, Amari, and
Minuet may all draw from it — dependency arrows point away from Thatch,
never toward it.

## License

Apache-2.0. Commercial licensing available — contact Industrial Algebra.
