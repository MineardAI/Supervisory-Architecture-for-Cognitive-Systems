# Evidence Index

| Evidence | Command or source | Purpose |
|---|---|---|
| E-001 | `cargo fmt --all -- --check` | formatting conformance |
| E-002 | `cargo test --workspace` | crate unit, deterministic, negative, round-trip, commitment tests |
| E-003 | `cargo clippy --workspace --all-targets -- -D warnings` | warning-free implementation quality |
| E-004 | `docs/IMPLEMENTATION_PROFILE.md` | declared scope and exclusions |
| E-005 | `docs/REQUIREMENT_IMPLEMENTATION_MATRIX.md` | source-to-implementation traceability |
| E-006 | `docs/TYPE_IMPLEMENTATION_MATRIX.md` | type ownership and prohibition traceability |
| E-007 | `CONFORMANCE_VECTOR_INDEX.md` | stable input/output vectors |
| E-008 | workspace member list | requested crate coverage |
| E-009 | `implementation/crates/observation` | Contract 001 source admission and handoff implementation |
| E-010 | `EV-OBS-0001.md` | Contract 001 implementation evidence and scope audit |
| E-011 | `implementation/crates/state-estimation` | Contract 002 state-estimation implementation |
| E-012 | `EV-EST-0001.md` | Contract 002 implementation evidence, decisions, and scope audit |
| E-013 | `crates/operational-state-vector` | Contract 003 OSV publication implementation |
| E-014 | `EV-OSV-0001.md` | Contract 003 implementation evidence, decisions, limitations, and scope audit |
| E-015 | `crates/governance` | Contract 004 Governance implementation |
| E-016 | `EV-GOV-0001.md` | Contract 004 implementation evidence, decisions, limitations, and scope audit |
| E-017 | `crates/supervisory-objectives` | Contract 005 Supervisory Objectives implementation |
| E-018 | `EV-OBJ-0001.md` | Contract 005 implementation evidence, decisions, limitations, and scope audit |
| E-019 | `crates/operational-profiles` | Contract 006 Operational Profiles and Reference States implementation |
| E-020 | `EV-PROFILE-0001.md` | Contract 006 implementation evidence, decisions, limitations, and scope audit |
| E-021 | `crates/controller` | Contract 007 Controller implementation |
| E-022 | `EV-CTRL-0001.md` | Contract 007 implementation evidence, decisions, limitations, and scope audit |
| E-023 | `crates/inference-control` | Contract 008 Inference Control implementation |
| E-024 | `EV-TRANS-0001.md` | Contract 008 implementation evidence, decisions, limitations, and scope audit |
| E-025 | `crates/execution-interface` | Contract 009 Execution Interface implementation |
| E-026 | `EV-EXEC-0001.md` | Contract 009 implementation evidence, decisions, limitations, and scope audit |
| E-027 | `crates/supervisory-continuity` | Contract 010 Supervisory Continuity implementation |
| E-028 | `EV-CONT-0001.md` | Contract 010 implementation evidence, decisions, limitations, and scope audit |
| E-029 | `crates/constitutional-history` | Contract 011 Realization Certification and Supervisory Provenance implementation |
| E-030 | `EV-HIST-0001.md` | Contract 011 implementation evidence, decisions, limitations, and scope audit |
| E-031 | `crates/external-verification-interface` | Contract 012 External Verification Interface implementation |
| E-032 | `EV-VERX-0001.md` | Contract 012 implementation evidence, decisions, limitations, and scope audit |

Evidence records in this directory document bounded realizations of Contracts 001-012; they are implementation evidence only and do not claim constitutional authority, production deployment, or external verification.
