# SACS-IMP-005 Supervisory Objectives Realization Profile

Status: CONDITIONALLY COMPLETE  
Crate: `sacs-supervisory-objectives`  
Normative owner: Contract 005  

## Boundary

The crate consumes only `CommittedOsvHandoff` and `CommittedGovernanceHandoff`. It validates identity, hashes, cycles, schemas, registries, and downstream-eligible statuses; admits explicit objective sources and immutable definitions; evaluates applicability, governance eligibility, dependencies, conflicts, and deterministic priority; constructs a noncanonical `ObjectiveSetCandidate`; validates it; and produces an immutable `CommittedObjectiveSet` for Contract 006.

Objectives remain declarative. The crate does not rewrite governance or OSV state, select operational profiles, construct reference states, compute control error, translate, execute, transition modes, certify history, or verify externally.

## Public API

The public model includes `ObjectiveSourceDeclaration`, `SupervisoryObjectiveDefinition`, `ObjectiveRegistrySnapshot`, `ObjectiveEvaluationProfile`, `ObjectiveEvaluationContext`, applicability/eligibility/status enums, `EvaluatedObjective`, `ObjectiveSetCandidate`, `ObjectiveSetValidationResult`, `CommittedObjectiveSet`, `ObjectiveCommitmentRecord`, `CommittedObjectiveSetHandoff`, `ObjectiveFailure`, `ObjectiveEvaluator`, and `ObjectiveStore`.

Applicability, eligibility, active/restricted/blocked/unresolved status, dependencies, conflicts, and priority are represented separately. Missing governance permission, unresolved OSV fields, invalid input, unresolved dependencies, or unresolved conflicts never activate a broader objective set.

## Source limitation and closure

Contract 005 adopts objective classes, statuses, profiles, registries, and conflict concepts but leaves production objective language, satisfaction mathematics, numeric weights, domain catalogs, and complete policy profiles under-specified. This realization therefore uses explicit source-bound definitions and a replaceable deterministic reference evaluation posture; it does not invent universal objective semantics.

