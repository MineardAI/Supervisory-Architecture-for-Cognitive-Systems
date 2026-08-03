# SACS-IMP-001 Source-to-Implementation Matrix

| ID | Contract 001 source | Rust implementation | Test/evidence | Status |
|---|---|---|---|---|
| OBS-001 | Sections 3-4: source-attributed evidence, no supervisory meaning | `SourceDeclaration`, `ObservationRecord`; crate has no later-contract dependencies | source/admission and source audit | implemented |
| OBS-002 | Section 6: registered source and required source bindings | `SourceDeclaration`, `register_source` | registered-source tests | implemented |
| OBS-003 | Sections 7-8, 36: capability/type/schema/version binding | source capability sets and schema/registry checks | unsupported type/version tests | implemented |
| OBS-004 | Sections 11-12: required record fields and distinct timestamps | `ObservationRecord`, `Timestamp` | record construction and timestamp-domain checks | implemented |
| OBS-005 | Sections 13-14: bounded window/cutoff/no reopen | `ObservationWindow`, `close`, `close_as` | lifecycle/cutoff tests | implemented |
| OBS-006 | Section 15: eight availability states | `Availability` enum and explicit absent value | unavailable/absence tests | implemented |
| OBS-007 | Sections 16-17: quality/confidence/uncertainty distinct | `Quality`, optional confidence/uncertainty fields | type inspection and batch tests | implemented |
| OBS-008 | Section 18: deterministic validation/no coercion | `admit` checks source, type, class, schema, registry, timestamps, cutoff, absence | negative admission tests | implemented |
| OBS-009 | Section 19: registered normalization metadata | `NormalizationRecord` | type matrix and profile | represented; transform deferred |
| OBS-010 | Section 20: canonical ordering | six-key stable sort in `build_batch` | insertion-order vector | implemented |
| OBS-011 | Section 21: duplicate/conflict distinction | identical duplicate idempotency; conflicting identity/sequence errors | duplicate/conflict tests | implemented |
| OBS-012 | Section 22: immutable Observation Batch | `ObservationBatch`, batch hash, `ObservationStore` | commit/idempotency tests | implemented |
| OBS-013 | Sections 24-26: late/stale/missing explicitness | cutoff rejection, explicit `Availability`, age field, required-manifest completeness | cutoff/absence/manifest tests | implemented; freshness policy deferred |
| OBS-014 | Section 27: adapter declaration | `NormalizationRecord` and source capability fields | profile | represented; adapter execution deferred |
| OBS-015 | Section 30: provenance completeness shape | `ObservationProvenance` batch records with source/adapter/window/validation/normalization bindings | batch construction | implemented |
| OBS-016 | Section 34: estimator handoff | `ObservationHandoff` over committed batch only | handoff/commit tests | implemented |
| OBS-017 | Sections 35, 39: prohibitions/non-goals | crate dependency and source audits | bounded source audit | implemented |
