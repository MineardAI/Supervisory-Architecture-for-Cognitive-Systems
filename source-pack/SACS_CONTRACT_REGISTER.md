# SACS Contract Register

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012  
Classification: Constitutional-supporting contract register

## Contract 000

Contract Number: 000  
Title: Architecture and Authority  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Establishes the constitutional structure, authority model, and contract ownership map.  
Constitutional Question: What is SACS, what is it not, and which contract owns each authority?  
Normative Authority: Foundational architecture and authority boundaries.  
Depends On: None.  
Canonical Inputs: None; this is the normative root.  
Canonical Internal Artifacts: Architecture model, flow model, contract ownership map, conformance rules.  
Canonical Success Output: The constitutional framework for all later contracts.  
Canonical Failure Output: N/A; if disputed, the conflict belongs in the ledger.  
Canonical Consumer: The entire corpus.  
Owns: Project identity, normative language, canonical flow, authority model, conformance rules, contract numbering claims.  
Must Never Do: Become a substitute for later contract-specific authority.  
Fundamental Invariant: No downstream contract may broaden authority beyond the architecture root.  
Current Drafting State: Active foundational reference; still contains historical numbering assumptions for later contracts.  
Known Conflicts or Open Questions: Contract 010 retains supervisory continuity authority; the former historical-certification draft has been reassigned to Contract 011; Contract 012 now owns the external-verification interface.  
Primary Source Documents: `SACS Contract 000 Architecture_and_Authority v0.1.0.md`

## Contract 001

Contract Number: 001  
Title: Observation  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Canonical entry point for runtime evidence.  
Constitutional Question: What evidence is admissible into SACS?  
Normative Authority: Foundational runtime input.  
Depends On: Contract 000.  
Canonical Inputs: Runtime evidence, source declarations, availability states, observation windows.  
Canonical Internal Artifacts: Observation records, provenance, quality metadata, handoff packaging.  
Canonical Success Output: Observation records consumed by State Estimation.  
Canonical Failure Output: Explicit observation failure or invalid/unavailable observation record.  
Canonical Consumer: Contract 002.  
Owns: Observation authority, source identity, timing, ordering, provenance, availability semantics.  
Must Never Do: Assign supervisory meaning or become a control decision.  
Fundamental Invariant: Observation records evidence without deciding what it means.  
Current Drafting State: Foundation contract with extensive test corpus.  
Known Conflicts or Open Questions: None material in the pack corpus.  
Primary Source Documents: `SACS Contract 001 Observation_v0.1.0.md`

## Contract 002

Contract Number: 002  
Title: State Estimation  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Source Detail: Foundational source material is less granular than later contracts.  
Constitutional Role: Reconstructs estimated state from committed observations.  
Constitutional Question: What does the observation imply about system state?  
Normative Authority: State estimation and uncertainty management.  
Depends On: Contracts 000 and 001.  
Canonical Inputs: Committed observations, observation lineage, estimator configuration.  
Canonical Internal Artifacts: Estimated state, estimator report, lineage records.  
Canonical Success Output: Estimated state for OSV publication.  
Canonical Failure Output: Explicit estimation failure.  
Canonical Consumer: Contract 003.  
Owns: Estimation semantics, uncertainty handling, estimator provenance.  
Must Never Do: Mutate observation history or authorize execution.  
Fundamental Invariant: Estimation must be deterministic over identical committed inputs and registered estimator definitions.  
Current Drafting State: Material exists but was not re-read in full for this pack.  
Known Conflicts or Open Questions: None material in the pack corpus.  
Primary Source Documents: `SACS Contract 002 State Estimation.txt`

## Contract 003

Contract Number: 003  
Title: Operational State Vector  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Source Detail: Foundational source material is less granular than later contracts.  
Constitutional Role: Publishes a temporally atomic supervisory state vector.  
Constitutional Question: What is the canonical state for this cycle?  
Normative Authority: OSV publication and temporal atomicity.  
Depends On: Contracts 000, 001, 002.  
Canonical Inputs: Estimated state, committed observations, lineage, schema/version rules.  
Canonical Internal Artifacts: OSV draft, OSV publication record, lineage attachments.  
Canonical Success Output: Committed Operational State Vector.  
Canonical Failure Output: Explicit OSV publication failure.  
Canonical Consumer: Contract 004.  
Owns: OSV schema, publication atomicity, partial-publication isolation.  
Must Never Do: Merge incompatible cycles or silently repair missing inputs.  
Fundamental Invariant: One cycle, one committed OSV.  
Current Drafting State: Material exists but was not re-read in full for this pack.  
Known Conflicts or Open Questions: None material in the pack corpus.  
Primary Source Documents: `SACS Contract 003 Operational State Vector.txt`

## Contract 004

Contract Number: 004  
Title: Governance  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Source Detail: Foundational source material is less granular than later contracts.  
Constitutional Role: Applies externally supplied governance constraints and dispositions.  
Constitutional Question: What governance constraints govern this cycle?  
Normative Authority: Governance snapshot and revocation semantics.  
Depends On: Contracts 000 and 003.  
Canonical Inputs: Committed OSV, governance source, revocation or restriction records.  
Canonical Internal Artifacts: Governance snapshot, disposition records, revocation evidence.  
Canonical Success Output: Committed Governance Snapshot.  
Canonical Failure Output: Explicit governance failure or withheld disposition.  
Canonical Consumer: Contract 005 and later contracts.  
Owns: Governance binding, restriction semantics, revocation enforcement.  
Must Never Do: Select objectives or execute commands.  
Fundamental Invariant: Governance is authoritative over downstream supervisory preference.  
Current Drafting State: Material exists but was not re-read in full for this pack.  
Known Conflicts or Open Questions: None material in the pack corpus.  
Primary Source Documents: `SACS Contract 004 Governance.txt`

## Contract 005

Contract Number: 005  
Title: Supervisory Objectives  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Source Detail: Foundational source material is less granular than later contracts.  
Constitutional Role: Derives and evaluates supervisory objectives under governance.  
Constitutional Question: What should be preserved or approached within the authorized boundary?  
Normative Authority: Objective evaluation and priority handling.  
Depends On: Contracts 000, 003, 004.  
Canonical Inputs: Committed OSV, committed governance snapshot, objective source material.  
Canonical Internal Artifacts: Objective candidate set, priority order, evaluation results.  
Canonical Success Output: Committed Objective Set.  
Canonical Failure Output: Explicit objective evaluation failure or restricted objective set.  
Canonical Consumer: Contract 006.  
Owns: Objective applicability, priority, determinism, governance-respecting evaluation.  
Must Never Do: Broaden permission, weaken prohibition, or select modes.  
Fundamental Invariant: Objective evaluation must fail toward reduced supervisory authority.  
Current Drafting State: Foundational objective material is present; supervisory-continuity and mode-transition authority belongs to Contract 010.  
Known Conflicts or Open Questions: The archived historical-certification draft was originally misnumbered as Contract 010 before reassignment to Contract 011.  
Primary Source Documents: `SACS Contract 005 Supervisory Objectives.txt`

## Contract 006

Contract Number: 006  
Title: Operational Profiles and Reference States  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Selects profile, constructs reference state, and prepares controller handoff.  
Constitutional Question: Given this state, governance, and objectives, what supervisory configuration should exist?  
Normative Authority: Operational profile selection, reference-state construction, and controller handoff.  
Depends On: Contracts 000, 003, 004, 005.  
Canonical Inputs: Committed OSV, committed governance snapshot, committed objective set, committed registry snapshots.  
Canonical Internal Artifacts: Registry entries, candidate set, eligibility report, compatibility report, selection report, committed operational profile, committed reference state.  
Canonical Success Output: Committed Operational Profile and Committed Reference State.  
Canonical Failure Output: Explicit selection, compatibility, or construction failure.  
Canonical Consumer: Contract 007.  
Owns: Profile definitions, registry, eligibility, compatibility, deterministic selection, reference construction.  
Must Never Do: Compute corrective control.  
Fundamental Invariant: Configuration is not control.  
Current Drafting State: The most mature pre-controller architectural sequence in the corpus.  
Known Conflicts or Open Questions: Older archive titles use "Operational Profiles & Reference States"; section naming evolved.  
Primary Source Documents: `SACS Contract 006 Section 1 Op Profiles-Reference States.md`, `SACS Contract 006 Section 2 Operational Profile Registry.md`, `SACS Contract 006 Section 3 Operational Profile Registry.md`, `SACS Contract 006 Section 4 Compatibility Evaluation.md`, `SACS Contract 006 Section 5 Deterministic Profile Selection.md`, `SACS Contract 006 Section 6 Committed Ops Profile Construction.md`, archive versions in `archives/`

## Contract 007

Contract Number: 007  
Title: Controller  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Computes bounded corrective control and commits command authorization.  
Constitutional Question: What bounded corrective action follows from the committed deviation, configuration, state, and authority?  
Normative Authority: Control error, controller state, authority binding, controller composition compatibility, command authorization, and controller decision commitment.  
Depends On: Contracts 000, 003, 004, 005, 006.  
Canonical Inputs: Committed OSV, committed governance snapshot, committed objective set, committed operational profile, committed reference state, committed controller configuration, committed controller state set, committed numeric profile, committed registry definitions.  
Canonical Internal Artifacts: Control error, control authority binding, constraint set, controller evaluation result, proposed control command, controller commitment record, next controller state outcome.  
Canonical Success Output: Committed Control Command.  
Canonical Failure Output: Explicit controller failure artifact.  
Canonical Consumer: Contract 008.  
Owns: Controller state, composition evaluation, command authorization, commitment.  
Must Never Do: Select objectives, select profiles, or execute commands.  
Fundamental Invariant: The controller computes correction without redefining supervisory intent.  
Current Drafting State: Integrated redraft with detailed section structure.  
Known Conflicts or Open Questions: Earlier archive naming differs from the integrated redraft.  
Primary Source Documents: `SACS Contract 007 Controller.txt` and section files under the same prefix

## Contract 008

Contract Number: 008  
Title: Inference Control  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Translates one committed control command into one committed runtime-specific translation transaction.  
Constitutional Question: How is this exact supervisory correction represented for one declared runtime?  
Normative Authority: Semantic-preserving inference command translation.  
Depends On: Contracts 000, 004, 006, 007.  
Canonical Inputs: Committed control command, committed runtime capability profile, committed translation configuration, committed translation numeric profile, committed registry snapshot set, authority references.  
Canonical Internal Artifacts: Translation profile, translation definition, translation validation report, Committed Inference Command Set, translation transaction, execution interface handoff envelope.  
Canonical Success Output: Committed Translation Transaction.  
Canonical Failure Output: Explicit Inference Translation Failure.  
Canonical Consumer: Contract 009.  
Owns: Translation semantics, registry binding, transaction construction, handoff eligibility.  
Must Never Do: Re-evaluate controller output or execute the command.  
Fundamental Invariant: Translation preserves supervisory meaning and may not broaden authority.  
Current Drafting State: Integrated redraft with a detailed transaction/commitment split.  
Known Conflicts or Open Questions: Contract 009 is now present as an authoritative baseline draft; later Council refinement remains possible.  
Primary Source Documents: `SACS Contract 008 Inference Control.txt`, `SACS Contract 008 Section 6 Committed Control Command Consumption Model.txt`, `SACS Contract 008 Section 18 Translation Transaction Rules.txt`, `SACS Contract 008 Section 19 Translation Transaction Commitment.txt`

## Contract 009

Contract Number: 009  
Title: Execution Interface  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Accepts the handed-off translated command and determines execution disposition.  
Constitutional Question: What execution disposition follows from the handed-off command?  
Normative Authority: Execution interface and disposition handling.  
Depends On: Contract 008 and upstream commitments.  
Canonical Inputs: one Committed Translation Transaction, one Execution Interface Handoff Envelope, and referenced validation, compatibility, registry, and provenance evidence.  
Canonical Internal Artifacts: Execution plan, execution outcome, committed execution disposition.  
Canonical Success Output: Committed Execution Disposition.  
Canonical Failure Output: Explicit execution failure or withheld disposition.  
Canonical Consumer: Contract 010 continuity evaluation and Contract 011 realization certification.  
Owns: Execution interface dispositioning, handoff reception, execution outcome materialization.  
Must Never Do: Redefine governance or rewrite translation semantics.  
Fundamental Invariant: Execution is downstream of commitment and translation.  
Current Drafting State: Authoritative baseline draft present and aligned to Contract 010 continuity evaluation and Contract 011 realization certification.  
Known Conflicts or Open Questions: Legacy corpus references still point to the archived historical-certification draft under Contract 010; the conflict ledger preserves that history, while the active canonical owner is Contract 011.  
Primary Source Documents: `SACS Contract 009 Execution Interface.txt`

## Contract 010

Contract Number: 010  
Title: Modes, Guards, Resets, Hysteresis, and Dwell Time  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Preserves supervisory continuity across cycles through mode evaluation, guard predicates, transition precedence, resets, hysteresis, and dwell-time enforcement.  
Constitutional Question: Which supervisory mode remains authoritative for the next cycle?  
Normative Authority: Supervisory continuity and mode transition authority.  
Depends On: Contract 009 execution disposition references and upstream supervisory state.  
Canonical Inputs: Committed OSV, governance snapshot, current supervisory mode, guard definitions, reset definitions, dwell-time definitions, transition policy, and continuity references.  
Canonical Internal Artifacts: Mode transition evaluation, guard evaluation, transition precedence result, reset application, mode commitment record, Committed Supervisory Continuity Record.  
Canonical Success Output: Committed Supervisory Mode and Committed Supervisory Continuity Record.  
Canonical Failure Output: Explicit mode-transition failure or withheld continuity decision.  
Canonical Consumer: Contract 011.  
Owns: Supervisory mode continuity, mode transition evaluation, guard predicates, resets, hysteresis, dwell-time enforcement, and mode commitment.  
Must Never Do: Perform execution, observation, governance, or historical certification.  
Fundamental Invariant: Supervisory continuity is established only through explicit constitutional mode commitment.  
Current Drafting State: Foundational Contract 010 supervisory-continuity material is active and the Committed Supervisory Continuity Record name is adopted.  
Known Conflicts or Open Questions: None for the Contract 010 canonical artifact name.  
Primary Source Documents: `SACS Contract 010 Section 1 Modes Guards Resets Dwell Time.md`, `SACS Contract 010 Section 2 Modes Guards Resets Dwell Time.md`, `SACS Contract 010 Section 3 Modes Guards Resets Dwell Time.md`, `SACS Contract 010 Section 4 Modes Guards Resets Dwell Time.md`, and `SACS Contract 010 Section 5 Modes Guards Resets Dwell Time.md`

## Contract 011

Contract Number: 011  
Title: Realization Certification, Supervisory Provenance, and Audit  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Certifies realization history, assembles provenance, and commits the whole-cycle supervisory record.  
Constitutional Question: What realization and cycle history become authoritative constitutional record?  
Normative Authority: Realization certification, historical finality, provenance, audit, replay, and cycle-record commitment.  
Depends On: Contract 009 and Contract 010.  
Canonical Inputs: Stage One consumes the Committed Execution Disposition from Contract 009 and the Committed Supervisory Continuity Record from Contract 010; Stage Two consumes only the Committed Realization Record and resolves additional committed cycle artifacts where required.  
Canonical Internal Artifacts: Constructed Realization Record, Committed Realization Record, Constructed Supervisory Lineage, Lineage Completeness Evaluation Result, Lineage Integrity Evaluation Result, canonical artifact-resolution set, provenance graph, audit index, replay package, Constructed Supervisory Cycle Record, Supervisory Cycle Commitment Record, Replay Verification Result, Audit Representation.  
Canonical Stage One Handoff: Committed Realization Record (internal Contract 011 handoff).  
Canonical Success Output: Committed Supervisory Cycle Record (terminal Contract 011 output).  
Canonical Failure Output: Realization Certification Failure or Committed Supervisory Provenance Failure.  
Canonical Consumer: Contract 012 External Verification Profile and Evidence Interface.  
Owns: Realization certification, historical lineage, provenance binding, audit assembly, cycle record commitment, replay packaging.  
Must Never Do: Reopen execution adjudication, reopen mode-transition adjudication, or perform external verification.  
Fundamental Invariant: Every supervisory cycle SHALL terminate in exactly one immutable constitutional outcome that faithfully preserves every committed constitutional transformation from operational intent through supervisory realization, producing one authoritative supervisory history that remains deterministic, reproducible, independently auditable, and permanently traceable without modification, reinterpretation, duplication, or loss of constitutional identity.  
Current Drafting State: Contract 011 Sections 1-28 are assembled in the canonical draft.  
Known Conflicts or Open Questions: None for the active Contract 011 to Contract 012 handoff.  
Primary Source Documents: `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` and the numbered Contract 011 section drafts.

## Contract 012

Contract Number: 012  
Title: External Verification Profile and Evidence Interface  
Status: ADOPTED - SACS v1.0.0 BASELINE  
Constitutional Role: Establishes the external-verification profile, claim, evidence, submission, custody, result-admission, result-certification, exchange-history, replay, and audit boundaries for one sovereign verification exchange.  
Constitutional Question: How may SACS submit committed claims and evidence to an independent constitutional verifier, receive the verifier's sovereign judgment, and commit an immutable SACS representation of the completed exchange without transferring or merging constitutional authority?  
Normative Authority: Constitutionally bounded external verification and the inter-constitutional trust exchange between SACS and an independent verification authority.  
Depends On: Contracts 000-011, especially the Committed Supervisory Cycle Record from Contract 011.  
Canonical Input: Committed Supervisory Cycle Record, with committed historical and evidentiary artifacts required for the verification claim, evidence package, and submission.  
Canonical Success Output: Committed Verification Exchange Record.  
Canonical Failure Output: Committed Verification Exchange Failure.  
Canonical Consumer: Authorized later archival, inspection, or ecosystem-level learning modules operating only on committed history. No current SACS Contract 013 is established.  
Owns: SACS-side construction and commitment of verification submissions, admission and certification of external results, custody records, and immutable exchange history.  
Must Never Do: Perform self-verification; merge SACS and verifier authority; reconstruct undisclosed external reasoning; modify the external judgment; treat unfavorable results as exchange failures; permit replay or audit to alter history.  
Fundamental Invariant: For each Verification Exchange Identity, `Count(Committed Verification Exchange Record) + Count(Committed Verification Exchange Failure) = 1`; the exchange preserves claims, evidence, custody, verifier judgment, admission, certification, limitations, and lineage without authority transfer or constitutional merger.  
Current Drafting State: Sections 1-23 completed and assembled; Section 23 is the terminal architectural-completion section and Section 22 remains the Fundamental Inter-Constitutional Trust Exchange Invariant.  
Known Conflicts or Open Questions: No unresolved Contract 012 existence or placement conflict. Institutional learning and constitutional evolution remain outside the completed SACS 000-012 core.  
Primary Source Documents: `SACS Contract 012 External Verification Profile and Evidence Interface.txt` and the numbered Contract 012 Section 2 through Section 22 files.

## Source Traceability

- `SACS Contract 000 Architecture_and_Authority v0.1.0.md` - root architecture, contract chain, and contract 010/011 references.
- `SACS Contract 001 Observation_v0.1.0.md` - Contract 001 identity and consumer boundary.
- `SACS Contract 002 State Estimation.txt` - Contract 002 exists as part of the corpus.
- `SACS Contract 003 Operational State Vector.txt` - Contract 003 existence and role.
- `SACS Contract 004 Governance.txt` - Contract 004 existence and role.
- `SACS Contract 005 Supervisory Objectives.txt` - Contract 005 role and deferral to Contract 010 continuity authority.
- `SACS Contract 006 Section 1 Op Profiles-Reference States.md` and section files - Contract 006 integrated redraft.
- `SACS Contract 007 Controller.txt` and section files - Contract 007 integrated redraft.
- `SACS Contract 008 Inference Control.txt` and section files - Contract 008 integrated redraft and Contract 009 boundary.
- `SACS Contract 010 Section 1 Modes Guards Resets Dwell Time.md` through `SACS Contract 010 Section 5 Modes Guards Resets Dwell Time.md` - Contract 010 continuity corpus.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - integrated Contract 011 draft.
- `SACS Contract 012 External Verification Profile and Evidence Interface.txt` - assembled Contract 012 Sections 1-23.
- `SACS Contract 012 Section 2 Authority Boundary.txt` through `SACS Contract 012 Section 23 Architectural Completion-Successor Contracts.txt` - numbered Contract 012 section sources.
