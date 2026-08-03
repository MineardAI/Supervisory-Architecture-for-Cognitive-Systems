# SACS Contract 010 — Modes, Guards, Resets, and Dwell Time

## Section 2 — Architectural Identity

### Purpose

Section 2 establishes the constitutional identity of supervisory modes and defines the architectural principles governing mode continuity, persistence, and transition across supervisory cycles.

Where Section 1 defines why Contract 010 exists, this section defines what Contract 010 fundamentally is within the SACS constitutional architecture.

Contract 010 is not an observation authority.

It is not a governance authority.

It is not a profile-selection authority.

It is not a controller.

It is the constitutional authority that preserves and transitions supervisory operating state across cycles.

---

## Architectural Identity

Contract 010 defines the constitutional continuity authority of SACS.

Its defining identity is:

> **Contract 010 deterministically establishes, preserves, and transitions the authoritative supervisory operating mode across supervisory cycles by applying committed mode definitions, guard predicates, transition precedence, hysteresis, dwell-time requirements, and reset rules without redefining operational state, governance, supervisory objectives, or control authority.**

The supervisory mode identifies the operating regime under which supervisory behavior is organized.

It does not independently determine supervisory intent.

It does not independently authorize execution.

It does not replace the Operational Profile.

---

## Constitutional Transformation

Contract 010 owns exactly one constitutional transformation:

```text
Current Committed Supervisory Mode
+
Committed Supervisory Context
        ↓
Committed Supervisory Mode
```

The input mode represents the authoritative operating regime entering the transition evaluation.

The output mode represents the authoritative operating regime governing subsequent supervisory continuity.

The output MAY preserve the existing mode or establish a lawful transition to another mode.

In either case, exactly one Committed Supervisory Mode and one Committed Supervisory Continuity Record SHALL result.

---

## Supervisory Mode

Contract 010 commitment produces the following canonical artifacts:

```text
Mode Commitment
    v
Committed Supervisory Mode
    v
Committed Supervisory Continuity Record
```

A Supervisory Mode is an immutable, versioned constitutional definition of an operating regime within SACS.

A mode SHALL define, at minimum:

* mode identity;
* mode purpose;
* entry requirements;
* continuation requirements;
* exit requirements;
* permitted transitions;
* prohibited transitions;
* applicable guard references;
* hysteresis requirements;
* dwell-time requirements;
* reset requirements;
* permitted Operational Profile relationships;
* applicable authority restrictions;
* failure behavior;
* version and integrity references.

A mode SHALL organize supervisory continuity.

A mode SHALL NOT:

* define raw observations;
* estimate operational state;
* author governance;
* create supervisory objectives;
* independently construct a Reference State;
* compute a Control Command;
* translate commands;
* apply runtime instructions.

---

## Distinction Between Mode and Operational Profile

Supervisory Modes and Operational Profiles are constitutionally distinct.

An Operational Profile defines the committed supervisory expectations, limits, gains, permissions, and Reference State parameters applicable to a supervisory condition.

A Supervisory Mode defines the cross-cycle operating regime within which those supervisory conditions may persist or transition.

Accordingly:

```text
Supervisory Mode
        ↓
Defines continuity regime

Operational Profile
        ↓
Defines supervisory operating expectations
```

A mode MAY constrain which Operational Profiles are eligible.

A mode MAY require transition to a different Operational Profile through the authority of Contract 006.

A mode SHALL NOT construct, modify, or commit an Operational Profile itself.

Contract 006 remains authoritative for Operational Profiles and Reference States.

Contract 010 remains authoritative for mode continuity and transition.

---

## Distinction Between Mode and State

A Supervisory Mode SHALL NOT be treated as an estimate of operational state.

Operational state describes the condition of the supervised system.

Supervisory mode describes how SACS is constitutionally organized to respond across cycles.

Therefore:

```text
Operational State
        ≠
Supervisory Mode
```

An Operational State Vector may contribute evidence to mode-transition evaluation.

It SHALL NOT itself become the mode.

Contract 010 SHALL consume committed operational state without redefining its meaning.

---

## Distinction Between Mode and Outcome

A Supervisory Mode SHALL remain distinct from an execution disposition or runtime outcome.

For example:

* a blocked execution does not automatically imply a Stabilize mode;
* a successful execution does not automatically imply a Harmonic mode;
* a runtime fault does not automatically imply a Fracture mode.

Mode transitions SHALL occur only when committed transition rules and guard predicates are satisfied.

Execution outcomes may contribute evidence.

They SHALL NOT independently establish mode authority.

---

## Principle of Continuity

Contract 010 establishes the Principle of Supervisory Continuity.

> **Every supervisory cycle SHALL begin with exactly one authoritative Committed Supervisory Mode inherited from a prior lawful commitment or initialized through an explicitly authorized initial-mode procedure.**

Supervisory continuity SHALL NOT depend upon:

* ambient process state;
* uncommitted memory;
* implementation defaults;
* runtime convenience;
* undeclared heuristics;
* prior mode assumptions.

The active mode SHALL always be constitutionally identifiable.

---

## Principle of Explicit Transition

Mode transition SHALL always be explicit.

A transition SHALL require:

* one authoritative current mode;
* one candidate destination mode;
* satisfied transition preconditions;
* satisfied guard predicates;
* satisfied dwell-time requirements;
* satisfied hysteresis requirements;
* applicable reset evaluation;
* deterministic transition precedence;
* one successful mode commitment.

A mode SHALL NOT change merely because implementation state changed.

No implicit transition SHALL possess constitutional authority.

---

## Principle of Mode Persistence

In the absence of a valid transition, the current Committed Supervisory Mode SHALL persist.

Persistence is a constitutional outcome.

It SHALL NOT be treated as failure to transition.

Formally:

```text
No authorized transition
        ↓
Current mode remains authoritative
```

A mode remains active until:

* an authorized transition is committed;
* the mode is explicitly superseded;
* the supervisory system enters an explicit terminal or fault state defined by this contract.

---

## Principle of Guarded Transition

Every transition SHALL be governed by explicit Guard Definitions.

Guards determine whether declared transition conditions are satisfied.

Guards SHALL:

* consume only committed constitutional inputs;
* remain deterministic;
* preserve uncertainty;
* produce explicit evaluation outcomes;
* remain independent of implementation-specific execution order.

Guards SHALL NOT:

* invent missing state;
* infer authority from absence;
* reinterpret governance;
* select arbitrary destination modes;
* modify the active mode directly.

Guard satisfaction makes a transition eligible.

It does not itself commit the transition.

---

## Principle of Dwell Stability

Dwell Time protects supervisory continuity against rapid, unstable, or oscillatory mode switching.

A mode SHALL remain active for its required minimum dwell interval unless an explicitly defined emergency or higher-precedence transition authorizes earlier departure.

Dwell Time SHALL be evaluated using committed temporal context.

Uncontrolled local wall-clock behavior SHALL NOT determine constitutional mode transitions.

---

## Principle of Hysteresis

Hysteresis SHALL prevent mode transitions from oscillating around uncertain or marginal transition boundaries.

Entry and exit conditions MAY differ.

Where hysteresis applies:

```text
Entry threshold
        ≠
Exit threshold
```

Hysteresis SHALL be explicitly defined.

It SHALL NOT arise from hidden implementation state.

---

## Principle of Reset Separation

A Reset is a constitutionally defined state-transition operation associated with a mode transition or stabilization requirement.

A Reset SHALL NOT be treated as a mode.

A Reset MAY:

* clear authorized transitional state;
* reinitialize declared estimator or continuity context;
* establish a declared post-transition reference condition;
* invalidate explicitly identified transient state;
* produce reset evidence.

A Reset SHALL NOT:

* rewrite historical artifacts;
* modify committed observations;
* alter governance;
* reconstruct missing state;
* erase evidence;
* silently broaden authority.

Reset authority SHALL remain bounded by committed Reset Definitions.

---

## Principle of Deterministic Precedence

Where multiple transitions are simultaneously eligible, Contract 010 SHALL resolve them using committed Transition Precedence Rules.

Transition selection SHALL NOT depend upon:

* evaluation order;
* arrival order;
* thread scheduling;
* storage order;
* nondeterministic arbitration;
* implementation preference.

Equivalent canonical inputs SHALL produce the same selected transition.

Where deterministic precedence cannot resolve competing transitions, Contract 010 SHALL produce an explicit transition conflict or fault rather than choosing arbitrarily.

---

## Principle of Reduced Authority

Contract 010 SHALL fail toward reduced authority.

When required state, governance, mode definitions, guard definitions, temporal context, dwell evidence, hysteresis state, reset definitions, or transition policy is unavailable, invalid, incompatible, or indeterminate, Contract 010 SHALL NOT transition into a mode that increases execution freedom.

Applicable safe outcomes MAY include:

* preserve current mode;
* transition to an explicitly authorized stabilization mode;
* defer transition;
* block transition;
* produce a mode-transition fault.

The exact outcome SHALL be determined by committed policy.

---

## Initial Mode Vocabulary

The initial SACS mode vocabulary is:

* Entry;
* Bloom;
* Harmonic;
* Fracture;
* Stabilize.

These names identify supervisory operating regimes only.

They SHALL NOT be interpreted as:

* emotional states;
* psychological diagnoses;
* persona labels;
* user classifications;
* content categories;
* aesthetic metaphors.

Each mode SHALL receive a precise normative definition within this contract or an authorized mode registry.

No implementation MAY claim conformance based solely upon use of the mode names.

---

## Committed Supervisory Mode

The canonical committed outputs of Contract 010 are the **Committed Supervisory Mode** and the **Committed Supervisory Continuity Record**.

A Committed Supervisory Mode SHALL identify, at minimum:

* supervisory_mode_commitment_id;
* supervisory_cycle_id;
* previous_mode_id;
* active_mode_id;
* mode_definition_version;
* transition_id, where a transition occurred;
* transition_evaluation_id;
* guard_evaluation_set_id;
* dwell_evaluation_id;
* hysteresis_context_id;
* reset_record_id, where applicable;
* transition_precedence_policy_id;
* governance_snapshot_id;
* operational_state_vector_id;
* applicable_operational_profile_id;
* commitment_status;
* schema_version;
* contract_version;
* provenance_id;
* artifact_hash.

The Committed Supervisory Mode SHALL constitute the sole authoritative mode for the applicable continuity boundary.

The **Committed Supervisory Continuity Record** is the immutable Contract 010 artifact that binds the Committed Supervisory Mode to the mode-transition evaluation, guard results, transition precedence, reset actions, hysteresis evaluation, dwell-time enforcement, referenced Committed Execution Disposition, and all continuity lineage required for deterministic replay and Contract 011 consumption.

The Committed Supervisory Mode is the authoritative supervisory mode established for the next continuity boundary. The Committed Supervisory Continuity Record is the committed evidence and lineage artifact establishing how and why that mode became authoritative.

---

## Mode Commitment

Mode authority SHALL arise only through explicit commitment.

A mode definition is not an active mode.

A satisfied guard is not an active mode.

A selected transition is not an active mode.

A mode becomes authoritative only when a successful Mode Commitment Record binds the supervisory continuity context to exactly one mode identity and hash.

The required progression is:

```text
Mode Definitions
        ↓
Transition Evaluation
        ↓
Guard Evaluation
        ↓
Dwell and Hysteresis Evaluation
        ↓
Reset Evaluation
        ↓
Selected Mode Result
        ↓
Mode Commitment
        ↓
Committed Supervisory Mode
        â†“
Committed Supervisory Continuity Record
```

---

## Architectural Invariants

Contract 010 SHALL satisfy the following invariants:

1. Every supervisory continuity boundary SHALL possess exactly one authoritative Committed Supervisory Mode.

- Every supervisory continuity boundary SHALL possess exactly one immutable Committed Supervisory Continuity Record bound to that mode outcome.

2. Every mode transition SHALL originate from exactly one authoritative current mode.

3. Every transition SHALL be explicitly authorized by committed transition rules.

4. Guard satisfaction SHALL NOT independently establish mode authority.

5. In the absence of an authorized transition, the current mode SHALL persist.

6. Dwell-time and hysteresis requirements SHALL be explicit and deterministic.

7. Reset operations SHALL remain distinct from mode identity and mode commitment.

8. Mode transitions SHALL preserve governance, supervisory objectives, Operational Profile ownership, and upstream artifact identity.

9. Equivalent committed inputs SHALL produce identical mode-transition outcomes.

10. Mode authority SHALL arise only through commitment.

---

## Constitutional Progression

The continuity transformation established by Contract 010 is:

```text
Committed Supervisory Context
        ↓
Current Committed Supervisory Mode
        ↓
Guard Evaluation
        ↓
Dwell and Hysteresis Evaluation
        ↓
Transition Precedence
        ↓
Reset Evaluation
        ↓
Mode Commitment
        ↓
Committed Supervisory Mode
```

Section 2 establishes Contract 010 as the constitutional continuity authority of SACS. It defines Supervisory Modes as immutable operating-regime definitions, distinguishes modes from operational state, Operational Profiles, execution outcomes, and resets, and establishes that mode persistence and transition are lawful only through deterministic evaluation and explicit commitment. It also establishes the Committed Supervisory Continuity Record as the immutable evidence and lineage artifact for Contract 011 consumption.

Contract 010 therefore preserves continuity without redefining supervision.
