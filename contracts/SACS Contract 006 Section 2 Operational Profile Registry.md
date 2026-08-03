10. Operational Profile Identity

An Operational Profile is a versioned, deterministic supervisory configuration that translates a committed Objective Set and Governance Snapshot into a bounded supervisory operating configuration.

The defining identity of an Operational Profile is:

An Operational Profile defines the supervisory assumptions, operating regions, permissions, limits, and reference-state construction parameters under which the controller will operate.

An Operational Profile configures supervision.

It does not perform supervision.

An Operational Profile MAY define:

supervisory operating regions;
acceptable operating envelopes;
supervisory limits;
gain-set identifiers;
reference-state construction policies;
transition eligibility;
execution constraints inherited from governance;
supervisory tolerances;
profile capabilities;
profile persistence behavior.

An Operational Profile MUST NOT define:

controller equations;
control laws;
runtime commands;
inference instructions;
governance policy;
supervisory objectives;
OSV semantics;
estimator behavior;
execution behavior.
11. Operational Profile Authority

Contract 006 owns:

Operational Profile definitions;
Profile Registry;
profile eligibility;
profile compatibility;
deterministic profile selection;
deterministic tie-breaking;
Reference Builder;
Reference State construction;
profile transitions;
profile provenance.

Contract 006 does not own:

Observation
State Estimation
OSV publication
Governance
Objective evaluation
Controller mathematics
Inference translation
Execution
Verification

No later contract may redefine these authorities.

12. Canonical Operational Profile

Every committed Operational Profile SHALL include:

profile_id
profile_version
profile_name
profile_class
profile_status

objective_set_id
governance_snapshot_id

operating_region_id

gain_profile_id

reference_builder_id

transition_policy_id

numeric_profile_id

capability_profile

permissions

constraints

schema_version

registry_versions

provenance_id

artifact_hash

Optional fields MAY include:

parent_profile_id

supersedes_profile_id

deployment_scope

supported_modes

resource_requirements

extension_profiles

vendor_extensions

An Operational Profile identifier SHALL NOT be reused for a semantically different profile.

13. Profile Classes

The initial Profile Class vocabulary is:

NORMAL

RECOVERY

STABILIZATION

LEARNING

CALIBRATION

LOW_CONFIDENCE

HUMAN_OVERRIDE

DETERMINISTIC_REPLAY

EMERGENCY

These classes describe supervisory operating configurations.

They are not execution modes.

They are not controller states.

They are not user-facing classifications.

13.1 Normal

Represents nominal supervisory operation under expected operating conditions.

13.2 Recovery

Optimized for restoring acceptable operating conditions following deviation.

Recovery SHALL NOT redefine supervisory objectives.

13.3 Stabilization

Prioritizes bounded supervisory stability over performance.

Stabilization SHALL NOT be interpreted as Contract 010's Stabilize Mode.

13.4 Learning

Allows supervisory configurations appropriate for adaptive systems while remaining bounded by Governance.

Learning SHALL NOT authorize model training.

13.5 Calibration

Supports measurement validation, estimator confidence improvement, and supervisory recalibration.

Calibration SHALL remain bounded by Governance.

13.6 Low Confidence

Used when uncertainty or supervisory confidence exceeds registered thresholds.

Low Confidence SHALL reduce supervisory authority rather than expand it.

13.7 Human Override

Represents supervisory operation under explicit human authority.

Human Override SHALL remain constrained by Governance.

13.8 Deterministic Replay

Supports reproducible supervisory replay.

Replay Profiles SHALL disable nondeterministic supervisory configuration.

13.9 Emergency

Represents registered emergency supervisory configurations.

Emergency SHALL NOT bypass Governance.

14. Candidate Profile Generation

Profile generation begins only after:

OSV commitment;
Governance Snapshot commitment;
Objective Set commitment.

The Profile Selector SHALL resolve all registered profiles applicable to the current supervisory context.

Each candidate profile SHALL reference:

supported objectives;
governance requirements;
capability requirements;
operating regions;
transition constraints.

Generation SHALL NOT perform profile selection.

Its sole purpose is candidate discovery.

15. Profile Eligibility

Each candidate profile SHALL be evaluated for eligibility.

Eligibility MAY depend upon:

committed OSV;
committed Governance Snapshot;
committed Objective Set;
capability profile;
deployment scope;
implementation support;
registered profile requirements.

Eligibility SHALL NOT depend upon:

controller state;
runtime commands;
execution results from the current cycle;
hidden implementation state;
estimator internals not represented in canonical artifacts.

Eligibility SHALL resolve to exactly one of:

ELIGIBLE

INELIGIBLE

UNSUPPORTED

PROHIBITED

UNRESOLVED

Profiles with unresolved eligibility SHALL NOT be selected as nominal.

16. Profile Compatibility

Eligibility determines whether a profile may participate.

Compatibility determines whether it fits the committed supervisory context.

Compatibility evaluation SHALL consider:

Governance compatibility;
Objective compatibility;
capability compatibility;
implementation compatibility;
operating-region compatibility;
profile transition rules.

Compatibility SHALL NOT modify:

Governance;
Objectives;
OSV;
candidate profiles.

Compatibility SHALL resolve to:

COMPATIBLE

COMPATIBLE_WITH_RESTRICTIONS

INCOMPATIBLE

CONFLICTED

UNRESOLVED

Only compatible profiles may proceed to deterministic selection.

Council Note

At this point the contract has cleanly separated the three major phases:

Discovery — "Which profiles exist?"
Eligibility — "Which profiles are allowed?"
Compatibility — "Which allowed profiles correctly satisfy this supervisory context?"

Only after these phases should Contract 006 define deterministic scoring, tie-breaking, and Profile Selection Reports. That preserves the single-transformation philosophy established throughout SACS and keeps selection logic transparent and replayable. It also reinforces the architectural boundary from Contract 000 that each layer owns exactly one category of authority and transforms one committed artifact into the next.

## Revisions Needed
Revised Section 12
12. Canonical Operational Profile Definition

An Operational Profile Definition is a reusable, versioned supervisory configuration published through the Profile Registry.

Its defining identity is:

An Operational Profile Definition specifies how supervision may be configured. It does not represent that the configuration has been selected for any particular supervisory cycle.

An Operational Profile Definition SHALL be immutable after publication.

Every canonical Operational Profile Definition SHALL include:

profile_id
profile_version
profile_name

profile_class

definition_status

supported_objective_classes

governance_requirements

operating_region_id

gain_profile_id

reference_builder_id

transition_policy_id

numeric_profile_id

capability_profile

permission_policy

constraint_policy

schema_version

registry_dependencies

definition_hash

Optional fields MAY include:

parent_profile_id

vendor_metadata

deployment_scope

documentation_reference

An Operational Profile Definition SHALL NOT contain:

objective_set_id
governance_snapshot_id
osv_id
provenance_id
artifact_hash
committed_at
selection evidence

Those belong exclusively to a committed artifact.

13. Canonical Committed Operational Profile

A Committed Operational Profile represents the immutable supervisory configuration selected for one supervisory cycle.

Its defining identity is:

A Committed Operational Profile binds one Operational Profile Definition to one committed supervisory context.

A Committed Operational Profile SHALL be produced only after deterministic profile selection.

Every committed profile SHALL include:

committed_profile_id

profile_id

profile_version

osv_id

governance_snapshot_id

objective_set_id

selection_report_id

effective_permissions

effective_constraints

effective_operating_region_id

effective_gain_profile_id

registry_versions

provenance_id

committed_at

artifact_hash

The Committed Operational Profile SHALL be immutable.

Any subsequent supervisory cycle SHALL produce a new committed artifact.

A committed profile SHALL NOT mutate after publication.

14. Profile Lifecycle

The Council also correctly identified the overloaded profile_status.

It should be split.

Definition Lifecycle

Operational Profile Definitions SHALL use:

DRAFT

ACTIVE

DEPRECATED

RETIRED

These describe registry lifecycle only.

They SHALL NOT indicate runtime selection.

Commitment Lifecycle

A Committed Operational Profile is immutable.

Therefore it SHOULD NOT contain a mutable status field.

Lifecycle SHALL instead be represented through provenance relationships:

superseded_by

replaced_by

terminated_by

next_committed_profile

This preserves replay integrity and avoids mutating canonical artifacts.

Revised Candidate Discovery

The Council's wording is better.

Section 14 becomes:

Candidate generation SHALL resolve all registered and implementation-supported Operational Profile Definitions within the declared registry, deployment, and capability scope.

Candidate generation SHALL NOT determine:

eligibility;
compatibility;
preference;
scoring;
selection.

Its sole purpose is discovery.

Every discovered profile SHALL remain visible in the Profile Selection Report, even if later rejected.

Revised Eligibility Identity

The Council's proposed distinction is exactly right.

I would make it normative.

Eligibility determines whether a candidate is permitted to participate in profile selection.

Eligibility SHALL evaluate mandatory admission conditions only.

Examples include:

profile definition active;
implementation support;
deployment scope;
required capabilities;
governance class admission;
transition admission.

Eligibility SHALL NOT determine semantic suitability.

Revised Compatibility Identity

Likewise:

Compatibility determines whether an eligible candidate can coherently construct a valid supervisory configuration for the committed context.

Compatibility SHALL evaluate:

Objective support;
operating region coverage;
Reference Builder capability;
numeric profile compatibility;
controller interface compatibility;
inherited restrictions.

Compatibility SHALL NOT determine preference.

Restricted Compatibility

The Council found a real ambiguity.

Instead of

COMPATIBLE_WITH_RESTRICTIONS

being descriptive, it becomes productive.

The compatibility stage SHALL emit:

compatibility_restrictions

effective_permission_ceiling

effective_constraint_additions

restriction_provenance

These become inputs to the Committed Operational Profile.

Thus restrictions become first-class supervisory artifacts rather than labels.

Human Override

I strongly prefer the Council wording.

Replace with:

HUMAN_OVERRIDE represents a registered supervisory configuration selected in response to authenticated human authority recognized by the committed Governance Snapshot.

It SHALL NOT:

create governance;
replace governance;
expand governance;
substitute governance.

This keeps Governance as the sole authority source.

Calibration

The Council also correctly identified a subtle authority leak.

Instead of:

estimator confidence improvement

use:

Supports supervisory conditions under which calibration evidence may be collected and evaluated by the owning estimator components.

Contract 006 influences conditions, not estimators.

New Invariant — Authority Monotonicity

The Council's final observation deserves promotion to a normative invariant.

15. Authority Monotonicity

Operational Profile evaluation, compatibility restriction, commitment, and transition SHALL preserve or reduce supervisory authority.

No profile operation SHALL expand permissions beyond those granted by the committed Governance Snapshot.

Formally,

Effective Authority

⊆

Governance Authority

The relationship is monotonic toward equal or reduced authority.

An Operational Profile SHALL never introduce new supervisory permissions.

Council Assessment

This review resolves what is likely the last major architectural identity question before Contract 006 enters its mechanism phase. Separating Operational Profile Definitions from Committed Operational Profiles aligns Contract 006 with the artifact lifecycle established across SACS: reusable definitions remain stable in registries, while immutable cycle-specific artifacts capture supervisory decisions and provenance. Combined with the new Authority Monotonicity invariant and the clarified eligibility-versus-compatibility boundary, the contract now preserves strict authority separation while strengthening replay, audit, and verification. From this point forward, the remaining work—Profile Registries, deterministic scoring, tie-breaking, Selection Reports, and Reference Builder semantics—can build on a stable ontological foundation rather than redefining it later.