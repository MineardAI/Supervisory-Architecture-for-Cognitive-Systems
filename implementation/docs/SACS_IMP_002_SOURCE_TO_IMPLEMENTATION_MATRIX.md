# SACS-IMP-002 Source-to-Implementation Matrix

| ID | Source | Implementation | Evidence | Status |
|---|---|---|---|---|
| EST-001 | Contract 002 §§1-5 | `ReferenceEstimator::estimate` accepts only `ObservationHandoff`; hash and window integrity validated | state-estimation tests | implemented |
| EST-002 | §6 | `EstimatedState` includes cycle, batch, cutoff, profile, basis, components, uncertainty, covariance status, validity, quality, coverage, status, versions, numeric profile, provenance | type/API tests | implemented |
| EST-003 | §§8-10 | `StateBasisProfile`, `EstimatorProfile`, `EstimatorConfiguration` | configuration tests; EST-DEC-002 | implemented |
| EST-004 | §11 | `EstimatorDefinition`, `EstimatorRegistrySnapshot` with explicit versioned resolution | registry/version tests | implemented |
| EST-005 | §§12-14 | explicit missing/stale/conflict treatments; no silent defaults or winners | negative tests/vectors | implemented |
| EST-006 | §§15-16 | tagged uncertainty; source confidence/uncertainty distinct; covariance explicitly `NOT_PRODUCED` | separation test | implemented |
| EST-007 | §17 | lifecycle enum and stateless profile rejecting prior state | prior-state decision/test | conditionally implemented; stateful transitions deferred |
| EST-008 | §18 | `EstimationStatus` and partial/unresolved component handling | state status tests | implemented |
| EST-009 | §19 | exact-rational deterministic reference estimator and canonical identity | repeatability/hash tests | implemented |
| EST-010 | §20 | `EstimatedStateHandoff` preserves state/report/provenance/identity/hash | handoff type | implemented |
| EST-011 | §§21-22 | public API excludes OSV and later authority; explicit failure enum | source/dependency audit | implemented |
| EST-012 | registry/Contract 003 boundary | no OSV fields, mapping, or publication in crate | `Estimated State is not OSV` test | implemented |
| EST-013 | lower source granularity rule | non-normative decisions recorded separately | decision record | implemented |
