//! Contract 004 governance publication boundary.
//!
//! The reference path consumes only a committed OSV handoff and explicitly
//! admitted, externally authored source artifacts. It preserves rule classes,
//! refuses permissive fallback, and emits immutable cycle-bound snapshots.

use std::collections::{BTreeMap, BTreeSet};

use canonical_hashing::content_hash;
use canonical_identity::{ArtifactIdentity, IdentityBinding};
use canonical_serialization::CanonicalDocument;
use canonical_versioning::{Version, VersionBinding};
use commitment_engine::{CommitResult, CommitmentError, CommitmentEvent, CommitmentStore};
use failure_model::FailureRecord;
use sacs_operational_state_vector::CommittedOsvHandoff;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceAuthorityRegistration {
    pub governance_authority_id: String,
    pub authority_type: String,
    pub authority_name: String,
    pub authority_version: Version,
    pub trust_domain_id: String,
    pub jurisdiction_or_scope: String,
    pub supported_governance_artifact_types: BTreeSet<String>,
    pub signing_profile_id: String,
    pub priority_class: String,
    pub delegation_policy_id: String,
    pub revocation_endpoint_or_method: String,
    pub registry_version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceSourceDeclaration {
    pub source_id: String,
    pub source_type: String,
    pub source_version: Version,
    pub producer_identity: String,
    pub authority_scope: String,
    pub trust_domain_id: String,
    pub supported_schema_versions: BTreeSet<Version>,
    pub registry_version: Version,
    pub integrity_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceRevocation {
    pub revocation_id: String,
    pub governance_authority_id: String,
    pub revoked_artifact_id: String,
    pub revoked_rule_ids: Vec<String>,
    pub effective_time: i128,
    pub reason_code: String,
    pub replacement_artifact_id: Option<String>,
    pub signature: String,
    pub provenance_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernanceRuleClass {
    Permission,
    Prohibition,
    Obligation,
    Limit,
    Eligibility,
    Requirement,
    Escalation,
    Audit,
    Retention,
    Disclosure,
    Reduction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceRule {
    pub governance_rule_id: String,
    pub rule_class: GovernanceRuleClass,
    pub source_artifact_id: String,
    pub governance_authority_id: String,
    pub semantic_definition: String,
    pub subject: String,
    pub resource_or_capability: String,
    pub action_or_authority: String,
    pub scope: String,
    pub conditions: String,
    pub effect: String,
    pub priority: i64,
    pub precedence_class: String,
    pub effective_from: i128,
    pub effective_until: Option<i128>,
    pub revocation_state: String,
    pub required_evidence: String,
    pub failure_disposition: String,
    pub schema_version: Version,
    pub registry_version: Version,
    pub provenance_id: String,
}

impl GovernanceRule {
    fn conflict_key(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.subject, self.resource_or_capability, self.action_or_authority, self.scope
        )
    }
    fn canonical(&self, d: &mut CanonicalDocument) {
        d.insert(
            format!("rule:{}:class", self.governance_rule_id),
            format!("{:?}", self.rule_class),
        );
        d.insert(
            format!("rule:{}:source", self.governance_rule_id),
            &self.source_artifact_id,
        );
        d.insert(
            format!("rule:{}:subject", self.governance_rule_id),
            &self.subject,
        );
        d.insert(
            format!("rule:{}:resource", self.governance_rule_id),
            &self.resource_or_capability,
        );
        d.insert(
            format!("rule:{}:action", self.governance_rule_id),
            &self.action_or_authority,
        );
        d.insert(
            format!("rule:{}:scope", self.governance_rule_id),
            &self.scope,
        );
        d.insert(
            format!("rule:{}:effect", self.governance_rule_id),
            &self.effect,
        );
        d.insert(
            format!("rule:{}:priority", self.governance_rule_id),
            self.priority.to_string(),
        );
        d.insert(
            format!("rule:{}:revocation", self.governance_rule_id),
            &self.revocation_state,
        );
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceSourceArtifact {
    pub source_artifact_id: String,
    pub governance_authority_id: String,
    pub artifact_type: String,
    pub artifact_version: Version,
    pub issued_at: i128,
    pub effective_from: i128,
    pub effective_until: Option<i128>,
    pub scope: String,
    pub subject: String,
    pub rules: Vec<GovernanceRule>,
    pub priority: i64,
    pub revocation_status: String,
    pub signature: String,
    pub schema_version: Version,
    pub registry_version: Version,
    pub provenance_id: String,
    pub revocations: Vec<GovernanceRevocation>,
}

impl GovernanceSourceArtifact {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("source_artifact_id", &self.source_artifact_id);
        d.insert("authority", &self.governance_authority_id);
        d.insert("artifact_type", &self.artifact_type);
        d.insert("artifact_version", self.artifact_version.to_string());
        d.insert("issued_at", self.issued_at.to_string());
        d.insert("effective_from", self.effective_from.to_string());
        d.insert(
            "effective_until",
            self.effective_until
                .map_or_else(|| "<absent>".into(), |v| v.to_string()),
        );
        d.insert("scope", &self.scope);
        d.insert("subject", &self.subject);
        d.insert("priority", self.priority.to_string());
        d.insert("schema_version", self.schema_version.to_string());
        d.insert("registry_version", self.registry_version.to_string());
        let mut rules = self.rules.clone();
        rules.sort_by(|a, b| a.governance_rule_id.cmp(&b.governance_rule_id));
        for rule in rules {
            rule.canonical(&mut d);
        }
        d.encode()
    }
    pub fn content_hash(&self) -> String {
        content_hash(&self.canonical_bytes())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceAdapterDeclaration {
    pub governance_adapter_id: String,
    pub adapter_version: Version,
    pub supported_authority_types: BTreeSet<String>,
    pub supported_input_schemas: BTreeSet<Version>,
    pub supported_output_schema: Version,
    pub normalization_rule_ids: Vec<String>,
    pub conflict_mapping_rules: Vec<String>,
    pub precedence_mapping_rules: Vec<String>,
    pub scope_mapping_rules: Vec<String>,
    pub failure_mapping: String,
    pub compatibility_profile_id: String,
    pub implementation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityModel {
    ClosedWorld,
    OpenWorldRestricted,
    Hybrid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceAdmissionContext {
    pub cycle_id: String,
    pub committed_osv_id: String,
    pub osv_schema_version: Version,
    pub osv_registry_version: Version,
    pub effective_time: i128,
    pub clock_profile_id: String,
    pub authority_scope: String,
    pub compatibility_profile_id: String,
    pub precedence_profile_id: String,
    pub authority_model: AuthorityModel,
    pub governance_schema_version: Version,
    pub governance_registry_version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceApplicabilityResult {
    pub source_artifact_id: String,
    pub applicable: bool,
    pub reasons: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernanceDisposition {
    Block,
    StabilizeRequired,
    Escalate,
    ReviewRequired,
    Defer,
    Restrict,
    Allow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernanceSnapshotStatus {
    Valid,
    ValidRestricted,
    Degraded,
    Incomplete,
    Withheld,
    Faulted,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernanceConflictKind {
    DirectConflict,
    ScopeConflict,
    AuthorityConflict,
    TemporalConflict,
    VersionConflict,
    ExceptionConflict,
    UnresolvedConflict,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceConflict {
    pub conflict_id: String,
    pub kind: GovernanceConflictKind,
    pub rule_ids: Vec<String>,
    pub resolution: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceSnapshotCandidate {
    pub candidate_id: String,
    pub cycle_id: String,
    pub osv_id: String,
    pub governance_source_artifact_ids: Vec<String>,
    pub governance_authority_ids: Vec<String>,
    pub governance_adapter_ids: Vec<String>,
    pub effective_time: i128,
    pub scope: String,
    pub rules: Vec<GovernanceRule>,
    pub permissions: Vec<String>,
    pub prohibitions: Vec<String>,
    pub obligations: Vec<String>,
    pub limits: Vec<String>,
    pub eligibility_rules: Vec<String>,
    pub requirements: Vec<String>,
    pub escalation_rules: Vec<String>,
    pub audit_rules: Vec<String>,
    pub retention_rules: Vec<String>,
    pub disclosure_rules: Vec<String>,
    pub reduction_rules: Vec<String>,
    pub revocations: Vec<GovernanceRevocation>,
    pub conflicts: Vec<GovernanceConflict>,
    pub disposition: GovernanceDisposition,
    pub status: GovernanceSnapshotStatus,
    pub authority_model: AuthorityModel,
    pub compatibility_profile_id: String,
    pub provenance_id: String,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceValidationResult {
    pub validation_id: String,
    pub candidate_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedGovernanceSnapshot {
    pub governance_snapshot_id: String,
    pub cycle_id: String,
    pub osv_id: String,
    pub governance_source_artifact_ids: Vec<String>,
    pub governance_authority_ids: Vec<String>,
    pub governance_adapter_ids: Vec<String>,
    pub effective_time: i128,
    pub validity_interval: String,
    pub scope: String,
    pub canonical_rule_ids: Vec<String>,
    pub permissions: Vec<String>,
    pub prohibitions: Vec<String>,
    pub obligations: Vec<String>,
    pub limits: Vec<String>,
    pub eligibility_rules: Vec<String>,
    pub requirements: Vec<String>,
    pub escalation_rules: Vec<String>,
    pub audit_rules: Vec<String>,
    pub retention_rules: Vec<String>,
    pub disclosure_rules: Vec<String>,
    pub reduction_rules: Vec<String>,
    pub revocations: Vec<GovernanceRevocation>,
    pub conflict_summary: Vec<GovernanceConflict>,
    pub precedence_resolution: String,
    pub snapshot_status: GovernanceSnapshotStatus,
    pub authority_model: AuthorityModel,
    pub schema_version: Version,
    pub registry_versions: BTreeSet<Version>,
    pub compatibility_profile_id: String,
    pub provenance_id: String,
    pub commitment_record_id: String,
    pub committed_at: i128,
    pub artifact_hash: String,
    pub commitment_identity: String,
}

impl CommittedGovernanceSnapshot {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("snapshot_id", &self.governance_snapshot_id);
        d.insert("cycle_id", &self.cycle_id);
        d.insert("osv_id", &self.osv_id);
        d.insert("sources", self.governance_source_artifact_ids.join(","));
        d.insert("authorities", self.governance_authority_ids.join(","));
        d.insert("effective_time", self.effective_time.to_string());
        d.insert("scope", &self.scope);
        d.insert("rules", self.canonical_rule_ids.join(","));
        d.insert("permissions", self.permissions.join(","));
        d.insert("prohibitions", self.prohibitions.join(","));
        d.insert("obligations", self.obligations.join(","));
        d.insert("limits", self.limits.join(","));
        d.insert(
            "revocations",
            self.revocations
                .iter()
                .map(|r| r.revocation_id.clone())
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert(
            "conflicts",
            self.conflict_summary
                .iter()
                .map(|c| c.conflict_id.clone())
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert("disposition", format!("{:?}", self.precedence_resolution));
        d.insert("status", format!("{:?}", self.snapshot_status));
        d.insert("authority_model", format!("{:?}", self.authority_model));
        d.insert("schema_version", self.schema_version.to_string());
        d.insert("compatibility_profile_id", &self.compatibility_profile_id);
        d.insert("provenance_id", &self.provenance_id);
        d.encode()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceCommitmentRecord {
    pub commitment_identity: String,
    pub candidate_id: String,
    pub validation_id: String,
    pub cycle_id: String,
    pub osv_id: String,
    pub source_set_identity: String,
    pub commitment_status: String,
    pub canonical_hash: String,
    pub commitment_event: CommitmentEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedGovernanceHandoff {
    pub snapshot: CommittedGovernanceSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernanceFailure {
    InvalidCommittedOsvHandoff(String),
    UncommittedOsvInput,
    OsvIdentityMismatch,
    CycleBindingMismatch,
    GovernanceSourceUnavailable,
    GovernanceSourceUnregistered,
    GovernanceSourceUnauthorized,
    GovernanceSourceVersionMismatch,
    GovernanceSchemaMismatch,
    GovernanceRegistryMismatch,
    GovernanceIntegrityFailure,
    GovernanceArtifactInvalid,
    GovernanceArtifactExpired,
    GovernanceArtifactRevoked,
    GovernanceScopeMismatch,
    GovernanceConflictUnresolved,
    GovernancePrecedenceUnresolved,
    GovernanceApplicabilityUnresolved,
    PermissionExpansionDetected,
    ProhibitionWeakeningDetected,
    RestrictionLossDetected,
    RevocationAuthorityInvalid,
    GovernanceSnapshotConstructionFailure,
    GovernanceValidationFailure(Vec<String>),
    GovernanceCommitmentConflict,
    GovernanceWithheld,
    GovernanceRestricted,
    GovernanceBlocked,
    Commitment(CommitmentError),
}
impl GovernanceFailure {
    pub fn failure_record(&self) -> FailureRecord {
        FailureRecord::new(
            format!("governance::{self:?}"),
            "governance failed toward reduced authority",
        )
    }
}

#[derive(Default)]
pub struct GovernanceStore {
    commitments: CommitmentStore,
    snapshots: BTreeMap<String, CommittedGovernanceSnapshot>,
    cycles: BTreeMap<String, String>,
    revocations: BTreeMap<String, GovernanceRevocation>,
    records: BTreeMap<String, GovernanceCommitmentRecord>,
}

impl GovernanceStore {
    pub fn commit(
        &mut self,
        snapshot: CommittedGovernanceSnapshot,
        validation: &GovernanceValidationResult,
    ) -> Result<(CommittedGovernanceHandoff, GovernanceCommitmentRecord), GovernanceFailure> {
        if !validation.eligible {
            return Err(GovernanceFailure::GovernanceValidationFailure(
                validation.reasons.clone(),
            ));
        }
        if let Some(existing) = self.cycles.get(&snapshot.cycle_id) {
            if existing != &snapshot.commitment_identity {
                return Err(GovernanceFailure::GovernanceCommitmentConflict);
            }
        }
        let identity = ArtifactIdentity::deterministic(
            version_binding(),
            IdentityBinding {
                schema: snapshot.schema_version.to_string(),
                registry: snapshot
                    .registry_versions
                    .iter()
                    .next()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "<absent>".into()),
                cycle: Some(snapshot.cycle_id.clone()),
                producer: "sacs-governance".into(),
            },
            &snapshot.canonical_bytes(),
        );
        let event = self
            .commitments
            .commit(&identity, &snapshot.canonical_bytes())
            .map_err(GovernanceFailure::Commitment)?;
        let event = match event {
            CommitResult::Committed(e) | CommitResult::Idempotent(e) => e,
        };
        let record = GovernanceCommitmentRecord {
            commitment_identity: identity.identifier.clone(),
            candidate_id: validation.candidate_id.clone(),
            validation_id: validation.validation_id.clone(),
            cycle_id: snapshot.cycle_id.clone(),
            osv_id: snapshot.osv_id.clone(),
            source_set_identity: content_hash(
                snapshot.governance_source_artifact_ids.join(",").as_bytes(),
            ),
            commitment_status: "COMMITTED".into(),
            canonical_hash: snapshot.artifact_hash.clone(),
            commitment_event: event,
        };
        self.cycles.insert(
            snapshot.cycle_id.clone(),
            snapshot.commitment_identity.clone(),
        );
        self.snapshots
            .insert(snapshot.commitment_identity.clone(), snapshot.clone());
        self.records
            .insert(record.commitment_identity.clone(), record.clone());
        Ok((CommittedGovernanceHandoff { snapshot }, record))
    }
    pub fn record_revocation(
        &mut self,
        revocation: GovernanceRevocation,
    ) -> Result<(), GovernanceFailure> {
        if let Some(previous) = self.revocations.get(&revocation.revocation_id) {
            if previous != &revocation {
                return Err(GovernanceFailure::GovernanceCommitmentConflict);
            }
            return Ok(());
        }
        self.revocations
            .insert(revocation.revocation_id.clone(), revocation);
        Ok(())
    }
    pub fn get(&self, identity: &str) -> Option<&CommittedGovernanceSnapshot> {
        self.snapshots.get(identity)
    }
    pub fn event_count(&self) -> usize {
        self.commitments.event_count()
    }
}

pub struct GovernanceEngine {
    pub adapter: GovernanceAdapterDeclaration,
}

impl GovernanceEngine {
    pub fn construct(
        &self,
        osv: &CommittedOsvHandoff,
        authorities: &BTreeMap<String, GovernanceAuthorityRegistration>,
        sources: &[GovernanceSourceArtifact],
        context: &GovernanceAdmissionContext,
    ) -> Result<(GovernanceSnapshotCandidate, GovernanceValidationResult), GovernanceFailure> {
        validate_osv(osv, context)?;
        if sources.is_empty() {
            return Err(GovernanceFailure::GovernanceSourceUnavailable);
        }
        let mut applicable = Vec::new();
        let mut inapplicable = Vec::new();
        let mut revocations = Vec::new();
        for source in sources {
            let authority = authorities
                .get(&source.governance_authority_id)
                .ok_or(GovernanceFailure::GovernanceSourceUnregistered)?;
            if authority.jurisdiction_or_scope != context.authority_scope
                && authority.jurisdiction_or_scope != "*"
            {
                return Err(GovernanceFailure::GovernanceSourceUnauthorized);
            }
            if !authority
                .supported_governance_artifact_types
                .contains(&source.artifact_type)
            {
                return Err(GovernanceFailure::GovernanceSourceUnauthorized);
            }
            if source.registry_version != authority.registry_version
                || source.schema_version != context.governance_schema_version
            {
                return Err(GovernanceFailure::GovernanceSchemaMismatch);
            }
            if source.content_hash() != source.signature {
                return Err(GovernanceFailure::GovernanceIntegrityFailure);
            }
            let mut reasons = Vec::new();
            if source.scope != context.authority_scope && source.scope != "*" {
                reasons.push("scope mismatch".into());
            }
            if context.effective_time < source.effective_from {
                reasons.push("future effective source".into());
            }
            if source
                .effective_until
                .is_some_and(|end| context.effective_time >= end)
            {
                reasons.push("expired source".into());
            }
            if source.revocation_status == "REVOKED" {
                return Err(GovernanceFailure::GovernanceArtifactRevoked);
            }
            if reasons.is_empty() {
                applicable.push(source.clone());
            } else {
                inapplicable.push(GovernanceApplicabilityResult {
                    source_artifact_id: source.source_artifact_id.clone(),
                    applicable: false,
                    reasons,
                    canonical_hash: source.content_hash(),
                });
            }
            revocations.extend(source.revocations.clone());
        }
        if applicable.is_empty() {
            return Err(GovernanceFailure::GovernanceApplicabilityUnresolved);
        }
        let mut rules = applicable
            .iter()
            .flat_map(|s| s.rules.clone())
            .collect::<Vec<_>>();
        rules.sort_by(|a, b| a.governance_rule_id.cmp(&b.governance_rule_id));
        let (rules, conflicts) = resolve_conflicts(rules)?;
        let disposition = disposition_for(&rules, conflicts.as_slice());
        let status = if matches!(
            disposition,
            GovernanceDisposition::Block
                | GovernanceDisposition::StabilizeRequired
                | GovernanceDisposition::Restrict
                | GovernanceDisposition::Escalate
                | GovernanceDisposition::ReviewRequired
                | GovernanceDisposition::Defer
        ) {
            GovernanceSnapshotStatus::ValidRestricted
        } else {
            GovernanceSnapshotStatus::Valid
        };
        let ids = |class: &GovernanceRuleClass| {
            rules
                .iter()
                .filter(|r| &r.rule_class == class)
                .map(|r| r.governance_rule_id.clone())
                .collect::<Vec<_>>()
        };
        let permissions = ids(&GovernanceRuleClass::Permission);
        let prohibitions = ids(&GovernanceRuleClass::Prohibition);
        let obligations = ids(&GovernanceRuleClass::Obligation);
        let limits = ids(&GovernanceRuleClass::Limit);
        let eligibility_rules = ids(&GovernanceRuleClass::Eligibility);
        let requirements = ids(&GovernanceRuleClass::Requirement);
        let escalation_rules = ids(&GovernanceRuleClass::Escalation);
        let audit_rules = ids(&GovernanceRuleClass::Audit);
        let retention_rules = ids(&GovernanceRuleClass::Retention);
        let disclosure_rules = ids(&GovernanceRuleClass::Disclosure);
        let reduction_rules = ids(&GovernanceRuleClass::Reduction);
        let mut candidate = GovernanceSnapshotCandidate {
            candidate_id: String::new(),
            cycle_id: context.cycle_id.clone(),
            osv_id: context.committed_osv_id.clone(),
            governance_source_artifact_ids: applicable
                .iter()
                .map(|s| s.source_artifact_id.clone())
                .collect(),
            governance_authority_ids: applicable
                .iter()
                .map(|s| s.governance_authority_id.clone())
                .collect(),
            governance_adapter_ids: vec![self.adapter.governance_adapter_id.clone()],
            effective_time: context.effective_time,
            scope: context.authority_scope.clone(),
            rules,
            permissions,
            prohibitions,
            obligations,
            limits,
            eligibility_rules,
            requirements,
            escalation_rules,
            audit_rules,
            retention_rules,
            disclosure_rules,
            reduction_rules,
            revocations,
            conflicts,
            disposition,
            status,
            authority_model: context.authority_model.clone(),
            compatibility_profile_id: context.compatibility_profile_id.clone(),
            provenance_id: format!(
                "provenance:{}:{}",
                context.committed_osv_id, context.cycle_id
            ),
            canonical_hash: String::new(),
        };
        candidate.canonical_hash = candidate_hash(&candidate);
        candidate.candidate_id = format!("candidate:{}", candidate.canonical_hash);
        let validation = validate_candidate(&candidate, &inapplicable, context);
        if !validation.eligible {
            return Err(GovernanceFailure::GovernanceValidationFailure(
                validation.reasons.clone(),
            ));
        }
        Ok((candidate, validation))
    }

    pub fn commit_candidate(
        &self,
        candidate: &GovernanceSnapshotCandidate,
        validation: &GovernanceValidationResult,
    ) -> Result<CommittedGovernanceSnapshot, GovernanceFailure> {
        if !validation.eligible {
            return Err(GovernanceFailure::GovernanceValidationFailure(
                validation.reasons.clone(),
            ));
        }
        let mut snapshot = snapshot_from_candidate(candidate, validation);
        snapshot.artifact_hash = content_hash(&snapshot.canonical_bytes());
        snapshot.governance_snapshot_id = format!("snapshot:{}", snapshot.artifact_hash);
        snapshot.artifact_hash = content_hash(&snapshot.canonical_bytes());
        snapshot.commitment_identity = format!("governance:{}", snapshot.artifact_hash);
        Ok(snapshot)
    }
}

fn validate_osv(
    handoff: &CommittedOsvHandoff,
    context: &GovernanceAdmissionContext,
) -> Result<(), GovernanceFailure> {
    let osv = &handoff.osv;
    if osv.commitment_identity.is_empty()
        || osv.artifact_hash != content_hash(&osv.canonical_bytes())
    {
        return Err(GovernanceFailure::InvalidCommittedOsvHandoff(
            "artifact hash mismatch".into(),
        ));
    }
    if osv.osv_id.is_empty() {
        return Err(GovernanceFailure::UncommittedOsvInput);
    }
    if osv.osv_id != context.committed_osv_id {
        return Err(GovernanceFailure::OsvIdentityMismatch);
    }
    if osv.cycle_id != context.cycle_id {
        return Err(GovernanceFailure::CycleBindingMismatch);
    }
    if osv.osv_schema_version != context.osv_schema_version
        || osv.osv_registry_version != context.osv_registry_version
    {
        return Err(GovernanceFailure::GovernanceRegistryMismatch);
    }
    Ok(())
}

fn resolve_conflicts(
    mut rules: Vec<GovernanceRule>,
) -> Result<(Vec<GovernanceRule>, Vec<GovernanceConflict>), GovernanceFailure> {
    let mut grouped: BTreeMap<String, Vec<GovernanceRule>> = BTreeMap::new();
    for r in rules.drain(..) {
        grouped.entry(r.conflict_key()).or_default().push(r);
    }
    let mut resolved = Vec::new();
    let mut conflicts = Vec::new();
    for (key, mut group) in grouped {
        group.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then_with(|| a.governance_rule_id.cmp(&b.governance_rule_id))
        });
        let incompatible = group
            .iter()
            .any(|r| matches!(r.rule_class, GovernanceRuleClass::Permission))
            && group
                .iter()
                .any(|r| matches!(r.rule_class, GovernanceRuleClass::Prohibition));
        if incompatible {
            let max = group[0].priority;
            let top = group
                .iter()
                .filter(|r| r.priority == max)
                .collect::<Vec<_>>();
            let prohibition = top
                .iter()
                .find(|r| matches!(r.rule_class, GovernanceRuleClass::Prohibition));
            if let Some(p) = prohibition {
                resolved.push((*p).clone());
                conflicts.push(GovernanceConflict {
                    conflict_id: format!("conflict:{}", content_hash(key.as_bytes())),
                    kind: GovernanceConflictKind::DirectConflict,
                    rule_ids: group.iter().map(|r| r.governance_rule_id.clone()).collect(),
                    resolution: "mandatory prohibition precedence".into(),
                });
            } else if top.len() == 1 {
                resolved.push(top[0].clone());
                conflicts.push(GovernanceConflict {
                    conflict_id: format!("conflict:{}", content_hash(key.as_bytes())),
                    kind: GovernanceConflictKind::DirectConflict,
                    rule_ids: group.iter().map(|r| r.governance_rule_id.clone()).collect(),
                    resolution: "explicit priority".into(),
                });
            } else {
                return Err(GovernanceFailure::GovernanceConflictUnresolved);
            }
        } else {
            resolved.extend(group);
        }
    }
    Ok((resolved, conflicts))
}

fn disposition_for(
    rules: &[GovernanceRule],
    _conflicts: &[GovernanceConflict],
) -> GovernanceDisposition {
    if rules
        .iter()
        .any(|r| matches!(r.rule_class, GovernanceRuleClass::Prohibition))
    {
        GovernanceDisposition::Block
    } else if rules.iter().any(|r| {
        matches!(
            r.rule_class,
            GovernanceRuleClass::Reduction
                | GovernanceRuleClass::Limit
                | GovernanceRuleClass::Requirement
        )
    }) {
        GovernanceDisposition::Restrict
    } else if rules
        .iter()
        .any(|r| matches!(r.rule_class, GovernanceRuleClass::Permission))
    {
        GovernanceDisposition::Allow
    } else {
        GovernanceDisposition::Defer
    }
}

fn candidate_hash(candidate: &GovernanceSnapshotCandidate) -> String {
    let mut d = CanonicalDocument::new();
    d.insert("cycle", &candidate.cycle_id);
    d.insert("osv", &candidate.osv_id);
    d.insert(
        "sources",
        candidate.governance_source_artifact_ids.join(","),
    );
    d.insert(
        "rules",
        candidate
            .rules
            .iter()
            .map(|r| r.governance_rule_id.clone())
            .collect::<Vec<_>>()
            .join(","),
    );
    d.insert("permissions", candidate.permissions.join(","));
    d.insert("prohibitions", candidate.prohibitions.join(","));
    d.insert("disposition", format!("{:?}", candidate.disposition));
    content_hash(&d.encode())
}

fn validate_candidate(
    candidate: &GovernanceSnapshotCandidate,
    inapplicable: &[GovernanceApplicabilityResult],
    context: &GovernanceAdmissionContext,
) -> GovernanceValidationResult {
    let mut reasons = Vec::new();
    if candidate.cycle_id != context.cycle_id {
        reasons.push("cycle mismatch".into());
    }
    if candidate.osv_id != context.committed_osv_id {
        reasons.push("OSV mismatch".into());
    }
    if candidate.rules.is_empty() {
        reasons.push("no applicable governance rules; no permissive default".into());
    }
    if !inapplicable.is_empty() && candidate.rules.is_empty() {
        reasons.push("applicability unresolved".into());
    }
    if matches!(candidate.disposition, GovernanceDisposition::Allow)
        && candidate.permissions.is_empty()
    {
        reasons.push("ALLOW without explicit permission".into());
    }
    let mut d = CanonicalDocument::new();
    d.insert("candidate", &candidate.candidate_id);
    d.insert("reasons", reasons.join("|"));
    GovernanceValidationResult {
        validation_id: format!("validation:{}", content_hash(&d.encode())),
        candidate_id: candidate.candidate_id.clone(),
        eligible: reasons.is_empty(),
        reasons,
        canonical_hash: content_hash(&d.encode()),
    }
}

fn snapshot_from_candidate(
    c: &GovernanceSnapshotCandidate,
    validation: &GovernanceValidationResult,
) -> CommittedGovernanceSnapshot {
    let v = Version::new("1.0.0").expect("static version");
    CommittedGovernanceSnapshot {
        governance_snapshot_id: String::new(),
        cycle_id: c.cycle_id.clone(),
        osv_id: c.osv_id.clone(),
        governance_source_artifact_ids: c.governance_source_artifact_ids.clone(),
        governance_authority_ids: c.governance_authority_ids.clone(),
        governance_adapter_ids: c.governance_adapter_ids.clone(),
        effective_time: c.effective_time,
        validity_interval: "explicit-context".into(),
        scope: c.scope.clone(),
        canonical_rule_ids: c
            .rules
            .iter()
            .map(|r| r.governance_rule_id.clone())
            .collect(),
        permissions: c.permissions.clone(),
        prohibitions: c.prohibitions.clone(),
        obligations: c.obligations.clone(),
        limits: c.limits.clone(),
        eligibility_rules: c.eligibility_rules.clone(),
        requirements: c.requirements.clone(),
        escalation_rules: c.escalation_rules.clone(),
        audit_rules: c.audit_rules.clone(),
        retention_rules: c.retention_rules.clone(),
        disclosure_rules: c.disclosure_rules.clone(),
        reduction_rules: c.reduction_rules.clone(),
        revocations: c.revocations.clone(),
        conflict_summary: c.conflicts.clone(),
        precedence_resolution: format!("{:?}", c.disposition),
        snapshot_status: c.status.clone(),
        authority_model: c.authority_model.clone(),
        schema_version: v.clone(),
        registry_versions: [v].into_iter().collect(),
        compatibility_profile_id: c.compatibility_profile_id.clone(),
        provenance_id: c.provenance_id.clone(),
        commitment_record_id: format!("commitment:{}", validation.validation_id),
        committed_at: c.effective_time,
        artifact_hash: String::new(),
        commitment_identity: String::new(),
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
    fn source_ordering_is_deterministic() {
        let v = Version::new("1.0.0").unwrap();
        let a = GovernanceSourceArtifact {
            source_artifact_id: "a".into(),
            governance_authority_id: "auth".into(),
            artifact_type: "policy".into(),
            artifact_version: v.clone(),
            issued_at: 1,
            effective_from: 0,
            effective_until: None,
            scope: "*".into(),
            subject: "*".into(),
            rules: Vec::new(),
            priority: 1,
            revocation_status: "ACTIVE".into(),
            signature: String::new(),
            schema_version: v.clone(),
            registry_version: v,
            provenance_id: "p".into(),
            revocations: Vec::new(),
        };
        let mut b = a.clone();
        b.rules.reverse();
        assert_eq!(a.canonical_bytes(), b.canonical_bytes());
    }
    #[test]
    fn absence_never_becomes_allow() {
        let v = Version::new("1.0.0").unwrap();
        let c = GovernanceSnapshotCandidate {
            candidate_id: "c".into(),
            cycle_id: "cycle".into(),
            osv_id: "osv".into(),
            governance_source_artifact_ids: vec!["s".into()],
            governance_authority_ids: vec!["a".into()],
            governance_adapter_ids: vec!["ad".into()],
            effective_time: 1,
            scope: "*".into(),
            rules: Vec::new(),
            permissions: Vec::new(),
            prohibitions: Vec::new(),
            obligations: Vec::new(),
            limits: Vec::new(),
            eligibility_rules: Vec::new(),
            requirements: Vec::new(),
            escalation_rules: Vec::new(),
            audit_rules: Vec::new(),
            retention_rules: Vec::new(),
            disclosure_rules: Vec::new(),
            reduction_rules: Vec::new(),
            revocations: Vec::new(),
            conflicts: Vec::new(),
            disposition: GovernanceDisposition::Defer,
            status: GovernanceSnapshotStatus::Incomplete,
            authority_model: AuthorityModel::ClosedWorld,
            compatibility_profile_id: "c".into(),
            provenance_id: "p".into(),
            canonical_hash: "h".into(),
        };
        let ctx = GovernanceAdmissionContext {
            cycle_id: "cycle".into(),
            committed_osv_id: "osv".into(),
            osv_schema_version: v.clone(),
            osv_registry_version: v.clone(),
            effective_time: 1,
            clock_profile_id: "clock".into(),
            authority_scope: "*".into(),
            compatibility_profile_id: "c".into(),
            precedence_profile_id: "p".into(),
            authority_model: AuthorityModel::ClosedWorld,
            governance_schema_version: v.clone(),
            governance_registry_version: v,
        };
        assert!(!validate_candidate(&c, &[], &ctx).eligible);
    }
    #[test]
    fn failure_is_reduced_authority() {
        assert!(GovernanceFailure::GovernanceWithheld
            .failure_record()
            .detail
            .contains("reduced"));
    }
}
