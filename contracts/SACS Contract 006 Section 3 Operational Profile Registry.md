# 17. Operational Profile Registry

The Operational Profile Registry is the canonical repository through which Operational Profile Definitions are published and made available for discovery.

Its defining identity is:

> **The Operational Profile Registry defines the published universe of supervisory configurations available for deterministic discovery. It does not evaluate, select, activate, or commit those configurations.**

The Operational Profile Registry SHALL consist of immutable Registry Entries.

Each Registry Entry SHALL reference exactly one immutable Operational Profile Definition and SHALL bind that definition to non-semantic publication metadata.

The Registry SHALL NOT contain:

* Committed Operational Profiles;
* Candidate Profile Sets;
* Eligibility Reports;
* Compatibility Reports;
* Profile Selection Reports;
* Reference States;
* Controller state;
* runtime evaluation results.

Registry publication SHALL remain independent of supervisory execution.

Publication of a new Registry Entry SHALL NOT modify a Profile Registry Snapshot already committed for an active supervisory cycle.

---

# 18. Registry Entry

A Registry Entry binds one Operational Profile Definition to its publication context.

Its defining identity is:

> **A Registry Entry records that a particular immutable Operational Profile Definition was published within a particular registry context.**

Every Registry Entry SHALL include:

```text
registry_entry_id

registry_id
registry_version

profile_id
profile_version
definition_hash

publisher_id
published_at

approval_record

publication_scope
visibility_class

publication_provenance_id

registry_entry_schema_version
entry_hash
```

Publication metadata SHALL NOT modify:

* the semantic identity of the Profile Definition;
* the Profile Definition version;
* the Profile Definition hash;
* the permissions or constraints defined by the Profile Definition.

A Registry Entry SHALL reference exactly one Profile Definition.

A Profile Definition MAY be referenced by multiple Registry Entries when it is published into multiple authorized registries or publication scopes.

---

# 19. Profile Registry Snapshot

Candidate Discovery SHALL NOT consume a live Operational Profile Registry.

It SHALL consume a committed Profile Registry Snapshot.

A Profile Registry Snapshot is an immutable representation of the Registry state authorized for one discovery operation.

Its defining identity is:

> **A Profile Registry Snapshot freezes the exact registry universe available to Candidate Discovery for a supervisory cycle.**

Every Profile Registry Snapshot SHALL include:

```text
profile_registry_snapshot_id

registry_id
registry_version

ordered_registry_entry_ids

registry_schema_version

visibility_policy_versions
publication_scope_versions

snapshot_created_at

snapshot_provenance_id

snapshot_hash
```

The ordered Registry Entry collection SHALL identify the complete set of entries visible within the committed snapshot.

Each referenced Registry Entry SHALL be resolvable by:

```text
registry_entry_id
profile_id
profile_version
definition_hash
entry_hash
```

The Profile Registry Snapshot SHALL be immutable after commitment.

Concurrent or subsequent Registry publication SHALL NOT alter a committed Profile Registry Snapshot.

A later publication SHALL become visible only through a later committed snapshot.

---

# 20. Discovery Scope

Candidate Discovery SHALL operate under one committed Discovery Scope.

The Discovery Scope defines the identity and visibility boundaries within which Registry Entries may be discovered.

Its defining identity is:

> **The Discovery Scope determines where Candidate Discovery may look. It does not determine whether a discovered Profile Definition is eligible, compatible, preferred, or selectable.**

Every Discovery Scope SHALL include:

```text
discovery_scope_id

authorized_registry_ids
authorized_registry_versions

deployment_namespaces

publication_scopes

visibility_classes

supervisory_domains

lifecycle_visibility_policy_id

scope_schema_version

scope_provenance_id

scope_hash
```

The Discovery Scope SHALL be immutable for the duration of the supervisory cycle.

Lifecycle visibility SHALL be determined exclusively through the referenced lifecycle visibility policy.

Candidate Discovery SHALL NOT invent lifecycle treatment.

The Discovery Scope MAY determine whether definitions with lifecycle states such as the following are visible:

```text
DRAFT
ACTIVE
DEPRECATED
RETIRED
```

Visibility SHALL NOT itself determine eligibility.

A visible DEPRECATED definition, for example, MAY still be discovered and later resolve as INELIGIBLE.

---

# 21. Candidate Profile Set

Candidate Discovery produces exactly one canonical Candidate Profile Set.

A Candidate Profile Set represents every Operational Profile Definition discovered within one committed Registry Snapshot and one committed Discovery Scope.

Its defining identity is:

> **A Candidate Profile Set records the complete authorized discovery universe before substantive supervisory evaluation begins.**

Every Candidate Profile Set SHALL include:

```text
candidate_set_id

cycle_id

osv_id
governance_snapshot_id
objective_set_id

profile_registry_snapshot_id
discovery_scope_id

discovered_definitions

excluded_visible_entries

candidate_set_schema_version
contract_version

discovery_provenance_id

created_at

artifact_hash
```

Each entry in `discovered_definitions` SHALL include:

```text
registry_entry_id
profile_id
profile_version
definition_hash
```

Each entry in `excluded_visible_entries` SHALL include:

```text
registry_entry_id
profile_id
profile_version

exclusion_rule_id
exclusion_reason
scope_dimension
```

The Candidate Profile Set SHALL be immutable after commitment.

The identifiers:

```text
osv_id
governance_snapshot_id
objective_set_id
```

establish cycle lineage only.

Candidate Discovery SHALL NOT inspect or evaluate the semantic content of the referenced OSV, Governance Snapshot, or Objective Set.

---

# 22. Candidate Discovery

Candidate Discovery SHALL perform only deterministic identity-scope resolution.

Its purpose is:

> **To determine which Operational Profile Definitions are visible within the committed Profile Registry Snapshot and Discovery Scope.**

Candidate Discovery MAY inspect only:

* Registry Entry identity;
* Registry version;
* Registry Snapshot membership;
* deployment namespace;
* publication scope;
* visibility class;
* supervisory domain;
* lifecycle visibility policy;
* Profile Definition identity and hash.

Candidate Discovery SHALL NOT evaluate:

* implementation support;
* required runtime capabilities;
* Governance permission;
* Objective compatibility;
* OSV conditions;
* transition admission;
* controller compatibility;
* Reference Builder capability;
* preference;
* score;
* selection.

Candidate Discovery SHALL answer only:

> **“Which Profile Definitions exist within the authorized discovery scope?”**

It SHALL NOT answer:

> “Which Profile Definitions may or should be selected?”

---

# 23. Excluded Visible Registry Entries

The `excluded_visible_entries` field SHALL contain only Registry Entries that:

1. were visible to the discovery mechanism;
2. were present in the committed Profile Registry Snapshot;
3. were examined under the committed Discovery Scope; and
4. were excluded solely by an explicit discovery-scope rule.

It SHALL NOT disclose Registry Entries hidden by:

* registry access control;
* publication confidentiality;
* unauthorized namespace boundaries;
* visibility rules that prevent discovery awareness.

Valid discovery exclusion reasons MAY include:

```text
OUTSIDE_DEPLOYMENT_NAMESPACE

OUTSIDE_PUBLICATION_SCOPE

OUTSIDE_SUPERVISORY_DOMAIN

LIFECYCLE_NOT_VISIBLE

REGISTRY_VERSION_EXCLUDED

VISIBILITY_CLASS_EXCLUDED
```

Invalid discovery exclusion reasons include:

```text
UNSUPPORTED

CAPABILITY_MISSING

PROHIBITED_BY_GOVERNANCE

OBJECTIVE_INCOMPATIBLE

CONTROLLER_INCOMPATIBLE

LOW_SCORE

NOT_PREFERRED
```

Those outcomes belong to later Contract 006 mechanism phases.

---

# 24. Discovery Completeness

Every Profile Definition discoverable within the committed Registry Snapshot and Discovery Scope SHALL appear in `discovered_definitions`.

A Profile Definition SHALL NOT be removed from discovery because:

* the implementation does not support it;
* required capabilities are unavailable;
* Governance may later prohibit it;
* its Objective support may be insufficient;
* its Reference Builder may be incompatible;
* it is unlikely to be selected.

Those determinations belong exclusively to Eligibility, Compatibility, and Selection.

This establishes the invariant of Discovery Completeness:

> **Every Profile Definition discoverable within authorized scope SHALL remain visible throughout replay, audit, and verification, regardless of its later evaluation outcome.**

Discovery Completeness SHALL NOT require disclosure of Registry Entries outside authorized visibility.

---

# 25. Canonical Ordering

Although the Candidate Profile Set represents a semantic set, its serialized form SHALL use deterministic canonical ordering.

The following collections SHALL be canonically ordered:

* Registry Entries within a Profile Registry Snapshot;
* discovered Profile Definitions;
* excluded visible Registry Entries;
* registry-version collections;
* namespace collections;
* visibility-policy collections.

Unless a registry-specific canonical ordering is declared, discovered Profile Definitions SHALL be ordered by:

1. `profile_id`;
2. `profile_version`;
3. `definition_hash`;
4. `registry_entry_id`.

Excluded visible Registry Entries SHALL use the same ordering, followed by:

5. `exclusion_rule_id`.

Canonical ordering SHALL occur before artifact hashing.

Two semantically equivalent Candidate Profile Sets SHALL NOT produce different hashes solely because of collection ordering.

---

# 26. Discovery Determinism

Candidate Discovery SHALL be a deterministic transformation.

Let:

```text
R_t = committed Profile Registry Snapshot
S_t = committed Discovery Scope
C_t = committed Candidate Profile Set
```

Then:

```text
C_t = D(R_t, S_t)
```

For any two discovery operations:

```text
R_a = R_b
and
S_a = S_b
```

the resulting Candidate Profile Sets SHALL be semantically identical and SHALL produce identical canonical artifact hashes.

Values such as `created_at` MAY differ operationally but SHALL NOT affect semantic equivalence unless explicitly included in the semantic hash domain.

The contract SHALL distinguish:

* semantic content;
* operational metadata;
* artifact-hash inputs.

---

# 27. Discovery Phase Invariants

Candidate Discovery SHALL satisfy all of the following invariants.

## 27.1 Discovery Purity

Discovery SHALL identify visible Profile Definitions without evaluating supervisory suitability.

## 27.2 Discovery Completeness

Every definition discoverable within authorized scope SHALL appear in the Candidate Profile Set.

## 27.3 Visibility Preservation

Discovery Completeness SHALL NOT reveal definitions outside authorized registry visibility.

## 27.4 Discovery Determinism

Identical Registry Snapshots and Discovery Scopes SHALL produce identical Candidate Profile Sets.

## 27.5 Discovery Immutability

A committed Candidate Profile Set SHALL NOT be modified.

## 27.6 Discovery Replayability

Every later Contract 006 evidence artifact SHALL reference the Candidate Profile Set from which it originated.

## 27.7 Append-Only Evidence

Later mechanism stages MAY append evaluation evidence.

They SHALL NOT:

* remove discovered definitions;
* alter discovery outcomes;
* conceal visible exclusions;
* re-execute discovery;
* replace the Candidate Profile Set.

---

# 28. Contract 006 Stage Dependencies

Each Contract 006 mechanism stage SHALL consume:

1. the immutable evidence artifact produced by the preceding Contract 006 stage; and
2. only those committed contextual artifacts explicitly authorized for that transformation.

A later stage MAY require more than one committed input.

For example:

```text
Eligibility Evaluation
    Candidate Profile Set
    Governance Snapshot
    Capability Declaration
    Transition Context
```

and:

```text
Compatibility Evaluation
    Eligibility Evaluation Report
    OSV
    Objective Set
    Governance Snapshot
    Controller Interface Profile
```

No stage SHALL:

* re-execute an earlier transformation;
* replace an earlier evidence artifact;
* reinterpret an earlier outcome;
* introduce a Profile Definition absent from the Candidate Profile Set.

---

# 29. Eligibility Evaluation

Eligibility Evaluation is the first substantive evaluation stage of Contract 006.

Its defining identity is:

> **Eligibility determines whether each discovered Profile Definition is permitted to participate in deterministic profile selection.**

Eligibility Evaluation SHALL evaluate every Profile Definition present in the Candidate Profile Set exactly once.

It SHALL NOT evaluate any Profile Definition absent from the Candidate Profile Set.

Eligibility Evaluation SHALL determine mandatory admission only.

It SHALL NOT determine:

* semantic fit;
* relative preference;
* profile score;
* final selection;
* Reference State quality;
* controller output.

---

# 30. Eligibility Evaluation Report

Eligibility Evaluation SHALL produce one immutable Eligibility Evaluation Report.

The Eligibility Evaluation Report is an append-only evidence artifact that records one eligibility outcome for every discovered Profile Definition.

Every Eligibility Evaluation Report SHALL include:

```text
eligibility_report_id

cycle_id

candidate_set_id

osv_id
governance_snapshot_id
objective_set_id

capability_declaration_id
transition_context_id

eligibility_policy_versions

eligibility_results

eligibility_report_schema_version
contract_version

evaluation_provenance_id

created_at

artifact_hash
```

Each entry in `eligibility_results` SHALL include:

```text
registry_entry_id

profile_id
profile_version
definition_hash

eligibility_outcome

evaluated_requirements

failed_requirements

unresolved_requirements

eligibility_restrictions

eligibility_rule_versions

result_provenance_id
```

The Eligibility Evaluation Report SHALL be immutable after commitment.

---

# 31. Eligibility Outcome Vocabulary

Every discovered Profile Definition SHALL receive exactly one of the following outcomes:

```text
ELIGIBLE

INELIGIBLE

UNSUPPORTED

PROHIBITED

UNRESOLVED
```

No other terminal eligibility outcome is permitted unless introduced by a later compatible contract version.

---

## 31.1 ELIGIBLE

`ELIGIBLE` means all mandatory admission requirements were satisfied.

An ELIGIBLE result does not mean the profile is:

* compatible;
* preferred;
* selected;
* safe to execute;
* guaranteed to produce a Reference State.

It means only that the candidate may proceed to Compatibility Evaluation.

---

## 31.2 INELIGIBLE

`INELIGIBLE` means one or more mandatory profile admission requirements were not satisfied.

Examples include:

* inactive definition lifecycle;
* deployment scope mismatch;
* unsatisfied transition-entry requirement;
* missing required approval state;
* unsatisfied mandatory profile precondition.

INELIGIBLE SHALL NOT be used when an authoritative prohibition applies.

---

## 31.3 UNSUPPORTED

`UNSUPPORTED` means the implementation lacks a required capability, schema, component, interface, or registered mechanism necessary to evaluate or use the candidate.

Examples include:

* unsupported Reference Builder type;
* unavailable Numeric Profile implementation;
* unsupported controller-interface version;
* unavailable profile capability;
* unsupported schema version.

UNSUPPORTED describes implementation capability.

It does not describe Governance authority.

---

## 31.4 PROHIBITED

`PROHIBITED` means participation is barred by an authoritative restriction.

Examples include:

* committed Governance prohibition;
* deployment prohibition;
* registry authority prohibition;
* security-class prohibition;
* transition prohibition recognized as authoritative.

A PROHIBITED candidate SHALL NOT proceed to Compatibility Evaluation.

---

## 31.5 UNRESOLVED

`UNRESOLVED` means the required evidence was unavailable, indeterminate, contradictory, or could not be resolved from committed inputs.

UNRESOLVED SHALL NOT be treated as ELIGIBLE.

A candidate with an UNRESOLVED eligibility result SHALL NOT be selected.

A fallback candidate may be selected only if that fallback candidate independently receives ELIGIBLE.

---

# 32. Eligibility Admission Conditions

Eligibility MAY evaluate only registered mandatory admission conditions.

These MAY include:

* Profile Definition lifecycle admission;
* implementation support;
* required capabilities;
* deployment admission;
* Governance class admission;
* transition entry permission;
* schema support;
* required registry dependencies;
* mandatory approval state.

Eligibility SHALL NOT evaluate:

* Objective semantic fit;
* operating-region coverage;
* target constructability;
* Reference Builder completeness;
* controller mathematical compatibility;
* profile preference;
* comparative score.

Those belong to Compatibility or Selection.

---

# 33. Eligibility Coverage Invariant

Eligibility Evaluation SHALL satisfy the following invariant:

> **Every Profile Definition present in the Candidate Profile Set SHALL receive exactly one eligibility result, and no Profile Definition absent from that Candidate Profile Set may receive one.**

Formally:

```text
Domain(Eligibility Results)
=
Discovered Definitions
```

For every discovered definition `d`:

```text
count(eligibility_result(d)) = 1
```

No candidate may be:

* silently omitted;
* evaluated more than once;
* merged with another candidate;
* replaced by a different version;
* introduced after discovery.

---

# 34. Eligibility Determinism

Eligibility Evaluation SHALL be deterministic.

Given identical:

* Candidate Profile Set;
* committed Governance Snapshot;
* Capability Declaration;
* Transition Context;
* eligibility-policy versions;
* registry dependencies;

Eligibility Evaluation SHALL produce:

* identical eligibility outcomes;
* identical requirement evidence;
* identical restrictions;
* identical canonical artifact hash.

Eligibility SHALL NOT depend on:

* undocumented runtime state;
* noncommitted capability assumptions;
* live registry changes;
* implementation-local preference;
* random selection;
* controller output.

---

# 35. Eligibility Evidence Preservation

Eligibility Evaluation SHALL append evidence to the discovery record.

It SHALL NOT modify the Candidate Profile Set.

For every candidate, the Eligibility Evaluation Report SHALL preserve:

* candidate identity;
* evaluated admission requirements;
* passed requirements;
* failed requirements;
* unresolved requirements;
* authoritative prohibitions;
* implementation-support findings;
* rule and registry versions.

A later stage SHALL be able to determine not only that a candidate failed Eligibility, but why it failed and under which committed rules.

---

# 36. Empty Eligibility Outcomes

Eligibility Evaluation MAY produce no ELIGIBLE candidates.

When no candidate receives ELIGIBLE, the Eligibility Evaluation Report SHALL record:

```text
NO_ELIGIBLE_PROFILE
```

This outcome SHALL NOT cause Contract 006 to:

* invent a default profile;
* reuse a previous cycle’s Committed Profile;
* bypass Eligibility;
* reinterpret an INELIGIBLE candidate as eligible;
* select an UNRESOLVED candidate;
* broaden Governance authority.

Subsequent behavior SHALL follow the registered failure and fallback policy.

Any fallback Profile Definition SHALL itself:

* appear in the Candidate Profile Set;
* receive an Eligibility result;
* satisfy all mandatory admission requirements.

---

# 37. Eligibility Phase Invariants

Eligibility Evaluation SHALL satisfy the following invariants.

## 37.1 Complete Evaluation

Every discovered candidate receives exactly one result.

## 37.2 Closed Evaluation Domain

No undiscovered definition may be evaluated.

## 37.3 Admission Purity

Eligibility determines participation permission only.

## 37.4 Failure Explicitness

Every noneligible outcome records a deterministic cause.

## 37.5 Unsupported Visibility

Implementation-unsupported candidates remain visible and resolve explicitly as UNSUPPORTED.

## 37.6 Governance Supremacy

A Governance prohibition resolves as PROHIBITED and cannot be weakened downstream.

## 37.7 Unresolved Conservatism

UNRESOLVED SHALL fail toward reduced authority and shall not proceed as eligible.

## 37.8 Append-Only Evidence

Eligibility evidence SHALL extend, not rewrite, discovery evidence.

---

# 38. Eligibility Output Boundary

Only candidates with:

```text
eligibility_outcome = ELIGIBLE
```

may proceed to Compatibility Evaluation.

All other candidates SHALL remain visible in the Eligibility Evaluation Report and later Profile Selection Report as rejected evidence.

Eligibility SHALL NOT remove candidates from historical consideration.

It SHALL only determine which candidates are admitted to the next mechanism phase.

The next Contract 006 stage SHALL determine whether each ELIGIBLE candidate can coherently construct a valid supervisory configuration for the committed context.
