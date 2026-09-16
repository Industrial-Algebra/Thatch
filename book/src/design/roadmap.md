# Roadmap

## 0.1.0 — The combinatorial substrate

**Shipped surface:**

- Strata substrate (S0): `Partition`, `Grassmannian`, `strata` —
  boundary posets with dimensions and closure pairs
- Ω↔stratum dictionary (S2): `OmegaValue`, `pairing`,
  `omega_pair`, `omega_composition`, `Partition::join_componentwise`
- Zero dependencies; CI matrix; pinned nightly toolchain

**Probes closed:** S0, S2, S4 (in Schubert).

## 0.2.0 — Toward the thickened edge (planned)

The directions under consideration, in dependency order:

1. **Interaction term consumption.** Schubert's B3 (`#17`) draws on the
   dictionary behind a feature gate — the first downstream consumer of
   Thatch geometry in an access-control engine.
2. **Surreal ε-thickening (S1).** Infinitesimal width for the boundary:
   log-geometry flavor without the log geometry, via the ecosystem's
   surreal machinery.
3. **Multi-way dictionary.** Where `GeometricZero` genuinely earns its
   place: products of three or more constraints, LR table combinatorics,
   the setting Workstream C (geometry-typed grants, Schubert 0.8.0)
   needs.

## Later / open

- **S3 — Minuet retrieval edge.** The boundary as the zero-recall face
  of holographic retrieval.
- **Skeletons and shadows.** Combinatorial cores retaining intersection
  theory — the bridge from posets to homological invariants.
- **The classifier question.** Whether Thatch's third column is a
  sibling of the topos-theoretic subobject classifier or a stratified,
  ε-thickened *instance* of it (founding doc §4, restated).

## Non-goals

- No general-purpose geometry library.
- No theorem proving — verified combinatorics only.
- No IA-ecosystem dependencies in `[dependencies]` — ever. The arrow
  points away from Thatch.
