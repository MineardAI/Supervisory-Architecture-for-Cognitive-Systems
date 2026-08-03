# SACS Contract 010 — Modes, Guards, Resets, and Dwell Time

## Section 4 — Supervisory Mode Identity

### Purpose

Section 4 defines the constitutional identity of every Supervisory Mode recognized by Contract 010.

Supervisory Mode Identity establishes the immutable definition through which a mode is registered, referenced, evaluated, activated, persisted, transitioned, and committed across supervisory cycles.

A mode identity defines one supervisory continuity regime.

It does not establish that the mode is currently active.

---

## Constitutional Question

Supervisory Mode Identity answers exactly one constitutional question:

> **What immutable constitutional definition identifies this supervisory operating mode?**

No additional constitutional question is answered by this section.

---

## Architectural Identity

Every Supervisory Mode SHALL possess exactly one immutable Supervisory Mode Identity.

Its defining identity is:

> **A Supervisory Mode Identity uniquely defines one versioned supervisory continuity regime, including its constitutional purpose, authority restrictions, transition relationships, dwell requirements, hysteresis requirements, reset relationships, and permitted Operational Profile associations.**

Mode identity establishes definition.

Mode commitment establishes activation.

These SHALL remain distinct.

---

## Mode Definition and Active Mode

Contract 010 SHALL distinguish between:

### Supervisory Mode Definition

The immutable declarative artifact describing one supervisory operating regime.

### Committed Supervisory Mode

The authoritative commitment establishing that one Supervisory Mode Definition governs the applicable continuity interval.

A mode definition MAY exist without being active.

A mode SHALL become active only through lawful Mode Commitment.

Accordingly:

```text
Supervisory Mode Definition
        ≠
Committed Supervisory Mode
```

A mode definition establishes what a mode is.

A mode commitment establishes which mode is authoritative.

---

## Mode Identity Requirements

Every Supervisory Mode Definition SHALL possess immutable identity.

At minimum, it SHALL include:

* supervisory_mode_id;
* mode_name;
* mode_version;
* mode_class;
* mode_purpose;
* mode_authority_level;
* mode_restriction_class;
* entry_policy_id;
* continuation_policy_id;
* exit_policy_id;
* permitted_transition_set_id;
* prohibited_transition_set_id;
* guard_definition_set_id;
* dwell_time_definition_id;
* hysteresis_definition_id;
* reset_definition_set_id;
* permitted_operational_profile_set_id;
* prohibited_operational_profile_set_id;
* applicable_governance_requirements;
* failure_policy_id;
* initialization_eligibility;
* terminal_mode_indicator;
* safe_mode_indicator;
* schema_version;
* contract_version;
* registry_snapshot_id;
* provenance_id;
* mode_definition_hash.

The mode-definition hash SHALL bind every constitutionally significant field.

---

## Mode Name

Every Supervisory Mode SHALL possess one canonical name.

The canonical name SHALL:

* be unique within the applicable Mode Definition Registry;
* remain stable for the lifetime of the definition;
* refer to exactly one constitutional meaning;
* remain distinct from display labels, aliases, descriptions, or implementation-specific names.

Aliases MAY be provided for explanatory purposes.

Aliases SHALL NOT establish constitutional identity.

Mode identity SHALL be determined by the immutable mode identifier, version, and definition hash.

---

## Mode Version

Every Supervisory Mode Definition SHALL be versioned.

A new version SHALL be required when any constitutionally significant property changes, including:

* entry requirements;
* continuation requirements;
* exit requirements;
* guard relationships;
* transition relationships;
* precedence relationships;
* dwell requirements;
* hysteresis rules;
* reset requirements;
* profile compatibility;
* authority restrictions;
* failure behavior.

A mode definition SHALL NOT be modified in place.

A revised definition SHALL receive:

* a new version;
* a new definition hash;
* explicit compatibility status;
* supersession lineage where applicable.

---

## Mode Class

Every Supervisory Mode SHALL declare one mode class.

Mode classes MAY include:

* INITIALIZATION;
* NOMINAL;
* EXPANSIVE;
* STABILIZING;
* RECOVERY;
* RESTRICTED;
* FAULT;
* TERMINAL.

The exact class vocabulary SHALL be defined by the authorized Mode Definition Registry.

Mode class provides organizational meaning.

It SHALL NOT independently determine:

* transition precedence;
* execution permission;
* governance authority;
* profile eligibility;
* safety priority.

Those properties SHALL remain explicitly defined.

---

## Mode Purpose

Every Supervisory Mode Definition SHALL state its constitutional purpose.

The purpose SHALL describe:

* the continuity condition represented by the mode;
* the supervisory behavior the mode organizes;
* the operational circumstances under which the mode is applicable;
* the constitutional outcome the mode is intended to preserve.

The purpose SHALL NOT:

* define implementation behavior;
* include hidden control logic;
* redefine operational state;
* act as a substitute for guard predicates;
* authorize undeclared transitions.

Purpose is explanatory and constitutional.

Transition behavior remains governed by explicit definitions.

---

## Mode Authority Level

Every mode SHALL declare its applicable authority level or restriction class.

The declaration SHALL identify whether the mode:

* preserves ordinary supervisory authority;
* restricts available supervisory actions;
* limits eligible Operational Profiles;
* requires reduced execution freedom;
* prohibits specific transitions;
* requires stabilization behavior;
* represents a terminal continuity condition.

A mode authority level SHALL NOT override Governance.

Where a mode definition and Governance conflict, Governance SHALL prevail.

A mode SHALL never increase authority beyond the active Governance Snapshot.

---

## Entry Policy

Every Supervisory Mode Definition SHALL reference one immutable Entry Policy.

The Entry Policy SHALL identify:

* eligible source modes;
* prohibited source modes;
* mandatory entry guards;
* optional entry guards;
* minimum confidence requirements;
* required temporal context;
* applicable dwell exceptions;
* required reset conditions;
* applicable governance requirements;
* required profile relationships;
* entry-failure behavior.

Entry Policy SHALL define eligibility only.

It SHALL NOT commit the mode.

---

## Continuation Policy

Every Supervisory Mode Definition SHALL reference one immutable Continuation Policy.

The Continuation Policy SHALL identify the conditions under which the current mode remains constitutionally valid.

It MAY include:

* continuation guards;
* maximum uncertainty limits;
* required profile compatibility;
* required governance compatibility;
* maximum dwell limits;
* minimum stability requirements;
* mandatory exit triggers;
* fault conditions.

If continuation remains valid and no higher-precedence transition is authorized, the current mode SHALL persist.

---

## Exit Policy

Every Supervisory Mode Definition SHALL reference one immutable Exit Policy.

The Exit Policy SHALL identify:

* mandatory exit conditions;
* permitted exit conditions;
* prohibited exits;
* destination constraints;
* exit guards;
* dwell completion requirements;
* hysteresis requirements;
* required reset relationships;
* failure behavior.

An Exit Policy establishes when departure is eligible or required.

It SHALL NOT select a destination unless that destination is explicitly defined by the applicable Transition Definition.

---

## Transition Relationships

Every Supervisory Mode Definition SHALL reference its permitted and prohibited transition relationships.

A permitted transition relationship SHALL identify:

* source mode;
* destination mode;
* transition definition;
* transition class;
* required guards;
* required dwell state;
* required hysteresis state;
* required reset definition;
* transition priority;
* failure behavior.

A prohibited transition relationship SHALL identify a transition that SHALL NOT occur under the applicable mode version.

Absence of a permitted transition SHALL NOT imply permission.

Transitions SHALL be closed by declaration.

---

## Profile Relationships

Every Supervisory Mode Definition SHALL declare its relationship to Operational Profiles.

The definition MAY identify:

* profiles permitted within the mode;
* profiles prohibited within the mode;
* profiles required for mode entry;
* profiles compatible with mode continuation;
* profile transition requests associated with mode transition;
* fallback profile requirements;
* safe-profile relationships.

Contract 010 SHALL preserve these relationships.

Contract 006 SHALL remain the exclusive authority for profile selection, construction, and commitment.

A mode definition SHALL NOT embed an Operational Profile.

---

## Guard Relationships

Every mode SHALL reference the Guard Definitions applicable to:

* entry;
* continuation;
* exit;
* emergency transition;
* transition prohibition;
* reset eligibility.

Guard references SHALL be immutable.

A mode SHALL NOT contain hidden guard behavior outside its declared Guard Definition set.

Every guard used to establish mode authority SHALL be identifiable, versioned, and auditable.

---

## Dwell-Time Relationship

Every mode SHALL reference one applicable Dwell-Time Definition or explicitly declare that no dwell requirement applies.

The Dwell-Time Definition MAY specify:

* minimum residence duration;
* maximum residence duration;
* minimum completed-cycle count;
* maximum completed-cycle count;
* emergency exit exceptions;
* post-entry stabilization interval;
* post-reset dwell requirements.

Dwell requirements SHALL remain external declarative artifacts referenced by the mode definition.

A mode SHALL NOT calculate dwell time itself.

---

## Hysteresis Relationship

Every mode SHALL reference one applicable Hysteresis Definition or explicitly declare that no hysteresis rule applies.

The Hysteresis Definition MAY govern:

* entry thresholds;
* exit thresholds;
* persistence bands;
* confidence margins;
* confirmation-cycle counts;
* directional transition boundaries;
* recovery margins.

A mode identity SHALL bind the applicable hysteresis definition by identifier, version, and hash.

---

## Reset Relationships

Every mode SHALL identify the Reset Definitions that may lawfully apply during:

* entry;
* continuation;
* exit;
* emergency transition;
* recovery;
* fault handling.

A mode MAY require a reset before commitment of the destination mode.

A mode MAY prohibit resets while active.

A mode SHALL NOT perform a Reset.

Reset execution SHALL remain the responsibility of the component owning the affected state through a bounded authorized interface.

---

## Safe Mode Designation

A Supervisory Mode Definition MAY be designated as a safe mode.

A safe mode designation SHALL mean that the mode is approved for reduced-authority continuity under declared failure conditions.

Safe mode designation SHALL NOT imply:

* universal eligibility;
* automatic entry;
* unrestricted transition;
* exemption from Governance;
* exemption from guard evaluation;
* exemption from commitment.

Entry into a safe mode SHALL remain governed by explicit transition rules.

---

## Terminal Mode Designation

A Supervisory Mode Definition MAY be designated as terminal.

A terminal mode indicates that ordinary mode transition is no longer permitted unless an explicit recovery or reinitialization policy authorizes further continuity.

A terminal mode SHALL define:

* terminal entry conditions;
* permitted terminal outputs;
* whether recovery is possible;
* whether reinitialization is permitted;
* required evidence;
* commitment behavior.

A terminal mode SHALL NOT be silently exited.

---

## Initial Mode Eligibility

A Supervisory Mode Definition SHALL explicitly declare whether it is eligible for initial mode commitment.

Initial eligibility SHALL require:

* authorization by Initial Mode Policy;
* compatibility with active Governance;
* compatibility with startup Operational State;
* required capability availability;
* satisfied initialization guards;
* deterministic initialization selection.

A mode SHALL NOT become the initial mode merely because it appears first in a registry or implementation list.

---

## Canonical Initial Mode Vocabulary

The initial SACS mode vocabulary is:

* Entry;
* Bloom;
* Harmonic;
* Fracture;
* Stabilize.

Each name SHALL correspond to exactly one registered Supervisory Mode Definition within an applicable registry version.

The names alone SHALL have no normative force.

Their constitutional meaning SHALL be established only through their complete registered definitions.

---

## Entry Mode Identity

The Entry mode SHALL represent the authorized initialization and early-assessment continuity regime.

Its precise definition SHALL establish:

* initialization eligibility;
* startup guards;
* required startup dwell;
* eligible next modes;
* prohibited transitions;
* applicable authority restrictions;
* reset behavior;
* failure behavior.

Entry SHALL NOT be interpreted merely as the first chronological mode.

It is a constitutionally defined operating regime.

---

## Bloom Mode Identity

The Bloom mode SHALL represent an authorized expansive or development-oriented supervisory continuity regime.

Its precise definition SHALL establish:

* required operational conditions;
* permitted authority bounds;
* eligible profiles;
* entry and exit guards;
* dwell requirements;
* hysteresis boundaries;
* transition relationships;
* failure behavior.

Bloom SHALL NOT imply unrestricted execution freedom.

All authority remains bounded by Governance and committed profile limits.

---

## Harmonic Mode Identity

The Harmonic mode SHALL represent an authorized nominal or coherently stabilized supervisory continuity regime.

Its precise definition SHALL establish:

* required operational conditions;
* continuation requirements;
* stability or coherence requirements;
* eligible profiles;
* transition relationships;
* dwell requirements;
* hysteresis boundaries;
* reset behavior.

Harmonic SHALL NOT be inferred merely from successful execution or low observed error.

Its authority SHALL arise only through explicit mode commitment.

---

## Fracture Mode Identity

The Fracture mode SHALL represent a constitutionally defined discontinuity, instability, conflict, or integrity-degraded supervisory continuity regime.

Its precise definition SHALL establish:

* qualifying guard conditions;
* required authority reduction;
* prohibited actions;
* eligible recovery transitions;
* maximum residence limits;
* escalation requirements;
* reset relationships;
* failure behavior.

Fracture SHALL NOT be treated as a psychological or semantic diagnosis.

It is a supervisory continuity classification.

---

## Stabilize Mode Identity

The Stabilize mode SHALL represent a reduced-authority recovery and convergence regime.

Its precise definition SHALL establish:

* eligible entry conditions;
* required restrictions;
* applicable stabilization profiles;
* minimum dwell requirements;
* exit guards;
* recovery margins;
* reset requirements;
* transition relationships;
* failure behavior.

Stabilize SHALL NOT guarantee recovery.

It defines the constitutional regime under which recovery may be attempted.

---

## Identity Preservation

A Supervisory Mode Identity SHALL remain immutable.

Accordingly:

* the mode identifier SHALL NOT be reassigned;
* the mode definition SHALL NOT be modified in place;
* the mode hash SHALL NOT change without a new version;
* one identity SHALL NOT represent multiple mode meanings;
* multiple identities SHALL NOT represent the same canonical version;
* implementation aliases SHALL NOT replace canonical identity.

Any change to constitutional meaning SHALL produce a new mode version.

---

## Mode Identity Compatibility

Compatibility between mode versions SHALL be explicit.

Permitted compatibility classifications MAY include:

* IDENTICAL;
* BACKWARD_COMPATIBLE;
* FORWARD_COMPATIBLE;
* CONDITIONALLY_COMPATIBLE;
* INCOMPATIBLE;
* SUPERSEDED.

Compatibility SHALL NOT be inferred from matching names.

A mode transition evaluation SHALL use exactly one committed mode-definition version for each referenced source and destination mode.

Silent mixing of mode versions is prohibited.

---

## Mode Definition Registry Binding

Every Supervisory Mode Definition SHALL belong to exactly one committed Mode Definition Registry snapshot.

The registry snapshot SHALL establish:

* the recognized mode set;
* mode versions;
* definition hashes;
* transition-reference validity;
* guard-reference validity;
* dwell-reference validity;
* hysteresis-reference validity;
* reset-reference validity;
* compatibility relationships;
* supersession relationships.

A mode definition outside the active registry snapshot SHALL NOT be eligible for commitment.

---

## Mode Identity Outcomes

Mode Identity evaluation SHALL produce exactly one of the following outcomes:

### VALID

The mode identity is complete, internally consistent, registered, and compatible with the active constitutional context.

### INVALID

The mode identity violates one or more mandatory structural or semantic requirements.

### UNREGISTERED

The mode identity does not exist within the applicable committed Mode Definition Registry snapshot.

### INCOMPATIBLE

The mode definition is incompatible with the applicable registry, transition, governance, profile, or contract context.

### SUPERSEDED

The mode definition has been replaced by a later version and is not eligible for new commitment under the active policy.

### INDETERMINATE

Available evidence is insufficient to determine identity validity.

An indeterminate identity SHALL NOT become authoritative.

---

## Identity Failure Handling

Contract 010 SHALL NOT:

* infer a missing mode identifier;
* select a definition by name alone;
* repair an invalid mode definition;
* merge multiple mode versions;
* substitute an allegedly equivalent mode;
* ignore a hash mismatch;
* commit an unregistered mode;
* silently upgrade a mode version.

A mode identity failure SHALL produce explicit failure evidence.

It SHALL NOT produce a successful Mode Commitment.

---

## Architectural Invariants

Supervisory Mode Identity SHALL satisfy the following invariants:

1. Every Supervisory Mode Definition SHALL possess exactly one immutable identity.

2. Every mode identity SHALL refer to exactly one constitutional mode meaning.

3. Mode definition and mode activation SHALL remain distinct.

4. A mode SHALL become authoritative only through successful Mode Commitment.

5. Every constitutionally significant definition change SHALL produce a new version and hash.

6. Every active mode SHALL reference one registered mode definition.

7. Transition, guard, dwell, hysteresis, reset, and profile relationships SHALL be explicitly declared.

8. Mode names and aliases SHALL NOT substitute for canonical identity.

9. Mode identity SHALL remain independent of implementation technology.

10. An invalid, unregistered, incompatible, superseded, or indeterminate mode SHALL NOT be committed as authoritative.

---

## Constitutional Sequence

The mode-identity progression established by this section is:

```text
Mode Definition Registry
        ↓
Supervisory Mode Definition
        ↓
Mode Identity Validation
        ↓
Transition and Continuity Evaluation
        ↓
Mode Commitment
        ↓
Committed Supervisory Mode
```

Section 4 establishes the immutable constitutional identity of every Supervisory Mode recognized by Contract 010.

It ensures that each mode is uniquely defined, versioned, registered, integrity-bound, and constitutionally distinct from its activation.

A mode definition states what a supervisory continuity regime is.

Only commitment determines whether that regime is authoritative.
