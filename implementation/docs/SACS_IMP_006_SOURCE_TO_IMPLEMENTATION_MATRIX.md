# SACS-IMP-006 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence | Status |
|---|---|---|---|---|
| Consume committed OSV, Governance, Objective Set only | Contract 006 Sections 1-2, 6 | upstream handoffs and `UpstreamBindings` | OPR-001; EV-PROFILE-0001 | implemented |
| Immutable profile definition and registry publication | Section 2 §§10-14; Section 3 §§17-19 | definition, entry, snapshot types | OPR-003/004 | implemented |
| Scope-only candidate discovery | Section 3 §§20-29 | `ProfileEvaluator::discover` | OPR-005 | implemented |
| Eligibility distinct from compatibility | Section 3 §§29-38; Section 4 | separate reports and stages | OPR-006/007 | implemented |
| Deterministic selection and tie-breaking | Section 5 §§60-86 | selection policy/report and ordered comparator | OPR-011/012 | implemented, reference policy |
| Profile construction and atomic commitment | Section 6 §§87-99 | constructed/validated/committed profile and store | OPR-014/015 | implemented |
| Reference-state builder and commitment | Section 6 §100 and prompt | reference definition, constructed/validated/committed state | OPR-016/017 | implemented, bounded |
| Controller-only downstream handoff | Section 1 and Section 6 §§99-100 | `ControllerConfigurationHandoff` | OPR-022/023 | implemented |
| No profile fallback/current pointer/control logic | Sections 1-6 prohibitions | bounded source/dependency audit | EV-PROFILE-0001 | implemented |
| Production catalogs, scoring, target mathematics, persistence | Sections 2, 5, 6 deferrals | explicit decision record | EV-PROFILE-0001 | deferred by source |

