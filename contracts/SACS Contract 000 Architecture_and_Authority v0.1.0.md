# SACS Contract 000 — Architecture and Authority

**Project:** SACS — Supervisory Architecture for Cognitive Systems  
**Contract:** 000  
**Title:** Architecture and Authority  
**Version:** 1.0.0  
**Status:** ADOPTED - SACS v1.0.0 BASELINE  
**Normative Authority:** Foundational  
**Verification Authority:** External — NullBlade Verification Engine  

---

## 1. Purpose

This contract establishes the foundational architecture, authority boundaries, conformance expectations, and contract ownership model for SACS.

SACS is a deterministic supervisory control-plane architecture for cognitive, generative, agentic, and adaptive software systems.

Its purpose is to observe runtime behavior, estimate operational state, publish a temporally atomic state representation, apply externally supplied governance constraints, evaluate supervisory objectives, activate an operational profile, construct a reference state, compute bounded corrective control, translate control into runtime instructions, submit those instructions through an execution gate, and preserve complete supervisory provenance.

SACS does not generate primary task outputs.

SACS supervises systems that do.

---

## 2. Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are normative conformance terms.

A requirement marked **MUST**, **MUST NOT**, **SHALL**, or **SHALL NOT** is binding on every conforming implementation within the applicable capability profile.

A requirement marked **SHOULD** or **SHOULD NOT** may be departed from only when the implementation records a justified reason and preserves all higher-order invariants.

A requirement marked **MAY** is optional.

---

## 3. Architectural Identity

SACS is a supervisory architecture.

It is not:

- a language model;
- a reasoning engine;
- a content-generation system;
- a model-training framework;
- a constitutional policy author;
- a theorem prover;
- a verification authority;
- a persona engine;
- a symbolic vocabulary owner;
- a model-hosting platform.

SACS may supervise any of these systems through typed interfaces.

The defining identity of SACS is:

> SACS observes, estimates, governs through supplied constraints, selects supervisory operating conditions, computes bounded corrective action, and preserves lineage without performing the supervised system’s primary work.

---

## 4. Canonical System Flow

A conforming SACS implementation MUST preserve the following logical flow:

```text
Observation
    ↓
State Estimation
    ↓
Operational State Vector
    ↓
Governance Snapshot
    ↓
Supervisory Objectives
    ↓
Operational Profile Selection
    ↓
Reference State Construction
    ↓
Controller
    ↓
Inference Control
    ↓
Execution Interface
    ↓
Observed Consequences
    ↺
```

The loop is closed only when execution consequences are returned through the Observation layer.

No downstream execution result may directly mutate supervisory state.

---

## 5. Foundational Architectural Invariant

Each layer MUST perform exactly one category of authority.

### 5.1 Observation

Observation measures or receives telemetry without assigning supervisory meaning.

Observation MUST NOT classify the system into an operational profile, apply governance, compute control, redefine source telemetry, or infer hidden state beyond source-declared metadata.

### 5.2 State Estimation

State Estimation transforms observations into an uncertainty-aware estimate of operational state.

State Estimation MUST NOT select objectives, activate profiles, modify governance, or issue execution commands.

### 5.3 Operational State Vector

The Operational State Vector, or OSV, publishes one immutable and temporally atomic state snapshot.

The OSV MUST NOT contain fields assembled from incompatible observation windows, mutate after publication, hide field validity or uncertainty, or redefine source semantics.

### 5.4 Governance

Governance supplies invariant constraints, permissions, prohibitions, and execution eligibility.

Governance MUST NOT be authored or modified by the controller.

### 5.5 Supervisory Objectives

Supervisory Objectives declare the operational conditions the system is intended to preserve or approach.

Objectives MUST be declarative and MUST NOT contain control implementation.

### 5.6 Operational Profiles

Operational Profiles translate active objectives and governance constraints into mode-specific expectations, limits, gains, permissions, and reference-state parameters.

Profiles MUST NOT redefine governance.

### 5.7 Controller

The Controller computes bounded corrective action intended to reduce deviation from the active reference state.

The Controller MUST NOT redefine objectives, reinterpret governance, alter OSV semantics, select its own authority, or generate task content.

### 5.8 Inference Control

Inference Control translates abstract control outputs into typed runtime instructions.

Inference Control MUST NOT silently broaden the authority of a control command.

### 5.9 Execution Interface

The Execution Interface accepts, rejects, defers, restricts, or applies runtime instructions.

Execution MUST NOT redefine supervision.

---

## 6. Separation of Authority

### 6.1 SACS Owns

SACS owns:

- observation contracts;
- observation-window lifecycle;
- estimator interfaces;
- uncertainty representation;
- OSV publication;
- supervisory objective evaluation;
- operational profile activation;
- reference-state construction;
- control-error computation;
- bounded supervisory commands;
- inference-control translation;
- stabilization transitions;
- supervisory cycle lineage;
- SACS conformance declarations.

### 6.2 SACS Does Not Own

SACS does not own:

- foundational model inference;
- semantic content generation;
- model training;
- constitutional policy authorship;
- external governance content;
- persona construction;
- symbolic vocabulary definition;
- fast-weight implementation;
- tool internals;
- model routing internals;
- verification verdicts;
- accreditation claims.

These authorities belong to external systems or adapters.

---

## 7. External System Relationships

### 7.1 IBOS

IBOS and SACS have separate authority.

```text
IBOS
Defines invariant governance, permissions, prohibitions,
execution eligibility, and audit requirements.

SACS
Maintains operational behavior within those constraints.
```

SACS MAY consume an immutable governance snapshot from IBOS.

SACS MUST NOT modify, reinterpret, weaken, or silently substitute that snapshot.

Let:

\[
\mathcal{G}_t = \operatorname{GovernanceSnapshot}(t)
\]

and:

\[
u_t =
\operatorname{Controller}
\left(
X_t^{\mathrm{OSV}},
X_{\mathrm{ref},t},
\mathcal{G}_t
\right)
\]

Then:

\[
u_t \in \mathcal{U}_{\mathrm{allowed}}(\mathcal{G}_t)
\]

A controller output outside the allowed set MUST be rejected before execution.

### 7.2 Fast Delta Engine

Fast Delta Engine, or FDE, is an observed and potentially controlled subsystem.

FDE MAY provide residual-state deltas, KV-cache deltas, activation strain, update magnitude, low-rank update estimates, transient-state telemetry, calibration confidence, and layer-level diagnostics.

SACS MAY issue bounded instructions such as reducing update gain, suspending adaptive writes, resetting transient state, clamping update rank, changing sampling parameters, or requesting deterministic fallback.

SACS MUST NOT redefine FDE’s internal update mathematics.

### 7.3 Presence Lattice

Presence Lattice MAY provide deterministic semantic-field observations or estimator inputs.

```text
Interaction Observations
        ↓
Presence Lattice
        ↓
Structured Semantic Field
        ↓
SACS Observation or State Estimation
```

Presence Lattice answers:

> What is present?

SACS answers:

> Given the committed operational state and active constraints, what supervisory response is required?

Presence Lattice MUST NOT independently issue SACS control actions unless authorized through a conforming adapter.

SACS MUST NOT redefine Presence Lattice semantics.

### 7.4 N&T Core

N&T Core MAY provide typed cognitive or operational telemetry, including noticing state, tensor-state summaries, recursion depth, affective weighting, tether state, symbolic activation, silence pressure, and action-eligibility signals.

Such data MUST enter SACS as observations or estimator inputs.

N&T Core MUST NOT directly activate a SACS operational profile except through an explicitly defined and governed interface.

### 7.5 NullBlade Verification Engine

SACS declares verifiable claims.

NullBlade Verification Engine evaluates those claims.

```text
SACS Contract
    ↓
NullBlade Verification Profile
    ↓
Analytical Checks
    ↓
Simulation Checks
    ↓
Runtime Evidence
    ↓
Verification Registry
```

SACS MUST NOT declare itself verified solely on the basis of its own runtime output.

Verification status MUST originate from an external assurance authority.

---

## 8. Canonical Operational State

All higher supervisory layers MUST operate on a committed OSV.

Higher layers MUST NOT consume raw telemetry directly.

The initial architectural grouping is:

\[
X_t^{\mathrm{OSV}}
=
\begin{bmatrix}
x_t^{\mathrm{stability}}\\
x_t^{\mathrm{drift}}\\
x_t^{\mathrm{strain}}\\
x_t^{\mathrm{resonance}}\\
x_t^{\mathrm{uncertainty}}\\
x_t^{\mathrm{recursion}}\\
x_t^{\mathrm{execution}}\\
x_t^{\mathrm{continuity}}
\end{bmatrix}
\]

This grouping is non-exhaustive and non-normative at the field-definition level.

Exact field names, units, ranges, confidence semantics, uncertainty semantics, validity rules, serialization, and registry ownership belong to Contract 003.

Terms such as *stability*, *drift*, *strain*, *resonance*, *uncertainty*, *recursion*, and *continuity* MUST NOT be implemented normatively until defined by their owning contracts or registries.

---

## 9. Temporal Atomicity

Every supervisory cycle MUST operate on one immutable OSV snapshot.

Let:

\[
X_t^{\mathrm{OSV}}
=
\operatorname{Commit}
\left(
\widehat X_t,
\tau_t,
v_t
\right)
\]

where:

- \(\widehat X_t\) is the estimated state;
- \(	au_t\) is the observation cutoff;
- \(v_t\) is the applicable schema and registry version set.

A conforming implementation MUST ensure that all mandatory OSV fields are derived from observations valid for the same supervisory cycle, the observation cutoff is recorded, the committed OSV is immutable, higher layers read only the committed snapshot, later observations produce a new OSV rather than mutating the active one, and incompatible registry versions are not silently combined.

Temporal atomicity is a foundational SACS invariant.

---

## 10. Supervisory Cycle

Each supervisory cycle MUST follow this logical sequence:

```text
1. Open observation window
2. Collect typed observations
3. Record unavailable or invalid observations
4. Close observation window
5. Estimate operational state
6. Commit immutable OSV
7. Load immutable governance snapshot
8. Evaluate supervisory objectives
9. Select operational profile
10. Construct reference state
11. Compute control error
12. Produce bounded control action
13. Translate control into inference commands
14. Submit commands through execution gate
15. Record execution disposition
16. Commit supervisory provenance
```

A cycle MUST either complete with a canonical lineage record or terminate with an explicit fault or incomplete-cycle disposition.

A conforming implementation MUST NOT silently skip a mandatory stage.

---

## 11. Control Boundary

The control error is:

\[
e_t =
X_{\mathrm{ref},t}
-
X_t^{\mathrm{OSV}}
\]

The controller produces:

\[
u_t =
\mathcal{K}_{q_t}
\left(
e_t,
X_t^{\mathrm{OSV}},
\mathcal{G}_t,
\Theta_t
\right)
\]

where:

- \(q_t\) is the active operational profile;
- \(\mathcal{G}_t\) is the immutable governance snapshot;
- \(\Theta_t\) is the versioned controller configuration;
- \(u_t\) is a bounded supervisory command.

The controller MAY reduce deviation.

The controller MUST NOT redefine the objective, governance snapshot, OSV, profile authority, state semantics, or execution permissions.

---

## 12. Initial Operational Modes

The initial SACS mode vocabulary is:

```text
Entry
Bloom
Harmonic
Fracture
Stabilize
```

These names denote supervisory operating regimes.

They are not emotional diagnoses, persona labels, content classifications, or user judgments.

Exact definitions for guards, transition precedence, hysteresis, dwell time, reset maps, permissions, entry conditions, exit conditions, and failure handling belong to Contract 010.

No implementation may claim conformance for these modes until their normative definitions are adopted.

---

## 13. Failure Principle

SACS MUST fail toward reduced authority.

When state certainty, observation timing integrity, estimator validity, governance availability, profile validity, controller configuration, or interface compatibility is unavailable, invalid, stale, incompatible, or below required confidence, SACS MUST NOT increase execution freedom.

The default authority precedence is:

```text
Block
    >
Stabilize
    >
Defer
    >
Restrict
    >
Allow
```

The exact disposition vocabulary and decision semantics belong to Contract 004 and Contract 009.

---

## 14. Determinism Boundary

SACS supervisory decisions MUST be deterministic with respect to identical canonical inputs.

For identical committed OSV, governance snapshot, objective set, active profile, reference state, controller configuration, registry versions, and implementation capability profile, a conforming implementation MUST produce the same canonical supervisory decision.

Formally:

\[
F(X,\mathcal G,\mathcal O,\mathcal P,\Theta,v)
=
F(X,\mathcal G,\mathcal O,\mathcal P,\Theta,v)
\]

across conforming implementations, subject to declared numeric profiles.

Probabilistic behavior MAY occur downstream in supervised systems.

Probabilistic generation MUST NOT occur inside the canonical supervisory decision path unless a later contract explicitly defines a deterministic normalization and conformance procedure.

---

## 15. Provenance Requirement

Every canonical supervisory decision MUST record at least:

```text
cycle_id
previous_cycle_id
observation_window_id
observation_cutoff
observation_registry_version
estimator_id
estimator_version
osv_id
osv_schema_version
governance_snapshot_id
objective_set_id
profile_id
profile_version
reference_state_id
controller_id
controller_version
control_error
control_command
inference_commands
execution_disposition
execution_receipt_id
lineage_schema_version
```

No control command is canonical without a lineage record.

Exact schemas and serialization belong to Contract 011.

---

## 16. Assumptions

Contract 000 assumes that:

1. external systems can expose at least a declared minimum set of typed observations;
2. observation sources can report unavailable, invalid, stale, or unsupported fields;
3. the estimator can represent uncertainty and missing information;
4. governance snapshots are immutable during a supervisory cycle;
5. profile definitions are versioned;
6. control commands can be bounded;
7. the execution interface can reject unauthorized commands;
8. all canonical artifacts can be assigned stable identifiers;
9. registry versions are available at decision time;
10. verification is performed by an external authority.

These are architectural assumptions, not verified guarantees.

Later contracts MUST state which assumptions they discharge, retain, or replace.

---

## 17. Constraints

A conforming SACS implementation is constrained by the following:

- higher layers may consume only committed OSV state;
- no layer may silently assume another layer’s authority;
- governance cannot be weakened by downstream components;
- uncertainty cannot be discarded without an explicit rule;
- missing telemetry cannot be silently replaced with a valid value;
- schema incompatibility cannot be ignored;
- execution cannot retroactively redefine the supervisory decision;
- self-verification is prohibited;
- control must remain bounded by active governance and profile limits;
- all canonical decisions require lineage.

---

## 18. Non-Goals

Contract 000 does not define observation field schemas, observation timing budgets, estimator mathematics, OSV fields or ranges, governance vocabulary, objective syntax, profile parameters, controller gains, inference-command schemas, execution adapters, mode guards, reset equations, dwell-time rules, serialization formats, transport protocols, verification proofs, deployment topology, or model-specific telemetry.

These are intentionally delegated to later contracts.

---

## 19. Terminology and Definition Ownership

| Term | Status in Contract 000 | Normative Owner |
|---|---|---|
| Observation | Architectural | Contract 001 |
| Observation Window | Architectural | Contract 001 |
| Estimated State | Architectural | Contract 002 |
| Uncertainty | Architectural | Contract 002 |
| Operational State Vector | Architectural | Contract 003 |
| Stability | Placeholder | Contract 003 |
| Drift | Placeholder | Contract 003 |
| Strain | Placeholder | Contract 003 |
| Resonance | Placeholder | Contract 003 |
| Continuity | Placeholder | Contract 003 |
| Governance Snapshot | Architectural | Contract 004 |
| Supervisory Objective | Architectural | Contract 005 |
| Operational Profile | Architectural | Contract 006 |
| Reference State | Architectural | Contract 006 |
| Control Error | Architectural | Contract 007 |
| Control Command | Architectural | Contract 007 |
| Inference Command | Architectural | Contract 008 |
| Execution Disposition | Architectural | Contract 009 |
| Entry | Placeholder | Contract 010 |
| Bloom | Placeholder | Contract 010 |
| Harmonic | Placeholder | Contract 010 |
| Fracture | Placeholder | Contract 010 |
| Stabilize | Placeholder | Contract 010 |
| Lineage Record | Architectural | Contract 011 |
| Verification Profile | Architectural | Contract 012 |

A separate SACS Glossary MAY provide explanatory definitions, but it MUST NOT override the contract that owns a normative term.

---

## 20. Concept Ownership Matrix

| Concept | Owner Contract | Canonical Producer | Canonical Consumer |
|---|---:|---|---|
| Observation Type | 001 | Observation Registry | Observation Sources |
| Observation Record | 001 | Observation Layer | State Estimator |
| Observation Window | 001 | Observation Layer | State Estimator |
| Estimated State | 002 | State Estimator | OSV Publisher |
| Estimator Uncertainty | 002 | State Estimator | OSV Publisher |
| Operational State Vector | 003 | OSV Publisher | Governance, Objectives, Profiles, Controller |
| Governance Snapshot | 004 | External Governance Adapter | Objectives, Profiles, Controller, Execution Gate |
| Supervisory Objective | 005 | Objective Evaluator | Profile Selector |
| Objective Set | 005 | Objective Evaluator | Profile Selector |
| Operational Profile | 006 | Profile Selector | Reference-State Builder, Controller |
| Reference State | 006 | Reference-State Builder | Controller |
| Control Error | 007 | Controller | Controller, Provenance |
| Control Command | 007 | Controller | Inference Control |
| Inference Command | 008 | Inference Control | Execution Interface |
| Execution Disposition | 009 | Execution Interface | Provenance, Observation |
| Mode Guard | 010 | Mode Evaluator | Profile Selector |
| Reset Map | 010 | Transition Runtime | OSV/Estimator Boundary as defined |
| Lineage Record | 011 | Provenance Layer | Audit and Verification |
| Verification Profile | 012 | SACS Verification Adapter | NullBlade Verification Engine |

A later contract MUST NOT redefine a concept owned by another contract.

A later contract MAY refine its use only within the limits established by the owning contract.

---

## 21. Contract Dependency Graph

```text
Contract 000
Architecture and Authority
        │
        ▼
Contract 001
Observation
        │
        ▼
Contract 002
State Estimation
        │
        ▼
Contract 003
Operational State Vector
        │
   ┌────┴────┐
   ▼         ▼
Contract 004 Contract 005
Governance   Objectives
   └────┬────┘
        ▼
Contract 006
Operational Profiles and Reference States
        │
        ▼
Contract 007
Controller
        │
        ▼
Contract 008
Inference Control
        │
        ▼
Contract 009
Execution Interface
        │
        ▼
Contract 010
Modes, Guards, Resets, and Dwell Time
        │
        ▼
Contract 011
Provenance and Audit
        │
        ▼
Contract 012
Verification Profile
```

The dependency graph establishes specification order, not necessarily runtime deployment topology.

---

## 22. Versioning and Compatibility

Every externally visible SACS artifact MUST include a schema or contract version.

Each contract MUST use semantic versioning in the form:

```text
MAJOR.MINOR.PATCH
```

A major version change indicates a breaking semantic or structural change.

A minor version change may add backward-compatible fields, capabilities, or rules.

A patch version may clarify language, correct non-semantic defects, or repair implementation guidance without changing normative meaning.

Registry compatibility MUST be explicit.

Semantic interpretation across registry versions requires declared compatibility or an explicit migration.

Silent coercion is prohibited.

Interface contracts MUST define whether version negotiation occurs at startup, per session, per cycle, or per artifact.

Contract 000 does not mandate a transport mechanism.

---

## 23. Interface Expectations

Every externally visible SACS interface MUST eventually define:

- canonical schema;
- field ownership;
- required and optional fields;
- units;
- valid ranges;
- nullability;
- unavailable-state semantics;
- confidence semantics;
- uncertainty semantics;
- timestamp semantics;
- ordering guarantees;
- serialization format;
- schema version;
- registry version;
- compatibility behavior;
- failure behavior.

No interface is conforming merely because it uses the same field names.

Semantic equivalence is required.

---

## 24. Architectural Prohibitions

A conforming SACS implementation MUST NOT:

- reason directly from uncommitted telemetry in higher layers;
- mutate a committed OSV;
- combine fields from incompatible supervisory cycles;
- modify governance constraints;
- select objectives inside the controller;
- conceal estimator uncertainty;
- silently substitute missing observations;
- infer availability from absence alone;
- merge incompatible registry versions;
- permit execution to redefine supervisory state;
- broaden a control command during translation;
- treat probabilistic output as a canonical control decision without normalization;
- claim external verification based solely on internal tests;
- omit required provenance;
- reuse an identifier for semantically different artifacts;
- treat mode names as normative before Contract 010 defines them.

---

## 25. Conformance

An implementation MAY claim conformance only by declaring a SACS capability profile.

A conformance declaration MUST include:

```text
implementation_id
implementation_version
supported_contract_versions
supported_capability_profile
supported_numeric_profile
supported_registry_versions
optional_features
known_exclusions
external_governance_source
external_verification_status
```

### 25.1 Core Conformance

A Core-conforming implementation MUST implement all mandatory requirements from Contracts 000 through 009 and Contract 011.

Contract 010 requirements apply when hybrid modes are claimed.

Contract 012 applies when a verification profile is published.

### 25.2 Partial Implementations

A partial implementation MAY be described as experimental, prototype, adapter-only, profile-limited, or non-conforming.

A partial implementation MUST NOT claim full SACS conformance.

### 25.3 Conformance Evidence

A conformance claim SHOULD be supported by schema validation, deterministic replay, contract tests, provenance inspection, compatibility tests, and NullBlade verification artifacts where applicable.

---

## 26. Verification Boundary

Contract 000 establishes claims suitable for later verification, including temporal atomicity, deterministic replay, bounded control, governance non-violation, fracture residency limits, reset contraction, switched-system stability, estimator reliability, guard reliability, and telemetry calibration.

The initial verification roadmap is:

```text
VP-1 — Stable-Mode Contraction
VP-2 — Fracture Residency Bound
VP-3 — Reset Contraction
VP-4 — Switched-System Stability
VP-5 — Observer and Guard Reliability
VP-6 — Telemetry Calibration
```

The exact protocols belong to Contract 012 and the external verification system.

---

## 27. Contract Ownership Map

```text
000
Architecture and authority

001
Observation types, source classes, availability, and collection windows

002
State estimation, confidence, and uncertainty propagation

003
Operational State Vector fields, units, ranges, and atomic publication

004
Governance snapshots, permissions, prohibitions, and dispositions

005
Supervisory objectives and objective-set evaluation

006
Operational profiles, profile selection, and reference states

007
Controller semantics, bounded commands, and numeric profiles

008
Inference-control translation and command compatibility

009
Execution interface, gating, receipts, and dispositions

010
Modes, guards, hysteresis, resets, transition precedence, and dwell time

011
Provenance, lineage, audit, and replay

012
External verification profile and evidence requirements
```

---

## 28. Required Next Contract

The next required contract is:

# Contract 001 — Observation

Contract 001 MUST define the canonical observation ontology, required, optional, and unavailable observations, source identity, observation timestamps, collection windows, late-arriving data, stale-data handling, invalid-data handling, units and ranges, confidence metadata, registry versioning, serialization expectations, deterministic ordering, observation provenance, and estimator handoff requirements.

No later contract may rely on observation semantics that Contract 001 has not defined.

---

## 29. Current Canonical Status

```text
Project:
SACS — Supervisory Architecture for Cognitive Systems

System Role:
Deterministic supervisory control plane

Contract:
000 — Architecture and Authority

Version:
0.1.0

Status:
Draft for Council Review

Architecture:
Established

Authority Boundaries:
Established

Temporal Atomicity:
Foundational Invariant

Governance Authority:
External

Verification Authority:
External — NullBlade Verification Engine

First Implementation Target:
SACS Core

Next Contract:
001 — Observation
```

---

## 30. Adoption Criteria

Contract 000 may advance from Draft to Adopted when:

1. the architectural identity is accepted;
2. the layer authority boundaries are accepted;
3. the ownership matrix is accepted;
4. the contract dependency graph is accepted;
5. the assumptions, constraints, and non-goals are accepted;
6. the versioning policy is accepted;
7. the conformance model is accepted;
8. no unresolved concept has overlapping normative ownership;
9. no later contract is required to override Contract 000 to remain implementable.

Upon adoption, all later SACS contracts MUST conform to this contract.

---

**End of Contract 000**
