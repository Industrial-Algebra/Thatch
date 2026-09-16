# The Edge — Geometry of the Something/Emptiness Boundary

> **Status:** Founding ideation document. Nothing here is validated. The
> goal is to surface research directions worth testing, not to claim
> results. Established mathematics is stated as fact and sourced; everything
> conjectural is marked as such.
>
> **Revision 2 (2026-08-21):** reconciled with the founding dive
> (IA-documents, `RABBIT_HOLE_2026-08-20_Thatch.md`). Changes: a fifth
> in-stack edge recorded (Schubert's time-axis grant-expiry cliff) and
> probe **S4** added (§7); the two-sided meeting restated as four-sided
> (§1 addendum); §4 sharpened to the one-object-or-four question; probe
> sequencing by consumer pull (§7); the conversation-vs-crate endgame
> recorded (§5).
>
> **Author:** Justin Elliott Cobb, Industrial Algebra
> **Date:** 2026-08-20
> **Project:** Thatch — named for Edward Teach/Thatch, Blackbeard, whose
> domain was the edge itself: the shoreline between land and open sea, the
> boundary of the map where the known meets the unmarked.
>
> **Cross-references:** Karpal `docs/design/structured-emptiness.md` (the Ω
> thesis); Schubert Roadmap #17 / `analyze_composed_stability` + the
> `wall_crossing_probe` baseline; Minuet holographic stores;
> `amari-surreal::EpsilonPolynomial`.

## 1. The thesis

Schubert's compositional wall-crossing and Karpal's structured emptiness were
developed independently, for different consumers. They meet — unexpectedly —
at the same question, viewed from two sides:

- **Schubert (wall-crossing):** a *wall* is the parameter value at which a
  capability crosses from stable to unstable — from **something to nothing**.
  The phase diagram counts what exists; the wall is the existence boundary.
- **Karpal (structured emptiness):** Ω types the *kinds* of emptiness —
  `Denied`, `Granted(0)`, `Granted(n)`, `Granted(∞)`, `Underdetermined` —
  insisting that "empty" is not one thing, and that the provenance of a zero
  is load-bearing.

Both stop at the boundary. Neither says what the boundary *is*.

**Revision 2 addendum — four sides, not two.** The meeting is larger than
two-sided. Karpal's Phase 16 topos work is building Ω into a **subobject
classifier** — in topos theory, precisely the categorical interface between
a subobject and its complement — and the gluing-failure analysis on the
stack's composition side is
**descent**: whether local boundary data composes to global data. The stack
approaches the edge from four sides, one of which (the topos) is already
far along building the general object. The honest restatement of §4: is
Thatch's "third column" a sibling of these, or a stratified, ε-thickened
*instance* of the classifier geometry?

**The thesis of Thatch: the infinitesimal boundary between something and
emptiness (or a singularity) is itself a geometric object** — it has strata,
it carries structure, and it composes when two somethings share an edge. The
layered reading:

| Layer | Question it answers | Home |
|---|---|---|
| Kinds | *What sort* of emptiness is this? | Karpal (Ω) |
| Dynamics | *When/how* does something cross into it? | Schubert (wall-crossing) |
| **Geometry** | *What is the interface itself?* | **Thatch (this document)** |

The origin is visualization, not yet mathematics: contemplating shadow
holograms and their edges. The sections below record the established fields
that already study pieces of this picture — which is evidence the question is
real — and then state honestly what Thatch would have to build.

## 2. Established mathematics (stated as fact)

None of this is Thatch's yet. These are the load-bearing existing theories;
Thatch's ambition is to be the compositional bridge between them in the IA
stack.

### 2.1 Logarithmic geometry

A **log scheme** is a space equipped with a sheaf of monoids recording
boundary and degeneration structure; the theory was invented to do geometry
precisely where families degenerate. The **log point** is the geometry of
"almost nothing": an infinitesimal neighborhood thickened just enough to
remember what crossed it. Log geometry is the leading existing candidate for
a formalism of "the edge as structure" — it makes the boundary a first-class
object rather than a failure.

### 2.2 Formal neighborhoods and normal cones

Given a boundary or singular locus `Y ⊂ X`, the **formal completion** `X̂_Y`
is the genuine infinitesimal neighborhood of the edge — the first-order and
thicker data of the boundary itself. **Deformation to the normal cone**
interpolates between a space and its boundary's normal cone, giving a
canonical family whose special fiber is "the space as seen from the edge."
This is the literal machinery of "what exists infinitesimally at the
boundary."

### 2.3 KS wall-crossing (the dynamical side)

In the BPS source of Schubert's #17, a **wall** is exactly the locus where a
stable state decays into nothing; the phase diagram counts *what exists*,
and the **Kontsevich–Soibelman formula** is the transition function across
the existence↔emptiness boundary. Schubert's wall-crossing engine already
borrows this vocabulary; Thatch would supply the geometry the walls live on.

**Measured baseline (Schubert, v0.5.0):** `analyze_composed_stability` +
`wall_crossing_probe` establish that under the current engine, composed
phase diagrams are **additive** — walls are per-capability. Any *genuine*
non-additivity must come from interaction — i.e., from exactly the
boundary/singularity structure Thatch proposes to study. The instrument and
its honest baseline already exist; the missing mathematics is the engine.

**Revision 2 — scope of the baseline.** The additive baseline was measured
along a *single* parameter axis (trust). Schubert v0.5.0 also ships an
implemented, tested wall on a *different* axis: the grant-expiry cliff
(`crypto.rs` — "a grant is dead the instant `now >= expires_at`; the
boundary is inclusive"). Composition of walls in heterogeneous coordinates
(time × trust) has never been run — see probe S4.

### 2.4 Schubert singular strata and resolution

The boundary (and singular locus) of a Schubert variety is itself a union of
smaller Schubert varieties, determined combinatorially by the partition —
Kazhdan–Lusztig theory governs the intersection cohomology along it. The
**Bott–Samelson resolution** desingularizes that boundary by gluing
ℙ¹-fibers compositionally: "the edge, resolved by composition" is an
existing theorem, and the combinatorics is implementable from partitions
alone.

### 2.5 Skeletons and shadows

In non-archimedean geometry, the **tropicalization** of a variety is
literally called its *skeleton* or *shadow*: a deformation retract carrying
the combinatorial essence of the space. The "shadow hologram" visualization
that seeded this project is standard vocabulary in this corner of the field.
Skeletons are the candidate bridge between the continuous edge and the
combinatorial data IA already computes.

### 2.6 Surreal infinitesimals (in-stack, concrete)

`amari-surreal` ships `EpsilonPolynomial`: the surreals are the natural home
for *infinitesimals as actual quantities*. An ε-thickening of a boundary
need not be a metaphor in this ecosystem — it can be a number. This is the
one piece of the picture that already exists in the IA stack as executable
code.

## 3. The convergence in the IA stack

| Crate | What it already has | What the edge-question means there |
|---|---|---|
| **Schubert** | Wall-crossing engine, phase diagrams, `analyze_composed_stability` (additive baseline measured) | Walls = capability existence boundaries; non-additivity, if it ever appears, lives at shared edges/singular strata |
| **Karpal** | Ω lattice (`IntersectionKind`, `BoundedLattice`, `HeytingAlgebra`); Phase 16 topos work (subobject classifier, sheaves) | Ω types the kinds of emptiness; gluing failures happen at overlaps — i.e., at interfaces |
| **Amari** | `WallCrossingEngine`, Schubert/enumerative core, tropical (`amari-tropical`), `EpsilonPolynomial` | The computational engine + the ε-quantities; skeletons as the combinatorial shadow |
| **Minuet** | Holographic stores (Simple/Sharded), retrieval | The retrieval edge: the locus where a stored memory crosses below retrievability — a candidate wall in memory space |

The unification sensed at the origin is real: Ω gives the kinds,
wall-crossing gives the dynamics, and the geometry of the interface is the
missing third column — the contribution Thatch would make.

## 4. The unifying question (open, deliberately)

> **Is there a compositional geometry of the interface itself — strata of the
> ε-neighborhood of an existence boundary — and does it compose when two
> somethings share an edge?**

Three candidate formalization lenses, not yet adjudicated (this is the
central open choice):

- **(L1) Log-structural:** boundary monoids on the combinatorial objects IA
  already computes (partitions, capability surfaces); walls as log
  degenerations. Strongest existing theory; heaviest to import.
- **(L2) Formal + surreal:** ε-thickenings of boundaries valued in
  `EpsilonPolynomial`; strata as the graded pieces of the thickening. Most
  in-stack-computable; least established as theory.
- **(L3) Skeleton-first:** define the edge as the tropical skeleton of the
  something; study composition of skeletons along shared rays. Bridges to
  the stack's tropical gluing work; risks being *only* the shadow, not the edge.

Conjecture (marked as such): the right answer composes all three — the
skeleton (L3) as the combinatorial shadow, the surreal thickening (L2) as its
quantitative measure, the log structure (L1) as its composition law.

**Revision 2 sharpening — one object or four?** The four in-stack edges
live in different categories: time-orders (grant expiry), truth values (Ω),
similarity scores (Minuet retrieval), Zariski strata (Schubert varieties).
The synthesis conjecture assumes a single geometric object of which these
are presentations. The alternative: Ω is already the whole story at the
discrete level, the stratum poset at the combinatorial level, and the
ε-thickening is a *measure* on strata rather than a geometry — in which
case Thatch's deliverable is a dictionary between existing columns, not a
new column. Current speculation (operator's instinct, recorded as §6.1):
the four may be frame presentations of one object, as Maxwell's four
vector-calculus equations are frame presentations of one bivector equation.

## 5. Placement — the Rich Toolbox direction

Thatch is **its own foundation crate**. The dependency arrow only points
away:

```
                 ┌─────────┐
                 │ Thatch  │  (edge geometry; depends on NOTHING in IA)
                 └────┬────┘
      ┌────────┬──────┼────────┬───────────┐
      ▼        ▼      ▼        ▼           ▼
   Amari     Karpal  Schubert  Minuet   (future consumers)
```

Principles:

- **Phase 0 is dependency-free.** Pure combinatorics and types; no IA deps,
  no circularity risk. (`amari-surreal` is the one tempting dep — for L2
  probes, take it as a dev-dependency or vendor the minimal ε-arithmetic
  until the direction survives.)
- **Amari's `WallCrossingEngine` stays where it is.** Any re-expression on
  Thatch primitives is a later, deliberate migration decision — not a
  prerequisite, and not breakage.
- **What each draws, eventually:** Schubert — interaction-aware wall
  structure (the non-additivity engine its baseline begs for); Karpal —
  geometric referents for Ω's kinds and gluing failures; Amari — boundary
  structure for its Schubert/tropical core; Minuet — a measured notion of
  the retrieval edge.

**Revision 2 — the conversation-vs-crate endgame.** If every probe result
files naturally as a PR in a consumer repo (S4 in Schubert, the Ω-extension
in Karpal, the retrieval measurement in Minuet), Thatch-as-repo may never
need to hold code. The modest outcome is a dictionary — a
`strata(partition) -> Poset` table, an Ω-indexed labeling of its elements,
an `EpsilonPolynomial`-valued grading — or even a vocabulary document with
probes living in their consumers. The program's success and the repo's
success are separable, and this document treats them as such.

**Name note:** `thatch` was verified available on crates.io 2026-08-20.

## 6. Open questions

1. **One object or four?** Is "the edge" a single geometric object of
   which the in-stack edges are presentations — and if one, what
   formalism makes the unity manifest, the way four vector-calculus
   Maxwell equations become one bivector equation in spacetime algebra?
   *(Revision 2 — current speculation focus.)*
2. Which lens (L1/L2/L3) survives contact with a probe — and is the
   three-lens synthesis (§4) real or wishful?
2. What is the boundary object for a *capability surface* specifically —
   what plays the role log structure plays for schemes?
3. Does the Ω ↔ stratum dictionary exist (§7 S2), and is Ω *exhaustive*
   over boundary classes, or does the edge have kinds Ω misses?
4. Can a surreal ε-thickening be made well-defined under the operations IA
   cares about (union, intersection, composition)?
5. Is Minuet's retrieval edge actually wall-like (discontinuous), or smooth
   — and if smooth, does that falsify the frame there or just measure a
   different regime?
6. What is the smallest theorem Thatch could state that is *new* rather
   than an import?
7. Is Thatch a crate or a dictionary between existing columns — what
   observation would distinguish the two? *(Revision 2; see §5 endgame.)*

## 7. Falsifiable probes (small, killable, ordered)

- **S0 — Strata of small Schubert varieties (pure combinatorics).** From
  partitions alone, compute the boundary/singular-stratum poset for all
  Schubert varieties in Gr(2,4) and Gr(3,6) (known combinatorics — Kazhdan–
  Lusztig/Bott–Samelson data). *Falsifies:* nothing established — but if the
  strata cannot be computed cleanly, the whole program loses its cheapest
  substrate. Deliverable: a `strata(partition) -> Poset` table, tested
  against hand-checked cases.
- **S1 — Surreal ε-thickening (L2).** Define `dim(σ_λ + ε)`-style boundary
  measures with `EpsilonPolynomial`; check well-definedness under union and
  intersection of capability surfaces. *Falsifies:* L2 if ill-defined or
  trivially equal to the un-thickened data.
- **S2 — Ω ↔ stratum dictionary (bridge to Karpal).** Map each Ω truth
  value to boundary-stratum classes of intersections (empty /
  zero-dimensional / positive-multiplicity / underdetermined ↔ structural
  zero / geometric zero / granted-n / granted-∞). Test exhaustiveness
  against S0's computed strata. *Falsifies:* the claim that Ω already types
  the edge — if the table has holes, Ω needs extension; if it collapses,
  Ω is redundant there.
- **S3 — Retrieval edge measurement (bridge to Minuet).** Instrument a
  Minuet store's decay/retrieval and test for wall-like discontinuities.
  *Falsifies:* the frame's applicability to memory edges (a negative here
  scopes Thatch to policy-space geometry only).
- **S4 — Heterogeneous wall coordinates (Revision 2; lives inside
  Schubert, needs no Thatch code).** Compose capabilities whose walls sit
  in different parameters — a time-walled grant (`expires_at` cliff) under
  a trust-walled capability — via `analyze_composed_stability`, and watch
  `non_additive_breakpoints`. The instrument and the wall are in the same
  crate; this is the cheapest candidate for *deliberate* non-additivity in
  the stack. *Falsifies:* the cheapest form of "non-additivity lives at
  shared edges" — if heterogeneous-coordinate composition is still
  additive, the conjecture narrows sharply; if breakpoints appear, Thatch
  inherits a validated problem. Either outcome is informative.

**Revision 2 — sequencing by consumer pull.** The list is ordered by cost;
ordered by *consumer pull* it inverts — S2 feeds Karpal (active), S4 feeds
Schubert within one sprint, S3 feeds Minuet (instrument already built), and
S0 feeds nobody yet: it is substrate. Working order: run **S0 + S2 as a
pair**; S4 is Schubert's to run at any time, independent of this repo; S3
waits for Minuet pull.

## 8. What this document is not

- **Not a theorem.** No claim that the boundary geometry exists as a coherent
  object, in general or for capability surfaces.
- **Not a spec.** No API, no crate layout beyond the scaffold. S0–S3 must
  survive first; the first public API follows the first surviving probe.
- **Not a commitment from the other crates.** Nothing in Amari, Karpal,
  Schubert, or Minuet changes because Thatch exists. Adoption, if ever, is
  driven by a probe result someone wants.

## 9. Decision asks and next steps

1. Does §4's question survive scrutiny — one object or four — and does the
   formalism speculation (§6.1) hold up?
2. Approve **S0 + S2 as the paired first target** (S0 the substrate, S2
   the adjudicator — where the program is vindicated or absorbed)?
3. Offer **S4 to Schubert** as an in-crate probe, independent of Thatch's
   own schedule?
4. Repo logistics: GitHub remote exists; Forgejo mirror when ready
   (`/skill:ia-forgejo-mirror`); branch discipline from the next change
   after the bootstrap closes.

*The map ends at the shoreline. Thatch is the mathematics of the waterline.*
