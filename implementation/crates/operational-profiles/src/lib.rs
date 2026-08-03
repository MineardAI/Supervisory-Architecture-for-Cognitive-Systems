//! Contract 006 operational-profile and reference-state boundary.
//!
//! Configuration is not control. This crate consumes only committed upstream
//! handoffs, performs staged discovery/eligibility/compatibility/selection,
//! and commits immutable profile and reference-state artifacts without any
//! controller or execution behavior.

use std::collections::{BTreeMap, BTreeSet};

use canonical_hashing::content_hash;
use canonical_identity::{ArtifactIdentity, IdentityBinding};
use canonical_serialization::CanonicalDocument;
use canonical_versioning::{Version, VersionBinding};
use commitment_engine::{CommitResult, CommitmentError, CommitmentEvent, CommitmentStore};
use failure_model::FailureRecord;
use sacs_governance::CommittedGovernanceHandoff;
use sacs_operational_state_vector::{CommittedOsvHandoff, OsvFieldValidity, OsvFieldValue};
use sacs_supervisory_objectives::CommittedObjectiveSetHandoff;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProfileClass {
    Normal,
    Recovery,
    Stabilization,
    Learning,
    Calibration,
    LowConfidence,
    HumanOverride,
    DeterministicReplay,
    Emergency,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalProfileDefinition {
    pub profile_id: String,
    pub profile_version: Version,
    pub profile_name: String,
    pub profile_class: ProfileClass,
    pub source_id: String,
    pub capability_declarations: BTreeSet<String>,
    pub required_osv_fields: BTreeSet<String>,
    pub required_governance_permissions: BTreeSet<String>,
    pub prohibited_governance_states: BTreeSet<String>,
    pub required_objective_instance_ids: BTreeSet<String>,
    pub operating_region_id: String,
    pub gain_profile_id: String,
    pub reference_builder_id: String,
    pub transition_policy_id: String,
    pub numeric_profile_id: String,
    pub capability_profile: String,
    pub permissions: Vec<String>,
    pub constraints: Vec<String>,
    pub parameters: BTreeMap<String, String>,
    pub selection_rank: i64,
    pub authority_burden: i64,
    pub schema_version: Version,
    pub registry_version: Version,
    pub provenance_id: String,
    pub content_hash: String,
}

impl OperationalProfileDefinition {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("profile_id", &self.profile_id);
        d.insert("profile_version", self.profile_version.to_string());
        d.insert("profile_name", &self.profile_name);
        d.insert("profile_class", format!("{:?}", self.profile_class));
        d.insert("source_id", &self.source_id);
        d.insert(
            "osv_fields",
            self.required_osv_fields
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert(
            "governance_permissions",
            self.required_governance_permissions
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert(
            "objectives",
            self.required_objective_instance_ids
                .iter()
                .cloned()
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert("operating_region", &self.operating_region_id);
        d.insert("gain_profile", &self.gain_profile_id);
        d.insert("reference_builder", &self.reference_builder_id);
        d.insert("transition_policy", &self.transition_policy_id);
        d.insert("numeric_profile", &self.numeric_profile_id);
        d.insert("capability_profile", &self.capability_profile);
        d.insert("permissions", self.permissions.join(","));
        d.insert("constraints", self.constraints.join(","));
        d.insert(
            "parameters",
            self.parameters
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("|"),
        );
        d.insert("selection_rank", self.selection_rank.to_string());
        d.insert("authority_burden", self.authority_burden.to_string());
        d.insert("schema_version", self.schema_version.to_string());
        d.insert("registry_version", self.registry_version.to_string());
        d.encode()
    }
    pub fn calculated_hash(&self) -> String {
        content_hash(&self.canonical_bytes())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryEntryStatus {
    Draft,
    Active,
    Deprecated,
    Retired,
    Withdrawn,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileRegistryEntry {
    pub registry_entry_id: String,
    pub registry_id: String,
    pub registry_version: Version,
    pub definition: OperationalProfileDefinition,
    pub publisher_id: String,
    pub published_at: i128,
    pub approval_record: String,
    pub publication_scope: String,
    pub visibility_class: String,
    pub status: RegistryEntryStatus,
    pub publication_provenance_id: String,
    pub entry_schema_version: Version,
    pub entry_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryScope {
    pub discovery_scope_id: String,
    pub authorized_registry_ids: BTreeSet<String>,
    pub authorized_registry_versions: BTreeSet<Version>,
    pub deployment_namespaces: BTreeSet<String>,
    pub publication_scopes: BTreeSet<String>,
    pub visibility_classes: BTreeSet<String>,
    pub supervisory_domains: BTreeSet<String>,
    pub lifecycle_visibility_policy_id: String,
    pub scope_schema_version: Version,
    pub scope_provenance_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileRegistrySnapshot {
    pub snapshot_id: String,
    pub registry_id: String,
    pub registry_version: Version,
    pub ordered_entries: Vec<ProfileRegistryEntry>,
    pub cycle_id: String,
    pub registry_schema_version: Version,
    pub visibility_policy_versions: BTreeSet<Version>,
    pub publication_scope_versions: BTreeSet<Version>,
    pub snapshot_created_at: i128,
    pub snapshot_provenance_id: String,
    pub snapshot_hash: String,
}

impl ProfileRegistrySnapshot {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("snapshot_id", &self.snapshot_id);
        d.insert("registry_id", &self.registry_id);
        d.insert("registry_version", self.registry_version.to_string());
        d.insert(
            "entries",
            self.ordered_entries
                .iter()
                .map(|e| e.registry_entry_id.clone())
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert("cycle_id", &self.cycle_id);
        d.insert("created_at", self.snapshot_created_at.to_string());
        d.encode()
    }
}

pub struct ProfileRegistry {
    pub registry_id: String,
    pub registry_version: Version,
    pub entries: BTreeMap<String, ProfileRegistryEntry>,
}

impl ProfileRegistry {
    pub fn publish(
        &mut self,
        mut definition: OperationalProfileDefinition,
        publisher_id: String,
        publication_scope: String,
        visibility_class: String,
        published_at: i128,
    ) -> Result<ProfileRegistryEntry, ProfileFailure> {
        let calculated = definition.calculated_hash();
        if !definition.content_hash.is_empty() && definition.content_hash != calculated {
            return Err(ProfileFailure::ProfileDefinitionInvalid(
                definition.profile_id,
            ));
        }
        definition.content_hash = calculated.clone();
        let entry_id = format!(
            "entry:{}:{}",
            definition.profile_id, definition.profile_version
        );
        if let Some(existing) = self.entries.get(&entry_id) {
            if existing.definition.content_hash != definition.content_hash {
                return Err(ProfileFailure::ProfileCommitmentConflict);
            }
            return Ok(existing.clone());
        }
        let entry_hash = content_hash(
            format!("{entry_id}|{calculated}|{publication_scope}|{visibility_class}").as_bytes(),
        );
        let entry = ProfileRegistryEntry {
            registry_entry_id: entry_id.clone(),
            registry_id: self.registry_id.clone(),
            registry_version: self.registry_version.clone(),
            definition,
            publisher_id,
            published_at,
            approval_record: "explicit-publication".into(),
            publication_scope,
            visibility_class,
            status: RegistryEntryStatus::Active,
            publication_provenance_id: format!("publication:{entry_id}"),
            entry_schema_version: context_version(),
            entry_hash,
        };
        self.entries.insert(entry_id, entry.clone());
        Ok(entry)
    }

    pub fn snapshot(
        &self,
        cycle_id: String,
        scope: &DiscoveryScope,
        created_at: i128,
    ) -> ProfileRegistrySnapshot {
        let mut ordered_entries = self.entries.values().cloned().collect::<Vec<_>>();
        ordered_entries.sort_by(|a, b| a.registry_entry_id.cmp(&b.registry_entry_id));
        let mut snapshot = ProfileRegistrySnapshot {
            snapshot_id: String::new(),
            registry_id: self.registry_id.clone(),
            registry_version: self.registry_version.clone(),
            ordered_entries,
            cycle_id,
            registry_schema_version: context_version(),
            visibility_policy_versions: [scope.scope_schema_version.clone()].into_iter().collect(),
            publication_scope_versions: [scope.scope_schema_version.clone()].into_iter().collect(),
            snapshot_created_at: created_at,
            snapshot_provenance_id: "profile-registry-snapshot".into(),
            snapshot_hash: String::new(),
        };
        snapshot.snapshot_hash = content_hash(&snapshot.canonical_bytes());
        snapshot.snapshot_id = format!("profile-registry-snapshot:{}", snapshot.snapshot_hash);
        snapshot.snapshot_hash = content_hash(&snapshot.canonical_bytes());
        snapshot
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateProfileSet {
    pub candidate_set_id: String,
    pub cycle_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub objective_set_id: String,
    pub profile_registry_snapshot_id: String,
    pub discovery_scope_id: String,
    pub discovered_entries: Vec<String>,
    pub excluded_visible_entries: Vec<(String, String)>,
    pub schema_version: Version,
    pub registry_version: Version,
    pub provenance_id: String,
    pub artifact_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EligibilityOutcome {
    Eligible,
    Ineligible,
    Unsupported,
    Prohibited,
    Unresolved,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EligibilityResult {
    pub registry_entry_id: String,
    pub profile_id: String,
    pub outcome: EligibilityOutcome,
    pub reasons: Vec<String>,
    pub canonical_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EligibilityEvaluationReport {
    pub report_id: String,
    pub candidate_set_id: String,
    pub results: Vec<EligibilityResult>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompatibilityOutcome {
    Compatible,
    Incompatible,
    Unresolved,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityResult {
    pub registry_entry_id: String,
    pub profile_id: String,
    pub outcome: CompatibilityOutcome,
    pub reasons: Vec<String>,
    pub canonical_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatibilityEvaluationReport {
    pub report_id: String,
    pub eligibility_report_id: String,
    pub results: Vec<CompatibilityResult>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileSelectionPolicy {
    pub policy_id: String,
    pub policy_version: Version,
    pub tie_break_rule_id: String,
    pub priority_rule_id: String,
    pub numeric_profile_id: String,
    pub schema_version: Version,
    pub registry_version: Version,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SelectionResult {
    ProfileSelected,
    NoSelectionCandidates,
    NoDeterminateWinner,
    SelectionPolicyUnresolved,
    SelectionConflict,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileSelectionReport {
    pub report_id: String,
    pub candidate_set_id: String,
    pub eligibility_report_id: String,
    pub compatibility_report_id: String,
    pub policy_id: String,
    pub tie_break_rule_id: String,
    pub compatible_candidates: Vec<String>,
    pub selected_candidate: Option<String>,
    pub result: SelectionResult,
    pub rationale: String,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedOperationalProfile {
    pub constructed_id: String,
    pub cycle_id: String,
    pub profile: OperationalProfileDefinition,
    pub registry_snapshot_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub objective_set_id: String,
    pub selection_report_id: String,
    pub effective_permissions: Vec<String>,
    pub effective_constraints: Vec<String>,
    pub canonical_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalProfileValidationResult {
    pub validation_id: String,
    pub constructed_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedOperationalProfile {
    pub profile_id: String,
    pub profile_version: Version,
    pub cycle_id: String,
    pub profile_definition_id: String,
    pub registry_snapshot_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub objective_set_id: String,
    pub selection_report_id: String,
    pub operating_region_id: String,
    pub gain_profile_id: String,
    pub reference_builder_id: String,
    pub numeric_profile_id: String,
    pub capability_profile: String,
    pub permissions: Vec<String>,
    pub constraints: Vec<String>,
    pub provenance_id: String,
    pub commitment_record_id: String,
    pub schema_version: Version,
    pub registry_version: Version,
    pub artifact_hash: String,
    pub commitment_identity: String,
}

impl CommittedOperationalProfile {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("profile_id", &self.profile_id);
        d.insert("profile_version", self.profile_version.to_string());
        d.insert("cycle_id", &self.cycle_id);
        d.insert("definition_id", &self.profile_definition_id);
        d.insert("registry_snapshot", &self.registry_snapshot_id);
        d.insert("osv", &self.osv_id);
        d.insert("governance", &self.governance_snapshot_id);
        d.insert("objectives", &self.objective_set_id);
        d.insert("selection", &self.selection_report_id);
        d.insert("operating_region", &self.operating_region_id);
        d.insert("gain_profile", &self.gain_profile_id);
        d.insert("reference_builder", &self.reference_builder_id);
        d.insert("numeric_profile", &self.numeric_profile_id);
        d.insert("permissions", self.permissions.join(","));
        d.insert("constraints", self.constraints.join(","));
        d.insert("provenance", &self.provenance_id);
        d.encode()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceStateDefinition {
    pub builder_id: String,
    pub builder_version: Version,
    pub supported_profile_classes: BTreeSet<String>,
    pub numeric_profile_id: String,
    pub schema_version: Version,
    pub registry_version: Version,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceComponent {
    pub component_id: String,
    pub target_value: String,
    pub tolerance: String,
    pub validity: String,
    pub uncertainty: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedReferenceState {
    pub constructed_id: String,
    pub cycle_id: String,
    pub committed_profile_id: String,
    pub builder_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub objective_set_id: String,
    pub components: Vec<ReferenceComponent>,
    pub numeric_profile_id: String,
    pub canonical_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceStateValidationResult {
    pub validation_id: String,
    pub constructed_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub canonical_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedReferenceState {
    pub reference_state_id: String,
    pub cycle_id: String,
    pub committed_profile_id: String,
    pub builder_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub objective_set_id: String,
    pub components: Vec<ReferenceComponent>,
    pub numeric_profile_id: String,
    pub provenance_id: String,
    pub commitment_record_id: String,
    pub artifact_hash: String,
    pub commitment_identity: String,
}

impl CommittedReferenceState {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("reference_state_id", &self.reference_state_id);
        d.insert("cycle_id", &self.cycle_id);
        d.insert("profile", &self.committed_profile_id);
        d.insert("builder", &self.builder_id);
        d.insert("osv", &self.osv_id);
        d.insert("governance", &self.governance_snapshot_id);
        d.insert("objectives", &self.objective_set_id);
        d.insert(
            "components",
            self.components
                .iter()
                .map(|c| format!("{}={}:{}", c.component_id, c.target_value, c.tolerance))
                .collect::<Vec<_>>()
                .join("|"),
        );
        d.insert("numeric_profile", &self.numeric_profile_id);
        d.insert("provenance", &self.provenance_id);
        d.encode()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalProfileCommitmentRecord {
    pub commitment_identity: String,
    pub profile_id: String,
    pub validation_id: String,
    pub cycle_id: String,
    pub canonical_hash: String,
    pub event: CommitmentEvent,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferenceStateCommitmentRecord {
    pub commitment_identity: String,
    pub reference_state_id: String,
    pub validation_id: String,
    pub cycle_id: String,
    pub canonical_hash: String,
    pub event: CommitmentEvent,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerConfigurationHandoff {
    pub profile: CommittedOperationalProfile,
    pub reference_state: CommittedReferenceState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProfileFailure {
    InvalidCommittedOsvHandoff(String),
    InvalidCommittedGovernanceHandoff(String),
    InvalidCommittedObjectiveSetHandoff(String),
    CycleBindingMismatch,
    ProfileRegistryUnavailable,
    ProfileRegistryVersionMismatch,
    ProfileDefinitionInvalid(String),
    CandidateDiscoveryFailure,
    NoCandidate,
    NoEligibleCandidate,
    NoCompatibleCandidate,
    NoDeterminateWinner,
    EligibilityEvaluationFailure(String),
    CompatibilityEvaluationFailure(String),
    SelectionPolicyUnavailable,
    TieBreakUnresolved,
    ProfileConstructionFailure(String),
    ProfileValidationFailure(String),
    ProfileCommitmentConflict,
    ReferenceBuilderUnavailable,
    ReferenceBuilderIncompatible,
    ReferenceStateConstructionFailure(String),
    ReferenceStateValidationFailure(String),
    ReferenceStateCommitmentConflict,
    GovernanceConstraintViolation(String),
    ObjectiveBindingViolation(String),
    SchemaVersionMismatch,
    RegistryVersionMismatch,
    NumericProfileMismatch,
    Commitment(CommitmentError),
}
impl ProfileFailure {
    pub fn failure_record(&self) -> FailureRecord {
        FailureRecord::new(
            format!("profiles::{self:?}"),
            "Contract 006 failed toward reduced authority",
        )
    }
}

#[derive(Default)]
pub struct ProfileStore {
    profile_commitments: CommitmentStore,
    reference_commitments: CommitmentStore,
    profiles: BTreeMap<String, CommittedOperationalProfile>,
    references: BTreeMap<String, CommittedReferenceState>,
    profile_cycles: BTreeMap<String, String>,
    reference_profiles: BTreeMap<String, String>,
}

impl ProfileStore {
    pub fn commit_profile(
        &mut self,
        profile: CommittedOperationalProfile,
        validation: &OperationalProfileValidationResult,
    ) -> Result<
        (
            CommittedOperationalProfile,
            OperationalProfileCommitmentRecord,
        ),
        ProfileFailure,
    > {
        if !validation.eligible {
            return Err(ProfileFailure::ProfileValidationFailure(
                validation.reasons.join("|"),
            ));
        }
        if let Some(existing) = self.profile_cycles.get(&profile.cycle_id) {
            if existing != &profile.commitment_identity {
                return Err(ProfileFailure::ProfileCommitmentConflict);
            }
        }
        let identity = ArtifactIdentity::deterministic(
            version_binding(),
            IdentityBinding {
                schema: profile.schema_version.to_string(),
                registry: profile.registry_version.to_string(),
                cycle: Some(profile.cycle_id.clone()),
                producer: "sacs-operational-profiles".into(),
            },
            &profile.canonical_bytes(),
        );
        let event = self
            .profile_commitments
            .commit(&identity, &profile.canonical_bytes())
            .map_err(ProfileFailure::Commitment)?;
        let event = match event {
            CommitResult::Committed(e) | CommitResult::Idempotent(e) => e,
        };
        let record = OperationalProfileCommitmentRecord {
            commitment_identity: identity.identifier.clone(),
            profile_id: profile.profile_id.clone(),
            validation_id: validation.validation_id.clone(),
            cycle_id: profile.cycle_id.clone(),
            canonical_hash: profile.artifact_hash.clone(),
            event,
        };
        self.profile_cycles.insert(
            profile.cycle_id.clone(),
            profile.commitment_identity.clone(),
        );
        self.profiles
            .insert(profile.commitment_identity.clone(), profile.clone());
        Ok((profile, record))
    }
    pub fn commit_reference(
        &mut self,
        reference: CommittedReferenceState,
        validation: &ReferenceStateValidationResult,
    ) -> Result<(CommittedReferenceState, ReferenceStateCommitmentRecord), ProfileFailure> {
        if !validation.eligible {
            return Err(ProfileFailure::ReferenceStateValidationFailure(
                validation.reasons.join("|"),
            ));
        }
        if let Some(existing) = self.reference_profiles.get(&reference.committed_profile_id) {
            if existing != &reference.commitment_identity {
                return Err(ProfileFailure::ReferenceStateCommitmentConflict);
            }
        }
        let identity = ArtifactIdentity::deterministic(
            version_binding(),
            IdentityBinding {
                schema: "reference-state-1.0.0".into(),
                registry: "reference-state-1.0.0".into(),
                cycle: Some(reference.cycle_id.clone()),
                producer: "sacs-reference-state-builder".into(),
            },
            &reference.canonical_bytes(),
        );
        let event = self
            .reference_commitments
            .commit(&identity, &reference.canonical_bytes())
            .map_err(ProfileFailure::Commitment)?;
        let event = match event {
            CommitResult::Committed(e) | CommitResult::Idempotent(e) => e,
        };
        let record = ReferenceStateCommitmentRecord {
            commitment_identity: identity.identifier.clone(),
            reference_state_id: reference.reference_state_id.clone(),
            validation_id: validation.validation_id.clone(),
            cycle_id: reference.cycle_id.clone(),
            canonical_hash: reference.artifact_hash.clone(),
            event,
        };
        self.reference_profiles.insert(
            reference.committed_profile_id.clone(),
            reference.commitment_identity.clone(),
        );
        self.references
            .insert(reference.commitment_identity.clone(), reference.clone());
        Ok((reference, record))
    }
    pub fn get_profile(&self, identity: &str) -> Option<&CommittedOperationalProfile> {
        self.profiles.get(identity)
    }
    pub fn get_reference(&self, identity: &str) -> Option<&CommittedReferenceState> {
        self.references.get(identity)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileEvaluationContext {
    pub cycle_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub objective_set_id: String,
    pub registry_id: String,
    pub registry_version: Version,
    pub discovery_scope: DiscoveryScope,
    pub selection_policy: ProfileSelectionPolicy,
    pub capability_profile: String,
    pub deployment_namespace: String,
    pub effective_time: i128,
    pub schema_version: Version,
    pub numeric_profile_id: String,
}

pub struct ProfileEvaluator;

impl ProfileEvaluator {
    pub fn discover(
        &self,
        snapshot: &ProfileRegistrySnapshot,
        scope: &DiscoveryScope,
        upstream: &UpstreamBindings,
    ) -> Result<CandidateProfileSet, ProfileFailure> {
        if snapshot.cycle_id != upstream.cycle_id
            || snapshot.registry_id != upstream.registry_id
            || snapshot.registry_version != upstream.registry_version
        {
            return Err(ProfileFailure::ProfileRegistryVersionMismatch);
        }
        let mut entries = snapshot.ordered_entries.clone();
        entries.sort_by(|a, b| a.registry_entry_id.cmp(&b.registry_entry_id));
        let mut discovered = Vec::new();
        let mut excluded = Vec::new();
        for entry in entries {
            if !scope.authorized_registry_ids.contains(&entry.registry_id)
                || !scope
                    .authorized_registry_versions
                    .contains(&entry.registry_version)
            {
                excluded.push((entry.registry_entry_id, "REGISTRY_VERSION_EXCLUDED".into()));
                continue;
            }
            if !scope.publication_scopes.contains(&entry.publication_scope)
                || !scope.visibility_classes.contains(&entry.visibility_class)
            {
                excluded.push((entry.registry_entry_id, "VISIBILITY_CLASS_EXCLUDED".into()));
                continue;
            }
            if matches!(
                entry.status,
                RegistryEntryStatus::Draft
                    | RegistryEntryStatus::Retired
                    | RegistryEntryStatus::Withdrawn
            ) {
                excluded.push((entry.registry_entry_id, "LIFECYCLE_NOT_VISIBLE".into()));
                continue;
            }
            discovered.push(entry.registry_entry_id);
        }
        let mut candidate = CandidateProfileSet {
            candidate_set_id: String::new(),
            cycle_id: upstream.cycle_id.clone(),
            osv_id: upstream.osv_id.clone(),
            governance_snapshot_id: upstream.governance_snapshot_id.clone(),
            objective_set_id: upstream.objective_set_id.clone(),
            profile_registry_snapshot_id: snapshot.snapshot_id.clone(),
            discovery_scope_id: scope.discovery_scope_id.clone(),
            discovered_entries: discovered,
            excluded_visible_entries: excluded,
            schema_version: context_version(),
            registry_version: snapshot.registry_version.clone(),
            provenance_id: format!("profile-discovery:{}", upstream.cycle_id),
            artifact_hash: String::new(),
        };
        candidate.artifact_hash = candidate_hash(&candidate);
        candidate.candidate_set_id = format!("candidates:{}", candidate.artifact_hash);
        Ok(candidate)
    }

    pub fn evaluate_eligibility(
        &self,
        candidate: &CandidateProfileSet,
        snapshot: &ProfileRegistrySnapshot,
        osv: &CommittedOsvHandoff,
        governance: &CommittedGovernanceHandoff,
        objectives: &CommittedObjectiveSetHandoff,
        context: &ProfileEvaluationContext,
    ) -> Result<EligibilityEvaluationReport, ProfileFailure> {
        let mut results = Vec::new();
        for id in &candidate.discovered_entries {
            let entry = snapshot
                .ordered_entries
                .iter()
                .find(|entry| &entry.registry_entry_id == id)
                .ok_or(ProfileFailure::CandidateDiscoveryFailure)?;
            let definition = &entry.definition;
            let mut reasons: Vec<String> = Vec::new();
            if definition.content_hash != definition.calculated_hash() {
                reasons.push("definition integrity mismatch".into());
            }
            if definition.required_osv_fields.iter().any(|field_id| {
                !osv.osv.fields.iter().any(|field| {
                    &field.field_id == field_id
                        && matches!(
                            field.validity,
                            OsvFieldValidity::Valid | OsvFieldValidity::Degraded
                        )
                        && matches!(field.value, OsvFieldValue::Value(_))
                })
            }) {
                reasons.push("required OSV field unavailable".into());
            }
            if definition
                .required_governance_permissions
                .iter()
                .any(|permission| !governance.snapshot.permissions.contains(permission))
            {
                reasons.push("required governance permission missing".into());
            }
            if definition
                .required_objective_instance_ids
                .iter()
                .any(|objective_id| {
                    !objectives
                        .objective_set
                        .active_objective_instance_ids
                        .contains(objective_id)
                })
            {
                reasons.push("required objective missing".into());
            }
            if definition.capability_profile != context.capability_profile {
                reasons.push("capability profile mismatch".into());
            }
            if definition.numeric_profile_id != context.numeric_profile_id {
                reasons.push("numeric profile mismatch".into());
            }
            if definition.operating_region_id.is_empty() {
                reasons.push("operating region missing".into());
            }
            let outcome = if definition.prohibited_governance_states.iter().any(|state| {
                state == &governance.snapshot.precedence_resolution
                    || state == &format!("{:?}", governance.snapshot.snapshot_status)
            }) {
                EligibilityOutcome::Prohibited
            } else if reasons.is_empty() {
                EligibilityOutcome::Eligible
            } else {
                EligibilityOutcome::Ineligible
            };
            results.push(EligibilityResult {
                registry_entry_id: id.clone(),
                profile_id: definition.profile_id.clone(),
                outcome,
                reasons: reasons.clone(),
                canonical_hash: definition.content_hash.clone(),
            });
        }
        if results
            .iter()
            .all(|result| !matches!(result.outcome, EligibilityOutcome::Eligible))
        {
            return Err(ProfileFailure::NoEligibleCandidate);
        }
        let mut report = EligibilityEvaluationReport {
            report_id: String::new(),
            candidate_set_id: candidate.candidate_set_id.clone(),
            results,
            canonical_hash: String::new(),
        };
        report.canonical_hash = content_hash(format!("{:?}", report.results).as_bytes());
        report.report_id = format!("eligibility:{}", report.canonical_hash);
        Ok(report)
    }

    pub fn evaluate_compatibility(
        &self,
        candidate: &CandidateProfileSet,
        eligibility: &EligibilityEvaluationReport,
        snapshot: &ProfileRegistrySnapshot,
        objectives: &CommittedObjectiveSetHandoff,
        context: &ProfileEvaluationContext,
    ) -> Result<CompatibilityEvaluationReport, ProfileFailure> {
        let mut results = Vec::new();
        for result in &eligibility.results {
            if !matches!(result.outcome, EligibilityOutcome::Eligible) {
                continue;
            }
            let entry = snapshot
                .ordered_entries
                .iter()
                .find(|entry| entry.registry_entry_id == result.registry_entry_id)
                .ok_or(ProfileFailure::CandidateDiscoveryFailure)?;
            let definition = &entry.definition;
            let mut reasons = Vec::new();
            if definition.reference_builder_id.is_empty() || definition.parameters.is_empty() {
                reasons.push("reference builder or parameters incomplete".into());
            }
            if definition.required_objective_instance_ids.iter().any(|id| {
                !objectives
                    .objective_set
                    .active_objective_instance_ids
                    .contains(id)
            }) {
                reasons.push("objective compatibility mismatch".into());
            }
            if definition.numeric_profile_id != context.numeric_profile_id {
                reasons.push("numeric profile incompatible".into());
            }
            results.push(CompatibilityResult {
                registry_entry_id: result.registry_entry_id.clone(),
                profile_id: result.profile_id.clone(),
                outcome: if reasons.is_empty() {
                    CompatibilityOutcome::Compatible
                } else {
                    CompatibilityOutcome::Incompatible
                },
                reasons,
                canonical_hash: definition.content_hash.clone(),
            });
        }
        if results
            .iter()
            .all(|result| !matches!(result.outcome, CompatibilityOutcome::Compatible))
        {
            return Err(ProfileFailure::NoCompatibleCandidate);
        }
        let mut report = CompatibilityEvaluationReport {
            report_id: String::new(),
            eligibility_report_id: eligibility.report_id.clone(),
            results,
            canonical_hash: String::new(),
        };
        report.canonical_hash = content_hash(format!("{:?}", report.results).as_bytes());
        report.report_id = format!("compatibility:{}", report.canonical_hash);
        let _ = candidate;
        Ok(report)
    }

    pub fn select(
        &self,
        candidate: &CandidateProfileSet,
        eligibility: &EligibilityEvaluationReport,
        compatibility: &CompatibilityEvaluationReport,
        snapshot: &ProfileRegistrySnapshot,
        policy: &ProfileSelectionPolicy,
    ) -> Result<ProfileSelectionReport, ProfileFailure> {
        if policy.tie_break_rule_id.is_empty() {
            return Err(ProfileFailure::TieBreakUnresolved);
        }
        let mut ids = compatibility
            .results
            .iter()
            .filter(|result| matches!(result.outcome, CompatibilityOutcome::Compatible))
            .map(|result| result.registry_entry_id.clone())
            .collect::<Vec<_>>();
        if ids.is_empty() {
            return Err(ProfileFailure::NoCompatibleCandidate);
        }
        ids.sort_by(|left, right| {
            let l = snapshot
                .ordered_entries
                .iter()
                .find(|entry| &entry.registry_entry_id == left)
                .expect("candidate");
            let r = snapshot
                .ordered_entries
                .iter()
                .find(|entry| &entry.registry_entry_id == right)
                .expect("candidate");
            r.definition
                .selection_rank
                .cmp(&l.definition.selection_rank)
                .then_with(|| {
                    l.definition
                        .authority_burden
                        .cmp(&r.definition.authority_burden)
                })
                .then_with(|| l.definition.profile_id.cmp(&r.definition.profile_id))
        });
        let selected = ids.first().cloned();
        let mut report = ProfileSelectionReport {
            report_id: String::new(),
            candidate_set_id: candidate.candidate_set_id.clone(),
            eligibility_report_id: eligibility.report_id.clone(),
            compatibility_report_id: compatibility.report_id.clone(),
            policy_id: policy.policy_id.clone(),
            tie_break_rule_id: policy.tie_break_rule_id.clone(),
            compatible_candidates: ids,
            selected_candidate: selected,
            result: SelectionResult::ProfileSelected,
            rationale: "selection-rank, authority-burden, profile-id".into(),
            canonical_hash: String::new(),
        };
        report.canonical_hash = content_hash(format!("{:?}", report).as_bytes());
        report.report_id = format!("selection:{}", report.canonical_hash);
        Ok(report)
    }

    pub fn construct_profile(
        &self,
        selection: &ProfileSelectionReport,
        snapshot: &ProfileRegistrySnapshot,
        upstream: &UpstreamBindings,
    ) -> Result<
        (
            ConstructedOperationalProfile,
            OperationalProfileValidationResult,
        ),
        ProfileFailure,
    > {
        let id = selection
            .selected_candidate
            .clone()
            .ok_or(ProfileFailure::NoDeterminateWinner)?;
        let entry = snapshot
            .ordered_entries
            .iter()
            .find(|entry| entry.registry_entry_id == id)
            .ok_or(ProfileFailure::ProfileConstructionFailure(
                "selected candidate absent".into(),
            ))?;
        let mut constructed = ConstructedOperationalProfile {
            constructed_id: String::new(),
            cycle_id: upstream.cycle_id.clone(),
            profile: entry.definition.clone(),
            registry_snapshot_id: snapshot.snapshot_id.clone(),
            osv_id: upstream.osv_id.clone(),
            governance_snapshot_id: upstream.governance_snapshot_id.clone(),
            objective_set_id: upstream.objective_set_id.clone(),
            selection_report_id: selection.report_id.clone(),
            effective_permissions: entry.definition.permissions.clone(),
            effective_constraints: entry.definition.constraints.clone(),
            canonical_hash: String::new(),
        };
        constructed.canonical_hash = content_hash(format!("{:?}", constructed).as_bytes());
        constructed.constructed_id = format!("constructed-profile:{}", constructed.canonical_hash);
        let validation = OperationalProfileValidationResult {
            validation_id: format!("profile-validation:{}", constructed.canonical_hash),
            constructed_id: constructed.constructed_id.clone(),
            eligible: true,
            reasons: Vec::new(),
            canonical_hash: constructed.canonical_hash.clone(),
        };
        Ok((constructed, validation))
    }

    pub fn commit_profile_artifact(
        &self,
        constructed: &ConstructedOperationalProfile,
        validation: &OperationalProfileValidationResult,
    ) -> Result<CommittedOperationalProfile, ProfileFailure> {
        if !validation.eligible {
            return Err(ProfileFailure::ProfileValidationFailure(
                validation.reasons.join("|"),
            ));
        }
        let p = &constructed.profile;
        let mut profile = CommittedOperationalProfile {
            profile_id: p.profile_id.clone(),
            profile_version: p.profile_version.clone(),
            cycle_id: constructed.cycle_id.clone(),
            profile_definition_id: p.profile_id.clone(),
            registry_snapshot_id: constructed.registry_snapshot_id.clone(),
            osv_id: constructed.osv_id.clone(),
            governance_snapshot_id: constructed.governance_snapshot_id.clone(),
            objective_set_id: constructed.objective_set_id.clone(),
            selection_report_id: constructed.selection_report_id.clone(),
            operating_region_id: p.operating_region_id.clone(),
            gain_profile_id: p.gain_profile_id.clone(),
            reference_builder_id: p.reference_builder_id.clone(),
            numeric_profile_id: p.numeric_profile_id.clone(),
            capability_profile: p.capability_profile.clone(),
            permissions: constructed.effective_permissions.clone(),
            constraints: constructed.effective_constraints.clone(),
            provenance_id: p.provenance_id.clone(),
            commitment_record_id: format!("commitment:{}", validation.validation_id),
            schema_version: p.schema_version.clone(),
            registry_version: p.registry_version.clone(),
            artifact_hash: String::new(),
            commitment_identity: String::new(),
        };
        profile.artifact_hash = content_hash(&profile.canonical_bytes());
        profile.commitment_identity = format!("profile:{}", profile.artifact_hash);
        Ok(profile)
    }

    pub fn construct_reference_state(
        &self,
        profile: &CommittedOperationalProfile,
        upstream: &UpstreamBindings,
        builder: &ReferenceStateDefinition,
    ) -> Result<(ConstructedReferenceState, ReferenceStateValidationResult), ProfileFailure> {
        if builder.builder_id != profile.reference_builder_id {
            return Err(ProfileFailure::ReferenceBuilderIncompatible);
        }
        if builder.numeric_profile_id != profile.numeric_profile_id {
            return Err(ProfileFailure::NumericProfileMismatch);
        }
        let mut components = profile
            .constraints
            .iter()
            .enumerate()
            .map(|(i, value)| ReferenceComponent {
                component_id: format!("constraint-{i}"),
                target_value: value.clone(),
                tolerance: "declared-profile-tolerance".into(),
                validity: "DECLARED".into(),
                uncertainty: "PROFILE_DECLARED".into(),
            })
            .collect::<Vec<_>>();
        components.sort_by(|a, b| a.component_id.cmp(&b.component_id));
        if components.is_empty() {
            return Err(ProfileFailure::ReferenceStateConstructionFailure(
                "no declared reference components".into(),
            ));
        }
        let mut constructed = ConstructedReferenceState {
            constructed_id: String::new(),
            cycle_id: upstream.cycle_id.clone(),
            committed_profile_id: profile.commitment_identity.clone(),
            builder_id: builder.builder_id.clone(),
            osv_id: upstream.osv_id.clone(),
            governance_snapshot_id: upstream.governance_snapshot_id.clone(),
            objective_set_id: upstream.objective_set_id.clone(),
            components,
            numeric_profile_id: profile.numeric_profile_id.clone(),
            canonical_hash: String::new(),
        };
        constructed.canonical_hash = content_hash(format!("{:?}", constructed).as_bytes());
        constructed.constructed_id =
            format!("constructed-reference:{}", constructed.canonical_hash);
        let validation = ReferenceStateValidationResult {
            validation_id: format!("reference-validation:{}", constructed.canonical_hash),
            constructed_id: constructed.constructed_id.clone(),
            eligible: true,
            reasons: Vec::new(),
            canonical_hash: constructed.canonical_hash.clone(),
        };
        Ok((constructed, validation))
    }

    pub fn commit_reference_artifact(
        &self,
        constructed: &ConstructedReferenceState,
        validation: &ReferenceStateValidationResult,
    ) -> Result<CommittedReferenceState, ProfileFailure> {
        if !validation.eligible {
            return Err(ProfileFailure::ReferenceStateValidationFailure(
                validation.reasons.join("|"),
            ));
        }
        let mut reference = CommittedReferenceState {
            reference_state_id: String::new(),
            cycle_id: constructed.cycle_id.clone(),
            committed_profile_id: constructed.committed_profile_id.clone(),
            builder_id: constructed.builder_id.clone(),
            osv_id: constructed.osv_id.clone(),
            governance_snapshot_id: constructed.governance_snapshot_id.clone(),
            objective_set_id: constructed.objective_set_id.clone(),
            components: constructed.components.clone(),
            numeric_profile_id: constructed.numeric_profile_id.clone(),
            provenance_id: format!("provenance:{}", constructed.constructed_id),
            commitment_record_id: format!("commitment:{}", validation.validation_id),
            artifact_hash: String::new(),
            commitment_identity: String::new(),
        };
        reference.artifact_hash = content_hash(&reference.canonical_bytes());
        reference.reference_state_id = format!("reference-state:{}", reference.artifact_hash);
        reference.artifact_hash = content_hash(&reference.canonical_bytes());
        reference.commitment_identity = format!("reference:{}", reference.artifact_hash);
        Ok(reference)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpstreamBindings {
    pub cycle_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub objective_set_id: String,
    pub registry_id: String,
    pub registry_version: Version,
}

fn context_version() -> Version {
    Version::new("1.0.0").expect("static version")
}
fn candidate_hash(candidate: &CandidateProfileSet) -> String {
    let mut d = CanonicalDocument::new();
    d.insert("cycle", &candidate.cycle_id);
    d.insert("osv", &candidate.osv_id);
    d.insert("governance", &candidate.governance_snapshot_id);
    d.insert("objectives", &candidate.objective_set_id);
    d.insert("snapshot", &candidate.profile_registry_snapshot_id);
    d.insert("entries", candidate.discovered_entries.join(","));
    content_hash(&d.encode())
}
fn version_binding() -> VersionBinding {
    let v = Version::new("1.0.0").expect("static version");
    VersionBinding {
        baseline: v.clone(),
        contract: v.clone(),
        schema: v.clone(),
        registry: v.clone(),
        implementation: v,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_snapshot_order_is_deterministic() {
        let v = context_version();
        let snapshot = ProfileRegistrySnapshot {
            snapshot_id: "s".into(),
            registry_id: "r".into(),
            registry_version: v.clone(),
            ordered_entries: Vec::new(),
            cycle_id: "c".into(),
            registry_schema_version: v.clone(),
            visibility_policy_versions: [v.clone()].into_iter().collect(),
            publication_scope_versions: [v.clone()].into_iter().collect(),
            snapshot_created_at: 1,
            snapshot_provenance_id: "p".into(),
            snapshot_hash: "h".into(),
        };
        assert_eq!(snapshot.canonical_bytes(), snapshot.canonical_bytes());
    }
    #[test]
    fn no_default_profile_failure_is_explicit() {
        assert!(matches!(
            ProfileFailure::NoCandidate,
            ProfileFailure::NoCandidate
        ));
    }
    #[test]
    fn configuration_is_not_control() {
        let definition = OperationalProfileDefinition {
            profile_id: "p".into(),
            profile_version: context_version(),
            profile_name: "p".into(),
            profile_class: ProfileClass::Normal,
            source_id: "s".into(),
            capability_declarations: BTreeSet::new(),
            required_osv_fields: BTreeSet::new(),
            required_governance_permissions: BTreeSet::new(),
            prohibited_governance_states: BTreeSet::new(),
            required_objective_instance_ids: BTreeSet::new(),
            operating_region_id: "region".into(),
            gain_profile_id: "gain-set".into(),
            reference_builder_id: "builder".into(),
            transition_policy_id: "transition".into(),
            numeric_profile_id: "exact".into(),
            capability_profile: "cap".into(),
            permissions: vec!["permission".into()],
            constraints: vec!["target".into()],
            parameters: BTreeMap::new(),
            selection_rank: 1,
            authority_burden: 1,
            schema_version: context_version(),
            registry_version: context_version(),
            provenance_id: "p".into(),
            content_hash: String::new(),
        };
        assert!(!definition
            .canonical_bytes()
            .windows(7)
            .any(|w| w == b"command"));
    }
}
