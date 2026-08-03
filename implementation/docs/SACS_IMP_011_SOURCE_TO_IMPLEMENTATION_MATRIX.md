# SACS-IMP-011 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence/vector | Status |
|---|---|---|---|---|
| Stage One exact input admission | Contract 011 §§1-7 | `admit_stage_one` | HIST-001..009 | implemented |
| realization construction/validation | §§7-9 | construction and validation types | HIST-010..012 | implemented |
| independent realization commitment/failure | §§9-10 | `commit_realization`, failure type | HIST-013..019 | implemented |
| Stage Two committed-realization-only boundary | §§10-12 | `resolve_artifacts` input type | HIST-020..024 | implemented |
| artifact and relationship resolution | §§12-15 | resolution sets and operations | HIST-025..037 | bounded |
| lineage completeness/integrity separation | §§15-17 | evaluation result types | HIST-038..049 | implemented |
| cycle construction/commitment/exact-one | §§17-19 | cycle types/store | HIST-050..062 | implemented, bounded |
| replay and non-constitutive audit | §§20-21 | `replay`, `audit` | HIST-063..071 | implemented |
| Contract 012 handoff | §21 and Contract 012 boundary | `Contract012Handoff` | HIST-072..078 | boundary only |
