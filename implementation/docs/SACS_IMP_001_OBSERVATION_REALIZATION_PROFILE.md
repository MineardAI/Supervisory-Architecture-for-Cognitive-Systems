# SACS-IMP-001 Observation Realization Profile

Status: implementation complete for the admitted Contract 001 scope  
Contract: 001 Observation  
Canonical success artifact: `Observation Batch`  
Canonical consumer: Contract 002 State Estimation

## Constitutional source set

This realization derives from the adopted SACS v1.0.0 baseline, Contract 000, Contract 001, the adopted Source Pack registries/invariants/architecture, the Phase 2 closure, and the SACS implementation review and IMP-000 baseline records. Archived material was not used as active authority.

## Authority owned

The crate owns source declaration admission, source capability binding, observation schema/version binding, observation-window lifecycle, timestamp/cutoff validation, availability and quality representation, deterministic ordering, evidence-preserving normalization metadata, provenance references, immutable batch construction, and the Contract 002 handoff shape.

It does not assign supervisory meaning.

## Implemented types

`SourceDeclaration`, `ObservationClass`, `ObservationValue`, `Availability`, `Quality`, `Timestamp`, `ValidityInterval`, `NormalizationRecord`, `ObservationRecord`, `ObservationWindow`, `ClosedObservationWindow`, `ObservationBatch`, `ObservationHandoff`, and typed `ObservationError`.

## Window lifecycle

The implementation supports `OPEN` windows and deterministic finalization as `CLOSED`, `FAULTED`, or `INCOMPLETE`. Admission is rejected after finalization. Receipt timestamps after the cutoff are rejected, deferred/recorded as late-data errors, or fault the window according to the declared late-data policy. No reopen operation exists.

## Normalization boundary

The implementation records adapter, adapter-version, input/output schema versions, and normalization-rule identity. It does not transform values, infer semantics, fill missing fields, change units, or create derived supervisory state. Such transformations require a registered Contract 001 adapter decision in a later bounded implementation pass.

## Commitment boundary

`ObservationStore` uses the IMP-000 commitment mechanism for stable identity, immutable in-memory batch references, append-only commitment events, identical retry idempotency, and conflicting retry rejection. It does not decide admission policy. No database, network service, broker, mutable `latest` pointer, or wall-clock decision input was introduced.

## Contract 002 handoff

`ObservationHandoff` exposes only a committed `ObservationBatch`, its stable commitment identity, and batch hash. Open windows, raw unadmitted records, and mutable records do not cross this API. No estimator behavior or `Estimated State` type is implemented.

## Explicit failures

Typed failures cover unknown/duplicate sources, unsupported types/classes, version and clock-domain mismatch, unavailable source, closed/faulted windows, cutoff/late data, missing available values, conflicting observation identity, source-sequence conflict, and commitment conflict. Failures are represented by `ObservationError` and deterministic IMP-000 `FailureRecord`, never null/default success.

## Known exclusions

Actual adapter transformations, source-priority conflict resolution, replay-source execution, security/authentication transport, persistence beyond in-memory references, Contract 002 estimation, and all Contracts 003-012 remain excluded. Expected-manifest identity and required-observation completeness are implemented; freshness/quality/confidence policy evaluation remains deferred to a registered policy implementation.
