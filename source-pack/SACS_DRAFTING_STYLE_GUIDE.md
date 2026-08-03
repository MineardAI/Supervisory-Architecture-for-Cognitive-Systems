# SACS Drafting Style Guide

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012  
Classification: Informative and subordinate drafting guidance

## Contract 011 Artifact Boundaries

When drafting Contract 011, keep construction, commitment-event evidence, and canonical history distinct. Use `Constructed Supervisory Cycle Record` for the precommit assembly, `Supervisory Cycle Commitment Record` for the final commitment event, and `Committed Supervisory Cycle Record` for the sole authoritative whole-cycle history. State the mutually exclusive `Committed Supervisory Provenance Failure` path whenever Stage Two success is described. Treat `Replay Verification Result` as verification-only and `Audit Representation` as non-constitutive; neither may mutate, replace, invalidate, or reinterpret a committed terminal outcome.

## Standard Section Pattern

Preferred structure:

```text
Section Title

Purpose

Architectural Identity

Constitutional Question

Authority Boundary

Inputs

Outputs

Canonical Artifact

Determinism

Immutability

Failure Conditions

Prohibitions

Invariants

Relationship to Adjacent Sections or Contracts
```

Not every section needs every heading. The governing structure should still be recognizable.

## Voice And Style

Use:

- declarative constitutional language;
- exact artifact names;
- short defining identities;
- explicit ownership;
- explicit prohibitions;
- deterministic formulations;
- SHALL and SHALL NOT statements;
- diagrams where they clarify authority flow;
- equations only where they add precision.

Avoid:

- promotional language;
- vague safety claims;
- anthropomorphic descriptions;
- hidden implementation assumptions;
- implementation-specific defaults;
- words like smart, best, or optimal without a registered definition;
- calling internal conformance verification when NullBlade owns verification;
- describing execution authority as governance authority;
- silently changing canonical artifact names.

## Defining Identity Pattern

Preferred sentence shape:

> [Artifact or process] deterministically [single transformation] without [prohibited authority expansion or reinterpretation].

Examples:

- Observation records evidence without deciding its supervisory meaning.
- The controller computes bounded correction without redefining supervisory intent.
- Translation preserves meaning without broadening authority.
- Historical certification makes realization history authoritative without performing execution.

## Fundamental Invariant Pattern

End mature contracts with one constitutional law that reduces the entire contract to a singular invariant. That final law should answer the contract's constitutional question in one sentence.

## Documentation Rules

- Use exact contract numbers when the number is part of authority.
- Preserve the difference between historical terminology and current adopted terminology.
- If a contract has a conflict, do not collapse it into a smooth narrative.
- If a source is missing, say it is missing.
- If a section is partial, label it partial.
- If a term changed across drafts, keep the canonical term and record the old term as deprecated.

## Canonical Status Labels

Use only these labels unless a source explicitly requires another:

- FOUNDATIONAL
- ADOPTED
- WORKING DRAFT
- PARTIAL
- SUPERSEDED
- DEPRECATED
- UNRESOLVED
- PROPOSED
- EXTERNAL

## Source Traceability

- `SACS Contract 000 Architecture_and_Authority v0.1.0.md` - normative language and architectural tone.
- `SACS Contract 001 Observation_v0.1.0.md` - constitutional identity phrasing for observation.
- `SACS Contract 006 Section 1 Op Profiles-Reference States.md` - defining identity and contract section pattern.
- `SACS Contract 007 Controller.txt` - mature contract structure and control identity pattern.
- `SACS Contract 008 Inference Control.txt` - semantic-preserving translation identity.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - concise defining identity for historical certification.
