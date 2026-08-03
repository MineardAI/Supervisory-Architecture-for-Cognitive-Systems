# SACS-IMP-004 Governance Realization Profile

Status: CONDITIONALLY COMPLETE  
Crate: `sacs-governance`  
Normative owner: Contract 004  

## Boundary

The crate consumes only `sacs_operational_state_vector::CommittedOsvHandoff`, validates its identity/hash/cycle/schema/registry bindings, admits explicitly registered external authorities and source artifacts, preserves typed governance rules, evaluates explicit effective-time and scope applicability, resolves the bounded reference precedence profile conservatively, constructs a noncanonical candidate, validates it, and produces an immutable `CommittedGovernanceSnapshot` for commitment.

It does not consume Estimated State, observations, OSV drafts, or mutable OSV state. It does not author policy, select objectives or profiles, calculate control, translate, execute, certify history, or verify externally.

## Public API

The public model includes `GovernanceAuthorityRegistration`, `GovernanceSourceDeclaration`, `GovernanceSourceArtifact`, `GovernanceAdapterDeclaration`, `GovernanceRule`, `GovernanceRevocation`, `GovernanceAdmissionContext`, `GovernanceApplicabilityResult`, `GovernanceSnapshotCandidate`, `GovernanceValidationResult`, `CommittedGovernanceSnapshot`, `GovernanceCommitmentRecord`, `CommittedGovernanceHandoff`, `GovernanceFailure`, `GovernanceEngine`, and `GovernanceStore`.

Permissions, prohibitions, obligations, limits, eligibility, requirements, escalation, audit, retention, disclosure, and reduction rule classes remain distinct. Missing or unresolved governance never becomes `ALLOW`; conflicts are explicit and prohibition precedence is conservative in the bounded reference path.

## Commitment and revocation

Candidates are not downstream authority. Validation precedes commitment. Store commitment is append-only and idempotent; a second different snapshot identity for one cycle is rejected. Revocations are retained as append-only evidence and are not used to mutate prior snapshots.

## Source limitation and closure

Contract 004 requires registered precedence, conflict, compatibility, update, revocation, trust, and governance profiles but does not supply production policy language or complete source federation semantics. The implementation therefore uses an explicit, replaceable reference adapter/precedence posture and defers production trust scoring, signatures, network adapters, policy engines, and legal interpretation.

