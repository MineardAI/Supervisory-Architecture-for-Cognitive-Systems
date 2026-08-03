# SACS-IMP-006 Operational Profiles and Reference States Realization Profile

Status: CONDITIONALLY COMPLETE  
Crate: `sacs-operational-profiles`  
Normative owner: Contract 006  

## Boundary

The crate consumes only committed OSV, Governance Snapshot, and Objective Set handoffs. It implements immutable profile definitions, registry entries/snapshots, scope-only candidate discovery, separate eligibility and compatibility reports, deterministic selection, constructed/validated/committed profiles, deterministic reference-state construction and validation, and a committed Contract 007 handoff.

Configuration is not control. No controller state, control error, command, translation, execution, mode transition, history certification, or external verification is implemented.

## Lifecycle

`ProfileRegistrySnapshot` freezes the registry universe. `CandidateProfileSet` records visible candidates without semantic evaluation. `EligibilityEvaluationReport` classifies mandatory admission. `CompatibilityEvaluationReport` evaluates constructability only for eligible candidates. `ProfileSelectionReport` selects only compatible candidates under an explicit policy and tie-break. Construction remains distinct from commitment. `CommittedOperationalProfile` and `CommittedReferenceState` are immutable cycle-bound artifacts, and `ControllerConfigurationHandoff` exposes only those committed outputs.

## Reference builder

The bounded reference builder uses declared profile constraints as deterministic reference components. It does not infer target state, calculate control error, compute gains, or reinterpret governance/objectives. Production target-vector mathematics and domain-specific builders remain replaceable and deferred.

## Source limitation and closure

Contract 006 defines the lifecycle and authority boundaries but leaves production profile catalogs, domain operating regions, reference-state mathematics, registry persistence, and complete selection scoring profiles under-specified. The implementation uses explicit ranks, authority burden, profile identity tie-breaking, and declared constraint components as a reference posture; these choices are non-normative and replaceable.

