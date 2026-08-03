# SACS Artifact And Interface Registry

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012  
Classification: Constitutional-supporting artifact registry

## Registry Principles

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0

Artifacts are listed here as constitutional interfaces, not implementation objects. A registry entry distinguishes:

- artifact class;
- owning contract;
- producer;
- canonical status;
- whether the artifact is reusable or cycle-bound;
- whether it is committed or proposed;
- who may consume it;
- whether it carries authority or merely evidence.

If a term changed across corpus generations, the canonical term is listed first and deprecated aliases follow.

## v1.0.0 Artifact Freeze

Registered artifact identities are frozen for SACS v1.0.0. No implementation, schema, protocol, or derived document may create a new canonical artifact identity, rename an existing canonical artifact, or relocate its owning authority without a later constitutional version. This phase does not perform the full artifact uniqueness audit; suspected duplicates or contradictions remain Phase 2 findings.

## Artifact Registry

### Observation Batch

Artifact Name: Observation Batch  
Artifact Class: Observation input bundle  
Owning Contract: 001  
Producer: Authorized source or adapter  
Canonical or Noncanonical: Canonical when admitted by Contract 001  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed until observation handoff is accepted  
Consumed By: 002  
Purpose: Group runtime evidence for one observation window  
Required Identity Bindings: source id, timestamps, ordering, observation window id, schema version  
Authority Carried: Evidence only  
Authority Not Carried: No supervisory authority  
Immutability Rule: Observation records become immutable once admitted  
Failure Relationship: invalid, unavailable, stale, or unsupported observation

### Estimated State

Artifact Name: Estimated State  
Artifact Class: Estimation output  
Owning Contract: 002  
Producer: State Estimation  
Canonical or Noncanonical: Canonical only when published according to Contract 002  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before OSV publication  
Consumed By: 003  
Purpose: Represent inferred state from committed observations  
Required Identity Bindings: observation lineage, estimator id, version, schema version  
Authority Carried: No direct governance authority  
Authority Not Carried: No control or execution authority  
Immutability Rule: Immutable after publication  
Failure Relationship: explicit estimation failure or unresolved inference

### Committed Operational State Vector

Artifact Name: Committed Operational State Vector  
Artifact Class: Canonical supervisory state  
Owning Contract: 003  
Producer: Operational State Vector publication  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 004, 005, 006, 007  
Purpose: Freeze the cycle-local supervisory state for downstream contracts  
Required Identity Bindings: cycle id, source observation lineage, schema version, hash  
Authority Carried: State authority only  
Authority Not Carried: No governance or execution authority  
Immutability Rule: Cannot be modified in place  
Failure Relationship: partial publication isolation or publication failure

### Committed Governance Snapshot

Artifact Name: Committed Governance Snapshot  
Artifact Class: Canonical governance state  
Owning Contract: 004  
Producer: Governance authority  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound or reusable only when explicitly declared  
Committed or Proposed: Committed  
Consumed By: 005, 006, 007, 008  
Purpose: Capture the governing constraints for the cycle  
Required Identity Bindings: governance source, disposition lineage, version, hash  
Authority Carried: Restriction and permission authority  
Authority Not Carried: No control authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: withheld governance or unresolved disposition

### Committed Objective Set

Artifact Name: Committed Objective Set  
Artifact Class: Canonical objective output  
Owning Contract: 005  
Producer: Supervisory Objectives  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 006, 007  
Purpose: State the approved supervisory objectives for the cycle  
Required Identity Bindings: objective set id, governance snapshot id, priority ordering, schema version  
Authority Carried: Objective authority only  
Authority Not Carried: No permission expansion  
Immutability Rule: Immutable after commitment  
Failure Relationship: objective evaluation failure or restricted objective set

### Operational Profile Definition

Artifact Name: Operational Profile Definition  
Artifact Class: Reusable definition  
Owning Contract: 006  
Producer: Profile authoring / registry publication  
Canonical or Noncanonical: Canonical if published to the registry  
Reusable or Cycle-Bound: Reusable  
Committed or Proposed: Proposed until published as a registry entry  
Consumed By: 006  
Purpose: Define a supervisory configuration candidate  
Required Identity Bindings: profile id, version, hash, capability declaration, constraints  
Authority Carried: Definition authority only  
Authority Not Carried: No cycle authority until committed  
Immutability Rule: Definition contents are immutable once published  
Failure Relationship: registry rejection or incompatibility

### Profile Registry Snapshot

Artifact Name: Profile Registry Snapshot  
Artifact Class: Registry snapshot  
Owning Contract: 006  
Producer: Registry snapshot publication  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Discovery, eligibility, compatibility, selection  
Purpose: Freeze the visible registry universe for one cycle  
Required Identity Bindings: registry id, registry version, ordered entry ids, snapshot hash  
Authority Carried: Discovery scope only  
Authority Not Carried: No selection authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: empty, incomplete, or incompatible snapshot

### Candidate Profile Set

Artifact Name: Candidate Profile Set  
Artifact Class: Selection input set  
Owning Contract: 006  
Producer: Candidate discovery  
Canonical or Noncanonical: Canonical within the cycle  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed until selection completes  
Consumed By: 006  
Purpose: List visible profiles that may be evaluated  
Required Identity Bindings: candidate id, registry entry id, profile version, hash  
Authority Carried: None beyond visibility  
Authority Not Carried: No eligibility or selection authority  
Immutability Rule: Candidate membership is fixed for the cycle  
Failure Relationship: empty candidate set

### Eligibility Evaluation Report

Artifact Name: Eligibility Evaluation Report  
Artifact Class: Evaluation report  
Owning Contract: 006  
Producer: Eligibility evaluator  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before selection consumes it  
Consumed By: 006  
Purpose: State which candidates are eligible, ineligible, unsupported, prohibited, or unresolved  
Required Identity Bindings: candidate set id, registry snapshot id, rule versions, hash  
Authority Carried: Diagnostic authority only  
Authority Not Carried: No selection authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: empty or unresolved evaluation

### Compatibility Evaluation Report

Artifact Name: Compatibility Evaluation Report  
Artifact Class: Evaluation report  
Owning Contract: 006  
Producer: Compatibility evaluator  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before selection  
Consumed By: 006  
Purpose: Record constructability and coherence of eligible candidates  
Required Identity Bindings: eligibility report id, policy versions, hash  
Authority Carried: Diagnostic authority only  
Authority Not Carried: No preference or selection authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: incompatible, conflicted, or unresolved outcome

### Profile Selection Report

Artifact Name: Profile Selection Report  
Artifact Class: Decision report  
Owning Contract: 006  
Producer: Deterministic selector  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before profile commitment  
Consumed By: 006  
Purpose: Record the deterministic selection decision and tie-breaking rationale  
Required Identity Bindings: candidate set, eligibility report, compatibility report, hash  
Authority Carried: Selection rationale only  
Authority Not Carried: No control authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: no candidate, no winner, policy unresolved, or conflict

### Committed Operational Profile

Artifact Name: Committed Operational Profile  
Artifact Class: Canonical supervisory configuration  
Owning Contract: 006  
Producer: Profile commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 007  
Purpose: Provide the selected supervisory configuration for the control cycle  
Required Identity Bindings: profile id, version, hash, objective set id, governance snapshot id  
Authority Carried: Configuration authority only  
Authority Not Carried: No control authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: selection or construction failure

### Committed Reference State

Artifact Name: Committed Reference State  
Artifact Class: Canonical reference artifact  
Owning Contract: 006  
Producer: Reference builder  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 007  
Purpose: Provide the target state against which control error is computed  
Required Identity Bindings: profile id, reference builder id, hash  
Authority Carried: Target reference authority only  
Authority Not Carried: No command authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: constructability failure

### Controller Definition

Artifact Name: Controller Definition  
Artifact Class: Reusable definition  
Owning Contract: 007  
Producer: Controller authoring / registry publication  
Canonical or Noncanonical: Canonical when registered  
Reusable or Cycle-Bound: Reusable  
Committed or Proposed: Proposed until committed as a definition binding  
Consumed By: 007  
Purpose: Define controller family, structure, and compatibility expectations  
Required Identity Bindings: controller id, version, hash  
Authority Carried: Definition authority only  
Authority Not Carried: No command authority  
Immutability Rule: Immutable once published  
Failure Relationship: composition incompatibility

### Committed Controller Configuration

Artifact Name: Committed Controller Configuration  
Artifact Class: Cycle-bound configuration  
Owning Contract: 007  
Producer: Controller setup  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 007  
Purpose: Bind the controller to the cycle-local authority and state context  
Required Identity Bindings: controller id, governance snapshot id, profile id, state set id  
Authority Carried: Control configuration authority only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable for the cycle  
Failure Relationship: invalid configuration or incompatible binding

### Committed Control Error

Artifact Name: Committed Control Error  
Artifact Class: Cycle diagnostic artifact  
Owning Contract: 007  
Producer: Control error evaluation  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 007  
Purpose: State the deviation driving control  
Required Identity Bindings: OSV id, reference state id, numeric profile id  
Authority Carried: Diagnostic only  
Authority Not Carried: No decision authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: invalid or unresolved error computation

### Controller Commitment Record

Artifact Name: Controller Commitment Record  
Artifact Class: Certification record  
Owning Contract: 007  
Producer: Controller commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 008  
Purpose: Certify the bounded control command and preserve lineage  
Required Identity Bindings: control command id, configuration id, state set id, hash  
Authority Carried: Command authorization authority only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: invalid commitment or failed certification

### Committed Control Command

Artifact Name: Committed Control Command  
Artifact Class: Canonical supervisory command  
Owning Contract: 007  
Producer: Controller commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 008  
Purpose: Represent the authorized bounded corrective action  
Required Identity Bindings: command id, controller commitment record id, numeric profile id  
Authority Carried: Bounded control authority only  
Authority Not Carried: No runtime execution authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: controller failure or authorization failure

### Runtime Capability Profile

Artifact Name: Runtime Capability Profile  
Artifact Class: Runtime declaration  
Owning Contract: 008  
Producer: Runtime declaration or committed profile source  
Canonical or Noncanonical: Canonical when committed  
Reusable or Cycle-Bound: Reusable if versioned and bound correctly  
Committed or Proposed: Committed  
Consumed By: 008  
Purpose: Declare what the target runtime can represent  
Required Identity Bindings: runtime id, capability version, hash  
Authority Carried: Capability only  
Authority Not Carried: No supervisory authority  
Immutability Rule: Immutable for the translation cycle  
Failure Relationship: missing, incompatible, or invalid capability profile

### Committed Translation Configuration

Artifact Name: Committed Translation Configuration  
Artifact Class: Cycle-bound configuration  
Owning Contract: 008  
Producer: Translation configuration commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 008  
Purpose: Bind translation behavior for the cycle  
Required Identity Bindings: translation config id, runtime capability profile id, hash  
Authority Carried: Translation configuration authority only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: incompatible or incomplete translation context

### Translation Profile

Artifact Name: Translation Profile  
Artifact Class: Reusable translation definition  
Owning Contract: 008  
Producer: Translation profile authoring  
Canonical or Noncanonical: Canonical when registered  
Reusable or Cycle-Bound: Reusable  
Committed or Proposed: Proposed until registered and committed  
Consumed By: 008  
Purpose: Define a translation strategy for a runtime family  
Required Identity Bindings: profile id, version, hash  
Authority Carried: Definition authority only  
Authority Not Carried: No command authority  
Immutability Rule: Immutable after publication  
Failure Relationship: validation failure or incompatibility

### Translation Definition

Artifact Name: Translation Definition  
Artifact Class: Reusable definition  
Owning Contract: 008  
Producer: Translation registry publication  
Canonical or Noncanonical: Canonical when published  
Reusable or Cycle-Bound: Reusable  
Committed or Proposed: Proposed until publication  
Consumed By: 008  
Purpose: Bind semantic-preserving translation rules to a runtime context  
Required Identity Bindings: definition id, version, hash  
Authority Carried: Definition authority only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable once published  
Failure Relationship: definition unavailable or inconsistent

### Translation Requirement Set

Artifact Name: Translation Requirement Set  
Artifact Class: Rule set / constraints  
Owning Contract: 008  
Producer: Translation analysis  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before validation  
Consumed By: 008  
Purpose: State mandatory translation constraints  
Required Identity Bindings: definition id, numeric profile id, hash  
Authority Carried: Constraint authority only  
Authority Not Carried: No broadened authority  
Immutability Rule: Immutable after validation commitment  
Failure Relationship: requirement loss or preservation failure

### Validated Translation Binding

Artifact Name: Validated Translation Binding  
Artifact Class: Validation result  
Owning Contract: 008  
Producer: Translation validation  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed until transaction creation  
Consumed By: 008  
Purpose: Confirm the translation context is coherent  
Required Identity Bindings: command id, runtime id, requirement set id, hash  
Authority Carried: Validation authority only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable after validation  
Failure Relationship: validation failure, unsupported semantics, or authority expansion

### Proposed Inference Command Set

Artifact Name: Proposed Inference Command Set  
Artifact Class: Precommit output  
Owning Contract: 008  
Producer: Translation construction  
Canonical or Noncanonical: Noncanonical until committed  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed  
Consumed By: 008  
Purpose: Represent the runtime-specific command before commitment  
Required Identity Bindings: command id, runtime id, translation config id  
Authority Carried: None until committed  
Authority Not Carried: Execution authority  
Immutability Rule: Immutable once constructed for the cycle  
Failure Relationship: construction failure or semantic mismatch

### Committed Inference Command Set

Artifact Name: Committed Inference Command Set  
Artifact Class: Intermediate committed semantic translation artifact  
Owning Contract: 008  
Producer: Translation semantic commitment before transaction commitment  
Canonical or Noncanonical: Canonical with respect to translation semantics; non-terminal for Contract 008 handoff  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 008 transaction construction and commitment  
Purpose: Preserve the semantically complete runtime-specific command set that is incorporated into the committed translation transaction  
Required Identity Bindings: committed_inference_command_set_id, source_control_command_id, runtime_target_id, translation_definition_ids, translation_equivalence_certificate_id, commitment_record_id, artifact_hash  
Authority Carried: Translation semantic authority only  
Authority Not Carried: No execution, execution-eligibility, execution-disposition, or runtime-application authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: semantic translation or transaction construction failure

### Translation Validation Report

Artifact Name: Translation Validation Report  
Artifact Class: Evaluation report  
Owning Contract: 008  
Producer: Translation validation stage  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before transaction commitment  
Consumed By: 008  
Purpose: Record translation validation findings  
Required Identity Bindings: command id, runtime id, policy versions, hash  
Authority Carried: Diagnostic authority only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: validation failure or unresolved compatibility

### Constructed Translation Transaction

Artifact Name: Constructed Translation Transaction  
Artifact Class: Transaction artifact  
Owning Contract: 008  
Producer: Translation transaction construction  
Canonical or Noncanonical: Canonical when committed  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before transaction commitment  
Consumed By: 008  
Purpose: Bundle the translated command and committed translation context  
Required Identity Bindings: transaction id, command id, runtime id, hash  
Authority Carried: Transaction structure only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable after construction  
Failure Relationship: transaction construction failure

### Translation Commitment Record

Artifact Name: Translation Commitment Record  
Artifact Class: Certification record  
Owning Contract: 008  
Producer: Transaction commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 008  
Purpose: Record commitment of one translation transaction  
Required Identity Bindings: transaction id, translation config id, hash  
Authority Carried: Commitment authority only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: commitment failure or idempotency rejection

### Committed Translation Transaction

Artifact Name: Committed Translation Transaction  
Artifact Class: Canonical translation artifact  
Owning Contract: 008  
Producer: Translation commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 009  
Purpose: Provide the exact runtime-specific command package for handoff  
Required Identity Bindings: transaction id, command id, runtime id, hash  
Authority Carried: Translation authority only  
Authority Not Carried: No execution authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: transaction failure or handoff failure

### Execution Interface Handoff Envelope

Artifact Name: Execution Interface Handoff Envelope  
Artifact Class: Handoff artifact  
Owning Contract: 008  
Producer: Contract 008 commitment stage  
Canonical or Noncanonical: Canonical only at handoff  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed when delivered  
Consumed By: 009  
Purpose: Preserve the translated command and its authority lineage for execution handling  
Required Identity Bindings: transaction id, handoff id, command id, hash  
Authority Carried: Preserved authority only  
Authority Not Carried: No new authority  
Immutability Rule: Immutable after delivery  
Failure Relationship: delivery failure or boundary mismatch

### Pre-Execution Disposition

Artifact Name: Pre-Execution Disposition  
Artifact Class: Execution-adjacent decision artifact  
Owning Contract: 009  
Producer: Execution interface  
Canonical or Noncanonical: Canonical if adopted by Contract 009  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before final disposition  
Consumed By: 009  
Purpose: State whether execution may proceed, be withheld, or fail  
Required Identity Bindings: handoff envelope id, runtime id, disposition rule version  
Authority Carried: Disposition authority only  
Authority Not Carried: No translation authority  
Immutability Rule: Immutable once committed  
Failure Relationship: execution refusal or unsupported runtime condition

### Execution Plan

Artifact Name: Execution Plan  
Artifact Class: Runtime execution artifact  
Owning Contract: 009  
Producer: Execution interface planning  
Canonical or Noncanonical: Canonical if produced by Contract 009  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Proposed before outcome  
Consumed By: 009 and 010  
Purpose: Represent the intended runtime execution path  
Required Identity Bindings: handoff envelope id, runtime id, plan id, hash  
Authority Carried: Execution planning only  
Authority Not Carried: No historical authority  
Immutability Rule: Immutable after publication  
Failure Relationship: plan failure or refusal

### Execution Outcome

Artifact Name: Execution Outcome  
Artifact Class: Runtime result artifact  
Owning Contract: 009  
Producer: Runtime execution  
Canonical or Noncanonical: Canonical as a runtime result, not as history  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed only when execution completes  
Consumed By: 010 and 011  
Purpose: Record what happened during execution  
Required Identity Bindings: runtime id, plan id, outcome id, evidence hash  
Authority Carried: Result evidence only  
Authority Not Carried: No historical certification  
Immutability Rule: Immutable after capture  
Failure Relationship: explicit execution failure or withheld disposition

### Committed Execution Disposition

Artifact Name: Committed Execution Disposition  
Artifact Class: Canonical execution-interface output  
Owning Contract: 009  
Producer: Execution interface commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: 010 and 011  
Purpose: Provide the authoritative execution result boundary for Contract 010 continuity evaluation and Contract 011 realization certification  
Required Identity Bindings: disposition id, execution outcome id, handoff id, hash  
Authority Carried: Execution disposition authority only  
Authority Not Carried: No historical authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: failed execution or withheld disposition

### Committed Supervisory Continuity Record

Artifact Name: Committed Supervisory Continuity Record  
Artifact Class: Canonical Contract 010 continuity artifact  
Owning Contract: 010  
Producer: Contract 010 continuity commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Contract 011 realization certification  
Purpose: Bind the Committed Supervisory Mode to the mode-transition evaluation, guard results, transition precedence, reset actions, hysteresis evaluation, dwell-time enforcement, referenced Committed Execution Disposition, and all continuity lineage required for deterministic replay and Contract 011 consumption  
Required Identity Bindings: committed_supervisory_continuity_record_id, supervisory_mode_commitment_id, execution_disposition_id, transition_evaluation_id, guard_evaluation_set_id, transition_precedence_policy_id, reset_record_id where applicable, hysteresis_evaluation_id, dwell_evaluation_id, lineage_hash  
Authority Carried: Continuity evidence and lineage only  
Authority Not Carried: No independent mode, execution, governance, observation, or historical certification authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: continuity commitment failure or unresolved transition evidence

### Committed Supervisory Mode

Artifact Name: Committed Supervisory Mode  
Artifact Class: Canonical Contract 010 supervisory mode artifact  
Owning Contract: 010  
Producer: Contract 010 mode commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Contract 011 realization certification and Contract 010 continuity processing where authorized  
Purpose: Establish the authoritative supervisory mode for the next continuity boundary  
Required Identity Bindings: supervisory_mode_id, mode_definition_id, transition_evaluation_id, guard_evaluation_set_id, commitment_id, hash  
Authority Carried: Supervisory mode authority for the committed continuity boundary  
Authority Not Carried: No execution, observation, realization-certification, or historical authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: unresolved mode transition or continuity commitment failure

### Committed Realization Record

Artifact Name: Committed Realization Record  
Artifact Class: Historical record  
Owning Contract: 011  
Producer: Contract 011 realization certification  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Contract 011 Stage Two only, plus replay, audit, disclosure, and retention functions where constitutionally authorized  
Purpose: Serve as the exclusive immutable constitutional representation of one certified supervisory realization  
Required Identity Bindings: execution disposition id, realization record id, evidence lineage  
Authority Carried: Authoritative realization history only  
Authority Not Carried: No execution, continuity, governance, or external-verification authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: certification failure or unresolved historical status  
Deprecated Aliases: Committed Historical Record

### Constructed Realization Record

Artifact Name: Constructed Realization Record  
Artifact Class: Constructed Contract 011 realization artifact  
Owning Contract: 011  
Producer: Contract 011 realization certification  
Canonical or Noncanonical: Noncanonical until successful realization commitment  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Constructed  
Consumed By: Contract 011 realization commitment only  
Purpose: Assemble one constitutionally complete realization representation before it becomes the Committed Realization Record  
Required Identity Bindings: execution_disposition_id, committed_supervisory_continuity_record_id, realization_record_id, evidence_lineage, schema_version, hash  
Authority Carried: Construction evidence only  
Authority Not Carried: No authoritative realization history and no Stage Two cross-stage authority  
Immutability Rule: Immutable after construction; superseded by the committed artifact on successful commitment  
Failure Relationship: realization certification failure or commitment precondition failure

### Realization Certification Failure

Artifact Name: Realization Certification Failure  
Artifact Class: Contract 011 Stage One failure artifact  
Owning Contract: 011  
Producer: Contract 011 Stage One realization-certification failure path  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Contract 011 Stage One failure handling and later audit or disclosure where authorized  
Purpose: Represent the terminal inability to establish authoritative realization history without reconstruction or substitution  
Required Identity Bindings: failure id, execution disposition id, committed_supervisory_continuity_record_id, reason code, evidence hash  
Authority Carried: Failure evidence only  
Authority Not Carried: No realization history or external-truth claim  
Immutability Rule: Immutable after commitment  
Failure Relationship: failed realization certification or missing committed Stage One input

### Committed Supervisory Provenance Failure

Artifact Name: Committed Supervisory Provenance Failure  
Artifact Class: Contract 011 Stage Two failure artifact  
Owning Contract: 011  
Producer: Contract 011 Stage Two supervisory provenance failure path  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Contract 011 failure handling and later audit or disclosure where authorized  
Purpose: Represent the terminal inability to establish complete supervisory-cycle provenance without reconstruction or substitution  
Required Identity Bindings: failure id, realization record id, cycle id, reason code, lineage hash  
Authority Carried: Failure evidence only  
Authority Not Carried: No cycle-history authority or external-verification authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: unresolved artifact resolution, incomplete lineage, or failed provenance commitment

### Constructed Supervisory Lineage

Artifact Name: Constructed Supervisory Lineage  
Artifact Class: Constructed provenance artifact  
Owning Contract: 011  
Producer: Contract 011 Stage Two artifact and relationship resolution  
Canonical or Noncanonical: Noncanonical until downstream commitment  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Constructed  
Consumed By: Lineage Completeness Evaluation and Lineage Integrity Evaluation  
Purpose: Assemble the exact committed artifacts, relationships, authority paths, and lineage required for one supervisory cycle  
Required Identity Bindings: cycle id, Committed Realization Record identity, lineage identity, resolved artifact and relationship identities, schema and version bindings, lineage hash  
Authority Carried: Construction evidence only  
Authority Not Carried: No cycle-history or re-adjudication authority  
Immutability Rule: Immutable after construction  
Failure Relationship: unresolved, incomplete, substituted, or inconsistent lineage

### Lineage Completeness Evaluation Result

Artifact Name: Lineage Completeness Evaluation Result  
Artifact Class: Stage Two evaluation result  
Owning Contract: 011  
Producer: Lineage Completeness Evaluation  
Canonical or Noncanonical: Noncanonical supporting result  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Constructed  
Consumed By: Lineage Integrity Evaluation and Constructed Supervisory Cycle Record construction  
Purpose: Record whether every required artifact, relationship, dependency, authority path, and terminal realization element is present  
Required Identity Bindings: lineage identity, required-artifact set, relationship set, evaluation identity, status, result hash  
Authority Carried: Evaluation evidence only  
Authority Not Carried: No authority to reconstruct, substitute, or commit cycle history  
Immutability Rule: Immutable after evaluation  
Failure Relationship: incomplete or missing lineage element

### Lineage Integrity Evaluation Result

Artifact Name: Lineage Integrity Evaluation Result  
Artifact Class: Stage Two evaluation result  
Owning Contract: 011  
Producer: Lineage Integrity Evaluation  
Canonical or Noncanonical: Noncanonical supporting result  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Constructed  
Consumed By: Constructed Supervisory Cycle Record construction and Supervisory Cycle Commitment  
Purpose: Record whether the complete lineage preserves exact identity, commitment, relationship, version, temporal, derivation, and authority consistency  
Required Identity Bindings: lineage identity, completeness-result identity, evaluated dimensions, status, result hash  
Authority Carried: Evaluation evidence only  
Authority Not Carried: No authority to repair, reinterpret, or commit cycle history  
Immutability Rule: Immutable after evaluation  
Failure Relationship: integrity conflict, substitution, mutation, or invalid terminal binding

### Constructed Supervisory Cycle Record

Artifact Name: Constructed Supervisory Cycle Record  
Artifact Class: Precommit whole-cycle history artifact  
Owning Contract: 011  
Producer: Supervisory Cycle Record Construction  
Canonical or Noncanonical: Noncanonical until Supervisory Cycle Commitment  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Constructed  
Consumed By: Supervisory Cycle Commitment  
Purpose: Deterministically assemble the complete validated supervisory lineage and committed realization before final commitment  
Required Identity Bindings: Committed Realization Record identity, lineage identity, completeness-result identity, integrity-result identity, cycle identity, canonical serialization hash  
Authority Carried: Precommit construction evidence only  
Authority Not Carried: No authoritative supervisory history  
Immutability Rule: Immutable after construction  
Failure Relationship: failed construction or failed Supervisory Cycle Commitment

### Supervisory Cycle Commitment Record

Artifact Name: Supervisory Cycle Commitment Record  
Artifact Class: Stage Two commitment event record  
Owning Contract: 011  
Producer: Supervisory Cycle Commitment  
Canonical or Noncanonical: Canonical event evidence, not canonical whole-cycle history  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Committed Supervisory Cycle Record or Committed Supervisory Provenance Failure; later replay and audit where authorized  
Purpose: Record the final Stage Two commitment decision and its exact input bindings  
Required Identity Bindings: commitment record identity, constructed cycle record identity, lineage identities, commitment status, timestamp, hash  
Authority Carried: Commitment-event evidence only  
Authority Not Carried: No independent authority to represent or replace whole-cycle history  
Immutability Rule: Immutable after commitment  
Failure Relationship: COMMITMENT_FAILED

### Replay Verification Result

Artifact Name: Replay Verification Result  
Artifact Class: Deterministic verification result  
Owning Contract: 011  
Producer: Contract 011 deterministic replay  
Canonical or Noncanonical: Noncanonical verification result  
Reusable or Cycle-Bound: Cycle-bound to the replayed terminal outcome  
Committed or Proposed: Constructed  
Consumed By: Audit, disclosure, or retention functions where authorized  
Purpose: Record whether deterministic replay is equivalent to the committed terminal outcome  
Required Identity Bindings: terminal artifact identity, replay package identity, replay input hash, replay output hash, status, verifier version  
Authority Carried: Verification evidence only  
Authority Not Carried: No authority to mutate, replace, invalidate, or reconstruct constitutional history  
Immutability Rule: Immutable after result production  
Failure Relationship: REPLAY_NOT_EQUIVALENT

### Audit Representation

Artifact Name: Audit Representation  
Artifact Class: Non-constitutive audit result  
Owning Contract: 011  
Producer: Contract 011 audit authority  
Canonical or Noncanonical: Noncanonical representation of audit findings  
Reusable or Cycle-Bound: Cycle-bound to the audited terminal outcome  
Committed or Proposed: Constructed  
Consumed By: Authorized disclosure, compliance, or retention functions  
Purpose: Present immutable audit findings against the committed terminal outcome and its replay evidence  
Required Identity Bindings: audited terminal artifact identity, audit identity, replay-result identity where available, findings, status, representation hash  
Authority Carried: Audit evidence only  
Authority Not Carried: No authority to alter history, enact governance, or replace a terminal artifact  
Immutability Rule: Immutable after result production  
Failure Relationship: NON_CONFORMANT or INCONCLUSIVE

### Committed Supervisory Cycle Record

Artifact Name: Committed Supervisory Cycle Record  
Artifact Class: Cycle summary artifact  
Owning Contract: 011  
Producer: Supervisory provenance and audit commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Contract 012; later verification and archival use where authorized  
Purpose: Serve as the exclusive immutable constitutional representation of the complete supervisory lineage for one supervisory cycle without re-adjudicating execution  
Required Identity Bindings: realization record id, provenance graph id, audit index id, cycle record id  
Authority Carried: Provenance and audit authority only  
Authority Not Carried: No execution, observation, or governance authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: exclusive terminal counterpart is Committed Supervisory Provenance Failure; the two artifacts SHALL NOT coexist for one cycle

### Verifiable Claim Set

Artifact Name: Verifiable Claim Set  
Artifact Class: Verification claim definition and subject binding  
Owning Contract: 012  
Producer: Contract 012 claim construction  
Canonical or Noncanonical: Canonical when committed within a submission  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Verification Profile Binding and Verification Applicability  
Purpose: Define the exact claims SACS submits for external verification  
Required Identity Bindings: claim set id, verification subject id, scope, version, hash  
Authority Carried: Claim-scope authority only  
Authority Not Carried: No verification judgment or external authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: unsupported, conflicting, or out-of-scope claim

### Committed Verification Profile Binding

Artifact Name: Committed Verification Profile Binding  
Artifact Class: Cycle-bound verification configuration  
Owning Contract: 012  
Producer: Verification Profile commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Verification Requirement Set, Applicability, and Submission Construction  
Purpose: Bind one authorized verification profile to one exchange  
Required Identity Bindings: profile id, version, verifier id, method ids, scope, hash  
Authority Carried: Verification configuration authority  
Authority Not Carried: No verification judgment or ownership of verifier authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: unavailable, incompatible, or unauthorized profile

### Committed Verification Requirement Set

Artifact Name: Committed Verification Requirement Set  
Artifact Class: Verification requirement set  
Owning Contract: 012  
Producer: Requirement-set commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Applicability, Evidence Resolution, and Evidence Validation  
Purpose: Bind mandatory evidence, method, scope, and trust requirements  
Required Identity Bindings: requirement set id, profile id, ordered requirements, version, hash  
Authority Carried: Submission constraint authority  
Authority Not Carried: No authority to issue or reinterpret external judgment  
Immutability Rule: Immutable after commitment  
Failure Relationship: incomplete, conflicting, or unsatisfied requirements

### Verification Applicability Result

Artifact Name: Verification Applicability Result  
Artifact Class: Applicability evaluation result  
Owning Contract: 012  
Producer: Verification Applicability Evaluation  
Canonical or Noncanonical: Canonical supporting result  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Verification Evidence Package Construction  
Purpose: Determine whether the exact verification operation may proceed within its committed scope  
Required Identity Bindings: claim set id, profile id, requirement set id, verifier id, status, hash  
Authority Carried: Applicability authority only  
Authority Not Carried: No claim truth or external verification authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: NOT_APPLICABLE, unresolved, or restricted beyond authorization

### Verification Evidence Package

Artifact Name: Verification Evidence Package  
Artifact Class: Canonical evidence package  
Owning Contract: 012  
Producer: Evidence Package Construction  
Canonical or Noncanonical: Canonical when committed  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Verification Submission Construction  
Purpose: Assemble the complete resolved evidence required for one submission  
Required Identity Bindings: exchange identity, subject id, claim set id, requirement set id, evidence identities, hash  
Authority Carried: Evidence-binding authority only  
Authority Not Carried: No external verification judgment  
Immutability Rule: Immutable after commitment  
Failure Relationship: missing, inadmissible, incomplete, or integrity-invalid evidence

### Constructed Verification Submission

Artifact Name: Constructed Verification Submission  
Artifact Class: Precommitment submission artifact  
Owning Contract: 012  
Producer: Verification Submission Construction  
Canonical or Noncanonical: Noncanonical until commitment  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Constructed  
Consumed By: Verification Submission Commitment  
Purpose: Assemble the exact bounded request to an external verifier  
Required Identity Bindings: exchange identity, claim set, profile, requirements, evidence package, verifier, serialization, hash  
Authority Carried: Construction evidence only  
Authority Not Carried: No external judgment or custody transfer  
Immutability Rule: Immutable after construction  
Failure Relationship: construction, serialization, or identity conflict

### Verification Submission Commitment Record

Artifact Name: Verification Submission Commitment Record  
Artifact Class: Submission commitment event  
Owning Contract: 012  
Producer: Verification Submission Commitment  
Canonical or Noncanonical: Canonical event evidence  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: External Verification Boundary and Contract 012 exchange history  
Purpose: Certify commitment of one exact verification submission  
Required Identity Bindings: submission id, exchange identity, commitment id, timestamp, hash  
Authority Carried: SACS submission-commitment authority  
Authority Not Carried: No verifier judgment or ownership transfer  
Immutability Rule: Immutable after commitment  
Failure Relationship: conflicting or failed submission commitment

### Committed Verification Submission

Artifact Name: Committed Verification Submission  
Artifact Class: Canonical submission artifact  
Owning Contract: 012  
Producer: Verification Submission Commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: External Verification Boundary  
Purpose: Represent the immutable request submitted under one verification exchange identity  
Required Identity Bindings: exchange identity, subject, claims, profile, requirements, evidence package, verifier, commitment, hash  
Authority Carried: SACS submission authority  
Authority Not Carried: No external verification judgment or verifier sovereignty  
Immutability Rule: Immutable after commitment  
Failure Relationship: custody-transfer or submission correlation failure

### Custody Transfer Record

Artifact Name: Custody Transfer Record  
Artifact Class: Inter-constitutional custody event  
Owning Contract: 012  
Producer: External Verification Boundary  
Canonical or Noncanonical: Canonical event evidence  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Result Admission and Exchange History  
Purpose: Record transfer of temporary custody while preserving SACS ownership and authority  
Required Identity Bindings: exchange identity, submission identity, SACS custodian, external custodian, timestamps, integrity hash  
Authority Carried: Custody evidence only  
Authority Not Carried: No ownership, sovereignty, historical, or commitment transfer  
Immutability Rule: Immutable after commitment  
Failure Relationship: unresolved custody lineage or identity mismatch

### External Verification Result

Artifact Name: External Verification Result  
Artifact Class: Foreign authoritative judgment  
Owning Contract: Independent external verification authority  
Producer: Authorized external verifier  
Canonical or Noncanonical: External-authoritative, not a SACS-created canonical artifact  
Reusable or Cycle-Bound: Bound to the submitted exchange  
Committed or Proposed: Issued by the external authority  
Consumed By: Verification Result Admission  
Purpose: Provide the verifier's sovereign judgment and declared limitations  
Required Identity Bindings: issuer identity, submission identity, exchange identity, judgment, scope, limitations, signature, hash  
Authority Carried: External verification judgment only  
Authority Not Carried: No SACS ownership or SACS commitment authority  
Immutability Rule: Preserved exactly as received  
Failure Relationship: unverifiable issuer, missing correlation, or malformed result

### Verification Result Admission Record

Artifact Name: Verification Result Admission Record  
Artifact Class: SACS admission record  
Owning Contract: 012  
Producer: Verification Result Admission  
Canonical or Noncanonical: Canonical event evidence  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Constructed Verification Result  
Purpose: Record that an external result entered SACS custody without reinterpretation  
Required Identity Bindings: exchange identity, external-result identity, issuer identity, custody record, admission status, hash  
Authority Carried: SACS custody and admission authority  
Authority Not Carried: No authority over the external judgment  
Immutability Rule: Immutable after commitment  
Failure Relationship: admission failure or correlation failure

### Verification Result Admission Failure

Artifact Name: Verification Result Admission Failure  
Artifact Class: Contract 012 admission failure artifact  
Owning Contract: 012  
Producer: Verification Result Admission  
Canonical or Noncanonical: Canonical failure evidence  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Committed Verification Exchange Failure path  
Purpose: Record why an external result could not enter SACS custody  
Required Identity Bindings: exchange identity, failure identity, result identity where available, reason, evidence hash  
Authority Carried: Failure evidence only  
Authority Not Carried: No exchange history or external judgment authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: issuer, integrity, correlation, or admission failure

### Constructed Verification Result

Artifact Name: Constructed Verification Result  
Artifact Class: SACS precommitment result representation  
Owning Contract: 012  
Producer: Verification Result Construction  
Canonical or Noncanonical: Noncanonical until certification and commitment  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Constructed  
Consumed By: Verification Result Certification and Commitment  
Purpose: Represent the admitted external judgment in SACS schema without changing its meaning  
Required Identity Bindings: external-result identity, admission record, issuer, judgment, limitations, scope, hash  
Authority Carried: Construction evidence only  
Authority Not Carried: No committed SACS history until certification  
Immutability Rule: Immutable after construction  
Failure Relationship: representation or binding failure

### Verification Result Certification Record

Artifact Name: Verification Result Certification Record  
Artifact Class: Result certification event  
Owning Contract: 012  
Producer: Verification Result Certification and Commitment  
Canonical or Noncanonical: Canonical event evidence  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Committed Verification Result and Exchange Record Construction  
Purpose: Certify the SACS representation of the admitted external judgment for commitment  
Required Identity Bindings: result identity, admission identity, certification identity, exchange identity, hash  
Authority Carried: SACS certification authority over its representation  
Authority Not Carried: No authority to alter the foreign judgment  
Immutability Rule: Immutable after commitment  
Failure Relationship: certification failure or conflicting result identity

### Verification Result Certification Failure

Artifact Name: Verification Result Certification Failure  
Artifact Class: Contract 012 certification failure artifact  
Owning Contract: 012  
Producer: Verification Result Certification and Commitment  
Canonical or Noncanonical: Canonical failure evidence  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Committed Verification Exchange Failure path  
Purpose: Record why an admitted external judgment could not be certified as a SACS representation  
Required Identity Bindings: exchange identity, failure identity, admission identity, reason, hash  
Authority Carried: Failure evidence only  
Authority Not Carried: No committed verification result or exchange history  
Immutability Rule: Immutable after commitment  
Failure Relationship: identity, integrity, certification, or commitment failure

### Committed Verification Result

Artifact Name: Committed Verification Result  
Artifact Class: Canonical SACS representation of external judgment  
Owning Contract: 012  
Producer: Verification Result Certification and Commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Constructed Verification Exchange Record  
Purpose: Preserve the exact admitted and certified external judgment in SACS history  
Required Identity Bindings: exchange identity, result identity, issuer identity, judgment, limitations, admission, certification, hash  
Authority Carried: SACS historical authority over its representation only  
Authority Not Carried: No external judgment ownership or altered meaning  
Immutability Rule: Immutable after commitment  
Failure Relationship: result certification or commitment failure

### Constructed Verification Exchange Record

Artifact Name: Constructed Verification Exchange Record  
Artifact Class: Precommitment exchange-history artifact  
Owning Contract: 012  
Producer: Verification Exchange Record Construction  
Canonical or Noncanonical: Noncanonical until exchange commitment  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Constructed  
Consumed By: Verification Exchange Commitment  
Purpose: Assemble the complete SACS and external lineage of one verification exchange before final commitment  
Required Identity Bindings: exchange identity, submission, custody, external result, admission, committed result, provenance, hash  
Authority Carried: Construction evidence only  
Authority Not Carried: No completed exchange history  
Immutability Rule: Immutable after construction  
Failure Relationship: incomplete lineage or exchange construction failure

### Verification Exchange Commitment Record

Artifact Name: Verification Exchange Commitment Record  
Artifact Class: Exchange commitment event  
Owning Contract: 012  
Producer: Verification Exchange Commitment  
Canonical or Noncanonical: Canonical event evidence  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Committed Verification Exchange Record or Committed Verification Exchange Failure  
Purpose: Record the atomic commitment decision for one complete exchange  
Required Identity Bindings: exchange identity, constructed record identity, commitment identity, status, timestamp, hash  
Authority Carried: SACS exchange-commitment authority  
Authority Not Carried: No external judgment authority  
Immutability Rule: Immutable after commitment  
Failure Relationship: conflicting or failed exchange commitment

### Committed Verification Exchange Record

Artifact Name: Committed Verification Exchange Record  
Artifact Class: Canonical inter-constitutional exchange history  
Owning Contract: 012  
Producer: Verification Exchange Commitment  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Authorized archival, inspection, replay, audit, or later ecosystem-level modules  
Purpose: Preserve one complete bounded verification exchange as immutable SACS history  
Required Identity Bindings: exchange identity, committed submission, custody transitions, external result, admission, committed result, provenance, commitment, limitations, hash  
Authority Carried: SACS historical authority over the exchange representation  
Authority Not Carried: No ownership or sovereignty over external judgment  
Immutability Rule: Immutable after commitment  
Failure Relationship: exclusive counterpart is Committed Verification Exchange Failure; the two SHALL NOT coexist

### Committed Verification Exchange Failure

Artifact Name: Committed Verification Exchange Failure  
Artifact Class: Contract 012 terminal exchange failure artifact  
Owning Contract: 012  
Producer: Verification Exchange Commitment failure path  
Canonical or Noncanonical: Canonical  
Reusable or Cycle-Bound: Cycle-bound  
Committed or Proposed: Committed  
Consumed By: Authorized archival, inspection, replay, audit, or later ecosystem-level modules  
Purpose: Preserve failure to constitutionally complete or preserve one verification exchange  
Required Identity Bindings: exchange identity, failure identity, failure classification, commitment context, verified conditions, upstream lineage, hash  
Authority Carried: Failure history only  
Authority Not Carried: No external judgment or successful exchange history  
Immutability Rule: Immutable after commitment  
Failure Relationship: exclusive counterpart is Committed Verification Exchange Record; the two SHALL NOT coexist

### Verification Exchange Replay Result

Artifact Name: Verification Exchange Replay Result  
Artifact Class: Replay-only verification result  
Owning Contract: 012  
Producer: Verification Exchange Replay  
Canonical or Noncanonical: Noncanonical replay evidence  
Reusable or Cycle-Bound: Bound to one terminal exchange outcome  
Committed or Proposed: Constructed  
Consumed By: Verification Exchange Audit Representation and authorized inspection  
Purpose: Reproduce only SACS-controlled exchange material without recreating undisclosed external reasoning  
Required Identity Bindings: exchange identity, terminal artifact identity, replay package, replay class, input and output hashes  
Authority Carried: Replay evidence only  
Authority Not Carried: No authority to alter, replace, invalidate, or recommit exchange history  
Immutability Rule: Immutable after result production  
Failure Relationship: MISMATCH, INCOMPLETE, UNAVAILABLE, NONREPLAYABLE_EXTERNAL_SEGMENT, or REPLAY_FAILURE

### Verification Exchange Audit Representation

Artifact Name: Verification Exchange Audit Representation  
Artifact Class: Audit-only representation  
Owning Contract: 012  
Producer: Verification Exchange Audit  
Canonical or Noncanonical: Noncanonical audit evidence  
Reusable or Cycle-Bound: Bound to one terminal exchange outcome  
Committed or Proposed: Constructed  
Consumed By: Authorized inspection, disclosure, or retention  
Purpose: Present audit findings about exchange conformance without constituting or changing authority  
Required Identity Bindings: exchange identity, terminal artifact identity, audit identity, findings, replay-result identity where available, hash  
Authority Carried: Audit evidence only  
Authority Not Carried: No authority to modify history, alter judgment, or enact governance  
Immutability Rule: Immutable after result production  
Failure Relationship: nonconformant or inconclusive audit

## Terminology Notes

- "Committed Realization Record" is the canonical term in the Contract 011 realization-certification stage.
- "Committed Supervisory Continuity Record" is the canonical Contract 010 evidence and lineage artifact.
- "Committed Supervisory Mode" is the authoritative mode established for the next continuity boundary; it is distinct from the Committed Supervisory Continuity Record that explains how and why it became authoritative.
- "Committed Historical Record" remains a deprecated alias.
- "Committed Supervisory Cycle Record" is the canonical whole-cycle artifact produced by Contract 011.
- "Realization Record Identity" is the canonical identity label for the Contract 011 success artifact.
- "Inference Command Set" in the corpus sometimes appears as "Inference Command Set" or "Committed Inference Command Set"; this pack uses the committed form for the canonical artifact.
- "Execution Disposition" is the canonical runtime-to-continuity and runtime-to-realization boundary term under Contracts 009, 010, and 011.  

## Source Traceability

- `SACS Contract 001 Observation_v0.1.0.md` - observation batch and handoff basis.
- `SACS Contract 003 Operational State Vector.txt` - OSV publication and partial publication isolation.
- `SACS Contract 004 Governance.txt` - governance snapshot and revocation basis.
- `SACS Contract 005 Supervisory Objectives.txt` - committed objective set and reduced authority failure.
- `SACS Contract 006 Section 2 Operational Profile Registry.md` - registry entries and snapshots.
- `SACS Contract 006 Section 3 Operational Profile Registry.md` - candidate profile set, eligibility, and report vocabulary.
- `SACS Contract 006 Section 4 Compatibility Evaluation.md` - compatibility report and outcome vocabulary.
- `SACS Contract 006 Section 5 Deterministic Profile Selection.md` - selection report, tie-breaking, and no-winner handling.
- `SACS Contract 006 Section 6 Committed Ops Profile Construction.md` - committed operational profile, reference state, authority reduction certificate.
- `SACS Contract 007 Controller.txt` and section files - controller configuration, error, commitment, and command artifacts.
- `SACS Contract 008 Inference Control.txt` and sections 6-19 - translation profile, translation transaction, commitment, and handoff artifacts.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - realization record terminology.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - supervisory cycle record terminology.
- `SACS Contract 012 External Verification Profile and Evidence Interface.txt` - assembled external-verification and exchange-history artifacts.
- `SACS Contract 012 Section 2 Authority Boundary.txt` through `SACS Contract 012 Section 23 Architectural Completion-Successor Contracts.txt` - numbered Contract 012 artifact, authority, and completion definitions.
