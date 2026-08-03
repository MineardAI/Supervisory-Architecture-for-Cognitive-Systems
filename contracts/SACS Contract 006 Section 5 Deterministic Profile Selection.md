# 60. Deterministic Profile Selection

Deterministic Profile Selection is the third substantive evaluation stage of Contract 006.

Its defining identity is:

> **Profile Selection determines which compatible Operational Profile Definition shall be committed for the current supervisory cycle.**

Selection compares only candidates whose Compatibility outcome is:

```text
COMPATIBLE

COMPATIBLE_WITH_RESTRICTIONS
```

Selection SHALL NOT reconsider:

* discovery;
* eligibility;
* compatibility;
* Governance authority;
* Objective validity;
* Reference State constructability;
* Controller interface support.

Those determinations are already committed as evidence.

Selection SHALL compare only valid compatible candidates under registered selection policies.

---

# 61. Selection Inputs

Deterministic Profile Selection SHALL consume:

```text
Compatibility Evaluation Report

Committed OSV

Committed Governance Snapshot

Committed Objective Set

Previous Cycle Selection Context

Profile Selection Policy

Profile Priority Registry

Tie-Break Policy

Persistence Policy

Selection Numeric Profile
```

The Previous Cycle Selection Context MAY include:

```text
previous_cycle_id

previous_committed_profile_id

previous_profile_id
previous_profile_version

previous_selection_report_id

previous_transition_reason

profile_dwell_metadata

profile_persistence_metadata
```

Previous-cycle data SHALL influence Selection only through registered selection and persistence rules.

A previous Committed Operational Profile SHALL NOT be reused as the current cycle’s commitment.

---

# 62. Selection Candidate Domain

The Selection Candidate Domain SHALL contain exactly those candidates whose Compatibility outcome is:

```text
COMPATIBLE

COMPATIBLE_WITH_RESTRICTIONS
```

Formally:

```text
SelectionCandidates
=
{
    d ∈ CompatibilityResults
    |
    outcome(d) ∈
    {
        COMPATIBLE,
        COMPATIBLE_WITH_RESTRICTIONS
    }
}
```

No candidate absent from the Compatibility Evaluation Report may enter Selection.

No candidate with an outcome of:

```text
INCOMPATIBLE

CONFLICTED

UNRESOLVED
```

may enter Selection.

Selection SHALL NOT alter the candidate domain after construction.

---

# 63. Profile Selection Policy

A Profile Selection Policy is a versioned, immutable policy that defines how compatible candidates are compared.

Its defining identity is:

> **A Profile Selection Policy defines deterministic comparison rules among already compatible candidates. It does not create eligibility or compatibility.**

Every Profile Selection Policy SHALL include:

```text
selection_policy_id

selection_policy_version

selection_dimensions

dimension_precedence

dimension_weights

score_normalization_rules

restriction_penalty_rules

persistence_rules

authority_minimization_rules

priority_resolution_rules

tie_break_policy_id

numeric_profile_id

policy_schema_version

policy_hash
```

The Selection Policy SHALL be committed before Selection begins.

Selection SHALL NOT use implementation-local comparison rules absent from the committed Selection Policy.

---

# 64. Selection Dimensions

Selection MAY compare compatible candidates across registered dimensions such as:

```text
objective_alignment

governance_conservatism

authority_expansion_risk

restriction_burden

operating_region_fit

reference_construction_margin

profile_priority

transition_cost

persistence_preference

resource demand

uncertainty suitability

replay suitability
```

Every dimension SHALL have:

```text
dimension_id

dimension_definition

input_artifacts

evaluation_rule_id

numeric_domain

normalization_rule

precedence

weight

direction

missing_evidence_rule
```

Direction SHALL be one of:

```text
MAXIMIZE

MINIMIZE

LEXICOGRAPHIC

BOOLEAN_GATE
```

Selection dimensions SHALL NOT evaluate:

* expected Controller output;
* predicted execution success;
* future model behavior;
* uncommitted telemetry;
* hidden implementation state;
* subjective preference;
* random utility.

---

# 65. Selection Scoring

When a Selection Policy uses numerical scoring, each compatible candidate SHALL receive one deterministic Selection Score.

Every Selection Score SHALL include:

```text
registry_entry_id

profile_id
profile_version
definition_hash

dimension_values

normalized_dimension_values

dimension_weights

restriction_adjustments

persistence_adjustments

priority_adjustments

aggregate_score

score_numeric_profile_id

score_rule_versions

score_provenance_id
```

The aggregate score SHALL be computed only from committed inputs and registered rules.

Conceptually:

```text
Score(p)
=
Σ
w_i × N_i(p)
+
PriorityAdjustment(p)
+
PersistenceAdjustment(p)
-
RestrictionPenalty(p)
```

where:

```text
w_i
```

is the registered weight for dimension `i`, and:

```text
N_i(p)
```

is the canonically normalized value for candidate `p`.

This formula is illustrative unless adopted as the declared Selection Policy.

Contract 006 SHALL support lexicographic policies that do not require aggregate weighted scoring.

---

# 66. Selection Scoring Boundary

Selection scoring SHALL compare compatible configurations.

It SHALL NOT:

* redefine Objectives;
* generate target values;
* alter Compatibility restrictions;
* compute Controller gains;
* predict control performance;
* optimize future supervisory trajectories;
* perform planning;
* create new Profile Definitions.

Selection scoring answers:

> **Which already valid configuration is preferred under the committed Selection Policy?**

It does not answer:

> “Which configuration will produce the best future behavior?”

---

# 67. Objective Alignment

Objective Alignment MAY measure how directly a candidate supports the classes, priorities, and declared requirements of the committed Objective Set.

Objective Alignment SHALL use only Objective metadata explicitly authorized for selection.

It SHALL NOT:

* modify Objective priorities;
* reinterpret Objective meaning;
* introduce new Objectives;
* estimate future Objective satisfaction;
* convert Objectives into Controller targets.

Where Objective Alignment cannot be determined from committed metadata, the registered missing-evidence rule SHALL apply.

A candidate SHALL NOT receive fabricated alignment evidence.

---

# 68. Governance Conservatism

Selection MAY prefer a candidate that preserves a narrower or more restrictive authority envelope.

Governance Conservatism MAY consider:

* effective permissions;
* effective resource ceilings;
* effective execution dispositions;
* review requirements;
* delegation limits;
* operating-region width;
* temporal validity.

A more permissive candidate SHALL NOT receive preference merely because it permits broader action.

Where two candidates are otherwise equivalent, the more restrictive governance-compatible candidate SHOULD prevail according to the committed tie-break policy.

---

# 69. Restriction Burden

A candidate with:

```text
COMPATIBLE_WITH_RESTRICTIONS
```

MAY receive a deterministic restriction burden value.

Restriction burden MAY reflect:

* number of restrictions;
* authority dimensions affected;
* operating-region reduction;
* required Gain Profile substitution;
* added review obligations;
* reduced temporal validity;
* additional constraints.

Restriction burden SHALL NOT imply that unrestricted candidates always prevail.

A restricted candidate MAY still rank higher when:

* its Objective alignment is stronger;
* its profile priority is higher;
* its uncertainty suitability is superior;
* the Selection Policy explicitly permits it.

All restriction penalties SHALL be versioned and reproducible.

---

# 70. Profile Priority

Operational Profile Definitions MAY declare a registered Profile Priority.

Priority SHALL be interpreted only through the committed Profile Priority Registry and Selection Policy.

Every Profile Priority record SHALL include:

```text
profile_id
profile_version

priority_class
priority_value

applicable_context_class

priority_rule_id

priority_registry_version

priority_hash
```

Profile Priority SHALL NOT:

* bypass Eligibility;
* bypass Compatibility;
* override Governance;
* create authority;
* guarantee Selection.

Priority applies only among candidates already admitted to Selection.

---

# 71. Profile Persistence

Profile persistence MAY influence preference for reselecting the same Profile Definition in a subsequent cycle.

Persistence SHALL mean:

> **A registered preference for continuity of profile definition across cycles when continued use remains independently eligible, compatible, and preferred under policy.**

Persistence SHALL NOT:

* extend a prior Committed Profile into the current cycle;
* bypass current-cycle Discovery;
* bypass current-cycle Eligibility;
* bypass current-cycle Compatibility;
* preserve obsolete restrictions without reevaluation;
* prevent selection of a safer candidate;
* override Governance changes.

A Profile Definition selected again in a later cycle SHALL produce:

* a new Selection Report;
* a new Committed Operational Profile;
* a new Reference State;
* new artifact hashes.

---

# 72. Transition Cost

Selection MAY consider deterministic transition cost between:

* the previous cycle’s selected Profile Definition; and
* each current compatible candidate.

Transition cost MAY consider:

* required configuration changes;
* operating-region discontinuity;
* Gain Profile change;
* permission reduction;
* new review requirements;
* profile dwell policy;
* state continuity requirements;
* Reference Builder change.

Transition cost SHALL NOT include:

* Controller output simulation;
* predicted execution response;
* speculative future behavior;
* hidden runtime state.

Transition cost SHALL be derived from registered profile transition metadata only.

---

# 73. Authority Minimization

Selection SHALL preserve Authority Monotonicity.

Where candidates are otherwise equivalent, Selection SHALL prefer the candidate that grants the least effective authority necessary to support the committed Objective Set.

This establishes the principle of Authority Minimization:

> **Selection shall not prefer broader supervisory authority when an equally valid, more restrictive configuration is available.**

Authority Minimization SHALL be evaluated under the authority ordering defined by Contract 004.

It SHALL NOT reduce authority below the level required to construct a valid supervisory configuration.

---

# 74. Selection Result States

Deterministic Profile Selection SHALL resolve to exactly one of:

```text
PROFILE_SELECTED

NO_SELECTION_CANDIDATES

NO_DETERMINATE_WINNER

SELECTION_POLICY_UNRESOLVED

SELECTION_CONFLICT
```

---

## 74.1 PROFILE_SELECTED

`PROFILE_SELECTED` means exactly one compatible candidate prevailed after scoring, priority resolution, and deterministic tie-breaking.

---

## 74.2 NO_SELECTION_CANDIDATES

`NO_SELECTION_CANDIDATES` means the Compatibility Evaluation Report contained no candidate with:

```text
COMPATIBLE

COMPATIBLE_WITH_RESTRICTIONS
```

This result SHALL be consistent with:

```text
NO_COMPATIBLE_PROFILE
```

when that earlier outcome was recorded.

---

## 74.3 NO_DETERMINATE_WINNER

`NO_DETERMINATE_WINNER` means multiple candidates remained indistinguishable after every authorized tie-break rule was exhausted.

A conforming production policy SHOULD prevent this condition through a final canonical fallback rule.

Where the committed policy permits this outcome, Contract 006 SHALL NOT choose arbitrarily.

---

## 74.4 SELECTION_POLICY_UNRESOLVED

`SELECTION_POLICY_UNRESOLVED` means the committed Selection Policy could not be executed because required policy evidence, versioning, numeric support, or rule resolution was unavailable.

This outcome SHALL NOT be treated as Profile Selection.

---

## 74.5 SELECTION_CONFLICT

`SELECTION_CONFLICT` means two or more authoritative selection rules produced contradictory mandatory outcomes and registered precedence could not resolve them.

The conflicting rules SHALL be identified explicitly.

---

# 75. Deterministic Tie-Breaking

Tie-breaking SHALL occur only after all primary Selection Policy comparisons are complete.

The canonical tie-break hierarchy SHALL be:

1. higher mandatory Profile Priority;
2. stronger satisfaction of higher-priority Objectives;
3. lower effective authority;
4. stronger Governance conservatism;
5. lower unresolved restriction burden;
6. narrower valid operating region;
7. lower transition cost;
8. persistence preference;
9. lower resource demand;
10. canonical profile identity order.

The final canonical profile identity order SHALL be:

```text
profile_id

profile_version

definition_hash

registry_entry_id
```

The committed Selection Policy MAY define a stricter domain-specific hierarchy.

It SHALL NOT omit a deterministic final fallback unless `NO_DETERMINATE_WINNER` is an explicitly permitted policy outcome.

---

# 76. Tie-Break Restrictions

Tie-breaking SHALL NOT:

* introduce a new candidate;
* change Eligibility;
* change Compatibility;
* remove Compatibility restrictions;
* broaden candidate authority;
* alter Objective priority;
* inspect Controller outputs;
* use publication time unless explicitly registered;
* use nondeterministic iteration order;
* use implementation-local registry order.

Registry order SHALL NOT serve as an implicit tie-break unless that order is itself canonical, committed, and identified by rule.

---

# 77. Profile Selection Report

Deterministic Profile Selection SHALL produce one immutable Profile Selection Report.

Its defining identity is:

> **The Profile Selection Report records the complete deterministic comparison and final selection outcome for every candidate admitted to Selection.**

Every Profile Selection Report SHALL include:

```text
selection_report_id

cycle_id

candidate_set_id
eligibility_report_id
compatibility_report_id

osv_id
governance_snapshot_id
objective_set_id

selection_policy_id
selection_policy_version

profile_priority_registry_version

tie_break_policy_id
persistence_policy_id

selection_numeric_profile_id

selection_candidates

selection_result

selected_candidate

selection_trace

selection_report_schema_version
contract_version

selection_provenance_id

created_at

artifact_hash
```

The Profile Selection Report SHALL be immutable after commitment.

---

# 78. Selection Candidate Record

Every candidate admitted to Selection SHALL have one Selection Candidate Record.

Each record SHALL include:

```text
registry_entry_id

profile_id
profile_version
definition_hash

compatibility_outcome

effective_permissions

effective_constraints

effective_operating_region_projection

effective_gain_profile_resolution

compatibility_restrictions

selection_dimension_values

selection_score

profile_priority

persistence_adjustment

transition_cost

restriction_burden

authority_ordering_result

rank

tie_group_id

elimination_stage

elimination_reason

candidate_selection_provenance_id
```

A candidate that loses Selection SHALL remain visible.

No Selection Candidate Record may be deleted from the report because it did not win.

---

# 79. Selected Candidate Record

When:

```text
selection_result = PROFILE_SELECTED
```

the `selected_candidate` field SHALL include:

```text
registry_entry_id

profile_id
profile_version
definition_hash

final_rank

final_score

selection_reason

applied_tie_break_rules

effective_permissions

effective_constraints

effective_operating_region_projection

effective_gain_profile_resolution

compatibility_restrictions

selection_evidence_hash
```

The selected candidate record SHALL contain all information required to construct the Committed Operational Profile without re-executing Selection.

---

# 80. Selection Trace

The Selection Trace SHALL record the deterministic comparison sequence.

It SHALL include:

```text
initial_candidate_domain

applied_selection_dimensions

normalized_values

score_calculations

priority_resolutions

restriction_adjustments

persistence_adjustments

transition_adjustments

tie_groups

applied_tie_break_rules

eliminated_candidates

final_candidate

terminal_selection_result
```

Every elimination SHALL identify:

```text
candidate_identity

elimination_stage

elimination_rule_id

comparison_basis

winning_candidate_or_rule

evidence_reference
```

The Selection Trace SHALL permit an independent implementation to reproduce the final result.

---

# 81. Selection Determinism

Profile Selection SHALL be deterministic.

Given identical:

* Compatibility Evaluation Report;
* OSV;
* Governance Snapshot;
* Objective Set;
* Previous Cycle Selection Context;
* Profile Selection Policy;
* Profile Priority Registry;
* Tie-Break Policy;
* Persistence Policy;
* Selection Numeric Profile;

Profile Selection SHALL produce:

* identical candidate scores;
* identical candidate ranks;
* identical tie groups;
* identical tie-break results;
* identical selected candidate;
* identical Selection Result;
* identical semantic artifact hash.

Selection SHALL NOT depend upon:

* random values;
* evaluation order;
* thread scheduling;
* registry iteration order;
* mutable local state;
* undocumented heuristics;
* operator preference;
* Controller output.

---

# 82. No-Winner Handling

When Selection does not produce:

```text
PROFILE_SELECTED
```

Contract 006 SHALL NOT:

* invent a default candidate;
* select the highest incomplete score;
* select a candidate with unresolved evidence;
* reuse the prior cycle’s Committed Profile;
* bypass the Selection Policy;
* broaden Governance authority;
* construct a normal Reference State.

The Profile Selection Report SHALL record the terminal no-winner outcome and complete evidence.

Any fallback mechanism SHALL operate through a registered Profile Definition that independently passed:

* Discovery;
* Eligibility;
* Compatibility;
* Selection.

Fallback SHALL NOT mean selecting a failed candidate.

---

# 83. Selection Failure Toward Reduced Authority

Unresolved or conflicted Selection SHALL fail toward reduced authority.

The registered failure policy MAY require:

* no Committed Operational Profile;
* a separately eligible and compatible fail-safe profile;
* human review recognized by Governance;
* transition to a later Contract 010 mode;
* execution restriction;
* supervisory hold.

Contract 006 SHALL NOT define mode transition semantics.

It may only emit the registered Selection failure outcome for downstream handling.

---

# 84. Selection Evidence Preservation

The Profile Selection Report SHALL append evidence to the Contract 006 evidence chain.

It SHALL preserve:

* Candidate Profile Set identity;
* Eligibility Report identity;
* Compatibility Report identity;
* all admitted Selection candidates;
* all Selection dimensions;
* all scores;
* all priority values;
* all restrictions;
* all tie groups;
* every tie-break decision;
* the terminal Selection Result;
* the selected candidate, if any;
* all policy and registry versions;
* provenance.

Selection SHALL NOT rewrite:

* Discovery evidence;
* Eligibility evidence;
* Compatibility evidence.

---

# 85. Selection Phase Invariants

Profile Selection SHALL satisfy all of the following invariants.

## 85.1 Compatible-Domain Closure

Only compatible candidates may enter Selection.

## 85.2 Complete Candidate Accounting

Every candidate admitted to Selection appears exactly once in the Selection Report.

## 85.3 Preference Purity

Selection compares valid candidates but does not redefine their validity.

## 85.4 Deterministic Scoring

Identical committed inputs produce identical scores.

## 85.5 Deterministic Tie-Breaking

Every tie is resolved through committed rules or results explicitly in `NO_DETERMINATE_WINNER`.

## 85.6 Authority Minimization

Selection SHALL NOT prefer broader authority where an equally valid more restrictive candidate exists.

## 85.7 Restriction Preservation

Compatibility restrictions SHALL remain attached to the selected candidate.

## 85.8 No Silent Default

Selection failure SHALL NOT produce an undeclared default profile.

## 85.9 No Prior-Commitment Reuse

A previous cycle’s Committed Profile SHALL NOT become the current commitment.

## 85.10 Append-Only Evidence

Selection extends the evidence chain without rewriting prior stages.

---

# 86. Selection Output Boundary

The Profile Selection Report is the final decision artifact before profile commitment.

Selection SHALL NOT:

* create the Committed Operational Profile;
* construct the Reference State;
* compute target values;
* execute the Reference Builder;
* compute Controller outputs;
* issue inference commands.

Its authority ends when it identifies:

1. the terminal Selection Result; and
2. the selected candidate and all effective restrictions, when a candidate is selected.

The next Contract 006 stage SHALL transform the selected candidate evidence into one immutable Committed Operational Profile.
