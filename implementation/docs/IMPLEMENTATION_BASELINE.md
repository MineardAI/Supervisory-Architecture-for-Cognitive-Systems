# SACS-IMP-000 Implementation Baseline

This implementation baseline is subordinate to the SACS Constitutional Baseline v1.0.0 and the normative Contracts 000-012. It records only the first realization layer.

## Included

- deterministic version and conformance declarations;
- canonical field serialization and SHA-256 hashing;
- deterministic artifact identity;
- lifecycle-state types;
- authority-neutral commitment mechanism with idempotency/conflict detection;
- lineage relationship primitives;
- explicit immutable failure records;
- tests and traceability evidence.

SACS-IMP-001 adds the bounded Contract 001 Observation realization in `crates/observation`, SACS-IMP-002 adds bounded Contract 002 State Estimation in `crates/state-estimation`, SACS-IMP-003 adds bounded Contract 003 OSV publication in `crates/operational-state-vector`, SACS-IMP-004 adds bounded Contract 004 Governance in `crates/governance`, SACS-IMP-005 adds bounded Contract 005 Supervisory Objectives in `crates/supervisory-objectives`, SACS-IMP-006 adds bounded Contract 006 Operational Profiles and Reference States in `crates/operational-profiles`, SACS-IMP-007 adds bounded Contract 007 Controller realization in `crates/controller`, SACS-IMP-008 adds bounded Contract 008 Inference Control translation in `crates/inference-control`, SACS-IMP-009 adds bounded Contract 009 Execution Interface disposition commitment in `crates/execution-interface`, SACS-IMP-010 adds bounded Contract 010 Supervisory Continuity mode commitment in `crates/supervisory-continuity`, SACS-IMP-011 adds bounded Contract 011 Realization Certification and Supervisory Provenance in `crates/constitutional-history`, and SACS-IMP-012 adds bounded Contract 012 External Verification Interface exchange handling in `crates/external-verification-interface`, without transferring external verifier sovereignty or implementing successor contracts.

## Not included

No Contract 007+ policy or runtime logic is included. State Estimation does not implement production estimator mathematics; the OSV crate does not invent universal field semantics, projection mathematics, or covariance computation; Governance does not author policy, select objectives/profiles, control, execute, or verify externally; Objectives do not select profiles, construct reference states, control, or execute; Operational Profiles do not compute control or execute.

## Baseline rule

The implementation may be reused by later contracts only through contract-owned adapters and policy. Its mechanisms do not decide whether a contract operation is authorized or whether a canonical artifact should be committed.
