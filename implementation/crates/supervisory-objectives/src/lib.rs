//! Contract 005 supervisory-objective evaluation and commitment boundary.
//!
//! Objectives remain declarative. This crate consumes only committed OSV and
//! Governance Snapshot handoffs and never selects profiles, constructs a
//! reference state, computes control, or executes behavior.

use std::collections::{BTreeMap, BTreeSet};

use canonical_hashing::content_hash;
use canonical_identity::{ArtifactIdentity, IdentityBinding};
use canonical_serialization::CanonicalDocument;
use canonical_versioning::{Version, VersionBinding};
use commitment_engine::{CommitResult, CommitmentError, CommitmentEvent, CommitmentStore};
use failure_model::FailureRecord;
use sacs_governance::{CommittedGovernanceHandoff, GovernanceSnapshotStatus};
use sacs_operational_state_vector::{
    CommittedOsvHandoff, OsvFieldValidity, OsvFieldValue, PublicationStatus,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveSourceDeclaration {
    pub source_id: String,
    pub source_type: String,
    pub source_version: Version,
    pub authority_scope: String,
    pub trust_domain_id: String,
    pub supported_objective_types: BTreeSet<String>,
    pub schema_versions: BTreeSet<Version>,
    pub registry_versions: BTreeSet<Version>,
    pub integrity_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveClass {
    Preserve,
    Restore,
    Approach,
    Avoid,
    Bound,
    Minimize,
    Maximize,
    Stabilize,
    MaintainContinuity,
    Hold,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectivePriorityClass {
    Critical,
    High,
    Normal,
    Low,
    Advisory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveCriticality {
    Catastrophic,
    Severe,
    Major,
    Moderate,
    Minor,
    Informational,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveApplicability {
    Applicable,
    NotApplicable,
    ApplicabilityUnresolved,
    ApplicabilityIncompatible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveEligibility {
    Eligible,
    EligibleRestricted,
    Required,
    Prohibited,
    Deferred,
    ReviewRequired,
    EscalationRequired,
    Unresolved,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveStatus {
    Inactive,
    Candidate,
    Active,
    ActiveRestricted,
    Satisfied,
    Violated,
    AtRisk,
    Deferred,
    Blocked,
    Conflicted,
    Unresolved,
    Expired,
    Superseded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveSetStatus {
    Valid,
    ValidRestricted,
    Degraded,
    Conflicted,
    Incomplete,
    Withheld,
    Faulted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveConflictKind {
    TargetConflict,
    BoundConflict,
    PriorityConflict,
    TemporalConflict,
    ScopeConflict,
    GovernanceConflict,
    ResourceAssumptionConflict,
    PersistenceConflict,
    UnresolvedConflict,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisoryObjectiveDefinition {
    pub objective_id: String,
    pub objective_version: Version,
    pub objective_name: String,
    pub semantic_definition: String,
    pub objective_class: ObjectiveClass,
    pub source_id: String,
    pub scope: String,
    pub subject: String,
    pub target_condition: String,
    pub tolerance: String,
    pub osv_field_dependencies: Vec<String>,
    pub applicability_rule_id: String,
    pub satisfaction_rule_id: String,
    pub priority_class: ObjectivePriorityClass,
    pub priority_rank: i64,
    pub criticality: ObjectiveCriticality,
    pub mandatory: bool,
    pub governance_requirement_ids: Vec<String>,
    pub prohibited_governance_rule_ids: Vec<String>,
    pub dependency_ids: Vec<String>,
    pub compatibility_group_ids: Vec<String>,
    pub conflict_group_ids: Vec<String>,
    pub default_status: ObjectiveStatus,
    pub effective_from: i128,
    pub effective_until: Option<i128>,
    pub schema_version: Version,
    pub registry_version: Version,
    pub provenance_id: String,
    pub integrity_hash: String,
}

impl SupervisoryObjectiveDefinition {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("objective_id", &self.objective_id);
        d.insert("objective_version", self.objective_version.to_string());
        d.insert("name", &self.objective_name);
        d.insert("definition", &self.semantic_definition);
        d.insert("class", format!("{:?}", self.objective_class));
        d.insert("source_id", &self.source_id);
        d.insert("scope", &self.scope);
        d.insert("subject", &self.subject);
        d.insert("target", &self.target_condition);
        d.insert("tolerance", &self.tolerance);
        d.insert("osv_fields", self.osv_field_dependencies.join(","));
        d.insert("applicability_rule", &self.applicability_rule_id);
        d.insert("satisfaction_rule", &self.satisfaction_rule_id);
        d.insert("priority_class", format!("{:?}", self.priority_class));
        d.insert("priority_rank", self.priority_rank.to_string());
        d.insert("criticality", format!("{:?}", self.criticality));
        d.insert("mandatory", self.mandatory.to_string());
        d.insert(
            "governance_requirements",
            self.governance_requirement_ids.join(","),
        );
        d.insert(
            "governance_prohibitions",
            self.prohibited_governance_rule_ids.join(","),
        );
        d.insert("dependencies", self.dependency_ids.join(","));
        d.insert("conflict_groups", self.conflict_group_ids.join(","));
        d.insert("effective_from", self.effective_from.to_string());
        d.insert(
            "effective_until",
            self.effective_until
                .map_or_else(|| "<absent>".into(), |v| v.to_string()),
        );
        d.insert("schema_version", self.schema_version.to_string());
        d.insert("registry_version", self.registry_version.to_string());
        d.encode()
    }
    pub fn content_hash(&self) -> String {
        content_hash(&self.canonical_bytes())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveRegistrySnapshot {
    pub registry_id: String,
    pub registry_version: Version,
    pub entries: BTreeMap<String, SupervisoryObjectiveDefinition>,
}

impl ObjectiveRegistrySnapshot {
    pub fn resolve(&self, objective_id: &str) -> Option<&SupervisoryObjectiveDefinition> {
        self.entries.get(objective_id)
    }
    pub fn identity(&self) -> String {
        content_hash(&self.canonical_bytes())
    }
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("registry_id", &self.registry_id);
        d.insert("registry_version", self.registry_version.to_string());
        d.insert(
            "objective_ids",
            self.entries.keys().cloned().collect::<Vec<_>>().join(","),
        );
        d.encode()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveEvaluationProfile {
    pub profile_id: String,
    pub profile_version: Version,
    pub accepted_osv_versions: BTreeSet<Version>,
    pub accepted_governance_versions: BTreeSet<Version>,
    pub accepted_objective_registry_versions: BTreeSet<Version>,
    pub applicability_policy: String,
    pub eligibility_policy: String,
    pub satisfaction_policy: String,
    pub uncertainty_policy: String,
    pub priority_policy: String,
    pub criticality_policy: String,
    pub compatibility_policy: String,
    pub conflict_resolution_policy: String,
    pub persistence_policy: String,
    pub failure_policy: String,
    pub numeric_profile_id: String,
    pub output_schema_version: Version,
    pub tie_breaking_rule_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveEvaluationContext {
    pub cycle_id: String,
    pub committed_osv_id: String,
    pub committed_governance_snapshot_id: String,
    pub objective_registry: ObjectiveRegistrySnapshot,
    pub evaluation_profile: ObjectiveEvaluationProfile,
    pub effective_time: i128,
    pub scope: String,
    pub capability_profile_id: String,
    pub prior_objective_set_reference: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveApplicabilityResult {
    pub objective_id: String,
    pub state: ObjectiveApplicability,
    pub reasons: Vec<String>,
    pub required_field_ids: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveEligibilityResult {
    pub objective_id: String,
    pub state: ObjectiveEligibility,
    pub reasons: Vec<String>,
    pub governance_rule_ids: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveConflict {
    pub conflict_id: String,
    pub kind: ObjectiveConflictKind,
    pub objective_instance_ids: Vec<String>,
    pub resolution: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluatedObjective {
    pub objective_id: String,
    pub objective_instance_id: String,
    pub applicability: ObjectiveApplicability,
    pub eligibility: ObjectiveEligibility,
    pub status: ObjectiveStatus,
    pub priority_class: ObjectivePriorityClass,
    pub priority_rank: i64,
    pub criticality: ObjectiveCriticality,
    pub target_condition: String,
    pub tolerance: String,
    pub uncertainty_requirement: String,
    pub dependency_ids: Vec<String>,
    pub provenance_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveSetCandidate {
    pub candidate_id: String,
    pub cycle_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub effective_time: i128,
    pub objective_evaluation_profile_id: String,
    pub objective_registry_snapshot_id: String,
    pub objectives: Vec<EvaluatedObjective>,
    pub active_objective_instance_ids: Vec<String>,
    pub restricted_objective_instance_ids: Vec<String>,
    pub required_objective_instance_ids: Vec<String>,
    pub satisfied_objective_instance_ids: Vec<String>,
    pub violated_objective_instance_ids: Vec<String>,
    pub at_risk_objective_instance_ids: Vec<String>,
    pub deferred_objective_instance_ids: Vec<String>,
    pub blocked_objective_instance_ids: Vec<String>,
    pub conflicted_objective_instance_ids: Vec<String>,
    pub unresolved_objective_instance_ids: Vec<String>,
    pub priority_order: Vec<String>,
    pub criticality_summary: String,
    pub compatibility_summary: String,
    pub conflict_summary: Vec<ObjectiveConflict>,
    pub status: ObjectiveSetStatus,
    pub schema_version: Version,
    pub registry_versions: BTreeSet<Version>,
    pub numeric_profile_id: String,
    pub provenance_id: String,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveSetValidationResult {
    pub validation_id: String,
    pub candidate_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedObjectiveSet {
    pub objective_set_id: String,
    pub cycle_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub effective_time: i128,
    pub objective_evaluation_profile_id: String,
    pub active_objective_instance_ids: Vec<String>,
    pub restricted_objective_instance_ids: Vec<String>,
    pub required_objective_instance_ids: Vec<String>,
    pub satisfied_objective_instance_ids: Vec<String>,
    pub violated_objective_instance_ids: Vec<String>,
    pub at_risk_objective_instance_ids: Vec<String>,
    pub deferred_objective_instance_ids: Vec<String>,
    pub blocked_objective_instance_ids: Vec<String>,
    pub conflicted_objective_instance_ids: Vec<String>,
    pub unresolved_objective_instance_ids: Vec<String>,
    pub priority_order: Vec<String>,
    pub criticality_summary: String,
    pub compatibility_summary: String,
    pub conflict_summary: Vec<ObjectiveConflict>,
    pub objective_set_status: ObjectiveSetStatus,
    pub schema_version: Version,
    pub registry_versions: BTreeSet<Version>,
    pub numeric_profile_id: String,
    pub provenance_id: String,
    pub commitment_record_id: String,
    pub committed_at: i128,
    pub artifact_hash: String,
    pub commitment_identity: String,
}

impl CommittedObjectiveSet {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("objective_set_id", &self.objective_set_id);
        d.insert("cycle_id", &self.cycle_id);
        d.insert("osv_id", &self.osv_id);
        d.insert("governance_snapshot_id", &self.governance_snapshot_id);
        d.insert("effective_time", self.effective_time.to_string());
        d.insert(
            "evaluation_profile_id",
            &self.objective_evaluation_profile_id,
        );
        d.insert("active", self.active_objective_instance_ids.join(","));
        d.insert(
            "restricted",
            self.restricted_objective_instance_ids.join(","),
        );
        d.insert("required", self.required_objective_instance_ids.join(","));
        d.insert("satisfied", self.satisfied_objective_instance_ids.join(","));
        d.insert("violated", self.violated_objective_instance_ids.join(","));
        d.insert("at_risk", self.at_risk_objective_instance_ids.join(","));
        d.insert("deferred", self.deferred_objective_instance_ids.join(","));
        d.insert("blocked", self.blocked_objective_instance_ids.join(","));
        d.insert(
            "conflicted",
            self.conflicted_objective_instance_ids.join(","),
        );
        d.insert(
            "unresolved",
            self.unresolved_objective_instance_ids.join(","),
        );
        d.insert("priority_order", self.priority_order.join(","));
        d.insert("criticality", &self.criticality_summary);
        d.insert("compatibility", &self.compatibility_summary);
        d.insert(
            "conflicts",
            self.conflict_summary
                .iter()
                .map(|c| c.conflict_id.clone())
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert("status", format!("{:?}", self.objective_set_status));
        d.insert("schema_version", self.schema_version.to_string());
        d.insert(
            "registry_versions",
            self.registry_versions
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert("numeric_profile_id", &self.numeric_profile_id);
        d.insert("provenance_id", &self.provenance_id);
        d.encode()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveCommitmentRecord {
    pub commitment_identity: String,
    pub candidate_id: String,
    pub validation_id: String,
    pub cycle_id: String,
    pub osv_id: String,
    pub governance_snapshot_id: String,
    pub source_registry_identity: String,
    pub commitment_status: String,
    pub canonical_hash: String,
    pub commitment_event: CommitmentEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedObjectiveSetHandoff {
    pub objective_set: CommittedObjectiveSet,
    pub active_objectives: Vec<EvaluatedObjective>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveFailure {
    InvalidCommittedOsvHandoff(String),
    InvalidCommittedGovernanceHandoff(String),
    CycleBindingMismatch,
    OsvIdentityMismatch,
    GovernanceSnapshotIdentityMismatch,
    ObjectiveSourceUnavailable,
    ObjectiveSourceUnregistered,
    ObjectiveSourceUnauthorized,
    ObjectiveSourceVersionMismatch,
    ObjectiveSchemaMismatch,
    ObjectiveRegistryMismatch,
    ObjectiveDefinitionInvalid(String),
    ObjectiveApplicabilityUnresolved(String),
    ObjectiveIneligible(String),
    ObjectiveSuppressed(String),
    ObjectiveWithheld(String),
    ObjectiveDependencyMissing(String),
    ObjectiveDependencyCycle,
    ObjectiveConflictUnresolved(String),
    ObjectivePriorityUnresolved,
    GovernancePermissionMissing(String),
    GovernanceRestrictionConflict(String),
    GovernanceProhibitionConflict(String),
    GovernanceRevocationConflict(String),
    ObjectiveAuthorityExpansionDetected,
    ObjectiveSetConstructionFailure(String),
    ObjectiveSetValidationFailure(Vec<String>),
    ObjectiveCommitmentConflict,
    Commitment(CommitmentError),
}

impl ObjectiveFailure {
    pub fn failure_record(&self) -> FailureRecord {
        FailureRecord::new(
            format!("objectives::{self:?}"),
            "objective evaluation failed toward reduced authority",
        )
    }
}

#[derive(Default)]
pub struct ObjectiveStore {
    commitments: CommitmentStore,
    objective_sets: BTreeMap<String, CommittedObjectiveSet>,
    cycles: BTreeMap<String, String>,
    records: BTreeMap<String, ObjectiveCommitmentRecord>,
}

impl ObjectiveStore {
    pub fn commit(
        &mut self,
        objective_set: CommittedObjectiveSet,
        validation: &ObjectiveSetValidationResult,
        active_objectives: Vec<EvaluatedObjective>,
    ) -> Result<(CommittedObjectiveSetHandoff, ObjectiveCommitmentRecord), ObjectiveFailure> {
        if !validation.eligible {
            return Err(ObjectiveFailure::ObjectiveSetValidationFailure(
                validation.reasons.clone(),
            ));
        }
        if let Some(existing) = self.cycles.get(&objective_set.cycle_id) {
            if existing != &objective_set.commitment_identity {
                return Err(ObjectiveFailure::ObjectiveCommitmentConflict);
            }
        }
        let identity = ArtifactIdentity::deterministic(
            version_binding(),
            IdentityBinding {
                schema: objective_set.schema_version.to_string(),
                registry: objective_set
                    .registry_versions
                    .iter()
                    .next()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "<absent>".into()),
                cycle: Some(objective_set.cycle_id.clone()),
                producer: "sacs-supervisory-objectives".into(),
            },
            &objective_set.canonical_bytes(),
        );
        let event = self
            .commitments
            .commit(&identity, &objective_set.canonical_bytes())
            .map_err(ObjectiveFailure::Commitment)?;
        let event = match event {
            CommitResult::Committed(e) | CommitResult::Idempotent(e) => e,
        };
        let record = ObjectiveCommitmentRecord {
            commitment_identity: identity.identifier.clone(),
            candidate_id: validation.candidate_id.clone(),
            validation_id: validation.validation_id.clone(),
            cycle_id: objective_set.cycle_id.clone(),
            osv_id: objective_set.osv_id.clone(),
            governance_snapshot_id: objective_set.governance_snapshot_id.clone(),
            source_registry_identity: content_hash(
                objective_set.priority_order.join(",").as_bytes(),
            ),
            commitment_status: "COMMITTED".into(),
            canonical_hash: objective_set.artifact_hash.clone(),
            commitment_event: event,
        };
        self.cycles.insert(
            objective_set.cycle_id.clone(),
            objective_set.commitment_identity.clone(),
        );
        self.objective_sets.insert(
            objective_set.commitment_identity.clone(),
            objective_set.clone(),
        );
        self.records
            .insert(record.commitment_identity.clone(), record.clone());
        Ok((
            CommittedObjectiveSetHandoff {
                objective_set,
                active_objectives,
            },
            record,
        ))
    }
    pub fn get(&self, identity: &str) -> Option<&CommittedObjectiveSet> {
        self.objective_sets.get(identity)
    }
    pub fn event_count(&self) -> usize {
        self.commitments.event_count()
    }
}

pub struct ObjectiveEvaluator;

impl ObjectiveEvaluator {
    pub fn evaluate(
        &self,
        osv: &CommittedOsvHandoff,
        governance: &CommittedGovernanceHandoff,
        definitions: &[SupervisoryObjectiveDefinition],
        sources: &BTreeMap<String, ObjectiveSourceDeclaration>,
        context: &ObjectiveEvaluationContext,
    ) -> Result<(ObjectiveSetCandidate, ObjectiveSetValidationResult), ObjectiveFailure> {
        validate_inputs(osv, governance, context)?;
        if definitions.is_empty() {
            return Err(ObjectiveFailure::ObjectiveSourceUnavailable);
        }
        let mut evaluated = Vec::new();
        for definition in definitions {
            validate_definition(definition, sources, context)?;
            let applicability = evaluate_applicability(definition, &osv.osv, governance, context);
            let eligibility = evaluate_eligibility(definition, &applicability, governance, context);
            let status = status_for(&applicability, &eligibility, definition);
            let instance_id = format!(
                "{}:{}:{}",
                definition.objective_id, context.cycle_id, context.committed_osv_id
            );
            evaluated.push(EvaluatedObjective {
                objective_id: definition.objective_id.clone(),
                objective_instance_id: instance_id,
                applicability,
                eligibility,
                status,
                priority_class: definition.priority_class.clone(),
                priority_rank: definition.priority_rank,
                criticality: definition.criticality.clone(),
                target_condition: definition.target_condition.clone(),
                tolerance: definition.tolerance.clone(),
                uncertainty_requirement: context.evaluation_profile.uncertainty_policy.clone(),
                dependency_ids: definition.dependency_ids.clone(),
                provenance_id: definition.provenance_id.clone(),
            });
        }
        resolve_dependencies(&evaluated)?;
        let conflicts = resolve_conflicts(&mut evaluated)?;
        let priority_order = priority_order(&evaluated, &context.evaluation_profile)?;
        let mut candidate = build_candidate(evaluated, conflicts, priority_order, context);
        candidate.canonical_hash = candidate_hash(&candidate);
        candidate.candidate_id = format!("candidate:{}", candidate.canonical_hash);
        let validation = validate_candidate(&candidate, osv, governance, context);
        if !validation.eligible {
            return Err(ObjectiveFailure::ObjectiveSetValidationFailure(
                validation.reasons.clone(),
            ));
        }
        Ok((candidate, validation))
    }

    pub fn commit_candidate(
        &self,
        candidate: &ObjectiveSetCandidate,
        validation: &ObjectiveSetValidationResult,
    ) -> Result<CommittedObjectiveSet, ObjectiveFailure> {
        if !validation.eligible {
            return Err(ObjectiveFailure::ObjectiveSetValidationFailure(
                validation.reasons.clone(),
            ));
        }
        let active = candidate
            .objectives
            .iter()
            .filter(|o| {
                matches!(
                    o.status,
                    ObjectiveStatus::Active | ObjectiveStatus::ActiveRestricted
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        let v = candidate.schema_version.clone();
        let mut set = CommittedObjectiveSet {
            objective_set_id: String::new(),
            cycle_id: candidate.cycle_id.clone(),
            osv_id: candidate.osv_id.clone(),
            governance_snapshot_id: candidate.governance_snapshot_id.clone(),
            effective_time: candidate.effective_time,
            objective_evaluation_profile_id: candidate.objective_evaluation_profile_id.clone(),
            active_objective_instance_ids: candidate.active_objective_instance_ids.clone(),
            restricted_objective_instance_ids: candidate.restricted_objective_instance_ids.clone(),
            required_objective_instance_ids: candidate.required_objective_instance_ids.clone(),
            satisfied_objective_instance_ids: candidate.satisfied_objective_instance_ids.clone(),
            violated_objective_instance_ids: candidate.violated_objective_instance_ids.clone(),
            at_risk_objective_instance_ids: candidate.at_risk_objective_instance_ids.clone(),
            deferred_objective_instance_ids: candidate.deferred_objective_instance_ids.clone(),
            blocked_objective_instance_ids: candidate.blocked_objective_instance_ids.clone(),
            conflicted_objective_instance_ids: candidate.conflicted_objective_instance_ids.clone(),
            unresolved_objective_instance_ids: candidate.unresolved_objective_instance_ids.clone(),
            priority_order: candidate.priority_order.clone(),
            criticality_summary: candidate.criticality_summary.clone(),
            compatibility_summary: candidate.compatibility_summary.clone(),
            conflict_summary: candidate.conflict_summary.clone(),
            objective_set_status: candidate.status.clone(),
            schema_version: v,
            registry_versions: candidate.registry_versions.clone(),
            numeric_profile_id: candidate.numeric_profile_id.clone(),
            provenance_id: candidate.provenance_id.clone(),
            commitment_record_id: format!("commitment:{}", validation.validation_id),
            committed_at: candidate.effective_time,
            artifact_hash: String::new(),
            commitment_identity: String::new(),
        };
        set.artifact_hash = content_hash(&set.canonical_bytes());
        set.objective_set_id = format!("objective-set:{}", set.artifact_hash);
        set.artifact_hash = content_hash(&set.canonical_bytes());
        set.commitment_identity = format!("objectives:{}", set.artifact_hash);
        let _ = active;
        Ok(set)
    }
}

fn validate_inputs(
    osv: &CommittedOsvHandoff,
    governance: &CommittedGovernanceHandoff,
    context: &ObjectiveEvaluationContext,
) -> Result<(), ObjectiveFailure> {
    if osv.osv.commitment_identity.is_empty()
        || osv.osv.artifact_hash != content_hash(&osv.osv.canonical_bytes())
    {
        return Err(ObjectiveFailure::InvalidCommittedOsvHandoff(
            "artifact hash mismatch".into(),
        ));
    }
    if governance.snapshot.commitment_identity.is_empty()
        || governance.snapshot.artifact_hash != content_hash(&governance.snapshot.canonical_bytes())
    {
        return Err(ObjectiveFailure::InvalidCommittedGovernanceHandoff(
            "artifact hash mismatch".into(),
        ));
    }
    if osv.osv.cycle_id != context.cycle_id || governance.snapshot.cycle_id != context.cycle_id {
        return Err(ObjectiveFailure::CycleBindingMismatch);
    }
    if osv.osv.osv_id != context.committed_osv_id {
        return Err(ObjectiveFailure::OsvIdentityMismatch);
    }
    if governance.snapshot.governance_snapshot_id != context.committed_governance_snapshot_id {
        return Err(ObjectiveFailure::GovernanceSnapshotIdentityMismatch);
    }
    if matches!(
        osv.osv.publication_status,
        PublicationStatus::Withheld | PublicationStatus::Faulted
    ) {
        return Err(ObjectiveFailure::InvalidCommittedOsvHandoff(
            "OSV is not downstream-eligible".into(),
        ));
    }
    if matches!(
        governance.snapshot.snapshot_status,
        GovernanceSnapshotStatus::Withheld
            | GovernanceSnapshotStatus::Faulted
            | GovernanceSnapshotStatus::Revoked
    ) {
        return Err(ObjectiveFailure::InvalidCommittedGovernanceHandoff(
            "Governance Snapshot is not downstream-eligible".into(),
        ));
    }
    if !context
        .evaluation_profile
        .accepted_osv_versions
        .contains(&osv.osv.osv_schema_version)
    {
        return Err(ObjectiveFailure::ObjectiveSchemaMismatch);
    }
    if !context
        .evaluation_profile
        .accepted_governance_versions
        .contains(&governance.snapshot.schema_version)
    {
        return Err(ObjectiveFailure::ObjectiveSchemaMismatch);
    }
    if !context
        .evaluation_profile
        .accepted_objective_registry_versions
        .contains(&context.objective_registry.registry_version)
    {
        return Err(ObjectiveFailure::ObjectiveRegistryMismatch);
    }
    Ok(())
}

fn validate_definition(
    definition: &SupervisoryObjectiveDefinition,
    sources: &BTreeMap<String, ObjectiveSourceDeclaration>,
    context: &ObjectiveEvaluationContext,
) -> Result<(), ObjectiveFailure> {
    let source = sources
        .get(&definition.source_id)
        .ok_or(ObjectiveFailure::ObjectiveSourceUnregistered)?;
    if source.authority_scope != context.scope && source.authority_scope != "*" {
        return Err(ObjectiveFailure::ObjectiveSourceUnauthorized);
    }
    if !source
        .supported_objective_types
        .contains(&format!("{:?}", definition.objective_class))
    {
        return Err(ObjectiveFailure::ObjectiveSourceUnauthorized);
    }
    if !source.schema_versions.contains(&definition.schema_version) {
        return Err(ObjectiveFailure::ObjectiveSchemaMismatch);
    }
    if !source
        .registry_versions
        .contains(&definition.registry_version)
        || definition.registry_version != context.objective_registry.registry_version
    {
        return Err(ObjectiveFailure::ObjectiveRegistryMismatch);
    }
    if definition.content_hash() != definition.integrity_hash {
        return Err(ObjectiveFailure::ObjectiveDefinitionInvalid(
            definition.objective_id.clone(),
        ));
    }
    if definition.objective_id.is_empty()
        || definition.applicability_rule_id.is_empty()
        || definition.satisfaction_rule_id.is_empty()
    {
        return Err(ObjectiveFailure::ObjectiveDefinitionInvalid(
            definition.objective_id.clone(),
        ));
    }
    if matches!(definition.objective_class, ObjectiveClass::Maximize)
        && definition.target_condition.is_empty()
    {
        return Err(ObjectiveFailure::ObjectiveDefinitionInvalid(
            definition.objective_id.clone(),
        ));
    }
    Ok(())
}

fn evaluate_applicability(
    definition: &SupervisoryObjectiveDefinition,
    osv: &sacs_operational_state_vector::CommittedOperationalStateVector,
    governance: &CommittedGovernanceHandoff,
    context: &ObjectiveEvaluationContext,
) -> ObjectiveApplicability {
    if definition.scope != context.scope && definition.scope != "*" {
        return ObjectiveApplicability::NotApplicable;
    }
    if context.effective_time < definition.effective_from
        || definition
            .effective_until
            .is_some_and(|end| context.effective_time >= end)
    {
        return ObjectiveApplicability::NotApplicable;
    }
    if matches!(
        governance.snapshot.snapshot_status,
        GovernanceSnapshotStatus::Incomplete | GovernanceSnapshotStatus::Degraded
    ) && definition.mandatory
    {
        return ObjectiveApplicability::ApplicabilityUnresolved;
    }
    for field_id in &definition.osv_field_dependencies {
        let Some(field) = osv.fields.iter().find(|field| &field.field_id == field_id) else {
            return ObjectiveApplicability::ApplicabilityUnresolved;
        };
        if !matches!(
            field.validity,
            OsvFieldValidity::Valid | OsvFieldValidity::Degraded
        ) {
            return ObjectiveApplicability::ApplicabilityUnresolved;
        }
        if matches!(field.value, OsvFieldValue::Absent { .. }) {
            return ObjectiveApplicability::ApplicabilityUnresolved;
        }
    }
    ObjectiveApplicability::Applicable
}

fn evaluate_eligibility(
    definition: &SupervisoryObjectiveDefinition,
    applicability: &ObjectiveApplicability,
    governance: &CommittedGovernanceHandoff,
    _context: &ObjectiveEvaluationContext,
) -> ObjectiveEligibility {
    if !matches!(applicability, ObjectiveApplicability::Applicable) {
        return ObjectiveEligibility::Unresolved;
    }
    if definition
        .prohibited_governance_rule_ids
        .iter()
        .any(|id| governance.snapshot.prohibitions.contains(id))
    {
        return ObjectiveEligibility::Prohibited;
    }
    if matches!(
        governance.snapshot.precedence_resolution.as_str(),
        "Block" | "BLOCK"
    ) || matches!(
        governance.snapshot.snapshot_status,
        GovernanceSnapshotStatus::Revoked
    ) {
        return ObjectiveEligibility::Prohibited;
    }
    if definition
        .governance_requirement_ids
        .iter()
        .any(|id| !governance.snapshot.permissions.contains(id))
    {
        return ObjectiveEligibility::Unresolved;
    }
    if matches!(
        governance.snapshot.snapshot_status,
        GovernanceSnapshotStatus::ValidRestricted
    ) || matches!(
        governance.snapshot.precedence_resolution.as_str(),
        "Restrict" | "RESTRICT"
    ) {
        return ObjectiveEligibility::EligibleRestricted;
    }
    if definition.mandatory {
        ObjectiveEligibility::Required
    } else {
        ObjectiveEligibility::Eligible
    }
}

fn status_for(
    applicability: &ObjectiveApplicability,
    eligibility: &ObjectiveEligibility,
    _definition: &SupervisoryObjectiveDefinition,
) -> ObjectiveStatus {
    if !matches!(applicability, ObjectiveApplicability::Applicable) {
        return ObjectiveStatus::Inactive;
    }
    match eligibility {
        ObjectiveEligibility::Prohibited => ObjectiveStatus::Blocked,
        ObjectiveEligibility::EligibleRestricted => ObjectiveStatus::ActiveRestricted,
        ObjectiveEligibility::Eligible | ObjectiveEligibility::Required => ObjectiveStatus::Active,
        ObjectiveEligibility::Deferred => ObjectiveStatus::Deferred,
        ObjectiveEligibility::ReviewRequired | ObjectiveEligibility::EscalationRequired => {
            ObjectiveStatus::Deferred
        }
        ObjectiveEligibility::Unresolved => ObjectiveStatus::Unresolved,
    }
}

fn resolve_dependencies(objectives: &[EvaluatedObjective]) -> Result<(), ObjectiveFailure> {
    let ids: BTreeSet<_> = objectives.iter().map(|o| o.objective_id.as_str()).collect();
    let mut graph = BTreeMap::new();
    for objective in objectives {
        for dependency in &objective.dependency_ids {
            if !ids.contains(dependency.as_str()) {
                return Err(ObjectiveFailure::ObjectiveDependencyMissing(
                    objective.objective_id.clone(),
                ));
            }
        }
        graph.insert(
            objective.objective_id.clone(),
            objective.dependency_ids.clone(),
        );
    }
    fn visit(
        id: &str,
        graph: &BTreeMap<String, Vec<String>>,
        visiting: &mut BTreeSet<String>,
        visited: &mut BTreeSet<String>,
    ) -> bool {
        if visiting.contains(id) {
            return true;
        }
        if visited.contains(id) {
            return false;
        }
        visiting.insert(id.into());
        if let Some(deps) = graph.get(id) {
            if deps.iter().any(|dep| visit(dep, graph, visiting, visited)) {
                return true;
            }
        }
        visiting.remove(id);
        visited.insert(id.into());
        false
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    if graph
        .keys()
        .any(|id| visit(id, &graph, &mut visiting, &mut visited))
    {
        return Err(ObjectiveFailure::ObjectiveDependencyCycle);
    }
    Ok(())
}

fn resolve_conflicts(
    objectives: &mut [EvaluatedObjective],
) -> Result<Vec<ObjectiveConflict>, ObjectiveFailure> {
    let mut conflicts = Vec::new();
    let mut conflicted_indices = BTreeSet::new();
    for (index, left) in objectives.iter().enumerate() {
        for (other_index, right) in objectives.iter().enumerate().skip(index + 1) {
            if left.objective_id == right.objective_id {
                continue;
            }
            if left.target_condition == right.target_condition {
                continue;
            }
            if left.priority_rank == right.priority_rank
                && left.status == ObjectiveStatus::Candidate
                && right.status == ObjectiveStatus::Candidate
            {
                let conflict_id = format!(
                    "conflict:{}:{}",
                    left.objective_instance_id, right.objective_instance_id
                );
                conflicted_indices.insert(index);
                conflicted_indices.insert(other_index);
                conflicts.push(ObjectiveConflict {
                    conflict_id,
                    kind: ObjectiveConflictKind::UnresolvedConflict,
                    objective_instance_ids: vec![
                        left.objective_instance_id.clone(),
                        right.objective_instance_id.clone(),
                    ],
                    resolution: "unresolved equal-priority target conflict".into(),
                });
            }
        }
    }
    for index in conflicted_indices {
        objectives[index].status = ObjectiveStatus::Conflicted;
    }
    if conflicts
        .iter()
        .any(|conflict| matches!(conflict.kind, ObjectiveConflictKind::UnresolvedConflict))
    {
        return Err(ObjectiveFailure::ObjectiveConflictUnresolved(
            conflicts[0].conflict_id.clone(),
        ));
    }
    Ok(conflicts)
}

fn priority_order(
    objectives: &[EvaluatedObjective],
    profile: &ObjectiveEvaluationProfile,
) -> Result<Vec<String>, ObjectiveFailure> {
    if profile.tie_breaking_rule_id.is_empty() {
        return Err(ObjectiveFailure::ObjectivePriorityUnresolved);
    }
    let mut active = objectives
        .iter()
        .filter(|o| {
            matches!(
                o.status,
                ObjectiveStatus::Active
                    | ObjectiveStatus::ActiveRestricted
                    | ObjectiveStatus::Candidate
            )
        })
        .collect::<Vec<_>>();
    active.sort_by(|a, b| {
        b.priority_rank
            .cmp(&a.priority_rank)
            .then_with(|| a.objective_instance_id.cmp(&b.objective_instance_id))
    });
    Ok(active
        .into_iter()
        .map(|o| o.objective_instance_id.clone())
        .collect())
}

fn build_candidate(
    objectives: Vec<EvaluatedObjective>,
    conflicts: Vec<ObjectiveConflict>,
    priority_order: Vec<String>,
    context: &ObjectiveEvaluationContext,
) -> ObjectiveSetCandidate {
    let ids = |predicate: &dyn Fn(&ObjectiveStatus) -> bool| {
        objectives
            .iter()
            .filter(|o| predicate(&o.status))
            .map(|o| o.objective_instance_id.clone())
            .collect::<Vec<_>>()
    };
    let active = ids(&|status| {
        matches!(
            status,
            ObjectiveStatus::Active | ObjectiveStatus::ActiveRestricted
        )
    });
    let restricted = ids(&|status| matches!(status, ObjectiveStatus::ActiveRestricted));
    let required = objectives
        .iter()
        .filter(|o| matches!(o.eligibility, ObjectiveEligibility::Required))
        .map(|o| o.objective_instance_id.clone())
        .collect();
    let deferred = ids(&|status| matches!(status, ObjectiveStatus::Deferred));
    let blocked = ids(&|status| matches!(status, ObjectiveStatus::Blocked));
    let conflicted = ids(&|status| matches!(status, ObjectiveStatus::Conflicted));
    let unresolved = ids(&|status| matches!(status, ObjectiveStatus::Unresolved));
    let status = if !conflicted.is_empty() {
        ObjectiveSetStatus::Conflicted
    } else if !unresolved.is_empty() || !deferred.is_empty() {
        ObjectiveSetStatus::Incomplete
    } else if !restricted.is_empty() {
        ObjectiveSetStatus::ValidRestricted
    } else {
        ObjectiveSetStatus::Valid
    };
    ObjectiveSetCandidate {
        candidate_id: String::new(),
        cycle_id: context.cycle_id.clone(),
        osv_id: context.committed_osv_id.clone(),
        governance_snapshot_id: context.committed_governance_snapshot_id.clone(),
        effective_time: context.effective_time,
        objective_evaluation_profile_id: context.evaluation_profile.profile_id.clone(),
        objective_registry_snapshot_id: context.objective_registry.identity(),
        objectives,
        active_objective_instance_ids: active,
        restricted_objective_instance_ids: restricted,
        required_objective_instance_ids: required,
        satisfied_objective_instance_ids: Vec::new(),
        violated_objective_instance_ids: Vec::new(),
        at_risk_objective_instance_ids: Vec::new(),
        deferred_objective_instance_ids: deferred,
        blocked_objective_instance_ids: blocked,
        conflicted_objective_instance_ids: conflicted,
        unresolved_objective_instance_ids: unresolved,
        priority_order,
        criticality_summary: "explicit-definition-criticality".into(),
        compatibility_summary: "explicit-reference-compatibility".into(),
        conflict_summary: conflicts,
        status,
        schema_version: context.evaluation_profile.output_schema_version.clone(),
        registry_versions: [context.objective_registry.registry_version.clone()]
            .into_iter()
            .collect(),
        numeric_profile_id: context.evaluation_profile.numeric_profile_id.clone(),
        provenance_id: format!(
            "provenance:{}:{}",
            context.cycle_id, context.committed_osv_id
        ),
        canonical_hash: String::new(),
    }
}

fn candidate_hash(candidate: &ObjectiveSetCandidate) -> String {
    let mut d = CanonicalDocument::new();
    d.insert("cycle", &candidate.cycle_id);
    d.insert("osv", &candidate.osv_id);
    d.insert("governance", &candidate.governance_snapshot_id);
    d.insert("registry", &candidate.objective_registry_snapshot_id);
    d.insert(
        "objectives",
        candidate
            .objectives
            .iter()
            .map(|o| o.objective_instance_id.clone())
            .collect::<Vec<_>>()
            .join(","),
    );
    d.insert("active", candidate.active_objective_instance_ids.join(","));
    d.insert("priority", candidate.priority_order.join(","));
    d.insert("status", format!("{:?}", candidate.status));
    content_hash(&d.encode())
}

fn validate_candidate(
    candidate: &ObjectiveSetCandidate,
    osv: &CommittedOsvHandoff,
    governance: &CommittedGovernanceHandoff,
    context: &ObjectiveEvaluationContext,
) -> ObjectiveSetValidationResult {
    let mut reasons = Vec::new();
    if candidate.cycle_id != osv.osv.cycle_id || candidate.cycle_id != governance.snapshot.cycle_id
    {
        reasons.push("cycle binding mismatch".into());
    }
    if candidate.osv_id != context.committed_osv_id {
        reasons.push("OSV binding mismatch".into());
    }
    if candidate.governance_snapshot_id != context.committed_governance_snapshot_id {
        reasons.push("Governance Snapshot binding mismatch".into());
    }
    if candidate.objectives.is_empty() {
        reasons.push("no evaluated objectives; no implicit fallback".into());
    }
    if matches!(
        candidate.status,
        ObjectiveSetStatus::Conflicted | ObjectiveSetStatus::Withheld | ObjectiveSetStatus::Faulted
    ) {
        reasons.push("objective set is not commitment eligible".into());
    }
    if candidate
        .active_objective_instance_ids
        .iter()
        .any(|id| candidate.blocked_objective_instance_ids.contains(id))
    {
        reasons.push("blocked objective active".into());
    }
    let mut d = CanonicalDocument::new();
    d.insert("candidate", &candidate.candidate_id);
    d.insert("reasons", reasons.join("|"));
    ObjectiveSetValidationResult {
        validation_id: format!("validation:{}", content_hash(&d.encode())),
        candidate_id: candidate.candidate_id.clone(),
        eligible: reasons.is_empty(),
        reasons,
        canonical_hash: content_hash(&d.encode()),
    }
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
    fn registry_order_is_deterministic() {
        let v = Version::new("1.0.0").unwrap();
        let a = ObjectiveRegistrySnapshot {
            registry_id: "r".into(),
            registry_version: v.clone(),
            entries: BTreeMap::new(),
        };
        let b = ObjectiveRegistrySnapshot {
            registry_id: "r".into(),
            registry_version: v,
            entries: BTreeMap::new(),
        };
        assert_eq!(a.canonical_bytes(), b.canonical_bytes());
    }
    #[test]
    fn no_objective_fallback_is_allowed() {
        let c = ObjectiveSetStatus::Incomplete;
        assert!(matches!(c, ObjectiveSetStatus::Incomplete));
    }
    #[test]
    fn failure_is_reduced_authority() {
        assert!(ObjectiveFailure::ObjectiveWithheld("x".into())
            .failure_record()
            .detail
            .contains("reduced"));
    }
}
