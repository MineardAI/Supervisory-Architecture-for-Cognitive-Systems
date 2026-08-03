# SACS Constitutional Architecture

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012  
Classification: Constitutional-supporting architecture summary

## Foundational Identity

The architecture summary is frozen as a v1.0.0 constitutional-supporting document. Each contract owns one constitutional authority category. Authority may narrow downstream but may not broaden downstream.

SACS is a deterministic supervisory architecture. Its defining constitutional question is not "what content should be produced?" but "what supervisory condition is authorized, bounded, translated, executed, and historically certified?" SACS operates on committed artifacts, not live mutable guesses. It treats supervision as a sequence of explicit constitutional transforms.

## Separation Of Authority

| Layer | Constitutional question | Owns | Consumes | Produces | Must never do |
| --- | --- | --- | --- | --- | --- |
| Observation | What runtime evidence is admissible? | Source attribution, timing, normalization, provenance of measured evidence | Runtime evidence and source declarations | Observation records, availability declarations, handoff package | Infer supervisory meaning or decide action |
| State Estimation | What does the evidence imply? | Estimation, uncertainty, state reconstruction | Committed observations | Estimated state outputs and committed OSV inputs | Mutate observation history or govern |
| Operational State Vector | What state is canonically published? | Temporally atomic supervisory state publication | Estimated state, observation lineage | Committed OSV | Merge incompatible cycles or add authority |
| Governance | What external constraints apply? | Dispositions, restrictions, permissions, revocations | External governance source and committed state | Committed governance snapshot | Select objectives or execute commands |
| Supervisory Objectives | What should the system preserve or approach? | Objective registration, prioritization, applicability | Committed OSV and governance snapshot | Committed objective set | Broaden permission or override governance |
| Operational Profiles and Reference States | What configuration should supervise this cycle? | Profile registry, eligibility, compatibility, selection, reference construction | OSV, governance, objectives, registry snapshots | Profile selection report, committed operational profile, committed reference state | Compute control or execute runtime commands |
| Controller | What bounded correction follows? | Control error, controller state, authority binding, command authorization | OSV, governance, objectives, profile, reference state, controller config | Controller commitment record, committed control command | Select objectives, choose profile, or execute |
| Inference Control | How is the command represented for a runtime? | Semantic-preserving translation | Committed control command and committed translation context | Committed inference command set, translation transaction, handoff envelope | Reconsider control authority or broaden semantics |
| Execution Interface | How is the runtime handed off the command? | Execution disposition boundary | Handed-off inference artifacts | Execution plan, execution outcome, committed execution disposition | Rewrite supervisory meaning or pretend success |
| Modes, Guards, Resets, Hysteresis, and Dwell Time | What supervisory continuity is authorized across cycles? | Supervisory mode continuity, transition precedence, guard evaluation, reset handling, hysteresis, and dwell-time control | Committed OSV, governance snapshot, objective set, committed execution disposition references | Committed Supervisory Mode and Committed Supervisory Continuity Record | Perform execution, observation, historical certification, or provenance assembly |
| Realization Certification, Supervisory Provenance, and Audit | What realization became authoritative history and cycle lineage? | Realization certification, lineage, cycle provenance, audit, and replay | Committed execution disposition, Committed Supervisory Continuity Record, and supporting evidence | Committed Realization Record and Committed Supervisory Cycle Record | Perform execution, observation, governance, or recertify execution evidence |
| External Verification Profile and Evidence Interface | How may committed SACS claims and evidence cross into an independent verification authority without merging sovereignty? | Verification profile, claim, evidence, submission, custody, result admission, result certification, exchange history, replay, and audit | Committed Supervisory Cycle Record and committed verification definitions and evidence | Committed Verification Exchange Record or Committed Verification Exchange Failure | Perform self-verification, merge authorities, reconstruct undisclosed verifier reasoning, or alter external judgment |
| Observation of Consequences | What consequences were observed after execution? | New runtime evidence | Runtime results and external evidence | New observations for the next cycle | Rewrite historical record |

Contract 011 internal stages:

```text
Committed Execution Disposition + Committed Supervisory Continuity Record
    v
Stage One: Realization Certification
    v
Constructed Realization Record
    v
Realization Commitment
    v
Committed Realization Record
    v
Stage Two: Supervisory Provenance
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
```

Stage Two consumes only the Committed Realization Record. Constructed or intermediate realization artifacts are not cross-stage interfaces.

After the terminal Stage Two outcome, deterministic replay produces a Replay Verification Result and audit produces an Audit Representation. Replay is verification-only and audit is non-constitutive; neither may mutate, replace, invalidate, or reinterpret the committed terminal outcome.

Contract 012 handoff and exchange:

```text
Committed Supervisory Cycle Record
    v
Contract 012 Verification Profile and Evidence Interface
    v
Committed Verification Submission
    v
External Verification Boundary / Constitutional Custody
    v
External Verification Result
    v
Committed Verification Result
    v
Committed Verification Exchange Record
or
Committed Verification Exchange Failure
    v
Verification Exchange Replay Result / Verification Exchange Audit Representation
```

Verification responsibility may cross the external-verification boundary. Constitutional ownership, sovereignty, historical authority, and commitment authority do not transfer across it.

## Frozen Interfaces And Handoffs

The following interfaces are frozen for SACS v1.0.0:

```text
001 -> 002 -> 003 -> 004 -> 005 -> 006 -> 007 -> 008 -> 009
009 -> 010 continuity evaluation
009 -> 011 Stage One realization certification
010 -> 011 Stage One realization certification
011 Stage One -> 011 Stage Two
011 -> 012 external verification exchange
```

Contract 009's Committed Execution Disposition is an authorized multi-consumer handoff to Contract 010 continuity evaluation and Contract 011 Stage One realization certification. This is not duplicate ownership. Contract 011 Stage Two consumes only the Committed Realization Record as its mandatory cross-stage input; constructed or intermediate realization artifacts are not cross-stage interfaces.

## Canonical Artifact Chain

The canonical chain distinguishes reusable definitions from cycle-bound commitments.

```text
Definition / Registry / Rule Set
    ↓
Registry entry or registry snapshot
    ↓
Cycle-bound configuration or candidate artifact
    ↓
Deterministic evaluation
    ↓
Proposed or constructed artifact
    ↓
Validation
    ↓
Authority certification, where applicable
    ↓
Atomic commitment
    ↓
Canonical committed artifact
    ↓
Historical record or failure artifact
```

Artifact classes used across the corpus:

- reusable definitions: definition, profile definition, translation definition, controller definition, rule set;
- registry entries: immutable publication records that bind a definition to a registry context;
- registry snapshots: immutable views used for cycle-local discovery and selection;
- configurations: committed, cycle-specific settings used by the next layer;
- candidate artifacts: visible but not yet selected;
- proposed artifacts: evaluation outputs that still require validation;
- constructed artifacts: deterministic outputs assembled from valid inputs;
- validated artifacts: outputs that passed contract-defined checks;
- certified artifacts: artifacts carrying the contract's authority to finalize a cycle;
- committed artifacts: canonical, immutable outputs of the cycle;
- failure artifacts: explicit terminal outputs representing unresolved or rejected conditions;
- historical artifacts: records of realized execution or supervision history.

## Constitutional Transformation Pattern

Most mature SACS contracts follow the same constitutional pattern:

```text
Definition
    ↓
Cycle-bound configuration
    ↓
Deterministic evaluation
    ↓
Proposed or constructed result
    ↓
Validation
    ↓
Authority certification, where applicable
    ↓
Atomic commitment
    ↓
Canonical output
```

Not every contract uses every stage. The architecture is still strict about the distinction between them. Construction is not commitment. Validation is not certification. Certification is not execution. Execution is not history.

## Authority Monotonicity

Downstream layers may preserve authority or reduce it. They may not broaden authority. If a downstream artifact seems more permissive than its upstream source, the downstream artifact is nonconforming unless a later constitutional contract explicitly and narrowly grants that authority.

## Canonical Failure Doctrine

Failure is not an implementation defect by definition. It is a valid constitutional output when a contract cannot establish the conditions required for a canonical success artifact. Failure must be explicit, terminal where required, and named in the contract's artifact vocabulary. Approximation, silent repair, implicit defaults, and partial canonicalization are prohibited substitutes for failure.

## External Verification Boundary

SACS internal validation and certification are not the same as external verification.

```text
SACS internal validation or certification
≠
NullBlade external verification
```

SACS SHALL NOT declare itself externally verified solely because a canonical artifact passed internal checks. External verification remains an external authority boundary.

## Source Traceability

- `SACS Contract 000 Architecture_and_Authority v0.1.0.md` - system identity, flow, authority boundaries, and conformance rules.
- `SACS Contract 001 Observation_v0.1.0.md` - observation entry point and runtime evidence boundary.
- `SACS Contract 006 Section 1 Op Profiles-Reference States.md` - profile, reference-state, and controller handoff pattern.
- `SACS Contract 006 Section 3 Operational Profile Registry.md` - registry entries, snapshots, and candidate discovery.
- `SACS Contract 006 Section 4 Compatibility Evaluation.md` - compatibility evaluation and failure vocabulary.
- `SACS Contract 006 Section 5 Deterministic Profile Selection.md` - deterministic selection, tie-breaking, and failure outputs.
- `SACS Contract 006 Section 6 Committed Ops Profile Construction.md` - commitment, immutability, and authority reduction certificate.
- `SACS Contract 007 Controller.txt` - controller commitment and bounded correction.
- `SACS Contract 008 Inference Control.txt` - semantic-preserving translation.
- `SACS Contract 008 Section 19 Translation Transaction Commitment.txt` - commitment, handoff, and external verification boundary.
- `SACS Contract 010 Section 1 Modes Guards Resets Dwell Time.md` through `SACS Contract 010 Section 5 Modes Guards Resets Dwell Time.md` - supervisory continuity and mode commitment.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - realization record, provenance, and cycle record.
