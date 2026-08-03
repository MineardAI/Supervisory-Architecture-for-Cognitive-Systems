# SACS-IMP-005 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence | Status |
|---|---|---|---|---|
| Consume committed OSV and Governance Snapshot only | Contract 005 §§5, 11, 44 | `validate_inputs`, evaluator signature | OJT-002/003/017; EV-OBJ-0001 | implemented |
| Explicit objective source and registry binding | §§6-7 | source declaration, definition, registry snapshot | OJT-001, OJT-021 | implemented |
| Declarative objective classes/targets without control semantics | §§3, 8-10, 33-34 | `ObjectiveClass`, definition target/tolerance | OJT-018/019 | implemented, bounded |
| Applicability distinct from eligibility | §§12-14 | applicability/eligibility result enums | OJT-006, OJT-008 | implemented |
| Governance-respecting eligibility | §§14, 35, 37 | permission/prohibition/restriction checks | OJT-003/004/005 | implemented |
| Explicit status, priority, dependency, conflict | §§15, 18, 20-22 | status/priority/dependency/conflict model | OJT-010..012 | implemented, reference profile |
| Suppression/withholding/reduced authority | §§29, 37 | explicit statuses and typed failures | OJT-013, OJT-015 | implemented, bounded |
| Immutable candidate/validation/Objective Set | §§28-32, 39-41 | candidate, validation, committed set, store | OJT-015/016/020 | implemented |
| Deterministic Profile Selector handoff without selection | §§32-34 | `CommittedObjectiveSetHandoff` | OJT-018 | implemented |
| Production satisfaction/optimization/objective language | §§16-17, 27, 48 | explicitly deferred | EV-OBJ-0001 | deferred by source |

