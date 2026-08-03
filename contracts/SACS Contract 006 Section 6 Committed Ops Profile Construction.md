# 87. Committed Operational Profile Construction

Committed Operational Profile Construction is the commitment phase of Contract 006.

Its defining identity is:

> **Committed Operational Profile Construction transforms one selected Operational Profile Definition into the single immutable supervisory configuration authorized for the current supervisory cycle.**

Construction SHALL consume:

```text
Profile Selection Report

Committed OSV

Committed Governance Snapshot

Committed Objective Set

Profile Registry Snapshot
```

Construction SHALL NOT:

* re-run Discovery;
* re-run Eligibility;
* re-run Compatibility;
* re-run Selection;
* reinterpret Selection evidence.

Construction is a commitment operation, not an evaluation operation.

---

# 88. Commitment Preconditions

A Committed Operational Profile SHALL be constructed only when all of the following are true:

* Selection Result = PROFILE_SELECTED;
* exactly one selected candidate exists;
* the selected candidate references a valid Profile Definition;
* all required Compatibility restrictions are available;
* all required Governance artifacts remain valid for the supervisory cycle;
* all referenced artifacts belong to the same committed cycle.

If any precondition fails, Contract 006 SHALL NOT produce a Committed Operational Profile.

---

# 89. Canonical Committed Operational Profile

Every Committed Operational Profile SHALL include:

```text
committed_profile_id

cycle_id

profile_id
profile_version
definition_hash

registry_entry_id

osv_id

governance_snapshot_id

objective_set_id

selection_report_id

profile_registry_snapshot_id

effective_permissions

effective_constraints

effective_operating_region

effective_gain_profile

effective_numeric_profile

effective_reference_builder

compatibility_restrictions

authority_certificate

transition_lineage

commitment_schema_version

contract_version

committed_at

provenance_id

artifact_hash
```

The Committed Operational Profile SHALL be immutable.

---

# 90. Effective Value Resolution

The Committed Operational Profile SHALL contain resolved supervisory values.

These SHALL NOT simply duplicate the values declared by the Profile Definition.

Instead they SHALL represent the deterministic result of all preceding Contract 006 evidence.

Conceptually:

```text
Effective Value

=

Governance

∩

Profile Definition

∩

Compatibility Restrictions

∩

Selection Resolution
```

Each effective value SHALL identify:

* originating definition;
* governing authority;
* compatibility adjustments;
* final committed value;
* provenance.

Construction SHALL NOT invent effective values.

Every committed value SHALL be traceable to prior evidence.

---

# 91. Effective Permission Resolution

Effective permissions SHALL satisfy:

```text
A_effective

≼

A_governance
```

under the authority ordering defined by Contract 004.

Construction SHALL verify:

* no permission was expanded;
* no Governance prohibition was removed;
* every Compatibility restriction was preserved;
* every Selection restriction remained attached.

Failure SHALL prevent commitment.

---

# 92. Effective Constraint Resolution

Effective constraints SHALL represent the complete constraint set inherited by downstream components.

Every committed constraint SHALL identify:

* source artifact;
* authority level;
* precedence rule;
* resolution rule.

No committed constraint may disappear silently.

Constraint conflicts SHALL have been resolved before commitment.

Construction SHALL verify this invariant.

---

# 93. Effective Operating Region

Construction SHALL resolve one effective operating region.

That region SHALL equal the operating region established during Compatibility.

Construction SHALL NOT widen:

* dimensional coverage;
* numeric bounds;
* authority domain;
* execution region.

Construction MAY only preserve or further restrict the effective region.

---

# 94. Effective Gain Profile

The committed Gain Profile SHALL equal the Gain Profile resolved during Compatibility.

Construction SHALL NOT:

* calculate gains;
* tune gains;
* substitute a different Gain Profile;
* reinterpret controller mathematics.

Only the identifier SHALL be committed.

---

# 95. Authority Reduction Certificate

Every Committed Operational Profile SHALL contain an Authority Reduction Certificate.

Its defining identity is:

> **The Authority Reduction Certificate proves that supervisory authority was preserved or reduced during Contract 006 and never expanded.**

The certificate SHALL identify:

```text
governance_authority_reference

profile_definition_reference

compatibility_restrictions

effective_permissions

effective_constraints

authority_ordering_result

verification_rule_version

certificate_hash
```

Construction SHALL verify:

```text
A_effective

≼

A_governance
```

If verification fails:

Commitment SHALL fail.

---

# 96. Transition Lineage

Normal profile succession SHALL be represented through lineage rather than supersession.

Transition Lineage SHALL include:

```text
previous_cycle_id

previous_committed_profile_id

transition_reason

transition_policy_id
```

Routine succession SHALL NOT invalidate the historical correctness of the previous Committed Operational Profile.

Supersession SHALL be reserved exclusively for correction of defective artifacts.

---

# 97. Commitment Validity

A Committed Operational Profile is valid only for the supervisory cycle that produced it.

Validity SHALL NOT extend across cycle boundaries.

Reuse SHALL occur only through reselection of the same Profile Definition during a later supervisory cycle.

Every reselection SHALL produce:

* a new Selection Report;
* a new Committed Operational Profile;
* a new artifact hash.

---

# 98. Commitment Invariants

Committed Operational Profile Construction SHALL satisfy:

## Commitment Finality

Exactly one committed profile exists for a successful supervisory cycle.

## Commitment Immutability

A committed profile SHALL never mutate.

## Authority Preservation

Authority SHALL never expand.

## Constraint Preservation

All Compatibility restrictions SHALL remain attached.

## Cycle Isolation

A committed profile SHALL belong to one supervisory cycle only.

## Traceability

Every committed field SHALL resolve to earlier evidence.

## Replayability

Independent implementations SHALL reconstruct the same committed artifact from identical committed evidence.

---

# 99. Contract 006 Output Boundary

The Committed Operational Profile is the final supervisory configuration artifact produced by Contract 006.

It answers one question only:

> **What supervisory configuration has been authorized for this cycle?**

It SHALL NOT answer:

* What numeric targets should be generated?
* What Controller commands should be issued?
* What inference should execute?
* What execution behavior should occur?

Those responsibilities belong to the Reference Builder and subsequent contracts.

The Committed Operational Profile is therefore the authoritative handoff from supervisory configuration into supervisory target construction.

---

# 100. Reference Builder Handoff

The Reference Builder SHALL consume:

```text
Committed Operational Profile

Committed OSV

Committed Governance Snapshot

Committed Objective Set
```

The Reference Builder SHALL NOT consume:

* Profile Definitions;
* Candidate Profile Sets;
* Eligibility Reports;
* Compatibility Reports;
* Selection Reports.

Those artifacts remain replay evidence only.

The Committed Operational Profile is the sole Contract 006 supervisory configuration artifact authorized for downstream use.

This establishes the Contract 006 terminal invariant:

> **Every downstream supervisory component SHALL consume only the committed supervisory configuration and never reconstruct supervisory intent from earlier evaluation artifacts.**
