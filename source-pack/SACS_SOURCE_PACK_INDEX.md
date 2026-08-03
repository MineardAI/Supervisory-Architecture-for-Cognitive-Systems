# SACS Constitutional Source Pack Index

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012  
Classification: Constitutional-supporting source-pack index

## Project Identity

**Project name:** SACS - Supervisory Architecture for Cognitive Systems  
**Public identity:** SACS "The Watcher"  
**Architectural purpose:** SACS is the constitutional supervisory control plane for cognitive, generative, agentic, and adaptive systems. It observes runtime behavior, estimates state, applies supplied governance, selects supervisory conditions, computes bounded corrective action, translates that action into runtime-specific representation, submits execution through an interface, and preserves authoritative history.  
**What SACS is not:** SACS is not the supervised system, not a primary task generator, not a model host, and not an external verification authority.  
**Verification authority:** External verification belongs to NullBlade. SACS may validate, certify, or commit within its own constitutional scope, but it does not self-authorize external truth.  
**Normative language convention:** SHALL, SHALL NOT, MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT, MAY, and OPTIONAL are normative only when used in a contract or contract-derived constitutional statement.

## Canonical Architecture

SACS follows a supervised-to-history constitutional flow.

```text
Observation
    ↓
State Estimation
    ↓
Operational State Vector
    ↓
Governance
    ↓
Supervisory Objectives
    ↓
Operational Profiles and Reference States
    ↓
Controller
    ↓
Inference Control
    ↓
Execution Interface
    ↓
Modes, Guards, Resets, Hysteresis, and Dwell Time
    ↓
Realization Certification, Supervisory Provenance, and Audit
    ↓
Observation of Consequences
```

The conflict ledger preserves the historical numbering dispute: an archived historical-certification draft had been labeled Contract 010. The active canonical chain is resolved as Contract 010 supervisory continuity followed by Contract 011 realization certification, provenance, and audit.

The canonical adjacent-contract handoff is:

```text
Contract 009
Committed Execution Disposition
        v
Contract 010
Committed Supervisory Mode
Committed Supervisory Continuity Record
        v
Contract 011
Committed Realization Record
        v
Committed Supervisory Cycle Record
```

Contract-level architecture:

```text
Observation
    v
State Estimation / OSV
    v
Governance
    v
Objectives
    v
Contract 006 - Operational Profiles and Reference States
    v
Contract 007 - Controller
    v
Contract 008 - Inference Control
    v
Contract 009 - Execution Interface
    v
Contract 010 - Modes, Guards, Resets, Hysteresis, and Dwell Time
    v
Contract 011 - Realization Certification, Supervisory Provenance, and Audit
    v
Contract 012 - External Verification Profile and Evidence Interface
```

Contract 011 contains two sequential constitutional stages:

```text
Committed Execution Disposition + Committed Supervisory Continuity Record
    v
Realization Certification
    v
Committed Realization Record
    v
Supervisory Provenance
    v
Constructed Supervisory Lineage
    v
Lineage Completeness Evaluation Result + Lineage Integrity Evaluation Result
    v
Constructed Supervisory Cycle Record
    v
Supervisory Cycle Commitment
    +------------------------------+
    v                              v
Committed Supervisory Cycle Record  Committed Supervisory Provenance Failure
    v
Contract 012
Committed Verification Exchange Record
or
Committed Verification Exchange Failure
```

The terminal Stage Two artifact is consumed by deterministic replay and audit only through non-authoritative `Replay Verification Result` and `Audit Representation` outputs. Neither output modifies or replaces the terminal constitutional outcome.

Contract 012 is the terminal SACS contract. It consumes the Committed Supervisory Cycle Record and governs one bounded inter-constitutional verification exchange. External verifier sovereignty remains separate from SACS custody, ownership, historical authority, and commitment authority.

## Foundational Constitutional Doctrines

- One authority category per contract.
- Committed artifacts are the canonical inputs to downstream contracts.
- Canonical artifacts are immutable after commitment.
- No hidden authority may be introduced by implementation convenience.
- Authority may narrow downstream, but it may not broaden.
- No silent repair, no implicit defaults, and no best-effort canonicalization.
- Failure is explicit and legitimate.
- Deterministic replay is required for identical committed inputs and registered versions.
- Exact identifier, version, and registry binding are mandatory where a contract requires them.
- Construction is not commitment.
- Validation is not authority certification.
- Internal certification is not external verification.
- Execution is not governance.
- Observation is not historical reconstruction.

## How To Use The Pack

- **Drafting:** consult `SACS_DRAFTING_STYLE_GUIDE.md` first, then `SACS_TERMINOLOGY_AND_NAMING.md`, then the contract register.
- **Architectural review:** consult `SACS_CONSTITUTIONAL_ARCHITECTURE.md`, the invariants file, and the conflict ledger.
- **Terminology:** consult `SACS_TERMINOLOGY_AND_NAMING.md`.
- **Artifact schemas:** consult `SACS_ARTIFACT_AND_INTERFACE_REGISTRY.md`.
- **Implementation:** consult `SACS_IMPLEMENTATION_CONTEXT.md`; do not treat it as normative.
- **Conflict resolution:** consult `SACS_CONFLICT_AND_DECISION_LEDGER.md`.
- **Provenance and status:** consult `SACS_CONTRACT_REGISTER.md` and the traceability sections in each file.
- **Current drafting state:** consult the contract register for each contract and the conflict ledger for unresolved items.

## Status Summary

- **ADOPTED - SACS v1.0.0 BASELINE:** Contracts 000-012. Contracts 011 Sections 1-28 and 012 Sections 1-23 are assembled; Contracts 002-005 retain lower source granularity without being classified as partial.
- **PHASE 2 REVIEW STATUS:** The constitutional consistency audit, uniqueness audits, full handoff audit, and implementation conformance checks are not claimed complete by this freeze.
- Institutional learning, constitutional evolution, verifier-governance evolution, and ecosystem learning remain outside the SACS 000-012 core.

## Source Traceability

- `SACS Contract 000 Architecture_and_Authority v0.1.0.md` - project identity, canonical flow, authority model, and contract 010/011 references.
- `SACS Contract 001 Observation_v0.1.0.md` - observation role and canonical entry point.
- `SACS Contract 006 Section 1 Op Profiles-Reference States.md` - operational profile and reference-state role.
- `SACS Contract 007 Controller.txt` - controller authority and output boundary.
- `SACS Contract 008 Inference Control.txt` - semantic-preserving translation boundary and Contract 009 consumer boundary.
- `SACS Contract 009 Execution Interface.txt` - execution adjudication baseline.
- `SACS Contract 010 Section 1 Modes Guards Resets Dwell Time.md` through `SACS Contract 010 Section 5 Modes Guards Resets Dwell Time.md` - supervisory continuity baseline.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - assembled Contract 011 draft with Sections 1-28.
- `SACS Contract 011 Section 1 Realization Certification-Supervisory Provenance-Audit.txt` and `SACS Contract 011 Section 2.txt` through `SACS Contract 011 Section 28.txt` - latest numbered Contract 011 section sources.
- `SACS Contract 012 External Verification Profile and Evidence Interface.txt` - assembled Contract 012 Sections 1-23.
- `SACS Contract 012 Section 2 Authority Boundary.txt` through `SACS Contract 012 Section 23 Architectural Completion-Successor Contracts.txt` - latest numbered Contract 012 section sources.
