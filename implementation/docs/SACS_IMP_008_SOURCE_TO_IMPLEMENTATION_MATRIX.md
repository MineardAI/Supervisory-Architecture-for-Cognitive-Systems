# SACS-IMP-008 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence/vector | Status |
|---|---|---|---|---|
| committed command-only admission | Contract 008 §§6-7 | `TranslationEngine::admit` | TRANS-001/002 | implemented |
| capability and availability distinction | Section 8 | `RuntimeCapabilityProfile`, `RuntimeAvailability` | TRANS-005/006 | implemented |
| closed translation configuration/profile | Sections 10-12 | configuration/profile/definition types | TRANS-003/004 | bounded |
| requirement preservation | Contract 008 flow; Sections 13-15 | `TranslationRequirementSet` | TRANS-009 | implemented |
| deterministic compatibility binding | Sections 8, 10, 17 | `ValidatedTranslationBinding`, `bind` | TRANS-007/008 | implemented |
| normalization/numeric/representation/order | Sections 13-16 | `translate`, explicit rules | TRANS-010..015 | reference-derived |
| proposed and committed command sets | Sections 7, 17-19 | proposed/committed set types | TRANS-016..019 | implemented |
| transaction and commitment | Sections 18-19 | transaction/record/store types | TRANS-020..024 | implemented |
| execution interface boundary | Contract 008 §19; Contract 009 handoff | `ExecutionInterfaceHandoffEnvelope` | TRANS-025/029 | implemented, non-executing |
| production adapters and execution | Contract 008 exclusions | not implemented | decisions/evidence | deferred |
