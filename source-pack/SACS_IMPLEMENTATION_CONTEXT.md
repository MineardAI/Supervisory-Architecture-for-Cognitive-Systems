# SACS Implementation Context

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012  
Classification: Informative and subordinate implementation guidance

## Implementation Principles

This implementation context is informative. The constitutional contracts remain normative.

- content-addressable immutable artifacts;
- exact schema and version binding;
- committed registry snapshots;
- deterministic serialization;
- canonical hashing;
- explicit cycle identity;
- no mutable current pointers in canonical reasoning;
- transactional commitment;
- idempotent handoff;
- explicit failure artifacts;
- replay from committed inputs;
- provenance closure;
- external verification interfaces.

## Suggested Module Boundaries

- **Observation module:** source registration, normalization, timing, and handoff packaging.
- **State estimation module:** estimator logic, uncertainty handling, and estimated-state output.
- **OSV module:** temporal atomic publication and lineage closure.
- **Governance module:** external constraint binding and revocation handling.
- **Objective module:** objective evaluation and priority handling.
- **Profile module:** registry, eligibility, compatibility, selection, and reference-state construction.
- **Controller module:** control error, controller state, composition evaluation, and command commitment.
- **Translation module:** runtime capability binding, translation validation, transaction construction, and handoff.
- **Execution interface module:** runtime disposition and execution outcome capture.
- **Contract 010 supervisory-continuity module:** mode evaluation, guard predicates, transition precedence, resets, hysteresis, dwell-time enforcement, Committed Supervisory Mode commitment, and Committed Supervisory Continuity Record commitment.
- **Contract 011 constitutional-history module:** realization certification, historical finality, provenance, lineage, audit, replay, and supervisory-cycle record commitment.
- **Contract 012 external-verification module:** verification profiles, claims, evidence, submission construction and commitment, custody events, external-result admission, result certification, exchange-history commitment, replay, and audit.

Contract 011 implementation constraints:

- Stage One accepts only the committed Contract 009 and Contract 010 artifacts.
- Stage Two accepts only the Committed Realization Record as its mandatory cross-stage input.
- Stage One Realization Commitment and Stage Two Supervisory Cycle Commitment are independent, atomic, and idempotent commitment boundaries.
- Constructed or intermediate realization artifacts are not cross-stage interfaces.
- Missing evidence is never reconstructed; unresolved lineage fails explicitly.
- Artifact membership, relationship resolution, lineage construction, completeness evaluation, commitment, and committed artifact identity remain separate operations.
- Stage Two terminates exclusively in exactly one Committed Supervisory Cycle Record or one Committed Supervisory Provenance Failure; no partial cycle record is canonical.
- Supervisory Cycle Commitment consumes the Constructed Supervisory Cycle Record and does not re-adjudicate execution or continuity.
- The Committed Supervisory Cycle Record is the sole authoritative whole-cycle history; the Committed Supervisory Provenance Failure is its exclusive terminal counterpart.
- Contract 011 does not perform external verification.
- Deterministic replay consumes committed history and does not recreate constitutional authority.
- Replay produces a Replay Verification Result only; replay failure does not invalidate the committed terminal artifact.
- Audit produces an Audit Representation only; audit findings are non-constitutive and do not enact governance or modify history.

Contract 012 implementation guidance (non-normative):

- Use one stable Verification Exchange Identity for every exchange and bind it to every custody, submission, result, admission, certification, and history artifact.
- Record immutable custody-transfer events and preserve separate SACS and external-verifier lineage domains.
- Snapshot verifier credentials, profiles, requirements, methods, versions, and cryptographic integrity bindings.
- Correlate submitted claims, evidence, external results, admission, certification, and exchange commitment idempotently.
- Commit the exchange record atomically and enforce exactly one committed exchange record or committed exchange failure.
- Preserve the external-result payload and judgment exactly; do not classify an unfavorable judgment as exchange failure.
- Enforce the replay boundary: replay may reproduce only SACS-constructed, committed, transmitted, received, admitted, certified, and historically recorded material.
- Use append-only audit representations; replay and audit cannot modify or recommit exchange history.
- Do not infer institutional learning, constitutional evolution, or a successor SACS contract from Contract 012.

## Suggested Data Categories

- definitions;
- registry state;
- configurations;
- cycle artifacts;
- evidence;
- commitments;
- failures;
- historical records;
- verification results.

## Practical Design Notes

- Keep definitions immutable and separately addressable.
- Treat committed artifacts as the only authoritative inputs to downstream modules.
- Preserve exact identifier, version, and hash bindings in storage and transport.
- Use explicit failure records rather than null-heavy success objects.
- Avoid any implementation shortcut that relies on "current" or "latest" without a committed snapshot.
- Serialize deterministically so replay and comparison remain cheap and exact.
- Make handoff boundaries explicit in code and in data contracts.

## Non-Normative Warning

This implementation context is informative. The constitutional contracts remain normative.

## Source Traceability

- `SACS Contract 000 Architecture_and_Authority v0.1.0.md` - canonical flow, conformance, and implementation prohibitions.
- `SACS Contract 001 Observation_v0.1.0.md` - source and timing semantics for ingestion.
- `SACS Contract 003 Operational State Vector.txt` - temporal atomicity and publication isolation.
- `SACS Contract 004 Governance.txt` - governance snapshot semantics.
- `SACS Contract 006 Section 2 Operational Profile Registry.md`, `SACS Contract 006 Section 3 Operational Profile Registry.md`, `SACS Contract 006 Section 4 Compatibility Evaluation.md`, `SACS Contract 006 Section 5 Deterministic Profile Selection.md`, `SACS Contract 006 Section 6 Committed Ops Profile Construction.md` - profile lifecycle implementation model.
- `SACS Contract 007 Controller.txt` - controller implementation boundary.
- `SACS Contract 008 Inference Control.txt` and section files - translation and handoff implementation model.
- `SACS Contract 010 Section 1 Modes Guards Resets Dwell Time.md` through `SACS Contract 010 Section 5 Modes Guards Resets Dwell Time.md` - supervisory-continuity implementation boundary.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - constitutional-history, provenance, and audit implementation boundary.
- `SACS Contract 012 External Verification Profile and Evidence Interface.txt` and the numbered Contract 012 section files - bounded external-verification exchange boundary; external verifier sovereignty remains outside SACS.
