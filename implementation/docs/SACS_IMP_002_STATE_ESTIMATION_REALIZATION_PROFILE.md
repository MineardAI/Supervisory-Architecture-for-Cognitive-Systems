# SACS-IMP-002 State Estimation Realization Profile

Status: `SACS-IMP-002: CONDITIONALLY COMPLETE`  
Contract: 002 State Estimation  
Canonical success artifact: `Estimated State`  
Canonical consumer: Contract 003 OSV Publisher

## Authority and boundary

The `sacs-state-estimation` crate consumes only the committed Contract 001 `ObservationHandoff`. It validates the batch integrity hash, binds an explicit estimator definition, profile, state-basis profile, registry snapshot, numeric profile, and configuration, then produces `Estimated State`, `Estimator Report`, `Estimator Provenance`, and `EstimatedStateHandoff`.

It does not mutate observations, publish an OSV, apply governance, select objectives/profiles, control, execute, certify history, or verify externally.

## Estimator posture

The included `ReferenceEstimator` is stateless and non-normative. It computes an exact rational mean over explicitly configured numeric observation values. This is a replaceable reference implementation selected only to prove deterministic interfaces; it does not define constitutional estimator mathematics.

## Input admission

The estimator rejects altered batch content, mismatched handoff hashes, faulted batches, incompatible batch versions, missing estimator definitions, estimator/version mismatches, numeric-profile mismatches, and forbidden prior state. Raw records are not accepted by the estimator API.

## Data treatment

- Missing required input: explicit `MissingRequiredObservation` failure.
- Optional/missing input: explicit unresolved treatment.
- Stale input: rejected by the active reference profile; acceptance is a profile option but not selected here.
- Invalid, unavailable, unsupported, redacted, pending, and not-applicable inputs: explicit treatment; required non-usable input fails.
- Conflicts: preserved and fail under the active profile; no implicit winner.
- Source confidence and source uncertainty are retained separately from estimator confidence and estimated uncertainty.

## Uncertainty and state

The reference profile uses `Unknown` estimated uncertainty and explicitly marks covariance `NOT_PRODUCED`; it does not interpret absent covariance as independence. `EstimatorStateLifecycle` is represented, but the selected stateless profile forbids prior estimator state. Stateful lifecycle behavior remains deferred.

## Handoff and persistence

`EstimatedStateHandoff` contains only immutable Contract 002 output, report, provenance, commitment identity, and state hash. `StateEstimationStore` uses IMP-000’s in-memory commitment mechanism for immutable identity lookup, idempotent retry, and conflict rejection. It does not implement OSV mapping or publication.

## Source limitations and deferrals

Contract 002’s source does not specify production estimator mathematics, state dimensions, covariance calculation, freshness thresholds, source-priority semantics, or stateful estimator transitions in sufficient detail. These are documented in the non-normative decision record and remain replaceable/deferred.
