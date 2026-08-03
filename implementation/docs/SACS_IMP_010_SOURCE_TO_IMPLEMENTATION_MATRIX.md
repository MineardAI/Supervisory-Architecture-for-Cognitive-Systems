# SACS-IMP-010 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence/vector | Status |
|---|---|---|---|---|
| committed execution and supervisory context admission | Contract 010 Sections 1/3 | `ContinuityEngine::admit` | CONT-001..004 | implemented |
| explicit mode identity/registry | Sections 2/4/5 | mode/registry types | CONT-005/022 | bounded |
| guard evaluation and eligibility | Sections 2/3 | `evaluate_guards`, `eligible` | CONT-009..010 | implemented |
| deterministic precedence | Sections 2/3 | `precedence` | CONT-007/008 | implemented |
| reset evaluation/evidence | Sections 3/4 | `reset`, `ResetRecord` | CONT-011..013 | implemented |
| hysteresis evaluation | Sections 1/3 | `hysteresis` | CONT-014..016 | reference threshold |
| dwell-time evaluation | Sections 1/3 | `dwell`, explicit timing context | CONT-017..020 | implemented, integer reference |
| proposed decision and atomic mode/record commitment | Sections 1/3 | `evaluate`, `commit`, `ContinuityStore` | CONT-021..028 | implemented |
| Contract 011 handoff boundary | Section 1 and Stage One sources | `SupervisoryContinuityHandoff` | CONT-031..034 | boundary only |
| certification/history/audit | resolved Contract 010 boundary | omitted | decisions/evidence | deferred |
