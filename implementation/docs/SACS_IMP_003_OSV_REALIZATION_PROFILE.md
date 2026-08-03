# SACS-IMP-003 Operational State Vector Realization Profile

Status: CONDITIONALLY COMPLETE  
Crate: `sacs-operational-state-vector`  
Normative owner: Contract 003  

## Boundary

The crate consumes only `sacs_state_estimation::EstimatedStateHandoff`. It validates cycle, cutoff, identity, provenance, schema, registry, state-basis, and explicit publication-profile bindings; projects registered fields; preserves field validity, quality, uncertainty, and covariance posture; constructs a noncanonical `OsvDraft`; validates it; and commits an immutable `CommittedOperationalStateVector` through the authority-neutral commitment engine.

It does not read raw observations, reopen Contract 001, estimate state, select profiles, perform governance, objectives, control, execution, history certification, or external verification.

## Public API

`OsvPublisherProfile`, `OsvFieldDefinition`, `OsvFieldRegistrySnapshot`, `OsvSchema`, `OsvProjectionProfile`, `OsvPublicationProfile`, `OsvDraft`, `OsvValidationResult`, `OsvPublicationRecord`, `CommittedOperationalStateVector`, `CommittedOsvHandoff`, `OsvPublicationFailure`, and `OsvStore` are explicit immutable data boundaries. `OsvPublisher::publish` is the only publication constructor and accepts an `EstimatedStateHandoff`.

## Atomicity and commitment

Fields are deterministically ordered by field identity. A draft is never handed downstream. Validation precedes commitment; one cycle maps to one committed identity in `OsvStore`; identical commitment is idempotent, conflicting content is rejected, and a second identity for the same cycle is rejected. The committed artifact carries cycle, cutoff, upstream identities, schema/registry/profile bindings, field validity/quality/uncertainty, covariance reference, provenance, and hash.

## Source limitation and closure

Contract 003 expressly retains the Core8 names as a non-normative candidate until field definitions, units, ranges, validity, and projection rules are adopted. This realization therefore supplies no universal field vocabulary or production projection mathematics. The direct-binding profile is replaceable and documented in `SACS_IMP_003_NON_NORMATIVE_OSV_DECISIONS.md`.

