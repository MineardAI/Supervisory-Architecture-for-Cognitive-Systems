# SACS Terminology And Naming

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012  
Classification: Constitutional-supporting terminology registry

## Normative Terms

**Definition:** a reusable constitutional statement or identity that can be published and reused across cycles.  
**Configuration:** a cycle-bound binding of committed inputs into one supervisory context.  
**Profile:** a reusable or committed supervisory configuration descriptor.  
**Registry:** the canonical repository for published definitions.  
**Registry Entry:** one immutable publication record that binds a definition to a registry context.  
**Registry Snapshot:** one immutable view of registry state for a cycle.  
**Candidate:** a visible but not yet selected artifact.  
**Evaluation:** a contract-defined check that produces a report or result.  
**Report:** an evidence artifact describing evaluation or decision findings.  
**Binding:** a contract-defined association between an identity and a context.  
**Proposed:** a precommit artifact that is not yet canonical.  
**Constructed:** a deterministic artifact assembled from committed inputs.  
**Validated:** an artifact or binding that passed contract-defined checks.  
**Certified:** an artifact that has received the owning contract's authority to finalize.  
**Committed:** a canonical artifact that is immutable for the cycle.  
**Canonical:** the authoritative artifact or vocabulary for the contract boundary.  
**Disposition:** a decision about whether a runtime or historical result may proceed or stand.  
**Outcome:** the result of an evaluation, execution, or certification step.  
**Record:** an evidence artifact that preserves lineage or fact.  
**Failure Artifact:** the explicit terminal artifact used when success cannot be established.  
**Provenance:** the lineage and source-binding of an artifact.  
**Lineage:** the ancestry chain linking an artifact to its inputs and prior commitments.  
**Authority:** the constitutional right to make a specific class of decision.  
**Capability:** what a definition, runtime, or profile can represent or support.  
**Availability:** whether a capability or source is currently present and usable.  
**Execution:** the act of runtime realization under a disposition.  
**Realization:** the fact that an execution or supervisory action actually occurred.  
**Observation:** admissible runtime evidence entering the supervisory pipeline.  
**Realization Certification:** the constitutional act of turning realization into authoritative history.  
**Realization Record Identity:** the canonical identity label for the Contract 011 committed realization artifact.  
**Realization Certification, Supervisory Provenance, and Audit:** the constitutional act of assembling the whole-cycle canonical artifact from a committed realization record.  
**Committed Supervisory Mode:** the authoritative supervisory mode established for the next continuity boundary.  
**Committed Supervisory Continuity Record:** the immutable Contract 010 artifact that binds the Committed Supervisory Mode to the mode-transition evaluation, guard results, transition precedence, reset actions, hysteresis evaluation, dwell-time enforcement, referenced Committed Execution Disposition, and all continuity lineage required for deterministic replay and Contract 011 consumption.  
**Constructed Supervisory Lineage:** the deterministic, non-authoritative Stage Two assembly of the exact committed artifacts, relationships, authority paths, and lineage required for one supervisory cycle.  
**Lineage Completeness Evaluation Result:** the immutable Stage Two evaluation result stating whether every required lineage artifact, relationship, dependency, authority path, and terminal realization element is present.  
**Lineage Integrity Evaluation Result:** the immutable Stage Two evaluation result stating whether a complete lineage preserves exact identity, commitment, relationship, version, temporal, derivation, and authority consistency.  
**Constructed Supervisory Cycle Record:** the immutable precommit assembly of one complete and integrity-valid supervisory lineage; it is not authoritative whole-cycle history until Supervisory Cycle Commitment succeeds.  
**Supervisory Cycle Commitment:** the final Contract 011 commitment operation that evaluates one Constructed Supervisory Cycle Record without reopening upstream adjudication.  
**Supervisory Cycle Commitment Record:** the immutable event record certifying the Stage Two commitment decision; it is not the whole-cycle historical artifact.  
**Committed Supervisory Cycle Record:** the sole immutable authoritative complete supervisory history for one cycle after successful Stage Two commitment.  
**Committed Supervisory Provenance Failure:** the sole immutable terminal Stage Two failure artifact when complete supervisory provenance cannot be committed; it is mutually exclusive with the Committed Supervisory Cycle Record.  
**Replay Verification Result:** the immutable result of deterministic replay against exactly one terminal Stage Two artifact; it carries verification evidence only and cannot alter authority.  
**Audit Representation:** the immutable non-constitutive representation of audit findings; it cannot modify history or enact governance.  
**Verification:** external confirmation owned by NullBlade, not by SACS itself.
**Inter-Constitutional Trust Exchange:** the bounded exchange in which SACS submits committed claims and evidence to an independent verifier, receives its sovereign judgment, and commits an immutable SACS representation without merging authority.  
**Constitutional Custody:** temporary control of an artifact for an authorized constitutional operation; custody is not ownership, sovereignty, historical authority, or commitment authority.  
**Verification Exchange Identity:** the stable identity binding one submitted verification operation, its custody transitions, external result, admission, certification, commitment, replay, audit, and terminal exchange outcome.  
**External Verification Result:** the judgment issued by the independent external verifier; it is external-authoritative and is not a SACS-created committed artifact.  
**Constructed Verification Result:** the non-authoritative SACS representation of an admitted external result before certification and commitment.  
**Committed Verification Result:** the immutable SACS representation of the admitted and certified external judgment; it does not alter the judgment's meaning or transfer external authority.  
**Committed Verification Exchange Record:** the sole immutable SACS historical representation of one completed inter-constitutional verification exchange.  
**Committed Verification Exchange Failure:** the sole immutable terminal artifact when the exchange itself cannot be constitutionally completed or preserved; an unfavorable judgment is not this artifact.  
**Verification Exchange Replay Result:** retrospective, non-constitutive replay evidence limited to SACS-controlled exchange material.  
**Verification Exchange Audit Representation:** retrospective, non-constitutive audit evidence about one completed exchange.  

## Required Distinctions

The following pairs are constitutionally different and must not be collapsed:

- Operational Profile Definition != Committed Operational Profile
- Controller Definition != Committed Controller Configuration
- Controller State != supervised-system state
- Translation Profile != Committed Translation Configuration
- Translation Definition != Translation Evaluation
- Runtime capability != Runtime availability
- Constructed Translation Transaction != Committed Translation Transaction
- Execution Plan != Execution Outcome
- Execution Outcome != Committed Execution Disposition
- Constructed Supervisory Cycle Record != Supervisory Cycle Commitment Record != Committed Supervisory Cycle Record
- Replay Verification Result != Audit Representation
- Custody != Ownership != Authority != Historical Sovereignty != Commitment
- External Verification Result != Constructed Verification Result != Committed Verification Result != Committed Verification Exchange Record
- Verification Result != Verification Exchange Status
- Unfavorable Verification Judgment != Committed Verification Exchange Failure
- Realization certification != External verification

Additional distinctions:

- Observation != state estimation
- State estimation != OSV publication
- Governance != objective selection
- Objective selection != controller action
- Controller action != translation
- Translation != execution
- Execution != realization certification

## Deprecated Or Replaced Terms

- **"Legal"** should not be used when the meaning is constitutional authority or authorization. Use **authorized** or **constitutionally authorized** instead.
- **"Current"** should not be used as a canonical pointer unless the contract explicitly defines a current-state registry.
- **"Latest"** should not be used as a determinative selection rule.
- **"Best effort"** should not be used as a substitute for required failure handling.
- **"Approximation"** should not be used when a contract requires explicit failure.
- **"Realization record"** is deprecated as a generic lowercase phrase; use **Committed Realization Record** for the Contract 011 canonical artifact.
- **"Historical Record Identity"** should not be used for the Contract 011 success artifact; use **Realization Record Identity** instead.
- **"Inference command"** should be qualified as **Committed Inference Command Set** when it is the canonical translated artifact.
- **"Mode"** remains a current Contract 010 constitutional term. Historical-certification usage under the old 010 numbering is retained only as archived traceability and is assigned to Contract 011 in the active corpus.

## Naming Rules

- Reusable artifacts use `Definition`, `Profile`, `Rule Set`, or `Registry`.
- Cycle-authoritative artifacts use `Committed`.
- Precommit outputs use `Proposed` or `Constructed`.
- Evidence artifacts use `Report`, `Record`, or `Certificate` based on whether they evaluate, preserve, or certify.
- Failure artifacts must be explicitly named as failures, not hidden in success-shaped names.
- If a term has a deprecated alias, the canonical term comes first and the alias is listed in the artifact registry or conflict ledger.
- Names may not silently absorb authority. A renamed artifact is still the same constitutional class only if its binding contract says so.

## Usage Rules

- Use `Committed` for the canonical cycle-local artifact.
- Use `Committed Supervisory Mode` for the authoritative mode established for the next continuity boundary.
- Use `Committed Supervisory Continuity Record` for the committed evidence and lineage artifact establishing how and why that mode became authoritative.
- Use `Constructed Supervisory Lineage` for the non-authoritative Stage Two lineage assembly, and use the explicit completeness and integrity result names for its evaluations.
- Use `Constructed Supervisory Cycle Record` for the precommit whole-cycle assembly, `Supervisory Cycle Commitment Record` for the commitment event, and `Committed Supervisory Cycle Record` for the authoritative whole-cycle history.
- Use `Committed Supervisory Provenance Failure` for the exclusive terminal Stage Two failure artifact.
- Use `Replay Verification Result` for replay evidence and `Audit Representation` for non-constitutive audit findings; neither is a replacement for committed history.
- Use `External Verification Result` for the foreign verifier's judgment, `Committed Verification Result` for SACS' immutable representation of that judgment, and `Committed Verification Exchange Record` for the complete exchange history.
- Use `Committed Verification Exchange Failure` only when the exchange itself cannot be constitutionally completed or preserved, never for `NOT_VERIFIED`, `INCONCLUSIVE`, `INSUFFICIENT_EVIDENCE`, `REJECTED`, or `OUT_OF_SCOPE` judgments issued by the verifier.
- Use `Constitutional Custody` only for bounded temporary control; do not imply transfer of ownership, sovereignty, historical authority, or commitment authority.
- Use `Registry Snapshot` when the contract requires a frozen visibility set.
- Use `Candidate` only before selection.
- Use `Validated` only after a contract-defined check has succeeded.
- Use `Certified` only where the owning contract explicitly confers certification authority.
- Use `Historical Certification` only as an archived or descriptive label for the Contract 011 realization-certification stage; use `Realization Certification, Supervisory Provenance, and Audit` for the active Contract 011 title.
- Use `Realization Certification, Supervisory Provenance, and Audit` for the role owned by Contract 011 in the newer corpus state.

## v1.0.0 Terminology Freeze

The terminology in this file is frozen for SACS v1.0.0. A canonical term SHALL NOT be renamed, collapsed, broadened, or assigned additional authority except through a formally versioned constitutional amendment. Deprecated aliases remain only for historical traceability and SHALL NOT be treated as current canonical terms.

## Source Traceability

- `SACS Contract 000 Architecture_and_Authority v0.1.0.md` - normative language and terminology boundaries.
- `SACS Contract 001 Observation_v0.1.0.md` - observation vocabulary.
- `SACS Contract 006 Section 1 Op Profiles-Reference States.md` - operational profile naming and distinction.
- `SACS Contract 006 Section 2 Operational Profile Registry.md` - registry entry and snapshot terminology.
- `SACS Contract 006 Section 3 Operational Profile Registry.md` - candidate, eligibility, and report vocabulary.
- `SACS Contract 006 Section 4 Compatibility Evaluation.md` - compatibility vocabulary.
- `SACS Contract 006 Section 5 Deterministic Profile Selection.md` - selection report, tie-breaking, and no-winner vocabulary.
- `SACS Contract 006 Section 6 Committed Ops Profile Construction.md` - committed profile and authority reduction certificate.
- `SACS Contract 007 Controller.txt` - controller and control-command terminology.
- `SACS Contract 008 Inference Control.txt` and section files - translation and transaction terminology.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - realization record and supervisory cycle record terminology.
