# Type Implementation Matrix

| Type | Crate | Authority owned | State/purpose | Prohibited behavior |
|---|---|---|---|---|
| `CapabilityProfile` | constitutional-conformance | none; declaration only | implementation capability/version declaration | no compatibility downgrade or policy |
| `Version`, `VersionBinding` | canonical-versioning | none | explicit version dimensions | no inferred compatibility |
| `CanonicalDocument` | canonical-serialization | none | deterministic ordered field encoding | no contract schema policy |
| hash functions | canonical-hashing | none | SHA-256 over canonical bytes | no transport-dependent hashing |
| `ArtifactIdentity` | canonical-identity | none | deterministic identity binding | no random identity or authority |
| `Lifecycle`, `Artifact<T>` | canonical-artifacts | none | type-level lifecycle distinction | no policy transition decision |
| `CommitmentStore` | commitment-engine | none | atomic/idempotent mechanism | never decides whether to commit |
| `CommitmentEvent` | commitment-engine | none | append-only event evidence | not constitutional history |
| `LineageEdge` | lineage-core | none | reusable relationship primitive | no history/replay/audit construction |
| `FailureRecord` | failure-model | none | immutable typed failure | no null/implicit failure |
| `SourceDeclaration` | sacs-observation | Contract 001 source admission | registered source/capability binding | no capability-to-availability inference |
| `ObservationWindow` | sacs-observation | Contract 001 window lifecycle | bounded open/finalized collection | no reopen or post-close mutation |
| `ObservationRecord` | sacs-observation | Contract 001 observation evidence | typed source-attributed record | no supervisory interpretation |
| `ObservationBatch` | sacs-observation | Contract 001 committed evidence bundle | immutable ordered batch | no estimation or OSV construction |
| `ObservationHandoff` | sacs-observation | Contract 001 handoff packaging | committed batch only | no raw/open/uncommitted input to 002 |
| `ExpectedObservationManifest` | sacs-observation | Contract 001 expected evidence declaration | required/optional/conditional source expectations | no inferred defaults for missing evidence |
| `ObservationProvenance` | sacs-observation | Contract 001 provenance | source/adapter/window/validation/normalization binding | no hidden derivation lineage |
| `AdapterDeclaration` | sacs-observation | Contract 001 adapter declaration | transport/schema/availability mapping declaration | no invented source semantics |
| `EstimatorDefinition` | sacs-state-estimation | Contract 002 estimator registry | immutable estimator identity/version | no correctness or external-verification claim |
| `EstimatorProfile` | sacs-state-estimation | Contract 002 execution profile | explicit input/treatment/numeric policy | no ambient defaults |
| `StateBasisProfile` | sacs-state-estimation | Contract 002 state basis | caller-bound component definitions | no OSV field semantics |
| `EstimatorConfiguration` | sacs-state-estimation | Contract 002 cycle binding | explicit estimator/profile/basis/context | no hidden prior state |
| `EstimatedState` | sacs-state-estimation | Contract 002 canonical output | immutable uncertainty-aware estimate | not an OSV |
| `EstimatorReport` / `EstimatorProvenance` | sacs-state-estimation | Contract 002 evidence | input treatment and lineage | no supervisory authority |
| `EstimatedStateHandoff` | sacs-state-estimation | Contract 002 to 003 boundary | immutable state/report/provenance package | no OSV mapping/publication |
| `GovernanceAuthorityRegistration` / `GovernanceSourceArtifact` | sacs-governance | Contract 004 external governance boundary | explicit authority, source, version, scope, integrity binding | no authority invention |
| `GovernanceAdapterDeclaration` | sacs-governance | Contract 004 adapter boundary | declared normalization/compatibility/failure mapping | no policy authoring |
| `GovernanceRule` / `GovernanceRuleClass` | sacs-governance | Contract 004 canonical rule model | distinct permission/prohibition/obligation and related classes | no semantic collapse |
| `GovernanceAdmissionContext` / `GovernanceApplicabilityResult` | sacs-governance | Contract 004 applicability | explicit cycle, OSV, scope, version, and effective-time context | no ambient clock/source |
| `GovernanceSnapshotCandidate` / `GovernanceValidationResult` | sacs-governance | Contract 004 precommit boundary | noncanonical construction and eligibility evidence | never downstream authority |
| `CommittedGovernanceSnapshot` | sacs-governance | Contract 004 canonical artifact | immutable cycle-bound governance state | no objectives/profiles/control/execution |
| `GovernanceRevocation` / `GovernanceStore` | sacs-governance | Contract 004 revocation/commitment | append-only revocation and idempotent commitment | no prior snapshot mutation/latest pointer |
| `CommittedGovernanceHandoff` / `GovernanceFailure` | sacs-governance | Contract 004 downstream/failure boundary | committed-only access and reduced-authority failure | no ALLOW default |
| `OsvFieldRegistrySnapshot` / `OsvFieldDefinition` | sacs-operational-state-vector | Contract 003 field registry | explicit ordered field definitions | no invented canonical vocabulary |
| `OsvSchema` / `OsvPublicationProfile` | sacs-operational-state-vector | Contract 003 publication context | explicit schema, fields, policies, and registry binding | no ambient defaults or action selection |
| `OsvProjectionProfile` / `ProjectionRule` | sacs-operational-state-vector | Contract 003 projection registry | deterministic direct-binding reference projection | no new estimation or domain mathematics |
| `OsvDraft` / `OsvValidationResult` | sacs-operational-state-vector | Contract 003 precommit boundary | noncanonical construction and eligibility evidence | never downstream state |
| `CommittedOperationalStateVector` | sacs-operational-state-vector | Contract 003 canonical artifact | immutable cycle-bound supervisory state | no governance/objective/control authority |
| `CommittedOsvHandoff` | sacs-operational-state-vector | Contract 003 downstream boundary | committed OSV only | no draft/report/Estimated State bypass |
| `OsvStore` / `OsvPublicationRecord` | sacs-operational-state-vector | Contract 003 commitment evidence | in-memory append-only idempotent cycle commitment | no mutable current/latest pointer |
| `OsvPublicationFailure` | sacs-operational-state-vector | Contract 003 failure boundary | explicit immutable typed failure | no null/implicit failure |
| `ObjectiveSourceDeclaration` / `SupervisoryObjectiveDefinition` | sacs-supervisory-objectives | Contract 005 source/definition boundary | explicit source-bound declarative objective | no universal objective authoring |
| `ObjectiveRegistrySnapshot` / `ObjectiveEvaluationProfile` | sacs-supervisory-objectives | Contract 005 evaluation context | immutable registry and explicit policy binding | no current/latest lookup |
| `ObjectiveApplicabilityResult` / `ObjectiveEligibilityResult` | sacs-supervisory-objectives | Contract 005 evaluation evidence | distinct applicability and governance eligibility | no permissive fallback |
| `ObjectiveSetCandidate` / `ObjectiveSetValidationResult` | sacs-supervisory-objectives | Contract 005 precommit boundary | noncanonical objective evaluation and validation | never downstream authority |
| `CommittedObjectiveSet` / `CommittedObjectiveSetHandoff` | sacs-supervisory-objectives | Contract 005 canonical artifact/handoff | immutable objective set for Contract 006 | no profile/reference/control behavior |
| `ObjectiveCommitmentRecord` / `ObjectiveStore` | sacs-supervisory-objectives | Contract 005 commitment evidence | append-only idempotent commitment | no mutable current pointer |
| `ObjectiveFailure` | sacs-supervisory-objectives | Contract 005 failure boundary | explicit reduced-authority failure | no default objective |
| `OperationalProfileDefinition` / `ProfileRegistryEntry` | sacs-operational-profiles | Contract 006 reusable profile publication | immutable source-bound configuration definition | no control logic |
| `ProfileRegistrySnapshot` / `DiscoveryScope` | sacs-operational-profiles | Contract 006 registry freeze/discovery context | explicit immutable visible universe | no selection/eligibility authority |
| `CandidateProfileSet` | sacs-operational-profiles | Contract 006 discovery output | scope-only candidate universe | no compatibility/selection |
| `EligibilityEvaluationReport` / `CompatibilityEvaluationReport` | sacs-operational-profiles | Contract 006 staged evaluation | distinct admission and constructability evidence | no selection substitution |
| `ProfileSelectionPolicy` / `ProfileSelectionReport` | sacs-operational-profiles | Contract 006 deterministic selection | explicit tie-breaking and rationale | no default/random selection |
| `CommittedOperationalProfile` | sacs-operational-profiles | Contract 006 canonical configuration | immutable cycle-bound configuration | no control authority |
| `ReferenceStateDefinition` / `CommittedReferenceState` | sacs-operational-profiles | Contract 006 target representation | deterministic declared reference components | not control error |
| `ControllerConfigurationHandoff` / `ProfileFailure` | sacs-operational-profiles | Contract 006 downstream/failure boundary | committed profile/reference only and explicit failure | no controller behavior |
| `ControllerDefinition` / `CommittedControllerConfiguration` | sacs-controller | Contract 007 exact controller binding | immutable versioned mathematical configuration | no substitute selection |
| `ControllerStateInput` / `CommittedControllerStateSet` | sacs-controller | Contract 007 explicit controller memory | committed state or explicit stateless disposition | no hidden mutable state |
| `CommittedControlError` / `CommittedControlConstraintSet` | sacs-controller | Contract 007 mathematical deviation and authority | deterministic bounded inputs | no command authority alone |
| `CommittedValidatedControllerComposition` / `ControllerEvaluationResult` | sacs-controller | Contract 007 composition/evaluation | proposed command and complete state outcome | noncanonical |
| `AuthorityCertificate` / `ControllerCommitmentRecord` | sacs-controller | Contract 007 certification/commit switch | separate evidence and atomic canonicalization | no external verification |
| `ControllerCommitmentBundle` / `CommittedControlCommandHandoff` | sacs-controller | Contract 007 canonical output and Contract 008 boundary | immutable command-only downstream handoff | no translation/execution |
| `RuntimeCapabilityProfile` / `RuntimeAvailability` | sacs-inference-control | Contract 008 capability model | committed representation facts distinct from availability | no execution permission |
| `CommittedTranslationConfiguration` / `TranslationProfile` | sacs-inference-control | Contract 008 closed translation authority | exact cycle/runtime/rule binding | no profile negotiation |
| `TranslationRequirementSet` / `ValidatedTranslationBinding` | sacs-inference-control | Contract 008 translation proof | preserves command authority and context | noncanonical |
| `ProposedInferenceCommandSet` / `TranslationValidationReport` | sacs-inference-control | Contract 008 translation evaluation | deterministic proposed representation and separate validation | noncanonical |
| `CommittedInferenceCommandSet` / `CommittedTranslationTransaction` | sacs-inference-control | Contract 008 canonical translation boundary | distinct semantic set and transaction | no execution disposition |
| `ExecutionInterfaceHandoffEnvelope` | sacs-inference-control | Contract 009 handoff | preserves transaction and authority references | no runtime call |
| `ExecutionHandoffAdmissionRecord` / `ReceiptAndEligibilityReport` | sacs-execution-interface | Contract 009 receipt and eligibility | exact transaction/envelope admission evidence | no execution success claim |
| `PreExecutionDispositionRecord` / `ExecutionPlan` | sacs-execution-interface | Contract 009 execution path | explicit disposition and bounded plan | distinct artifacts |
| `ExecutionAdapterRequest` / `ExecutionAdapterResult` | sacs-execution-interface | Contract 009 adapter boundary | typed transport-neutral request/result | no provider integration |
| `ExecutionOutcome` / `ExecutionDispositionValidationReport` | sacs-execution-interface | Contract 009 runtime evidence | correlated outcome and separate validation | not history |
| `CommittedExecutionDisposition` / `ExecutionDispositionHandoff` | sacs-execution-interface | Contract 009 canonical output | immutable disposition for Contracts 010/011 | no continuity/certification |
| `SupervisoryModeDefinition` / `ModeRegistrySnapshot` | sacs-supervisory-continuity | Contract 010 mode identity and registry | immutable recognized modes | no active-mode authority alone |
| `GuardEvaluationSet` / `TransitionEligibilityEvaluation` | sacs-supervisory-continuity | Contract 010 guard and eligibility evaluation | deterministic evidence | no mode commitment |
| `TransitionPrecedenceResult` / `ResetRecord` | sacs-supervisory-continuity | Contract 010 precedence and reset | explicit transition evidence | no history |
| `HysteresisEvaluation` / `DwellTimeEvaluation` | sacs-supervisory-continuity | Contract 010 continuity constraints | explicit numeric/time evidence | no ambient clock |
| `CommittedSupervisoryMode` / `CommittedSupervisoryContinuityRecord` | sacs-supervisory-continuity | Contract 010 canonical continuity | distinct authoritative mode and evidence record | no certification |
| `SupervisoryContinuityHandoff` | sacs-supervisory-continuity | Contract 011 boundary | committed continuity reference | no Contract 011 logic |
| `ConstructedRealizationRecord` / `CommittedRealizationRecord` | sacs-constitutional-history | Contract 011 Stage One | realization construction and committed certification artifact | distinct stages |
| `CanonicalArtifactResolutionSet` / `ResolvedConstitutionalRelationshipSet` | sacs-constitutional-history | Contract 011 Stage Two resolution | membership and relationships remain separate | no inferred artifacts |
| `ConstructedSupervisoryLineage` / completeness/integrity results | sacs-constitutional-history | Contract 011 provenance evaluation | separate lineage construction and evaluations | no repair |
| `ConstructedSupervisoryCycleRecord` / `CommittedSupervisoryCycleRecord` | sacs-constitutional-history | Contract 011 whole-cycle commitment | noncanonical construction and canonical record | no Contract 012 logic |
| `ReplayVerificationResult` / `AuditRepresentation` | sacs-constitutional-history | Contract 011 post-commit evidence | non-mutating replay and non-constitutive audit | no external verification |
| `Contract012Handoff` | sacs-constitutional-history | Contract 012 boundary | committed cycle record only | no claim/evidence/verification |
| `VerificationExchangeIdentity` / `VerifiableClaimSet` | sacs-external-verification-interface | Contract 012 subject and claims | one stable exchange and bounded claims | no external judgment |
| `CommittedVerificationProfileBinding` / `CommittedVerificationRequirementSet` | sacs-external-verification-interface | Contract 012 verification configuration | explicit profile/requirements/method | no ambient defaults |
| `VerificationEvidencePackage` / `ConstructedVerificationSubmission` | sacs-external-verification-interface | Contract 012 evidence and submission | exact evidence-bound submission | no verification verdict |
| `CommittedVerificationSubmission` / `CustodyTransferRecord` | sacs-external-verification-interface | Contract 012 commitment/custody | immutable submission and custody evidence | no ownership transfer |
| `ExternalVerificationResult` / `CommittedVerificationResult` | sacs-external-verification-interface | Contract 012 result boundary | exact external payload/judgment preservation | SACS does not issue result |
| `CommittedVerificationExchangeRecord` / `CommittedVerificationExchangeFailure` | sacs-external-verification-interface | Contract 012 terminal history | exact-one exchange outcome | no successor authority |
| `VerificationExchangeReplayResult` / `VerificationExchangeAuditRepresentation` | sacs-external-verification-interface | Contract 012 post-commit evidence | non-mutating replay/audit | no external verification |
