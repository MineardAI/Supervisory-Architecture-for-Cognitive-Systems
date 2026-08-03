# SACS Conflict And Decision Ledger

Version: 1.0.0  
Status: ADOPTED  
Baseline: SACS Constitutional Baseline v1.0.0  
Normative Scope: Contracts 000-012  
Classification: Constitutional-supporting historical decision ledger

## Issue 001

Issue ID: 001  
Topic: Contract 010 numbering and role  
Earlier Position: Contract 000 assigns Contract 010 to modes, guards, resets, hysteresis, and dwell time.  
Later Position: Newer redraft material assigns Contract 010 to historical certification of execution history.  
Relevant Contracts: 000, 005, 008, 010  
Council Assessment: The corpus contains a genuine architectural conflict, not a simple wording drift.  
User Decision: The active Contract 010 continuity corpus is the current authoritative baseline; the historical-certification material is assigned to Contract 011.  
Canonical Resolution: Preserve the historical conflict as traceability, but treat Contract 010 as the canonical modes, guards, resets, hysteresis, and dwell-time authority and Contract 011 as the canonical realization-certification, provenance, and audit authority.  
Superseded Language: Any claim that Contract 010 owns historical certification in the current architecture.  
Remaining Ambiguity: None for the active 000-012 authority chain.  
Required Future Action: Preserve the conflict as historical traceability and keep active summaries aligned to Contracts 009, 010, and 011.

## Issue 002

Issue ID: 002  
Topic: Modes, guards, resets, and dwell time versus historical certification  
Earlier Position: Modes are the canonical subject of 010 in the older architecture.  
Later Position: Historical certification was assigned to 010 in an earlier redraft, then restored to 011 by the current active corpus.  
Relevant Contracts: 000, 005, 010  
Council Assessment: Both versions are internally coherent but mutually exclusive.  
User Decision: The active Contract 010 modes, guards, resets, hysteresis, and dwell-time material is adopted as the current baseline.  
Canonical Resolution: Treat the historical-certification assignment to 010 as superseded and preserve the active mode-continuity authority in Contract 010; Contract 011 owns realization certification, provenance, and audit.  
Superseded Language: Any current summary that assigns historical certification to Contract 010.  
Remaining Ambiguity: None for the Contract 010/011 ownership boundary; Contract 012 is now assembled and owns the external-verification exchange boundary.  
Required Future Action: Keep Contract 010 and Contract 011 summaries aligned to their active contracts.

## Issue 003

Issue ID: 003  
Topic: Contract 011 provenance and audit ownership  
Earlier Position: Contract 000 references Contract 011 in the conformance chain.  
Later Position: A standalone Contract 011 draft is now present in the corpus.  
Relevant Contracts: 000, 010, 011  
Council Assessment: The authority is now sourced and must be aligned to the 009 -> 010 -> 011 chain.  
User Decision: Provisional baseline accepted for the current pack context.  
Canonical Resolution: Treat Contract 011 as the canonical supervisory provenance and audit layer and Contract 012 as the canonical external-verification exchange authority.  
Superseded Language: Any claim that Contract 011 is missing or partial.  
Remaining Ambiguity: None for Contract 012 existence; institutional learning and constitutional evolution remain outside the completed SACS 000-012 core.  
Required Future Action: Keep Contract 011 aligned to the committed realization record and downstream audit use.

## Issue 004

Issue ID: 004  
Topic: Observation consuming runtime telemetry versus certified historical records  
Earlier Position: Observation is the canonical runtime entry point.  
Later Position: Historical certification creates a record for downstream historical reasoning, but it does not replace current observation of runtime telemetry.  
Relevant Contracts: 001, 010  
Council Assessment: These are different temporal points and must remain distinct.  
User Decision: None recorded.  
Canonical Resolution: Observation consumes runtime evidence and operational telemetry; historical records may provide correlation, provenance, temporal continuity, and prior-cycle context, but they are not substitutes for current observation.  
Superseded Language: Any claim that observation is the same as historical certification.  
Remaining Ambiguity: How future sections will use historical records as inputs without collapsing them into telemetry.  
Required Future Action: Define the future observation-vs-history interface if needed.

## Issue 005

Issue ID: 005  
Topic: Committed Realization Record as the intermediate artifact versus Committed Supervisory Cycle Record  
Earlier Position: The older terminology leans toward realization language.  
Later Position: The current Contract 011 draft integrates Committed Realization Record as the first-stage canonical artifact before the Committed Supervisory Cycle Record.  
Relevant Contracts: 008, 010, 011  
Council Assessment: The names identify distinct Contract 011 stages: Committed Realization Record is the first-stage historical artifact, while Committed Supervisory Cycle Record is the whole-cycle provenance artifact.  
User Decision: None recorded.  
Canonical Resolution: Use Committed Realization Record for the first-stage realization-certification artifact and Committed Supervisory Cycle Record for the whole-cycle provenance artifact; list Committed Historical Record as deprecated.  
Superseded Language: "Committed Historical Record" as the primary term.  
Remaining Ambiguity: None for the current pack baseline.  
Required Future Action: Preserve the adopted distinction between the first-stage Committed Realization Record and the whole-cycle Committed Supervisory Cycle Record.

## Issue 006

Issue ID: 006  
Topic: Committed Inference Command Set versus Committed Translation Transaction  
Earlier Position: The translated runtime command is the canonical output of Contract 008.  
Later Position: The transaction structure and the translated command are separated in the integrated redraft.  
Relevant Contracts: 008, 009  
Council Assessment: The separation is architecturally sound and should be preserved.  
User Decision: None recorded.  
Canonical Resolution: The pack treats the committed translation transaction as the canonical translation artifact, with the committed inference command set as the runtime-facing semantic result.  
Superseded Language: Any model that collapses transaction and translated command into one undifferentiated object.  
Remaining Ambiguity: None for the current 009 interface baseline.  
Required Future Action: Preserve the committed translation transaction, execution-interface handoff, and Contract 009 disposition boundary as the adopted interface.

## Issue 007

Issue ID: 007  
Topic: Earlier versus revised Contract 006 artifact model  
Earlier Position: Older 006 material used a simpler operational profile model.  
Later Position: The integrated redraft adds explicit registry snapshots, eligibility, compatibility, and selection evidence.  
Relevant Contracts: 006  
Council Assessment: The integrated redraft is more complete and should be treated as the canonical pack basis.  
User Decision: None recorded.  
Canonical Resolution: Preserve the richer registry/selection model and note the older shorthand as historical terminology.  
Superseded Language: Any simplified 006 model that omits explicit registry or compatibility artifacts.  
Remaining Ambiguity: None material for the pack.  
Required Future Action: Keep the detailed artifact chain as the drafting baseline.

## Issue 008

Issue ID: 008  
Topic: Earlier controller section numbering versus integrated redraft  
Earlier Position: Controller material appeared in a less explicit layout.  
Later Position: The integrated redraft names explicit internal artifacts, outputs, and commitment boundary.  
Relevant Contracts: 007  
Council Assessment: The integrated redraft should be treated as the stronger authority for this pack.  
User Decision: None recorded.  
Canonical Resolution: Use the integrated redraft terminology and the explicit controller commitment record.  
Superseded Language: Any controller model that omits commitment or state continuity.  
Remaining Ambiguity: None material for the pack.  
Required Future Action: Preserve the section-level controller sequence in future drafting.

## Issue 009

Issue ID: 009  
Topic: Whether failure records require explicit commitment  
Earlier Position: Some sections lean toward explicit failure artifacts; others focus on success artifacts.  
Later Position: The mature sequence requires failure to be explicit when success cannot be established.  
Relevant Contracts: 006, 007, 008, 010  
Council Assessment: Failure is constitutionally meaningful and should not be hidden.  
User Decision: None recorded.  
Canonical Resolution: Where a contract has a terminal failure path, treat the failure artifact as explicit and immutable.  
Superseded Language: Silent fallback, silent repair, or best-effort substitution.  
Remaining Ambiguity: Which future contracts require a separately committed failure record versus a simpler failure output.  
Required Future Action: Standardize failure artifact commitment rules by contract.

## Issue 010

Issue ID: 010  
Topic: Internal certification versus external truth verification  
Earlier Position: SACS validates and certifies within its own constitutional boundary.  
Later Position: NullBlade owns external verification.  
Relevant Contracts: 000, 008, 010  
Council Assessment: The boundary is clear and must be preserved.  
User Decision: None recorded.  
Canonical Resolution: SACS may certify canonical artifacts, but it may not claim external verification.  
Superseded Language: Any claim that internal conformance equals external truth.  
Remaining Ambiguity: None for the pack.  
Required Future Action: Keep the verification boundary explicit in every future draft.

## Issue 011

Issue ID: 011  
Topic: Contract 011 two-stage commitment and provenance boundary  
Relevant Contracts: 009, 010, 011, 012  
Canonical Resolution: Contract 011 Sections 1-28 are assembled in the canonical draft and use two independent commitment boundaries. Stage One commits the Committed Realization Record as an internal constitutional handoff; Stage Two consumes only that committed artifact and commits the terminal Committed Supervisory Cycle Record. Construction, commitment, and committed artifact identity remain distinct operations. Stage Two terminates in exactly one Committed Supervisory Cycle Record or one Committed Supervisory Provenance Failure.  
Authority Boundary: Contract 011 does not reopen Contracts 009 or 010, does not reconstruct or substitute missing artifacts, and does not perform external verification. Artifact resolution, relationship resolution, lineage construction, completeness evaluation, integrity evaluation, and commitment remain separate operations.  
Failure Doctrine: Failure does not authorize reconstruction, substitution, inferred history, or an alternate canonical output. Replay is verification-only and produces a Replay Verification Result; audit is non-constitutive and produces an Audit Representation. Neither modifies or replaces the terminal outcome. The final invariant is `COUNT(Committed Supervisory Cycle Record) + COUNT(Committed Supervisory Provenance Failure) = 1`.  
Remaining Ambiguity: None for the active Contract 011 to Contract 012 handoff.

## Issue 012

Issue ID: 012  
Topic: Contract 012 external verification and inter-constitutional trust exchange  
Relevant Contracts: 000, 011, 012  
Earlier Position: Contract 012 was referenced as a future downstream authority but was absent from the active source corpus.  
Canonical Resolution: Contract 012 now exists as `External Verification Profile and Evidence Interface`, Sections 1-23. Sections 1-22 govern one bounded inter-constitutional verification exchange through immutable exchange history; Section 23 establishes architectural completion and the boundary to any future successor contracts without creating successor authority. Its terminal outcome is exactly one Committed Verification Exchange Record or one Committed Verification Exchange Failure.  
Authority Boundary: Verification responsibility may cross the external-verification boundary, but constitutional ownership, sovereignty, historical authority, and commitment authority do not transfer. The external verifier remains authoritative for its judgment; SACS remains authoritative for submission construction, result admission, internal certification, and exchange-history commitment.  
Replay and Audit Doctrine: Replay reproduces only SACS-controlled exchange material and does not recreate undisclosed external reasoning. Replay and audit are retrospective and non-constitutive.  
Scope Resolution: Institutional learning, verifier-governance evolution, constitutional amendment, and successor contracts are outside the completed SACS 000-012 core; no Contract 013 or 014 is established.  
Remaining Ambiguity: None for Contract 012 existence, placement, or terminal exchange semantics.

## Source Traceability

- `SACS Contract 000 Architecture_and_Authority v0.1.0.md` - numbering, authority chain, conformance, and 010/011 references.
- `SACS Contract 005 Supervisory Objectives.txt` - explicit statement that Contract 010 owns modes in the older model.
- `SACS Contract 008 Inference Control.txt` - Contract 009 boundary and translation failure semantics.
- `SACS Contract 008 Section 19 Translation Transaction Commitment.txt` - commitment/handoff boundary and Contract 009 reference.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - realization certification and cycle record terminology.
- `SACS Contract 011 Realization Certification, Supervisory Provenance, and Audit.txt` - integrated Contract 011 role.
