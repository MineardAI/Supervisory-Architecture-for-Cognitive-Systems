# SACS Contract 010 — Modes, Guards, Resets, and Dwell Time

## Section 3 — Authority Boundary

### Purpose

Section 3 defines the exclusive constitutional authority of Contract 010.

It establishes the boundary within which Contract 010 may preserve supervisory continuity, evaluate mode transitions, enforce dwell and hysteresis requirements, authorize bounded reset operations, and commit one authoritative Supervisory Mode.

Contract 010 owns supervisory continuity.

It does not own operational state.

It does not own governance.

It does not own supervisory intent.

It does not own execution.

Its authority begins when the committed constitutional context required for mode evaluation is available.

Its authority ends when exactly one Committed Supervisory Mode has been established for the applicable continuity boundary.

---

## Constitutional Authority

Contract 010 SHALL possess exclusive constitutional authority to:

* define Supervisory Mode identity requirements;
* maintain the authoritative mode vocabulary;
* register immutable Mode Definitions;
* evaluate mode continuation;
* evaluate mode-entry eligibility;
* evaluate mode-exit eligibility;
* evaluate Guard Definitions;
* enforce transition preconditions;
* enforce transition prohibitions;
* enforce transition precedence;
* evaluate hysteresis;
* evaluate minimum and maximum dwell requirements;
* determine whether a mode persists;
* determine whether a mode transition is eligible;
* determine whether a mode transition is prohibited;
* determine whether a Reset is required;
* authorize only declared Reset operations;
* record transition evidence;
* commit exactly one authoritative Supervisory Mode.

These responsibilities collectively constitute the constitutional authority of supervisory continuity.

No other SACS contract may independently establish or alter the authoritative Supervisory Mode.

---

## Constitutional Transformation

Contract 010 owns exactly one constitutional transformation:

```text
Current Committed Supervisory Mode
+
Committed Transition Context
        ↓
Next Committed Supervisory Mode
```

The result MAY be:

* persistence of the current mode;
* transition to one authorized destination mode;
* transition to an explicitly authorized safe mode;
* explicit transition deferral;
* explicit mode-transition failure.

Where the evaluation completes successfully, exactly one Committed Supervisory Mode SHALL govern the next continuity interval and one Committed Supervisory Continuity Record SHALL establish how and why that mode became authoritative.

Contract 010 SHALL NOT produce multiple simultaneously authoritative modes for the same continuity boundary.

---

## Authority Preconditions

Contract 010 SHALL exercise transition authority only when the required constitutional inputs are available and valid.

Required inputs SHALL include, where applicable:

* one current Committed Supervisory Mode;
* one committed Operational State Vector;
* one applicable Governance Snapshot;
* one active committed Operational Profile;
* one committed Mode Definition Registry snapshot;
* applicable Guard Definitions;
* applicable Transition Definitions;
* applicable Transition Precedence Rules;
* applicable Hysteresis Definitions;
* applicable Dwell-Time Definitions;
* applicable Reset Definitions;
* committed temporal context;
* required provenance and integrity references.
* referenced Committed Execution Disposition.

No input becomes owned by Contract 010 merely because Contract 010 consumes it.

Every source artifact SHALL remain authoritative under its originating contract.

---

## Exclusive Mode Authority

Only Contract 010 may determine which Supervisory Mode is authoritative at a continuity boundary.

Accordingly:

* Contract 001 MAY observe conditions relevant to mode evaluation;
* Contract 002 MAY estimate operational state relevant to mode evaluation;
* Contract 003 MAY publish mode-relevant state fields;
* Contract 004 MAY constrain permissible modes and transitions;
* Contract 005 MAY declare objectives affected by mode context;
* Contract 006 MAY define Operational Profiles associated with modes;
* Contract 007 MAY compute commands under the active mode context;
* Contract 008 MAY translate commands;
* Contract 009 MAY realize commands;
* Contract 011 MAY preserve mode history;
* Contract 012 MAY verify mode conformance.

None of those contracts may independently commit the active Supervisory Mode.

---

## Relationship to Observation

Observation provides typed telemetry.

Contract 010 evaluates only committed supervisory context derived through the constitutional pipeline.

Contract 010 SHALL NOT:

* collect raw telemetry;
* open or close observation windows;
* assign source validity;
* alter observation timestamps;
* reinterpret unavailable observations;
* infer mode authority directly from uncommitted runtime signals.

A raw runtime event SHALL NOT trigger a constitutional mode transition.

The event must first enter the authorized observation and state-estimation path and become part of the committed context consumed by Contract 010.

---

## Relationship to State Estimation and the Operational State Vector

State Estimation determines the uncertainty-aware operational condition of the supervised system.

The Operational State Vector publishes that condition as one immutable and temporally atomic state snapshot.

Contract 010 MAY evaluate mode guards against the committed Operational State Vector.

It SHALL NOT:

* recompute estimated state;
* modify uncertainty;
* replace unavailable values;
* reinterpret OSV field semantics;
* combine fields from incompatible cycles;
* construct a substitute OSV;
* mutate the active OSV during transition evaluation.

Operational state informs mode evaluation.

It does not independently establish mode authority.

---

## Relationship to Governance

Governance defines invariant permissions, prohibitions, and eligibility constraints.

Contract 010 SHALL treat the active Governance Snapshot as binding.

Governance MAY:

* prohibit entry into a mode;
* prohibit departure from a mode;
* require transition to a reduced-authority mode;
* constrain available destination modes;
* require specific dwell or reset behavior;
* require stabilization under declared conditions.

Contract 010 SHALL NOT:

* weaken governance;
* reinterpret governance;
* create governance exceptions;
* infer permission from missing governance;
* permit a transition prohibited by governance;
* use mode logic to bypass governance.

A mode transition SHALL be invalid whenever it conflicts with the active Governance Snapshot.

---

## Relationship to Supervisory Objectives

Supervisory Objectives define desired operational conditions.

Contract 010 MAY consume objective references where transition policy explicitly requires them.

It SHALL NOT:

* author objectives;
* select undeclared objectives;
* modify objective priority;
* redefine objective satisfaction;
* treat a mode as a substitute objective.

A mode organizes continuity.

An objective defines what condition supervision seeks to preserve or approach.

These authorities SHALL remain distinct.

---

## Relationship to Operational Profiles

Contract 006 owns Operational Profiles and Reference States.

Contract 010 MAY determine whether the active mode:

* permits the current Operational Profile to persist;
* requires Contract 006 to evaluate a different eligible profile;
* prohibits a profile-mode combination;
* requires a profile transition request;
* constrains the set of profiles eligible for the next cycle.

Contract 010 SHALL NOT:

* construct an Operational Profile;
* modify a committed Operational Profile;
* alter profile gains, limits, permissions, or Reference State parameters;
* directly commit a replacement Operational Profile;
* treat mode commitment as profile commitment.

Where a mode transition requires a profile change, Contract 010 SHALL produce the appropriate immutable continuity requirement or transition reference for Contract 006.

Contract 006 SHALL retain exclusive profile authority.

---

## Relationship to the Controller

The Controller computes bounded corrective action.

Contract 010 MAY provide the active Committed Supervisory Mode as constitutional context to the Controller.

It SHALL NOT:

* compute control error;
* select controller gains;
* produce Control Commands;
* modify Controller Definitions;
* alter controller composition;
* reinterpret controller output;
* suppress a committed Control Command after the Controller has exercised its authority.

The mode may constrain which controller configuration is eligible through declared policy.

The mode itself SHALL NOT perform control.

---

## Relationship to Inference Control

Contract 008 translates committed control decisions into runtime-specific representations.

Contract 010 SHALL NOT:

* translate mode state into undeclared runtime commands;
* modify a Committed Translation Transaction;
* append mode-specific executable instructions after translation;
* reinterpret translation compatibility;
* broaden translated authority.

Any runtime instruction required by a mode transition SHALL originate through the normal constitutional sequence:

```text
Mode Continuity Requirement
        ↓
Operational Profile and Reference State
        ↓
Controller
        ↓
Committed Control Command
        ↓
Inference Control
        ↓
Committed Translation Transaction
```

Contract 010 SHALL NOT bypass this sequence.

---

## Relationship to Execution

Contract 009 owns execution adjudication, planning, runtime application, and execution disposition.

Contract 010 MAY consume committed execution context where a Transition Definition explicitly references it.

Contract 010 SHALL NOT:

* accept or reject a transaction for execution;
* construct an Execution Plan;
* schedule runtime operations;
* apply commands;
* modify an Execution Disposition;
* infer mode transition solely from execution success or failure;
* treat execution outcome as self-authorizing transition evidence.

Execution evidence MAY contribute to the next committed supervisory context.

Only Contract 010 may determine its mode-transition significance.

---

## Relationship to Contract 011 Realization Certification, Supervisory Provenance, and Audit

Downstream historical or provenance authority MAY preserve:

* the previous mode;
* guard evaluations;
* dwell evaluations;
* hysteresis context;
* transition selection;
* reset records;
* mode commitment;
* transition failures.

Historical preservation SHALL NOT modify mode authority.

Contract 010 SHALL NOT:

* certify historical truth;
* construct a full-cycle provenance record;
* rewrite previous mode history;
* erase failed transition attempts;
* replace committed transition evidence.

Mode authority is established by Contract 010.

Historical authority belongs to the downstream historical or provenance contract.

---

## Relationship to External Verification

External verification MAY evaluate whether:

* mode transitions followed declared guards;
* precedence was deterministic;
* dwell requirements were satisfied;
* hysteresis was correctly applied;
* resets were bounded;
* mode commitment was singular and atomic;
* forbidden transitions were prevented.

Contract 010 SHALL NOT declare itself externally verified.

Internal transition evidence SHALL NOT constitute an external verification verdict.

---

## Guard Authority

Contract 010 exclusively owns Guard Evaluation.

A Guard Definition declares a deterministic predicate or bounded evaluation procedure relevant to:

* mode continuation;
* mode entry;
* mode exit;
* transition prohibition;
* emergency transition;
* reset eligibility.

A Guard Evaluation SHALL determine only whether its declared condition is:

* SATISFIED;
* NOT_SATISFIED;
* INDETERMINATE;
* INVALID;
* FAULTED.

Guard Evaluation SHALL NOT:

* commit a mode;
* select among competing transitions unless the guard definition explicitly owns a local branch;
* alter its source inputs;
* infer missing evidence;
* convert uncertainty into satisfaction;
* execute a Reset.

Guard satisfaction creates transition eligibility only.

It does not establish transition authority by itself.

---

## Transition Authority

Contract 010 exclusively owns transition evaluation and selection.

A mode transition SHALL require:

* a valid source mode;
* a valid destination mode;
* an authorized transition edge;
* satisfied mandatory guards;
* absence of applicable prohibitions;
* satisfied dwell requirements;
* satisfied hysteresis requirements;
* valid transition precedence;
* required Reset eligibility;
* valid constitutional context.

Contract 010 SHALL NOT invent a transition edge that is absent from the committed transition model.

Where no transition is eligible, the current mode SHALL persist unless committed policy requires an explicit safe-mode transition or failure.

---

## Transition Precedence Authority

Where multiple transitions are simultaneously eligible, Contract 010 SHALL apply committed Transition Precedence Rules.

Precedence MAY consider only declared constitutional factors, including:

* governance-required transitions;
* safety class;
* emergency classification;
* transition priority;
* source-mode specificity;
* destination-mode restriction level;
* committed tie-breaking rules.

Precedence SHALL NOT depend on:

* order of evaluation;
* thread timing;
* implementation language;
* registry iteration order;
* network arrival order;
* operator preference;
* random selection.

Where the committed precedence model cannot produce one unique result, transition selection SHALL fail explicitly.

---

## Dwell-Time Authority

Contract 010 exclusively owns constitutional Dwell-Time Evaluation.

Dwell Time MAY define:

* minimum residence duration;
* maximum residence duration;
* minimum stable-cycle count;
* maximum transition deferral period;
* emergency dwell exceptions;
* recovery dwell requirements;
* post-reset stabilization intervals.

Dwell-Time Evaluation SHALL consume only committed temporal context.

It SHALL NOT rely upon uncontrolled wall-clock observations when those observations are not constitutionally bound.

Contract 010 SHALL NOT waive a mandatory dwell requirement unless an explicit higher-precedence exception is present in committed policy.

---

## Hysteresis Authority

Contract 010 exclusively owns constitutional hysteresis evaluation for mode transitions.

Hysteresis MAY define:

* distinct entry and exit thresholds;
* persistence bands;
* confirmation-cycle requirements;
* directional transition thresholds;
* recovery margins;
* confidence margins.

Hysteresis SHALL be explicit, versioned, and deterministic.

Contract 010 SHALL NOT introduce hidden hysteresis through cached implementation state or undeclared historical behavior.

---

## Reset Authority

Contract 010 owns the evaluation and authorization of declared Reset operations associated with supervisory continuity.

A Reset Definition SHALL specify:

* reset identity;
* triggering transition;
* eligible source and destination modes;
* affected state scope;
* permitted reset operation;
* prohibited effects;
* preconditions;
* postconditions;
* completion criteria;
* failure behavior;
* evidence requirements.

Contract 010 MAY authorize a Reset only where the Reset Definition permits it.

The component that owns the affected state SHALL perform the authorized Reset through its own contractually defined interface.

Contract 010 SHALL NOT directly mutate state owned by another contract unless that contract explicitly delegates a bounded reset interface.

---

## Reset Prohibitions

A Reset SHALL NOT:

* modify committed historical artifacts;
* alter prior observations;
* rewrite a committed OSV;
* weaken governance;
* erase lineage;
* conceal execution failure;
* fabricate a stable state;
* silently clear uncertainty;
* reset unrelated state;
* exceed its declared scope;
* create additional execution authority.

A Reset is a bounded continuity operation.

It is not a general recovery mechanism.

---

## Mode Persistence Authority

Persistence of the current mode is a valid constitutional determination.

Contract 010 SHALL commit persistence when:

* no authorized transition is eligible;
* all candidate transitions are prohibited;
* mandatory dwell time remains unsatisfied;
* hysteresis conditions require continued residence;
* transition evidence is insufficient;
* committed policy explicitly requires persistence.

Persistence SHALL be represented explicitly.

The absence of transition SHALL NOT be represented merely as missing activity.

---

## Initial Mode Authority

Where no prior Committed Supervisory Mode exists, Contract 010 MAY establish an initial mode only through an explicitly authorized initialization procedure.

Initial-mode selection SHALL use:

* an authorized Initial Mode Policy;
* valid committed startup context;
* applicable governance;
* declared capability requirements;
* deterministic selection rules.

The implementation SHALL NOT select its initial mode through:

* hard-coded convenience defaults;
* arbitrary startup ordering;
* stale prior process memory;
* undeclared operator preference.

If no lawful initial mode can be established, Contract 010 SHALL produce an explicit initialization failure or enter an authorized reduced-authority state.

---

## Authority Termination

Contract 010 authority terminates for a continuity boundary when one of the following has been immutably established:

* one successful Mode Commitment Record;
* one explicit Mode Transition Failure Record;
* one explicit Mode Initialization Failure Record.

After commitment, Contract 010 SHALL NOT modify the resulting artifact.

A later change requires a new continuity evaluation and a new commitment.

---

## Constitutional Prohibitions

Contract 010 SHALL NOT:

* redefine observations;
* estimate operational state;
* mutate the Operational State Vector;
* author or weaken governance;
* create supervisory objectives;
* construct Operational Profiles;
* construct Reference States;
* compute Control Commands;
* translate commands;
* adjudicate execution;
* apply runtime instructions;
* certify history;
* issue verification verdicts;
* maintain two authoritative modes for one continuity boundary;
* transition without explicit guards and policy;
* infer permission from missing evidence;
* invent a transition destination;
* suppress transition uncertainty;
* bypass dwell or hysteresis requirements;
* execute undeclared resets;
* silently remain mode-less.

---

## Authority Invariants

Contract 010 SHALL satisfy the following authority invariants:

1. Contract 010 SHALL possess exclusive authority to commit the active Supervisory Mode.

2. Contract 010 SHALL consume upstream artifacts without assuming their authority.

3. No Guard Evaluation SHALL independently establish a mode transition.

4. No execution result SHALL independently establish a Supervisory Mode.

5. No Operational Profile SHALL independently establish a Supervisory Mode.

6. Every transition SHALL follow one declared transition edge.

7. Every competing transition set SHALL be resolved deterministically or fail explicitly.

8. Every mandatory dwell, hysteresis, and reset requirement SHALL be enforced.

9. Every authorized Reset SHALL remain bounded by its owning Reset Definition.

10. In the absence of a lawful transition, the current mode SHALL persist unless committed policy requires a safer explicit outcome.

11. Every continuity boundary SHALL terminate with exactly one committed mode outcome and one Committed Supervisory Continuity Record, or one explicit continuity failure.

12. Contract 010 SHALL never broaden authority when required transition evidence is unavailable, invalid, or indeterminate.

---

## Constitutional Boundary

The constitutional boundary of Contract 010 is:

```text
Committed Operational Context
        │
        ▼
Current Committed Supervisory Mode
        │
        ▼
Guard Evaluation
        │
        ▼
Dwell and Hysteresis Evaluation
        │
        ▼
Transition Eligibility
        │
        ▼
Transition Precedence
        │
        ▼
Reset Evaluation
        │
        ▼
Mode Commitment
        │
        ▼
Next Committed Supervisory Mode
        â†“
Committed Supervisory Continuity Record
```

Everything above this boundary belongs to observation, estimation, governance, objectives, profiles, control, translation, or execution.

Everything below this boundary belongs to subsequent-cycle supervision, historical preservation, audit, and verification.

Contract 010 owns only the constitutional transformation from one authoritative continuity state to the next and the commitment of the Committed Supervisory Continuity Record that binds that transformation for deterministic replay and Contract 011 consumption.

---

## Fundamental Authority Rule

> **Contract 010 SHALL consume one authoritative current Supervisory Mode, its required committed transition context, and the referenced Committed Execution Disposition, and SHALL establish exactly one authoritative mode outcome and one Committed Supervisory Continuity Record for the applicable continuity boundary. Between those artifacts, it SHALL exercise exclusive constitutional authority over guard evaluation, transition eligibility, transition precedence, dwell time, hysteresis, reset authorization, mode persistence, and mode commitment without redefining operational state, governance, supervisory intent, execution, or historical fact.**

Section 3 establishes the exclusive constitutional authority of Contract 010 and prevents mode continuity from becoming a hidden capability distributed across observation, profile selection, control, execution, or implementation state.

Supervisory continuity is therefore explicit, bounded, deterministic, and committed.
