# SACS Contract 001 — Observation

**Project:** SACS — Supervisory Architecture for Cognitive Systems  
**Contract:** 001  
**Title:** Observation  
**Version:** 1.0.0  
**Status:** ADOPTED - SACS v1.0.0 BASELINE  
**Normative Authority:** Foundational Runtime Input  
**Depends On:** Contract 000 — Architecture and Authority  
**Canonical Consumer:** State Estimation  

---

## 1. Purpose

This contract defines the canonical observation model for SACS.

It establishes the authority, lifecycle, ontology, source identity, timestamps, observation windows, availability semantics, quality metadata, ordering, provenance, registry behavior, adapter requirements, failure handling, and estimator handoff rules for all data entering the SACS supervisory pipeline.

Observation is the only canonical entry point for runtime evidence into SACS.

Observation records what is measured, reported, received, or declared by an authorized source.

Observation does not assign supervisory meaning.

---

## 2. Normative Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **MAY**, and **OPTIONAL** are normative conformance terms as defined by Contract 000.

---

## 3. Observation Identity

An observation is a typed, source-attributed, time-bounded, versioned record of runtime evidence.

An observation MAY represent:

- a direct measurement;
- a source-declared state;
- a source-emitted event;
- a bounded aggregate over an observation window;
- an adapter-normalized representation of external telemetry;
- a declared unavailability, invalidity, staleness, or unsupported condition.

An observation MUST NOT represent:

- an operational profile selection;
- a governance decision;
- a supervisory objective;
- a control command;
- an execution disposition;
- an inferred hidden state unless explicitly classified as a derived observation and permitted by this contract;
- a semantic reinterpretation of source data that belongs to State Estimation.

The defining identity of Observation is:

> Observation records evidence without deciding what that evidence means for supervision.

---

## 4. Observation Authority

The Observation layer owns:

- observation type registration;
- source registration;
- source capability declarations;
- observation record construction;
- observation-window lifecycle;
- collection timing;
- source timestamp capture;
- receipt timestamp capture;
- normalization of transport and encoding;
- availability-state declaration;
- quality metadata capture;
- deterministic ordering;
- observation provenance;
- estimator handoff packaging.

The Observation layer does not own:

- supervisory interpretation;
- hidden-state inference;
- uncertainty propagation beyond source and collection metadata;
- OSV construction;
- governance;
- objective evaluation;
- profile selection;
- controller behavior;
- execution authorization;
- verification verdicts.

---

## 5. Canonical Observation Flow

A conforming implementation MUST preserve the following logical flow:

```text
Source Registration
        ↓
Capability Declaration
        ↓
Observation Window Open
        ↓
Collection / Receipt
        ↓
Validation
        ↓
Normalization
        ↓
Availability and Quality Classification
        ↓
Deterministic Ordering
        ↓
Observation Window Close
        ↓
Immutable Observation Batch
        ↓
State Estimator Handoff
```

No observation may enter State Estimation outside a closed observation batch except where a later contract explicitly defines a streaming estimator profile.

---

## 6. Observation Source

Every observation MUST originate from a registered observation source.

A source registration MUST include at least:

```text
source_id
source_type
source_version
producer_implementation_id
producer_implementation_version
capability_manifest_id
clock_domain_id
trust_domain_id
registry_version
```

A source MAY be:

- a model runtime;
- a tool runtime;
- an agent runtime;
- a memory subsystem;
- a retrieval subsystem;
- a Fast Delta Engine adapter;
- a Presence Lattice adapter;
- an N&T Core adapter;
- a user-interface telemetry source;
- a safety or policy adapter;
- an execution interface;
- an environmental sensor;
- a deterministic simulator;
- a replay source.

Source type names MUST be registry-defined.

A source MUST NOT claim capabilities it has not declared.

---

## 7. Source Capability Manifest

Each registered source MUST publish a capability manifest.

The manifest MUST declare:

```text
capability_manifest_id
source_id
supported_observation_types
supported_units
supported_ranges
sampling_behavior
timestamp_behavior
confidence_behavior
availability_behavior
ordering_behavior
late_data_behavior
error_behavior
schema_versions
registry_versions
```

The capability manifest MUST distinguish between:

- natively produced fields;
- adapter-derived fields;
- unsupported fields;
- conditionally available fields.

A source MUST NOT silently emit an observation type outside its declared manifest.

---

## 8. Observation Type Registry

Every canonical observation type MUST be defined in a versioned Observation Registry.

Each observation type definition MUST include:

```text
observation_type_id
name
description
semantic_owner
value_type
unit
valid_range
nullability
availability_rules
confidence_rules
uncertainty_rules
timestamp_rules
aggregation_rules
ordering_class
sensitivity_class
retention_class
schema_version
registry_version
```

Field-name similarity does not establish semantic equivalence.

An implementation MUST NOT merge observations from incompatible registry versions without an explicit compatibility declaration or migration.

---

## 9. Observation Classes

The initial canonical observation classes are:

### 9.1 Scalar Observation

A single numeric value with defined unit, valid range, timestamp semantics, and source identity.

### 9.2 Categorical Observation

A value selected from a registry-defined vocabulary.

### 9.3 Boolean Observation

A true or false value whose absence is not interpreted as false.

### 9.4 Vector Observation

A fixed- or declared-length ordered vector with field semantics defined by registry.

### 9.5 Event Observation

A discrete occurrence with an event time, source identity, event type, and optional payload.

### 9.6 Aggregate Observation

A deterministic summary over a declared window, including the aggregation method and contributing sample count.

### 9.7 Structured Observation

A registry-defined compound object with canonical field ownership.

### 9.8 Availability Observation

A canonical declaration that a required or expected observation is unavailable, invalid, stale, unsupported, redacted, or not applicable.

Additional classes MAY be introduced through minor or major registry revisions.

---

## 10. Raw and Derived Observations

An observation MUST declare whether it is:

```text
RAW
NORMALIZED
DERIVED
AGGREGATED
REPLAYED
```

### 10.1 Raw

A RAW observation preserves the source-emitted value and source semantics.

### 10.2 Normalized

A NORMALIZED observation changes representation, unit, encoding, or transport form without changing semantic meaning.

### 10.3 Derived

A DERIVED observation is computed from one or more observations using a registered deterministic derivation.

Derived observations MUST include:

```text
derivation_id
derivation_version
input_observation_ids
algorithm_id
algorithm_version
```

Derived observations MUST NOT perform supervisory interpretation reserved for State Estimation.

### 10.4 Aggregated

An AGGREGATED observation summarizes multiple values over a declared observation window using a registered deterministic aggregation.

### 10.5 Replayed

A REPLAYED observation reproduces a previously canonical observation for deterministic replay and MUST retain the original source identity and provenance while declaring the replay context.

---

## 11. Observation Record

Every canonical observation record MUST include at least:

```text
observation_id
observation_type_id
observation_class
observation_form
source_id
source_sequence
source_event_time
receipt_time
window_id
value
unit
availability_state
quality_state
confidence
uncertainty
validity_interval
schema_version
registry_version
provenance_id
```

Optional fields MAY include:

```text
correlation_id
causation_id
trace_id
parent_observation_ids
derivation_id
aggregation_id
sensitivity_label
redaction_state
signature
```

Observation identifiers MUST be globally unique within the declared SACS implementation scope.

An identifier MUST NOT be reused for a semantically different observation.

---

## 12. Timestamp Semantics

Every observation MUST distinguish at least:

```text
source_event_time
receipt_time
```

A source MAY also provide:

```text
measurement_start_time
measurement_end_time
processing_complete_time
```

### 12.1 Source Event Time

The time at which the source declares the observed event or measurement occurred.

### 12.2 Receipt Time

The time at which the SACS Observation layer received the observation.

### 12.3 Validity Interval

The time interval during which the observation is declared applicable.

### 12.4 Clock Domain

Every timestamp MUST be associated with a declared clock domain.

Cross-domain timestamp comparison MUST NOT occur without an explicit clock synchronization profile.

### 12.5 Unknown Time

Unknown or unavailable timestamps MUST be represented explicitly.

A missing timestamp MUST NOT be silently replaced with receipt time.

---

## 13. Observation Window

An observation window defines the bounded collection interval for one supervisory cycle.

Each window MUST include:

```text
window_id
cycle_id
open_time
scheduled_close_time
actual_close_time
observation_cutoff
window_policy_id
clock_domain_id
status
```

A window status MUST be one of:

```text
OPEN
CLOSED
FAULTED
INCOMPLETE
CANCELLED
```

Only CLOSED, FAULTED, or INCOMPLETE windows may be committed for estimator handoff.

An OPEN window MUST NOT be consumed by higher supervisory layers.

---

## 14. Observation Cutoff

Each observation window MUST define one canonical observation cutoff.

Observations with receipt time after the cutoff MUST NOT be silently inserted into the committed batch.

Late observations MUST be handled by one of the following declared policies:

```text
DEFER_TO_NEXT_WINDOW
RECORD_AS_LATE_ONLY
REOPEN_PROHIBITED
FAULT_WINDOW
```

A closed window MUST NOT be reopened in place.

A later correction MUST produce a new artifact with a new identifier and explicit supersession lineage.

---

## 15. Availability Semantics

Every expected observation MUST resolve to a canonical availability state.

The initial availability vocabulary is:

```text
AVAILABLE
UNAVAILABLE
INVALID
STALE
UNSUPPORTED
NOT_APPLICABLE
REDACTED
PENDING
```

### 15.1 Available

The value is present and passed observation-level validation.

### 15.2 Unavailable

The source was expected to provide the observation but did not provide a usable value.

### 15.3 Invalid

A value was provided but failed schema, range, type, signature, or declared validity checks.

### 15.4 Stale

The value is valid in form but outside its permitted freshness interval.

### 15.5 Unsupported

The source capability manifest does not support this observation type.

### 15.6 Not Applicable

The observation type is valid but does not apply to the current source or operating context.

### 15.7 Redacted

The source or adapter withheld the value under a declared policy.

### 15.8 Pending

The observation was expected but had not resolved before cutoff.

Absence MUST NOT be interpreted as AVAILABLE, false, zero, normal, or safe.

---

## 16. Quality State

Each observation MUST include an observation-level quality state.

The initial quality vocabulary is:

```text
NOMINAL
DEGRADED
SUSPECT
FAILED
UNKNOWN
```

Quality state MUST be computed using a registered deterministic rule.

Quality state does not replace confidence, uncertainty, or availability.

---

## 17. Confidence and Uncertainty Metadata

Observation-level confidence describes the source or collection mechanism's declared confidence in the emitted value.

Observation-level uncertainty describes the declared uncertainty associated with that value.

Confidence and uncertainty MUST remain distinct.

A conforming observation MAY represent uncertainty through:

- bounded interval;
- variance;
- covariance;
- probability mass over a finite vocabulary;
- declared qualitative class;
- unsupported or unknown state.

The exact canonical uncertainty model belongs to Contract 002.

Contract 001 requires only that source uncertainty be preserved without silent collapse.

---

## 18. Validation

Every observation MUST undergo deterministic validation before window commitment.

Validation MUST include, where applicable:

- schema validation;
- type validation;
- unit validation;
- range validation;
- timestamp validation;
- source registration validation;
- capability validation;
- signature validation;
- registry compatibility validation;
- sequence validation.

A failed validation MUST produce either an INVALID observation record or a window fault according to the active window policy.

Invalid values MUST NOT be silently coerced into valid values.

---

## 19. Normalization

Normalization MAY convert:

- transport encoding;
- serialization;
- units;
- field naming;
- enum representation;
- numeric precision;
- time representation.

Normalization MUST preserve semantic meaning.

Every normalization MUST be performed by a registered deterministic adapter and MUST record:

```text
adapter_id
adapter_version
input_schema_version
output_schema_version
normalization_rule_id
```

Normalization MUST NOT create supervisory interpretation.

---

## 20. Deterministic Ordering

Every committed observation batch MUST define a deterministic canonical order.

Unless overridden by a registry-defined ordering rule, the default order is:

```text
1. source_event_time
2. receipt_time
3. source_id
4. source_sequence
5. observation_type_id
6. observation_id
```

Ties MUST be resolved deterministically.

Transport arrival order MUST NOT be treated as canonical unless explicitly declared by the source profile.

---

## 21. Duplicate and Conflicting Observations

Duplicate observations MUST be detected using registry-defined identity and equivalence rules.

A conforming implementation MUST distinguish:

```text
EXACT_DUPLICATE
SEMANTIC_DUPLICATE
CORRECTION
CONFLICT
```

Exact duplicates MAY be deduplicated if lineage is preserved.

Corrections MUST supersede rather than mutate prior observations.

Conflicts MUST be preserved for State Estimation unless a registered source-priority rule resolves them at the Observation layer.

Source-priority rules MUST be explicit, versioned, and deterministic.

---

## 22. Observation Batch

At window close, the Observation layer MUST produce one immutable Observation Batch.

The batch MUST include:

```text
observation_batch_id
window_id
cycle_id
observation_cutoff
ordered_observation_ids
expected_observation_manifest_id
missing_observation_summary
invalid_observation_summary
late_observation_summary
registry_versions
schema_versions
batch_hash
batch_status
provenance_id
```

The batch status MUST be one of:

```text
COMPLETE
DEGRADED
INCOMPLETE
FAULTED
```

A batch MUST NOT be modified after commitment.

Any correction MUST produce a new batch with explicit supersession lineage.

---

## 23. Expected Observation Manifest

Each supervisory cycle MUST resolve against an Expected Observation Manifest.

The manifest MUST declare:

```text
expected_observation_manifest_id
required_observation_types
optional_observation_types
conditional_observation_types
source_constraints
freshness_constraints
minimum_quality_constraints
minimum_confidence_constraints
window_policy_id
registry_versions
```

A required observation that does not resolve before cutoff MUST produce an explicit non-AVAILABLE state.

The expected manifest MAY vary by implementation capability profile, operational context, or supervised-system type, but MUST be selected before the observation window closes.

---

## 24. Late-Arriving Data

Late-arriving data MUST retain its original source event time and receipt time.

Late-arriving data MUST NOT retroactively alter a committed batch.

A conforming implementation MAY:

- defer it to the next window;
- attach it to a late-data ledger;
- trigger an explicit correction cycle;
- mark the prior cycle for replay review.

Any use of late-arriving data MUST be visible in provenance.

---

## 25. Stale Data

Freshness rules MUST be defined per observation type or expected manifest.

A stale value MUST remain distinguishable from:

- unavailable;
- invalid;
- unsupported;
- current.

A stale observation MAY be passed to State Estimation only when the active estimator profile explicitly permits stale input.

The stale status and age MUST be preserved.

---

## 26. Missing Data

Missing data MUST be represented explicitly.

The Observation layer MUST NOT:

- replace missing numeric values with zero;
- replace missing categorical values with a default class;
- infer normality from absence;
- infer false from absence;
- copy forward a previous value without declaring a carry-forward rule;
- synthesize confidence for an absent observation.

Carry-forward behavior, if allowed, MUST be implemented as a registered derived observation and MUST preserve the original source observation identifier and age.

---

## 27. Observation Adapters

An adapter connects an external source to the canonical Observation interface.

Every adapter MUST declare:

```text
adapter_id
adapter_version
supported_source_types
supported_input_schemas
supported_output_observation_types
normalization_rules
error_mapping
availability_mapping
clock_mapping
compatibility_profile
```

Adapters MUST fail toward reduced semantic authority.

An adapter MUST NOT invent source capabilities, source confidence, source timestamps, or source semantics.

---

## 28. External System Mappings

### 28.1 IBOS

IBOS governance data does not enter SACS as ordinary operational telemetry when used as the canonical Governance Snapshot.

Observation MAY record IBOS adapter health, snapshot availability, version compatibility, or delivery integrity.

The governance content itself belongs to Contract 004.

### 28.2 Fast Delta Engine

FDE MAY provide observations including:

- residual-state delta magnitude;
- KV-cache delta magnitude;
- adaptive update gain;
- update rank;
- layer-local strain;
- calibration confidence;
- transient-state reset status;
- adaptive-write state.

These observations MUST preserve FDE-defined source semantics.

### 28.3 Presence Lattice

Presence Lattice MAY provide structured semantic-field observations.

Such observations MUST identify:

- lattice schema version;
- field registry version;
- derivation method;
- source interaction range;
- confidence and uncertainty metadata;
- semantic ownership.

Presence Lattice outputs MUST NOT be treated as supervisory conclusions.

### 28.4 N&T Core

N&T Core MAY provide typed observations including:

- noticing state;
- recursion depth;
- affective weighting;
- tether state;
- symbolic activation;
- silence pressure;
- action-eligibility signals.

These terms remain non-normative until registered by the appropriate owning specification.

### 28.5 Execution Interface

Execution receipts and observed runtime consequences re-enter SACS through Observation.

Execution MUST NOT directly mutate State Estimation, OSV, profile, or controller state.

---

## 29. Replay Sources

A deterministic replay source MUST reproduce canonical observation records and batches without semantic modification.

A replay profile MUST declare:

```text
replay_id
source_run_id
original_cycle_range
replay_clock_policy
observation_registry_versions
schema_versions
integrity_hashes
```

Replay MUST preserve original ordering, identifiers, timestamps, availability states, quality states, confidence, uncertainty, and provenance.

Replay-context metadata MAY be added but MUST NOT overwrite original fields.

---

## 30. Provenance

Every observation MUST be traceable to its source, adapter, transformation, registry, and collection window.

Observation provenance MUST include at least:

```text
provenance_id
observation_id
source_id
capability_manifest_id
adapter_id
adapter_version
window_id
collection_path
validation_result
normalization_result
registry_version
schema_version
parent_observation_ids
created_at
```

Every committed batch MUST have a batch-level provenance record.

No observation is canonical without provenance.

---

## 31. Security and Integrity

Observation interfaces SHOULD support:

- source authentication;
- message integrity;
- replay protection;
- sequence validation;
- tamper-evident logging;
- sensitivity labeling;
- redaction declarations.

A failed integrity check MUST NOT produce an AVAILABLE observation.

Security controls MUST NOT silently alter source semantics.

---

## 32. Privacy and Sensitive Observations

Observation types MAY carry sensitivity labels.

A sensitivity profile SHOULD define:

- collection eligibility;
- minimization requirements;
- retention limits;
- redaction behavior;
- permitted consumers;
- audit requirements.

A REDACTED observation MUST preserve the fact of redaction without exposing the protected value.

Privacy enforcement does not authorize the Observation layer to reinterpret supervisory meaning.

---

## 33. Failure Principle

Observation MUST fail toward reduced authority.

When source identity, timing, integrity, schema compatibility, registry compatibility, validity, availability, or provenance is uncertain, the Observation layer MUST NOT upgrade the observation to a more authoritative state.

The default observation authority precedence is:

```text
INVALID
    >
UNAVAILABLE
    >
STALE
    >
DEGRADED
    >
AVAILABLE
```

This precedence expresses conservatism, not semantic equivalence.

Exact escalation behavior MUST be defined by the active window policy and expected observation manifest.

---

## 34. Estimator Handoff

The Observation layer MUST hand State Estimation one immutable Observation Batch per supervisory cycle.

The handoff MUST include:

```text
observation_batch_id
cycle_id
window_id
observation_cutoff
ordered_observation_records
expected_observation_manifest
availability_summary
quality_summary
late_data_summary
registry_versions
schema_versions
provenance_reference
```

State Estimation MUST NOT receive hidden, out-of-band, or unregistered observation values.

If an implementation supports multiple estimator inputs, each input path MUST resolve to the same canonical batch or to explicitly partitioned sub-batches with declared ownership.

---

## 35. Architectural Prohibitions

A conforming Observation implementation MUST NOT:

- assign operational profiles;
- apply governance;
- evaluate supervisory objectives;
- issue control commands;
- mutate a closed observation window;
- mutate a committed observation batch;
- silently replace missing values;
- silently coerce invalid values;
- silently merge incompatible registry versions;
- treat absence as normality;
- treat unsupported as unavailable;
- treat stale as current;
- erase source timestamps;
- invent source confidence;
- hide derivation lineage;
- deliver uncommitted telemetry to higher layers;
- allow execution results to bypass Observation on re-entry.

---

## 36. Conformance Requirements

A Contract 001-conforming implementation MUST provide:

```text
implementation_id
implementation_version
supported_contract_version
supported_observation_registry_versions
supported_source_types
supported_observation_classes
supported_window_policies
supported_timestamp_profiles
supported_integrity_profiles
known_exclusions
```

Core conformance requires:

1. registered sources;
2. capability manifests;
3. versioned observation types;
4. canonical observation records;
5. observation windows;
6. explicit availability semantics;
7. deterministic ordering;
8. immutable committed batches;
9. provenance;
10. estimator handoff.

A partial implementation MUST NOT claim full Contract 001 conformance.

---

## 37. Conformance Tests

A conforming implementation SHOULD pass at least the following tests:

```text
OT-001 — Registered Source Enforcement
OT-002 — Unsupported Capability Rejection
OT-003 — Schema Validation
OT-004 — Unit Validation
OT-005 — Timestamp Preservation
OT-006 — Observation Cutoff Enforcement
OT-007 — Late Data Non-Mutation
OT-008 — Explicit Missingness
OT-009 — Invalid Value Non-Coercion
OT-010 — Registry Compatibility
OT-011 — Deterministic Ordering
OT-012 — Duplicate Classification
OT-013 — Batch Immutability
OT-014 — Provenance Completeness
OT-015 — Deterministic Replay
OT-016 — Estimator Handoff Completeness
```

Exact test vectors MAY be published in a companion conformance specification.

---

## 38. Assumptions

Contract 001 assumes that:

1. observation sources can be uniquely identified;
2. sources can declare capabilities;
3. a canonical clock profile or clock-domain mapping is available;
4. observation types can be versioned;
5. source and receipt timestamps can be represented;
6. the runtime can close and commit immutable observation windows;
7. unavailable and invalid states can be represented explicitly;
8. provenance can be retained;
9. State Estimation can consume immutable batches;
10. adapters can fail without inventing semantic content.

Later contracts MAY refine but MUST NOT silently invalidate these assumptions.

---

## 39. Non-Goals

Contract 001 does not define:

- estimator mathematics;
- uncertainty propagation;
- hidden-state inference;
- OSV fields;
- governance content;
- supervisory objectives;
- profile selection;
- controller gains;
- inference commands;
- execution dispositions;
- mode transitions;
- verification proofs;
- model-specific telemetry vocabularies beyond registry examples.

These belong to later contracts or external registries.

---

## 40. Required Registries and Companion Artifacts

A complete SACS Core implementation will require:

```text
Observation Type Registry
Source Type Registry
Source Capability Manifest Schema
Observation Record Schema
Observation Window Policy Registry
Availability Vocabulary Registry
Quality Rule Registry
Adapter Registry
Timestamp and Clock Profile Registry
Expected Observation Manifest Schema
Observation Batch Schema
Observation Provenance Schema
```

These artifacts MAY be maintained separately but MUST remain version-compatible with Contract 001.

---

## 41. Required Next Contract

The next required contract is:

# Contract 002 — State Estimation

Contract 002 MUST define:

- estimator authority;
- estimator input contract;
- hidden-state representation;
- uncertainty propagation;
- confidence interpretation;
- missing-data treatment;
- stale-data treatment;
- conflicting-observation treatment;
- estimator identity and versioning;
- estimator state lifecycle;
- deterministic estimator profiles;
- estimated-state schema;
- OSV publisher handoff;
- estimator provenance;
- estimator conformance tests.

Contract 002 MUST consume only canonical Observation Batches defined by Contract 001.

---

## 42. Current Canonical Status

```text
Project:
SACS — Supervisory Architecture for Cognitive Systems

System Role:
Deterministic supervisory control plane

Contract:
001 — Observation

Version:
0.1.0

Status:
Draft for Council Review

Observation Authority:
Established

Source Identity:
Established

Observation Windows:
Established

Availability Semantics:
Established

Deterministic Ordering:
Established

Batch Immutability:
Established

Estimator Handoff:
Established

Next Contract:
002 — State Estimation
```

---

## 43. Adoption Criteria

Contract 001 may advance from Draft to Adopted when:

1. the observation identity is accepted;
2. source and capability ownership is accepted;
3. the observation record is accepted;
4. timestamp semantics are accepted;
5. observation-window behavior is accepted;
6. availability and quality vocabularies are accepted;
7. late, stale, missing, duplicate, and conflicting data rules are accepted;
8. deterministic ordering is accepted;
9. batch immutability is accepted;
10. estimator handoff is accepted;
11. no observation-layer rule overlaps State Estimation authority;
12. Contract 002 can be implemented without overriding Contract 001.

Upon adoption, all SACS observation sources, adapters, registries, and estimator handoffs MUST conform to this contract.

---

**End of Contract 001**
