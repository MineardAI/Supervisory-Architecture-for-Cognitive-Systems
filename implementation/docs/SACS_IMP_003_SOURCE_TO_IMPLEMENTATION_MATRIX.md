# SACS-IMP-003 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence | Status |
|---|---|---|---|---|
| Consume one immutable Estimated State; no raw observations | Contract 003 §§1,5 | `OsvPublisher::publish`, `validate_handoff` | OVT-001/002; EV-OSV-0001 | implemented |
| Bind cycle, cutoff, identity, provenance | Contract 003 §2 | `validate_handoff`, `OsvDraft` | OVT-009/016 | implemented |
| Registered fields and explicit schema/profile | Contract 003 §§7-9 | `OsvFieldRegistrySnapshot`, `OsvSchema`, `OsvPublicationProfile` | OVT-003/004/008 | implemented |
| Registered deterministic projection | Contract 003 §§10-11 | `OsvProjectionProfile`, `ProjectionRule`, direct binding | OVT-003/013 | implemented, bounded |
| Preserve validity, quality, uncertainty, covariance posture | Contract 003 §§12-14 | `OsvField`, `CovariancePolicy`, draft/committed artifact | OVT-005/006/007 | implemented |
| Atomic immutable publication | Contract 003 §§16-17 | `OsvDraft`, validation, `OsvStore` | OVT-009/010/011 | implemented |
| One-cycle sequence and no mutation | Contract 003 §18 | `previous_osv_id`, cycle guard | OVT-012/015 | implemented, bounded |
| Committed OSV-only downstream access | Contract 003 §19 | `CommittedOsvHandoff` | OVT-014 | implemented |
| Explicit publication failures | Contract 003 §15 and §21 | `OsvPublicationFailure` | EV-OSV-0001 | implemented |
| Production Core8 vocabulary/mathematics | Contract 003 §§9,11 | deferred; no invented semantics | decision record | deferred by source |

