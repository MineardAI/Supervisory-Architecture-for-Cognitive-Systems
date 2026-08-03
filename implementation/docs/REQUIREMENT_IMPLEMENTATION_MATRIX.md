# Requirement Implementation Matrix

| ID | Source | Requirement | Implementation | Evidence | Status |
|---|---|---|---|---|---|
| IMP-000-01 | Contract 000; review | Declare implementation/baseline/contract/registry/numeric versions and exclusions | `constitutional-conformance::CapabilityProfile` | profile tests; `IMPLEMENTATION_PROFILE.md` | implemented |
| IMP-000-02 | Contract 000; review | Reject unsupported versions without downgrade | `CapabilityProfile::validate`, `VersionBinding::validate` | mismatch tests | implemented |
| IMP-000-03 | review; invariants | Deterministic immutable identity without random UUIDs | `canonical-identity::ArtifactIdentity` | identity vector | implemented |
| IMP-000-04 | review; invariants | Stable field ordering and round-trip encoding | `canonical-serialization::CanonicalDocument` | serialization tests | implemented |
| IMP-000-05 | review; invariants | Stable content and identity hashing | `canonical-hashing` | SHA-256 vectors | implemented |
| IMP-000-06 | review; invariants | Explicit deterministic ordering | `deterministic-ordering` | sort test | implemented |
| IMP-000-07 | review; architecture | Preserve constructed/validated/certified/committed distinctions | `canonical-artifacts::Lifecycle` | lifecycle regression test | implemented |
| IMP-000-08 | review; invariants | Atomic, immutable, idempotent commitment with conflict detection | `commitment-engine::CommitmentStore` | commitment tests | implemented |
| IMP-000-09 | review; architecture | Reusable parent/child/cycle/correlation/provenance relationships | `lineage-core::LineageEdge` | lineage test | implemented |
| IMP-000-10 | review; invariants | Typed deterministic failures, never null failure | `failure-model::FailureRecord` | failure vector | implemented |
| IMP-000-11 | attachment | Every crate has unit, round-trip/determinism/negative coverage appropriate to its API | crate-local tests | cargo test | implemented |
| IMP-000-12 | attachment | No contract policy or runtime semantics | workspace scope and exclusions | scope audit | implemented |
| IMP-001-01 | Contract 001 Sections 3-6 | Registered, source-attributed evidence without supervisory meaning | `sacs-observation::SourceDeclaration`, `ObservationRecord` | EV-OBS-0001 | implemented |
| IMP-001-02 | Contract 001 Sections 13-15 | Window lifecycle, cutoff, and explicit availability | `ObservationWindow`, `Availability` | observation tests, vector index | implemented |
| IMP-001-03 | Contract 001 Sections 18-22 | Validation, normalization metadata, ordering, immutable batch | `admit`, `NormalizationRecord`, `ObservationBatch` | observation tests | implemented |
| IMP-001-04 | Contract 001 Sections 30, 34-37 | Provenance and committed estimator handoff boundary | `ObservationHandoff`, identity/commitment reuse | handoff tests, EV-OBS-0001 | implemented |
| IMP-002-01 | Contract 002 Sections 1-5 | Accept only committed ObservationHandoff and bind explicit estimator context | `EstimatorInputBinding`, `ReferenceEstimator` | EV-EST-0001 | implemented |
| IMP-002-02 | Contract 002 Sections 6-11 | Explicit Estimated State, basis/profile/configuration, registry, and versions | state-estimation public types | EST-001..004 | implemented |
| IMP-002-03 | Contract 002 Sections 12-16 | Explicit missing/stale/conflict and confidence/uncertainty treatment | treatment, uncertainty, covariance types | EST-005..006 | implemented |
| IMP-002-04 | Contract 002 Sections 18-20 | Deterministic immutable Estimated State and Contract 003 handoff | `EstimatedState`, `EstimatedStateHandoff` | EST-008..010 | implemented |
| IMP-002-05 | Contract 002 Sections 21-22 | Explicit failure and authority prohibition surface | `EstimationFailure`, source audit | EST-011..012 | implemented |
| IMP-003-01 | Contract 003 Sections 1-5 | Immutable EstimatedStateHandoff admission and cycle/cutoff binding | `OsvPublisher`, `validate_handoff` | OVT-001, OVT-009 | implemented |
| IMP-003-02 | Contract 003 Sections 7-11 | Explicit field registry, schema, publication, and projection profiles | OSV registry/profile types | OVT-003, OVT-004, OVT-013 | implemented, bounded |
| IMP-003-03 | Contract 003 Sections 12-17 | Preserve validity, quality, uncertainty, and atomic immutable publication | draft, validation, committed OSV | OVT-005..011 | implemented |
| IMP-003-04 | Contract 003 Sections 18-21 | Cycle uniqueness, committed-only handoff, explicit failures | `OsvStore`, handoff, failure | OVT-012, OVT-014, OVT-015 | implemented |
| IMP-003-05 | Contract 003 Sections 9, 11 | Do not invent Core8 semantics or production projection mathematics | decision record | EV-OSV-0001 | deferred by source |
| IMP-004-01 | Contract 004 Sections 3-8, 11 | Registered external governance and committed-OSV-only admission | `GovernanceEngine`, source/authority types | GT-001..003, GT-007 | implemented |
| IMP-004-02 | Contract 004 Sections 9-15 | Distinct rule classes, applicability, permissions, prohibitions, obligations, and statuses | governance rule/snapshot types | GT-004..010, GT-013 | implemented, bounded |
| IMP-004-03 | Contract 004 Sections 16-19, 25 | Deterministic conflict/precedence/effective-time evaluation without permissive fallback | `resolve_conflicts`, admission context | GT-008..012 | implemented, reference profile |
| IMP-004-04 | Contract 004 Sections 26-35 | Append-only revocation, immutable snapshot, provenance, reduced-authority failure | revocation/store/failure types | GT-010, GT-020..022 | implemented |
| IMP-004-05 | Contract 004 Sections 36, 39 | Defer production policy language, trust federation, policy engines, and later contracts | decision record | EV-GOV-0001 | deferred by source |
| IMP-005-01 | Contract 005 Sections 5, 11, 44 | Consume only committed OSV and Governance Snapshot handoffs | `validate_inputs`, evaluator signature | OJT-002, OJT-003, OJT-017 | implemented |
| IMP-005-02 | Contract 005 Sections 6-10 | Explicit objective source, registry, declarative class, target, and dependency bindings | source/definition/registry types | OJT-001, OJT-021 | implemented |
| IMP-005-03 | Contract 005 Sections 12-22 | Distinct applicability, eligibility, status, governance binding, priority, dependency, and conflict evaluation | evaluator and result types | OJT-004..012 | implemented, bounded |
| IMP-005-04 | Contract 005 Sections 28-42 | Immutable candidate/validation/Objective Set, provenance, atomic commitment, reduced authority | committed set/store/failure | OJT-015, OJT-016, OJT-020 | implemented |
| IMP-005-05 | Contract 005 Sections 33, 48 | No profile selection, reference state, control, or universal objective mathematics | decision record | EV-OBJ-0001 | deferred by source |
| IMP-006-01 | Contract 006 Sections 1-3 | Committed upstream handoff admission, immutable profile definitions, registry entries/snapshots | `ProfileRegistrySnapshot`, evaluator admission | OPR-001..004 | implemented |
| IMP-006-02 | Contract 006 Section 3 | Scope-only candidate discovery with complete deterministic candidate accounting | `ProfileEvaluator::discover`, `CandidateProfileSet` | OPR-005 | implemented |
| IMP-006-03 | Contract 006 Sections 3-4 | Separate eligibility and compatibility reports with explicit outcomes | eligibility/compatibility types | OPR-006..010 | implemented |
| IMP-006-04 | Contract 006 Section 5 | Deterministic selection, tie-breaking, no silent default | selection policy/report | OPR-011..013 | implemented, reference policy |
| IMP-006-05 | Contract 006 Section 6 | Constructed/validated/committed Operational Profile and Reference State | profile/reference types and store | OPR-014..021 | implemented, bounded |
| IMP-006-06 | Contract 006 Sections 1, 6 | Controller-only committed configuration handoff; no control logic | `ControllerConfigurationHandoff` | OPR-022..024 | implemented |
| IMP-007-01 | Contract 007 Controller, Sections 014/04 | committed upstream admission, definitions, configuration, and explicit state | `UpstreamContext`, `ControllerDefinition`, `CommittedControllerConfiguration`, state types | CTRL-001..006 | implemented, bounded |
| IMP-007-02 | Contract 007 Section 05 | deterministic separately committed Control Error | `CommittedControlError`, `control_error` | CTRL-007..008 | implemented, reference metric |
| IMP-007-03 | Contract 007 Sections 017/06 | authority-monotonic constraints and Action Space | `CommittedControlConstraintSet`, `bind_constraints` | CTRL-009..010 | implemented, bounded |
| IMP-007-04 | Contract 007 Section 019 | validated composition and deterministic evaluation | composition/evaluation types and methods | CTRL-011..014 | implemented, reference controller |
| IMP-007-05 | Contract 007 Controller validation/certification | separate command/state validation and authority evidence | validation/certificate types | CTRL-015..020 | implemented |
| IMP-007-06 | Contract 007 Section 022 | atomic commitment and downstream command-only handoff | bundle/store/handoff types | CTRL-021..030 | implemented, bounded |
| IMP-008-01 | Contract 008 Sections 6-12 | committed command admission, capability, profile, definitions, and closed configuration | `TranslationEngine::admit`, capability/configuration types | TRANS-001..006 | implemented, bounded |
| IMP-008-02 | Contract 008 Sections 13-17 | requirement preservation, binding, normalization, numeric transformation, ordering, validation | requirement/binding/translation/report types | TRANS-007..018 | implemented, reference translator |
| IMP-008-03 | Contract 008 Sections 18-19 | command-set, transaction, commitment, and Contract 009 envelope | transaction/record/store/envelope types | TRANS-019..030 | implemented, bounded |
| IMP-009-01 | Contract 009 Sections 6-8 | strict handoff admission, eligibility, and explicit disposition vocabulary | admission/report/disposition types | EXEC-001..005 | implemented |
| IMP-009-02 | Contract 009 Sections 9-11 | bounded plan, adapter boundary, outcome capture, and validation | plan/request/adapter/outcome types | EXEC-006..009 | implemented, fixture-only |
| IMP-009-03 | Contract 009 Sections 12-15 | atomic disposition commitment, persistence, replay, and downstream handoff | commitment/disposition/store/handoff types | EXEC-010..013 | implemented, bounded |
| IMP-010-01 | Contract 010 Sections 1-5 | continuity admission, mode identity/registry, guards, eligibility, precedence | context/mode/guard/transition types | CONT-001..010 | implemented, bounded |
| IMP-010-02 | Contract 010 Sections 1-5 | reset, hysteresis, dwell, proposed decision | reset/hysteresis/dwell/decision types | CONT-011..021 | implemented, reference semantics |
| IMP-010-03 | Contract 010 Sections 1/3 | atomic mode and continuity-record commitment and Contract 011 handoff | committed mode/record/store/handoff types | CONT-022..034 | implemented, bounded |
| IMP-011-01 | Contract 011 Sections 1-10 | Stage One admission, realization construction/validation/commitment/failure | Stage One artifact types and operations | HIST-001..019 | implemented, bounded |
| IMP-011-02 | Contract 011 Sections 10-17 | Stage Two committed-realization boundary, artifact/relationship resolution, lineage completeness/integrity | resolution, lineage, evaluation types | HIST-020..049 | implemented, bounded |
| IMP-011-03 | Contract 011 Sections 17-21 | cycle commitment, exact-one terminal outcome, replay, audit, Contract 012 handoff | cycle/store/replay/audit/handoff types | HIST-050..078 | implemented, bounded |
| IMP-012-01 | Contract 012 Sections 2-10 | committed-cycle admission, exchange identity, claims, profile, requirements, methods, applicability | context/identity/claim/profile/applicability types | VERX-001..016 | implemented, bounded |
| IMP-012-02 | Contract 012 Sections 11-17 | evidence resolution/validation/package, submission commitment, custody | evidence/submission/custody types | VERX-017..037 | implemented, fixture-only |
| IMP-012-03 | Contract 012 Sections 18-23 | external-result preservation, result certification, terminal exchange, replay/audit | result/exchange/store/replay/audit types | VERX-038..070 | implemented, bounded |
