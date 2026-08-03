# SACS Contract 010 — Modes, Guards, Resets, and Dwell Time

## Section 5 — Supervisory Mode Registry

### Purpose

Section 5 defines the constitutional registry through which Supervisory Modes are declared, versioned, validated, and made available for supervisory continuity.

The Supervisory Mode Registry is the sole authoritative source of mode definitions recognized by Contract 010.

The registry governs the existence of modes.

It does not determine which mode is active.

---

## Constitutional Question

The Supervisory Mode Registry answers exactly one constitutional question:

> **Which Supervisory Mode Definitions are constitutionally recognized for supervisory continuity?**

No additional constitutional question is answered by this section.

---

## Architectural Identity

The Supervisory Mode Registry establishes the immutable constitutional catalog of Supervisory Mode Definitions.

Its defining identity is:

> **The Supervisory Mode Registry SHALL maintain the complete, versioned, integrity-protected set of Supervisory Mode Definitions recognized by Contract 010 without determining supervisory state or committing supervisory continuity.**

The registry defines available modes.

Mode Commitment determines the active mode.

---

## Registry Scope

The Supervisory Mode Registry SHALL maintain every constitutionally recognized Supervisory Mode Definition.

At minimum, the registry SHALL preserve:

* Supervisory Mode Definitions;
* mode identities;
* mode versions;
* compatibility relationships;
* supersession relationships;
* Transition Definitions;
* Guard Definition references;
* Dwell-Time Definition references;
* Hysteresis Definition references;
* Reset Definition references;
* Operational Profile compatibility references;
* governance compatibility references;
* initialization eligibility;
* registry metadata.

No recognized Supervisory Mode SHALL exist outside the active registry.

---

## Registry Identity

Every registry snapshot SHALL possess immutable identity.

At minimum, it SHALL include:

* registry_snapshot_id;
* registry_version;
* registry_name;
* schema_version;
* contract_version;
* creation_reference;
* superseded_snapshot_reference;
* compatibility_profile;
* registry_hash.

A registry snapshot SHALL remain immutable after commitment.

---

## Registry Principles

### Completeness

Every Supervisory Mode recognized by Contract 010 SHALL exist within exactly one committed registry snapshot.

No recognized mode SHALL exist outside the registry.

---

### Determinism

Equivalent registry contents SHALL produce identical registry identities.

Registry ordering SHALL NOT affect constitutional meaning.

---

### Immutability

Committed registry snapshots SHALL NOT be modified.

Changes SHALL require creation of a new registry snapshot.

---

### Referential Integrity

Every registry reference SHALL resolve to exactly one immutable artifact.

Broken references SHALL prevent successful registry validation.

---

### Version Preservation

Historical supervisory cycles SHALL continue referencing the registry snapshot active when those cycles occurred.

Historical interpretation SHALL NOT silently migrate to newer registry versions.

---

## Registry Contents

Every Supervisory Mode Registry SHALL contain, for each mode:

* immutable mode identity;
* current version;
* supported compatibility classes;
* transition relationships;
* permitted source modes;
* permitted destination modes;
* prohibited transitions;
* applicable guard references;
* dwell definition references;
* hysteresis references;
* reset references;
* profile compatibility;
* governance compatibility;
* authority classification;
* lifecycle status.

---

## Registry Lifecycle

Every Supervisory Mode Definition SHALL exist in exactly one lifecycle state.

Permitted states include:

* DRAFT
* ACTIVE
* SUPERSEDED
* DEPRECATED
* RETIRED

Only ACTIVE mode definitions SHALL be eligible for new Mode Commitments unless explicitly authorized by governing policy.

Historical records SHALL continue referencing superseded definitions without modification.

---

## Registry Compatibility

The registry SHALL preserve explicit compatibility relationships.

Compatibility SHALL include, where applicable:

* backward compatibility;
* forward compatibility;
* conditional compatibility;
* incompatibility;
* supersession.

Compatibility SHALL NOT be inferred from matching names or similar descriptions.

---

## Registry Validation

Registry validation SHALL verify:

* registry integrity;
* identity uniqueness;
* version consistency;
* definition completeness;
* transition consistency;
* guard consistency;
* reset consistency;
* dwell consistency;
* hysteresis consistency;
* profile compatibility;
* governance compatibility.

Registry validation SHALL NOT evaluate supervisory continuity.

It validates registry integrity only.

---

## Registry Relationships

The Supervisory Mode Registry SHALL maintain immutable relationships with:

* Transition Definition Registry;
* Guard Definition Registry;
* Reset Definition Registry;
* Dwell-Time Definition Registry;
* Hysteresis Definition Registry;
* Operational Profile Registry;
* Governance policy references.

These relationships preserve constitutional consistency.

They SHALL NOT transfer authority.

---

## Registry Independence

The Supervisory Mode Registry SHALL remain independent of:

* implementation language;
* storage engine;
* runtime architecture;
* deployment topology;
* transport protocol.

Registry identity SHALL remain constitutionally identical regardless of implementation.

---

## Registry Outcomes

Registry validation SHALL produce exactly one of:

### VALID

The registry snapshot is constitutionally complete and internally consistent.

### INCOMPLETE

One or more mandatory definitions are absent.

### INCONSISTENT

Conflicting constitutional relationships exist.

### UNRESOLVABLE

One or more required references cannot be resolved.

### INVALID

The registry violates one or more constitutional requirements.

Registry failures SHALL prevent use of the affected snapshot for new Mode Commitments.

---

## Registry Failure Handling

Contract 010 SHALL NOT:

* fabricate missing mode definitions;
* infer undeclared transitions;
* repair invalid references;
* merge conflicting registries;
* silently replace registry versions;
* ignore integrity failures.

Registry failures SHALL be represented explicitly.

They SHALL NOT produce an authoritative Supervisory Mode.

---

## Relationship to Mode Commitment

The Supervisory Mode Registry defines what modes exist.

Mode Commitment determines which mode is active.

Accordingly:

```text
Mode Registry
        ↓
Mode Definition
        ↓
Transition Evaluation
        ↓
Mode Commitment
        ↓
Committed Supervisory Mode
```

The registry SHALL NOT:

* commit modes;
* perform transition evaluation;
* evaluate guards;
* evaluate dwell;
* authorize resets.

Those authorities belong to subsequent sections.

---

## Architectural Invariants

The Supervisory Mode Registry SHALL satisfy the following invariants:

1. Every recognized Supervisory Mode SHALL exist within exactly one committed registry snapshot.

2. Every mode identity SHALL be unique.

3. Every mode version SHALL remain immutable.

4. Every constitutional relationship SHALL be explicitly represented.

5. Registry validation SHALL remain independent of supervisory continuity.

6. Historical cycles SHALL preserve their original registry references.

7. Registry updates SHALL occur only through creation of new snapshots.

8. Registry identity SHALL remain implementation independent.

9. Registry failures SHALL remain explicit.

10. Registry authority SHALL never establish the active Supervisory Mode.

---

## Constitutional Sequence

The constitutional progression established by this section is:

```text
Supervisory Mode Definitions
        ↓
Supervisory Mode Registry
        ↓
Registry Validation
        ↓
Validated Registry Snapshot
        ↓
Transition Evaluation
        ↓
Mode Commitment
```

Section 5 establishes the Supervisory Mode Registry as the immutable constitutional catalog of supervisory operating regimes. It defines which modes are recognized by Contract 010, preserves their identities and relationships, and guarantees deterministic, versioned, integrity-protected mode definitions without assuming the authority to evaluate transitions or establish the active supervisory mode.

The registry defines supervisory continuity possibilities.

Mode Commitment determines supervisory continuity reality.
