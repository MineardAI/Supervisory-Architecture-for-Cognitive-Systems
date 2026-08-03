# SACS-IMP-012 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence/vector | Status |
|---|---|---|---|---|
| committed-cycle-only subject admission | Contract 012 §§2-5 | `admit_committed_cycle` | VERX-001..005 | implemented |
| identity, claims, profile, requirements, method | §§6-9 | exchange/claim/profile/requirement types | VERX-006..015 | implemented, bounded |
| applicability and evidence resolution/validation | §§10-14 | applicability/evidence operations | VERX-016..024 | implemented |
| package/submission construction and commitment | §§13-16 | package/submission types | VERX-025..032 | implemented |
| custody boundary | §17 | `CustodyTransferRecord` | VERX-033..037 | fixture-only |
| external result admission and preservation | §§18-19 | external/admission/result types | VERX-038..047 | implemented |
| result certification/commitment | §19 | certification/committed result | VERX-048..053 | implemented |
| terminal exchange record/failure | §20/22 | exchange/store types | VERX-054..061 | implemented, bounded |
| replay/audit/handoff | §§21/23 | replay/audit operations | VERX-062..070 | implemented |
| external verification/network/provider | authority boundary | omitted | decisions/evidence | deferred |
