# The Probe Program

Thatch grows by **falsifiable probes** — small, killable units, ordered so
that early probes are prerequisites for later ones. A probe either earns
its place in the crate or dies honestly. The founding document (§7) is
the authority; this chapter tracks status.

## S0 — Strata substrate ✅

*Build the boundary.* Partitions as codimension data, Grassmannians,
boundary-stratum posets with dimensions and closure pairs.

**Outcome: landed.** The [strata substrate](../api/strata.md) is the
crate's foundation. Two authoring-fixture errors were caught by
independent exhaustive enumeration during verification (Gr(3,6)
λ=(2,1,0) boundary is 13, not 14; λ=(3,0,0) open cell is 6, not 3) —
enumeration beats recall.

## S2 — The Ω↔stratum dictionary ✅

*Adjudicate the correspondence.* Map the ecosystem's structured-emptiness
lattice onto the poset.

**Outcome: landed, with a theorem and a headline.** For pairs,
`GeometricZero` is unreachable (the set-meet is always again a Schubert
variety) and Ω's richer values are exactly what multi-way products need.
The headline — λ=μ=(2,1) on Gr(2,4) is **compatible but incomposable** —
separates the two edges the word "intersection" conflates. This is the
design input for Schubert's B3 interaction term and Workstream C's
geometry-typed grants.

## S4 — Heterogeneous wall coordinates ✅ (in Schubert)

*Cross a time wall with a trust wall.* An expiring grant backing a
capability that shares a partition with a permanent one — does the
composed phase diagram shift at the cliff?

**Outcome: measured ADDITIVE** (Schubert PR #57). The λ=[2] wall survives
`temp_sign`'s death because `audit` holds it; breakpoints identical
across the time cliff. The trust×time product baseline is pinned by six
regression tests — an interaction-aware engine would have to move those
numbers to claim emergence.

## S1 — Surreal ε-thickening ⏳

*Thicken the boundary.* Use surreal infinitesimals (the Amari/Minuet
in-stack tradition) to give the edge positive-but-infinitesimal width.

**Status: open.** The natural home is a Thatch 0.2 direction; depends on
the surreal number machinery that lives in the ecosystem today.

## S3 — Minuet retrieval edge ⏳

*The boundary as retrieval structure.* Minuet's holographic store maps
queries to geometric positions; the edge is where retrieval succeeds with
zero recall.

**Status: open.** Awaits a deliberate design pairing with Minuet's
`sharded`/`store` machinery.

## The Discipline

Every probe that lands adds a pinned, regression-tested measurement —
not prose. When a probe's prediction fails, the failure is the finding:
the S4 probe would have reported heterogeneous emergence exactly as
honestly as it reports additivity. The program's results feed the
[roadmap](./roadmap.md).
