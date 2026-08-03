# SACS Constitutional Baseline v1.0.0

## Baseline Identity

Baseline: SACS Constitutional Baseline v1.0.0  
Baseline Version: 1.0.0  
Baseline Status: ADOPTED  
Effective Date: 2026-07-20  
Repository Commit Hash: unavailable; the SACS directory is not a Git repository.  
Normative Scope: Contracts 000-012  
Companion Set: SACS Source Pack v1.0.0

The SACS Constitutional Baseline v1.0.0 establishes Contracts 000-012 as the complete normative supervisory constitution for SACS Version 1.0. All implementations, schemas, protocols, documentation, registries, and derived specifications SHALL conform to this corpus and SHALL NOT broaden, merge, relocate, or silently reinterpret constitutional authority.

Contract 012 -- External Verification Profile and Evidence Interface -- is the terminal SACS contract. Institutional learning, constitutional evolution, verifier-governance evolution, and ecosystem learning are outside the SACS 000-012 core.

## Contract Corpus

| Number | Canonical Title | Baseline Status |
| --- | --- | --- |
| 000 | Architecture and Authority | ADOPTED - SACS v1.0.0 BASELINE |
| 001 | Observation | ADOPTED - SACS v1.0.0 BASELINE |
| 002 | State Estimation | ADOPTED - SACS v1.0.0 BASELINE; source detail is less granular than later contracts |
| 003 | Operational State Vector | ADOPTED - SACS v1.0.0 BASELINE; source detail is less granular than later contracts |
| 004 | Governance | ADOPTED - SACS v1.0.0 BASELINE; source detail is less granular than later contracts |
| 005 | Supervisory Objectives | ADOPTED - SACS v1.0.0 BASELINE; source detail is less granular than later contracts |
| 006 | Operational Profiles and Reference States | ADOPTED - SACS v1.0.0 BASELINE |
| 007 | Controller | ADOPTED - SACS v1.0.0 BASELINE |
| 008 | Inference Control | ADOPTED - SACS v1.0.0 BASELINE |
| 009 | Execution Interface | ADOPTED - SACS v1.0.0 BASELINE |
| 010 | Modes, Guards, Resets, Hysteresis, and Dwell Time | ADOPTED - SACS v1.0.0 BASELINE |
| 011 | Realization Certification, Supervisory Provenance, and Audit | ADOPTED - SACS v1.0.0 BASELINE |
| 012 | External Verification Profile and Evidence Interface | ADOPTED - SACS v1.0.0 BASELINE |

## Normative and Informative Layers

The contract corpus is normative. The Source Pack is a constitutional-supporting companion set whose register, terminology, artifact registry, architecture summary, conflict resolutions, invariants, and review tests support conformance to the contracts. The Implementation Context, drafting guidance, and derived documentation are informative and subordinate unless a document explicitly identifies a narrow constitutional-supporting role.

No file may silently alter constitutional authority after this freeze. A conflict between an informative or derived document and a normative contract SHALL be resolved in favor of the applicable normative contract and recorded as an erratum or Phase 2 finding as appropriate.

## Normative Precedence

For Version 1.0, precedence is:

1. This SACS Constitutional Baseline v1.0.0 for baseline scope, status, and change control.
2. Contract 000 for the architecture-wide authority model and ownership map.
3. Contract-specific normative text in Contracts 001-012 for each contract's authority domain.
4. Adopted constitutional registries and terminology.
5. Conflict and Decision Ledger resolutions for documented historical conflicts where the active corpus adopts a resolution.
6. Invariants and Review Tests.
7. Constitutional Architecture summary.
8. Source Pack summaries.
9. Implementation Context and other informative guidance.
10. Derived documentation and implementation artifacts.

Contract-specific normative text governs its own authority domain. This precedence does not authorize the baseline, a registry, or an informative document to invent authority absent from the applicable contract.

## Frozen Chain and Authority Rules

The frozen constitutional chain is Contracts 000-012. The resolved downstream handoff is:

```text
Committed Translation Transaction
    -> Contract 009 -> Committed Execution Disposition
    -> Contract 010 -> Committed Supervisory Mode
                         Committed Supervisory Continuity Record
    -> Contract 011 Stage One -> Committed Realization Record
    -> Contract 011 Stage Two -> Committed Supervisory Cycle Record
                              or Committed Supervisory Provenance Failure
    -> Contract 012 -> Committed Verification Exchange Record
                       or Committed Verification Exchange Failure
```

Each contract owns one constitutional authority category. Authority may narrow downstream; authority may not broaden downstream. Internal validation or certification is not external verification. SACS does not self-verify. An authorized independent verifier remains the authority for its external verification judgment.

## Source Pack v1.0.0

Source Pack: SACS Source Pack v1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012

Included Source Pack files:

- `SACS_SOURCE_PACK_INDEX.md`
- `SACS_CONTRACT_REGISTER.md`
- `SACS_CONSTITUTIONAL_ARCHITECTURE.md`
- `SACS_TERMINOLOGY_AND_NAMING.md`
- `SACS_ARTIFACT_AND_INTERFACE_REGISTRY.md`
- `SACS_INVARIANTS_AND_REVIEW_TESTS.md`
- `SACS_CONFLICT_AND_DECISION_LEDGER.md`
- `SACS_IMPLEMENTATION_CONTEXT.md` (informative)
- `SACS_DRAFTING_STYLE_GUIDE.md` (informative)
- `SACS_CONTRACT_006_010_CORE.md` (constitutional-supporting summary)

## Change Control

### Editorial Erratum - v1.0.x

Permitted changes are spelling and grammar corrections, broken cross-reference repair, formatting corrections, filename and path corrections, and clarifications that do not change authority, semantics, ownership, artifact identity, required input, terminal output, or failure behavior.

Each erratum SHALL record the affected files, the non-semantic reason for change, and the incremented patch version.

### Backward-Compatible Constitutional Addition - v1.x.0

This category is not authorized during the current freeze pass. A later addition may use this category only if it does not broaden or relocate authority, redefine a canonical artifact, make a conforming implementation nonconforming, or introduce mandatory behavior without a constitutional version decision.

### Constitutional Revision - v2.0.0

A major revision is required for changes to contract authority, numbering, canonical artifact identity, ownership, mandatory inputs or outputs, commitment semantics, failure semantics, constitutional boundaries, verification sovereignty, or the terminal position of Contract 012.

## Historical Material

Archived drafts, superseded numbering, deprecated terms, prior conflict records, older Source Pack versions, and Council review material remain preserved for traceability. They are historical and non-normative after the v1.0.0 freeze. Historical material SHALL NOT appear as current constitutional authority merely because it remains in the repository.

## Baseline Rule

The baseline is the sole normative constitutional source for SACS Version 1.0. Later audits, schemas, protocols, runtime implementations, and derived specifications SHALL derive from this baseline and SHALL identify any deviation as an erratum, a Phase 2 finding, or a later constitutional version change.
