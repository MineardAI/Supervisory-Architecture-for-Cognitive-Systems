# 39. Compatibility Evaluation

Compatibility Evaluation is the second substantive evaluation stage of Contract 006.

Its defining identity is:

> **Compatibility determines whether an eligible Operational Profile Definition can coherently produce a valid supervisory configuration for the committed context.**

Compatibility evaluates constructability and coherence.

It does not determine preference.

Compatibility Evaluation SHALL consume:

```text
Eligibility Evaluation Report

Committed OSV

Committed Governance Snapshot

Committed Objective Set

Controller Interface Profile

Reference Builder Capability Declaration

Numeric Profile Compatibility Declaration

Compatibility Policy Versions
```

Compatibility Evaluation SHALL evaluate only candidates whose Eligibility outcome is:

```text
ELIGIBLE
```

No candidate with an outcome of:

```text
INELIGIBLE

UNSUPPORTED

PROHIBITED

UNRESOLVED
```

may proceed to Compatibility Evaluation.

---

# 40. Compatibility Evaluation Report

Compatibility Evaluation SHALL produce one immutable Compatibility Evaluation Report.

Its defining identity is:

> **The Compatibility Evaluation Report records whether every eligible candidate can be transformed into a coherent, bounded supervisory configuration for the committed context.**

Every Compatibility Evaluation Report SHALL include:

```text
compatibility_report_id

cycle_id

candidate_set_id
eligibility_report_id

osv_id
governance_snapshot_id
objective_set_id

controller_interface_profile_id

reference_builder_capability_declaration_id
numeric_profile_compatibility_declaration_id

compatibility_policy_versions

compatibility_results

compatibility_report_schema_version
contract_version

evaluation_provenance_id

created_at

artifact_hash
```

The Compatibility Evaluation Report SHALL be immutable after commitment.

---

# 41. Compatibility Result

Each ELIGIBLE candidate SHALL receive exactly one Compatibility Result.

Every Compatibility Result SHALL include:

```text
registry_entry_id

profile_id
profile_version
definition_hash

compatibility_outcome

evaluated_compatibility_requirements

satisfied_requirements

failed_requirements

conflicting_requirements

unresolved_requirements

compatibility_restrictions

effective_permission_ceiling

effective_constraint_additions

effective_operating_region_projection

effective_gain_profile_resolution

reference_constructability_result

numeric_compatibility_result

controller_interface_compatibility_result

compatibility_rule_versions

result_provenance_id
```

Compatibility results SHALL be canonically ordered according to the ordering inherited from the Candidate Profile Set.

---

# 42. Compatibility Outcome Vocabulary

Every ELIGIBLE candidate SHALL receive exactly one of the following outcomes:

```text
COMPATIBLE

COMPATIBLE_WITH_RESTRICTIONS

INCOMPATIBLE

CONFLICTED

UNRESOLVED
```

No other terminal Compatibility outcome is permitted unless introduced by a later compatible contract version.

---

## 42.1 COMPATIBLE

`COMPATIBLE` means the candidate can coherently produce a complete supervisory configuration for the committed context without requiring additional restrictions beyond those already contained in Governance and the Profile Definition.

A COMPATIBLE result does not mean that the candidate:

* is preferred;
* has the highest score;
* will be selected;
* guarantees successful control;
* guarantees Objective satisfaction.

It means only that valid configuration construction is possible.

---

## 42.2 COMPATIBLE_WITH_RESTRICTIONS

`COMPATIBLE_WITH_RESTRICTIONS` means the candidate can coherently produce a complete supervisory configuration only after additional authority-preserving restrictions are applied.

Every restricted Compatibility result SHALL materialize those restrictions explicitly.

Restrictions SHALL NOT remain descriptive or implicit.

The result SHALL identify:

```text
compatibility_restrictions

effective_permission_ceiling

effective_constraint_additions

effective_operating_region_projection

restriction_rule_ids

restriction_provenance
```

A restriction SHALL preserve or reduce authority.

It SHALL NOT:

* broaden Governance permissions;
* broaden Profile Definition permissions;
* remove mandatory constraints;
* expand operating regions beyond authorized bounds;
* select a more permissive gain profile;
* alter committed Objectives.

Candidates with `COMPATIBLE_WITH_RESTRICTIONS` MAY proceed to deterministic selection.

---

## 42.3 INCOMPATIBLE

`INCOMPATIBLE` means a coherent supervisory configuration cannot be constructed from the candidate and the committed context.

Examples include:

* the Profile Definition does not support required Objective classes;
* required OSV dimensions are not covered by its operating regions;
* the Reference Builder cannot construct all required target dimensions;
* the Numeric Profile cannot represent required values;
* the Controller Interface Profile cannot consume the resulting configuration;
* inherited restrictions make the configuration incomplete;
* a required target cannot be expressed within permitted bounds.

INCOMPATIBLE means constructability failed.

It does not mean the candidate was prohibited from participating.

---

## 42.4 CONFLICTED

`CONFLICTED` means two or more individually valid compatibility requirements cannot be satisfied together without contradiction.

Examples include:

* an Objective requires a target range disallowed by Governance;
* a Profile Definition requires a capability that conflicts with an inherited restriction;
* two required operating-region constraints have an empty intersection;
* a required Reference Builder policy contradicts the selected Numeric Profile;
* a controller interface requires a parameter prohibited by Governance;
* two mandatory constraints prescribe mutually exclusive configurations.

CONFLICTED SHALL be used only when the contradiction is explicit and reproducible.

It SHALL NOT be used merely because one requirement failed.

The Compatibility Result SHALL identify every conflicting requirement and the evidence establishing the contradiction.

---

## 42.5 UNRESOLVED

`UNRESOLVED` means Compatibility could not be determined from the committed evidence.

Examples include:

* missing compatibility metadata;
* unavailable interface declarations;
* indeterminate operating-region mapping;
* unresolved registry dependency;
* contradictory evidence whose authority precedence cannot be determined;
* incomplete Reference Builder capability evidence.

UNRESOLVED SHALL NOT be treated as compatible.

A candidate with an UNRESOLVED Compatibility result SHALL NOT proceed to Selection.

---

# 43. Compatibility Evaluation Domain

Compatibility Evaluation SHALL evaluate semantic and structural coherence only.

It MAY evaluate:

* Objective class support;
* required target-dimension support;
* OSV-dimension coverage;
* operating-region intersection;
* Governance-bounded permission resolution;
* constraint coherence;
* Reference Builder constructability;
* Numeric Profile compatibility;
* Controller Interface compatibility;
* capability representation;
* restriction representability;
* transition-context coherence where authorized.

Compatibility Evaluation SHALL NOT evaluate:

* candidate preference;
* comparative score;
* profile priority;
* tie-breaking order;
* expected control performance;
* predicted execution outcome;
* future Objective quality;
* runtime command effectiveness.

Those belong to later contracts or the Selection phase.

---

# 44. Reference Constructability

Every eligible candidate SHALL undergo a Reference Constructability evaluation.

Its defining question is:

> **Can the declared Reference Builder construct a complete and valid Reference State from the committed Objective Set, OSV, Governance Snapshot, and candidate Profile Definition?**

Reference Constructability SHALL verify:

* every required target dimension can be represented;
* every required Objective can be projected into an authorized target or region;
* every target is bounded by Governance;
* required operating regions are nonempty;
* tolerances can be represented;
* required constraints can be materialized;
* numeric values can be encoded by the selected Numeric Profile;
* no required target must be invented by the Controller.

Reference Constructability SHALL NOT produce the final Reference State.

It SHALL determine only whether valid construction is possible.

The result SHALL resolve to:

```text
CONSTRUCTABLE

CONSTRUCTABLE_WITH_RESTRICTIONS

NOT_CONSTRUCTABLE

CONFLICTED

UNRESOLVED
```

This result SHALL contribute to, but SHALL NOT independently replace, the overall Compatibility outcome.

---

# 45. Operating Region Compatibility

An Operational Profile Definition MAY declare one or more operating regions.

Compatibility Evaluation SHALL determine whether those regions can coherently represent the committed context.

For each required OSV or Reference State dimension, Compatibility Evaluation SHALL determine:

* whether the dimension is represented;
* whether the region is defined;
* whether Governance permits the region;
* whether Objective requirements intersect the region;
* whether compatibility restrictions reduce the region;
* whether the final region remains nonempty.

Let:

```text
R_g = region permitted by Governance

R_p = region declared by the Profile Definition

R_o = region required by the Objective Set

R_c = region imposed by Compatibility restrictions
```

The effective operating region SHALL be derived as:

```text
R_effective
=
R_g
∩
R_p
∩
R_o
∩
R_c
```

Where Objective requirements are declarative rather than region-based, the Reference Builder compatibility rules SHALL determine the corresponding authorized target domain.

If:

```text
R_effective = ∅
```

the candidate SHALL resolve as:

```text
CONFLICTED
```

when the empty intersection results from mutually contradictory valid requirements.

It SHALL resolve as:

```text
INCOMPATIBLE
```

when the Profile Definition simply lacks the required representational coverage.

---

# 46. Permission Compatibility

Compatibility Evaluation SHALL resolve whether Profile permissions can be represented within committed Governance authority.

Let:

```text
A_g = authority granted by Governance

A_p = authority permitted by the Profile Definition

A_r = authority remaining after Compatibility restrictions
```

The effective authority SHALL satisfy:

```text
A_effective
≼
A_g
```

and:

```text
A_effective
≼
A_p
```

where:

```text
≼
```

means equal or more restrictive under the authority ordering defined by Contract 004.

Conceptually:

```text
A_effective
=
Restrict(
    A_g,
    A_p,
    Compatibility Restrictions
)
```

Compatibility SHALL NOT create permissions absent from Governance.

Compatibility SHALL NOT create permissions absent from the Profile Definition unless the Profile Definition explicitly delegates a bounded choice authorized by Governance.

Any contradiction that cannot be resolved through monotonic restriction SHALL produce:

```text
CONFLICTED
```

or:

```text
INCOMPATIBLE
```

according to the cause.

---

# 47. Constraint Compatibility

Constraints SHALL accumulate toward equal or greater restriction.

Let:

```text
C_g = Governance constraints

C_p = Profile Definition constraints

C_o = Objective-derived construction requirements

C_r = Compatibility restriction additions
```

The effective constraint set SHALL be derived through the canonical constraint-composition operation defined by Contract 004 and Contract 006.

Conceptually:

```text
C_effective
=
C_g
⊕
C_p
⊕
C_o
⊕
C_r
```

where:

```text
⊕
```

means deterministic constraint accumulation without authority expansion.

Constraint composition SHALL verify:

* no mandatory constraint is discarded;
* no weaker constraint overrides a stronger authoritative constraint;
* precedence is determined by registered authority rules;
* contradictory constraints are reported explicitly;
* unresolved precedence does not default toward greater authority.

When constraints contradict and no authorized resolution exists, Compatibility SHALL resolve as:

```text
CONFLICTED
```

---

# 48. Gain Profile Compatibility

Contract 006 owns selection of a Gain Profile identifier.

It does not own the mathematical meaning of controller gains.

Compatibility Evaluation SHALL determine whether:

* the Profile Definition references a valid Gain Profile;
* the Gain Profile is available in the committed registry snapshot;
* the Gain Profile is permitted by Governance;
* the Controller Interface Profile supports the Gain Profile type and version;
* Compatibility restrictions require a more restrictive registered Gain Profile;
* deterministic resolution is possible.

Compatibility Evaluation SHALL NOT:

* calculate control gains;
* modify gain equations;
* estimate control performance;
* dynamically tune gains;
* create an unregistered Gain Profile.

If the declared Gain Profile cannot be used but a deterministic, registered, more restrictive substitute is authorized by policy, the candidate MAY resolve as:

```text
COMPATIBLE_WITH_RESTRICTIONS
```

The substitute Gain Profile identifier and substitution rule SHALL be recorded explicitly.

If no authorized resolution exists, the candidate SHALL resolve as:

```text
INCOMPATIBLE
```

or:

```text
CONFLICTED
```

according to the cause.

---

# 49. Numeric Profile Compatibility

Compatibility Evaluation SHALL verify that the candidate’s declared Numeric Profile can represent every value required for Committed Profile and Reference State construction.

The evaluation SHALL include:

* numeric type support;
* precision requirements;
* range support;
* saturation semantics;
* unit representation;
* missing-value prohibition;
* uncertainty representation;
* tolerance representation;
* canonical serialization;
* deterministic hashing compatibility.

Compatibility Evaluation SHALL NOT perform numerical control.

It SHALL determine only whether the selected numeric representation is sufficient and coherent.

Failure of representational sufficiency SHALL resolve as:

```text
INCOMPATIBLE
```

A contradiction between mandatory numeric requirements SHALL resolve as:

```text
CONFLICTED
```

Unavailable numeric evidence SHALL resolve as:

```text
UNRESOLVED
```

---

# 50. Controller Interface Compatibility

Compatibility Evaluation SHALL verify that the configuration which would result from the candidate can be consumed by the authorized Controller interface.

This evaluation MAY verify:

* required input dimensions;
* required target dimensions;
* accepted Numeric Profile;
* accepted Gain Profile type;
* constraint representation;
* tolerance representation;
* operating-region representation;
* schema compatibility;
* version compatibility.

It SHALL NOT:

* execute the Controller;
* evaluate control output;
* estimate corrective success;
* compare control laws;
* select a Controller implementation.

Controller Interface compatibility answers only:

> **Can this complete supervisory configuration be represented through the committed Controller interface?**

---

# 51. Compatibility Restrictions

Compatibility restrictions are deterministic, explicit reductions applied to make an otherwise valid candidate coherent with the committed context.

Every Compatibility restriction SHALL include:

```text
restriction_id

restriction_type

source_authority

source_artifact_id

rule_id

affected_field

original_value_or_domain

restricted_value_or_domain

restriction_reason

restriction_precedence

restriction_hash
```

Restrictions MAY:

* reduce permissions;
* narrow operating regions;
* add constraints;
* reduce resource ceilings;
* require review;
* substitute a registered more restrictive Gain Profile;
* reduce execution disposition;
* shorten temporal validity;
* require additional Controller-side safety conditions already authorized by Contract 007.

Restrictions SHALL NOT:

* expand permissions;
* weaken Governance;
* remove mandatory constraints;
* broaden operating regions;
* introduce new Objectives;
* alter Objective priority;
* authorize unsupported capability;
* bypass Eligibility failure;
* create Controller commands.

---

# 52. Restriction Derivation

Restrictions SHALL be derived only from committed and versioned authority sources.

Valid sources MAY include:

* Governance Snapshot;
* Profile Definition;
* Objective Set construction requirements;
* Compatibility policies;
* Controller Interface Profile;
* Numeric Profile rules;
* Reference Builder capability rules;
* transition-context restrictions authorized for Contract 006.

No restriction may originate from:

* undocumented implementation preference;
* mutable runtime state;
* hidden Controller state;
* uncommitted registry content;
* predicted performance;
* random choice.

Every derived restriction SHALL be included in Compatibility evidence and later materialized in the Committed Operational Profile.

---

# 53. Compatibility Coverage Invariant

Compatibility Evaluation SHALL satisfy:

> **Every ELIGIBLE candidate receives exactly one Compatibility Result, and no noneligible or undiscovered candidate receives one.**

Formally:

```text
Domain(Compatibility Results)
=
{
    d ∈ Eligibility Results
    |
    eligibility_outcome(d) = ELIGIBLE
}
```

For every eligible candidate `d`:

```text
count(compatibility_result(d)) = 1
```

No eligible candidate may be:

* silently omitted;
* evaluated more than once;
* merged with another candidate;
* replaced by a different definition;
* admitted directly to Selection without Compatibility evidence.

---

# 54. Compatibility Determinism

Compatibility Evaluation SHALL be deterministic.

Given identical:

* Eligibility Evaluation Report;
* Committed OSV;
* Committed Governance Snapshot;
* Committed Objective Set;
* Controller Interface Profile;
* Reference Builder Capability Declaration;
* Numeric Profile Compatibility Declaration;
* Compatibility policy versions;
* registry dependencies;

Compatibility Evaluation SHALL produce:

* identical Compatibility outcomes;
* identical constructability results;
* identical restrictions;
* identical effective authority ceilings;
* identical effective constraint additions;
* identical operating-region projections;
* identical Gain Profile resolutions;
* identical canonical artifact hash.

Compatibility SHALL NOT depend upon:

* noncommitted Controller state;
* live registry publication;
* hidden implementation preference;
* randomized evaluation;
* undocumented fallback behavior.

---

# 55. Compatibility Failure Classification

Compatibility failures SHALL be classified according to cause.

Use:

```text
INCOMPATIBLE
```

when the candidate cannot represent or construct the required supervisory configuration.

Use:

```text
CONFLICTED
```

when two or more authoritative requirements are individually valid but mutually contradictory.

Use:

```text
UNRESOLVED
```

when committed evidence is insufficient to determine compatibility.

Implementations SHALL NOT substitute one failure class for another merely to simplify processing.

The evidence report SHALL make the classification reproducible.

---

# 56. Empty Compatibility Outcome

Compatibility Evaluation MAY produce no candidate with:

```text
COMPATIBLE
```

or:

```text
COMPATIBLE_WITH_RESTRICTIONS
```

When this occurs, the Compatibility Evaluation Report SHALL record:

```text
NO_COMPATIBLE_PROFILE
```

This outcome SHALL NOT cause Contract 006 to:

* invent a new Profile Definition;
* bypass Compatibility;
* select an INCOMPATIBLE candidate;
* select a CONFLICTED candidate;
* select an UNRESOLVED candidate;
* reuse a previous cycle’s Committed Operational Profile;
* broaden Governance authority;
* alter the Objective Set.

Any fallback Profile Definition SHALL have independently passed:

* Discovery;
* Eligibility;
* Compatibility.

---

# 57. Compatibility Evidence Preservation

Compatibility Evaluation SHALL append evidence to the existing Contract 006 evidence chain.

It SHALL preserve:

* Candidate Profile Set identity;
* Eligibility Report identity;
* candidate identity;
* Compatibility requirement evidence;
* constructability evidence;
* permission resolution;
* constraint resolution;
* operating-region resolution;
* Gain Profile resolution;
* Numeric Profile resolution;
* Controller Interface resolution;
* restrictions;
* failure reasons;
* rule versions;
* provenance.

Compatibility Evaluation SHALL NOT rewrite:

* Registry evidence;
* Discovery evidence;
* Eligibility evidence.

---

# 58. Compatibility Phase Invariants

Compatibility Evaluation SHALL satisfy all of the following invariants.

## 58.1 Eligible-Domain Closure

Only ELIGIBLE candidates may receive Compatibility Results.

## 58.2 Complete Compatibility Evaluation

Every ELIGIBLE candidate receives exactly one result.

## 58.3 Constructability Purity

Compatibility determines coherent construction, not preference.

## 58.4 Restriction Materialization

Every required restriction is explicit, versioned, attributable, and reproducible.

## 58.5 Authority Monotonicity

Compatibility may preserve or reduce authority.

It may never expand authority.

## 58.6 Constraint Preservation

No mandatory authoritative constraint may be silently discarded.

## 58.7 Conflict Explicitness

Contradictory valid requirements resolve as CONFLICTED with recorded evidence.

## 58.8 Unresolved Conservatism

UNRESOLVED does not proceed to Selection.

## 58.9 Interface Nonexecution

Compatibility may verify interface representation but SHALL NOT execute the Controller.

## 58.10 Append-Only Evidence

Compatibility extends the evidence chain without rewriting earlier stages.

---

# 59. Compatibility Output Boundary

Only candidates with:

```text
COMPATIBLE
```

or:

```text
COMPATIBLE_WITH_RESTRICTIONS
```

may proceed to deterministic Profile Selection.

Compatibility SHALL NOT:

* rank those candidates;
* compare their desirability;
* assign final preference;
* choose a winner;
* commit a Profile;
* construct the final Reference State.

Its final authority is limited to determining:

1. whether coherent configuration construction is possible; and
2. which explicit restrictions are required.

The next Contract 006 stage SHALL compare only compatible candidates and produce a deterministic, fully replayable Selection Decision.
