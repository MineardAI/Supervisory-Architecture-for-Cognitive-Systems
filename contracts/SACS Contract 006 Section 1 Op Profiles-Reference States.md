SACS Contract 006 — Operational Profiles & Reference States

Project: SACS — Supervisory Architecture for Cognitive Systems

Contract: 006

Title: Operational Profiles & Reference States

Version: 1.0.0

Status: ADOPTED - SACS v1.0.0 BASELINE

Normative Authority:
Operational Profile selection, supervisory configuration, Reference State construction, and Controller handoff.

Depends On

Contract 000 — Architecture and Authority
Contract 003 — Operational State Vector
Contract 004 — Governance
Contract 005 — Supervisory Objectives

Canonical Inputs

Committed Operational State Vector
Committed Governance Snapshot
Committed Objective Set

Canonical Outputs

Profile Selection Report
Committed Operational Profile
Committed Reference State

Canonical Consumer

Contract 007 — Controller

1. Purpose

Contract 006 defines the supervisory configuration layer of SACS.

It establishes:

Operational Profile identity
Profile Registry
Profile eligibility
Profile compatibility
deterministic profile selection
deterministic tie-breaking
supervisory operating regions
supervisory limits
gain-set selection
Reference Builder
Reference State construction
Reference State validation
Reference State publication
Controller handoff
profile provenance
profile-selection provenance
conformance requirements

Contract 006 translates declarative supervisory intent into a complete supervisory operating configuration suitable for bounded control.

It does not compute corrective control.

2. Architectural Identity

Operational Profiles answer one question:

"Given this operational state, governance, and objective set, what supervisory configuration should exist?"

They do not answer:

"What command should be executed?"

That belongs to Contract 007.

3. Contract 006 Owns

Contract 006 exclusively owns:

Operational Profile definitions
Profile Registry
Candidate Profile generation
Profile eligibility
Profile compatibility
Profile scoring
deterministic tie-breaking
Profile selection
Profile transition policy
supervisory operating assumptions
operating regions
supervisory limits
gain-set identifiers
Reference Builder
Reference State construction
Reference State validation
Reference State publication
Controller handoff
Profile provenance
Profile Selection provenance

No other contract may redefine these concepts.

4. Contract 006 Produces

Contract 006 produces four canonical artifacts.

4.1 Profile Selection Report

Records every deterministic decision made during profile selection.

Consumers:

Replay
Audit
NullBlade
Provenance
Verification
4.2 Committed Operational Profile

Represents the supervisory configuration selected for the current supervisory cycle.

Consumers:

Reference Builder
Controller
Provenance
4.3 Committed Reference State

Represents the numerical supervisory target generated from the selected Operational Profile.

Consumers:

Controller
4.4 Profile Provenance

Records the complete lineage of profile evaluation and Reference State generation.

Consumers:

Contract 011
Contract 012
5. Contract 006 Must Never Do

Contract 006 SHALL NOT

redefine Governance
redefine Objectives
reinterpret OSV semantics
consume raw observations
consume Estimated State directly
compute controller outputs
generate inference commands
execute runtime actions
optimize supervisory objectives
search for superior objectives
redefine controller mathematics
define gain equations
broaden governance authority
modify committed artifacts

Everything entering Contract 006 is already canonical.

Everything leaving Contract 006 is configuration.

6. Canonical Supervisory Flow
Committed OSV
        │
        ▼
Committed Governance Snapshot
        │
        ▼
Committed Objective Set
        │
        ▼
Candidate Profiles
        │
        ▼
Eligibility Evaluation
        │
        ▼
Compatibility Evaluation
        │
        ▼
Deterministic Profile Selection
        │
        ▼
Profile Selection Report
        │
        ▼
Committed Operational Profile
        │
        ▼
Reference Builder
        │
        ▼
Committed Reference State
        │
        ▼
Controller

Every stage SHALL perform exactly one supervisory transformation.

No stage may combine responsibilities belonging to another contract.

7. Architectural Invariant

The fundamental invariant of Contract 006 is:

Profiles configure supervision. Controllers compute supervision.

Therefore,

Profiles never compute corrective actions.
Controllers never select Profiles.
Objectives never define controller parameters.
Governance never defines controller gains.
Reference States never compute control effort.

Each supervisory artifact has exactly one producer and one category of authority.

8. Canonical Artifact Chain

Contract 006 extends the immutable supervisory artifact chain defined by Contract 000:

Observation Batch
        ↓
Estimated State
        ↓
Operational State Vector
        ↓
Governance Snapshot
        ↓
Objective Set
        ↓
Profile Selection Report
        ↓
Operational Profile
        ↓
Reference State
        ↓
Controller

Every artifact SHALL be immutable after commitment.

A subsequent supervisory cycle SHALL produce a new artifact rather than modifying an existing one.

9. Configuration Completeness

Contract 006 SHALL ensure that the Controller receives a complete supervisory configuration.

The Controller SHALL NOT require:

Governance lookups
Objective evaluation
Profile evaluation
Registry searches
Eligibility evaluation
Compatibility evaluation

The Controller SHALL consume only committed supervisory artifacts.

This establishes the principle of Configuration Completeness:

The Controller receives everything required for bounded corrective control, and nothing beyond its authority.

This opening chapter establishes the contract's authority boundaries before introducing registries, profile semantics, selection algorithms, and the Reference Builder. It also incorporates the Council's strongest recommendations: elevating the Reference State to a first-class artifact, introducing a replayable Profile Selection Report, and making Configuration Completeness an explicit architectural invariant. These choices reinforce the single-responsibility authority model established in Contract 000 and preserve a clean separation between configuration and control.
