# SACS-IMP-004 Source-to-Implementation Matrix

| Requirement | Source | Implementation | Evidence | Status |
|---|---|---|---|---|
| OSV-only input and no mutation | Contract 004 §§3, 11, 35 | `validate_osv`, `GovernanceEngine::construct` | GT-007; EV-GOV-0001 | implemented |
| Registered external authority and source binding | §§4-7 | authority/source structs and admission checks | GT-001, GT-014 | implemented |
| Adapter declaration and non-invention boundary | §8 | `GovernanceAdapterDeclaration` | GT-003 | implemented, bounded |
| Distinct canonical rule classes | §9-10 | `GovernanceRuleClass`, `GovernanceRule` | GT-004/005 | implemented |
| Explicit applicability/effective-time context | §§11, 25 | `GovernanceAdmissionContext`, applicability checks | GT-008/009 | implemented |
| Snapshot candidate and validation | §§5, 12-14, 30 | candidate/validation types | GT-020/021 | implemented |
| Explicit permissions and conservative disposition | §§15, 18-19, 34 | `GovernanceDisposition`, `disposition_for` | GT-004/013/022 | implemented, bounded |
| Conflict preservation and deterministic precedence | §§16-17 | `resolve_conflicts`, `GovernanceConflict` | GT-011/012 | implemented, reference profile |
| Revocation as append-only evidence | §26 | `GovernanceRevocation`, `GovernanceStore::record_revocation` | GT-010; EV-GOV-0001 | implemented |
| Immutable committed Governance Snapshot | §§12-14, 27 | `CommittedGovernanceSnapshot`, `GovernanceStore` | GT-006 | implemented |
| Reduced-authority failure | §§28-29, 34 | typed `GovernanceFailure`, no empty/ALLOW default | GT-013 | implemented |
| Downstream committed-only handoff | §§12, 19, 24 | `CommittedGovernanceHandoff` | GT-022 | implemented |
| Production policy language/trust federation | §§6, 8, 31, 36, 39 | explicitly deferred | EV-GOV-0001 | deferred by source |

