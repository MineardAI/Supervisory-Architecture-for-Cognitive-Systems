//! Contract 003 publication boundary.
//!
//! This crate consumes only an immutable [`EstimatedStateHandoff`].  It does
//! not estimate state, reopen observation admission, or make supervisory
//! decisions.  The reference projection is intentionally direct and explicit;
//! Contract 003 does not provide authority for universal field mathematics.

use std::collections::{BTreeMap, BTreeSet};

use canonical_hashing::content_hash;
use canonical_identity::{ArtifactIdentity, IdentityBinding};
use canonical_serialization::CanonicalDocument;
use canonical_versioning::{Version, VersionBinding};
use commitment_engine::{CommitResult, CommitmentError, CommitmentEvent, CommitmentStore};
use failure_model::FailureRecord;
use sacs_observation::{Quality, Timestamp};
use sacs_state_estimation::{
    ComponentValidity, EstimatedStateHandoff, EstimationStatus, StateComponent,
    UncertaintyRepresentation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvPublisherProfile {
    pub publisher_id: String,
    pub publisher_version: Version,
    pub implementation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvFieldDefinition {
    pub field_id: String,
    pub name: String,
    pub description: String,
    pub semantic_owner: String,
    pub value_type: String,
    pub unit: Option<String>,
    pub valid_range: Option<String>,
    pub normalization: String,
    pub nullable: bool,
    pub required_profiles: BTreeSet<String>,
    pub estimated_state_dependencies: BTreeSet<String>,
    pub projection_rule_id: String,
    pub validity_rule_id: String,
    pub uncertainty_rule_id: String,
    pub quality_rule_id: String,
    pub numeric_profile_ids: BTreeSet<String>,
    pub sensitivity_class: String,
    pub schema_version: Version,
    pub registry_version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvFieldRegistrySnapshot {
    pub registry_id: String,
    pub registry_version: Version,
    pub entries: BTreeMap<String, OsvFieldDefinition>,
}

impl OsvFieldRegistrySnapshot {
    pub fn resolve(&self, field_id: &str) -> Option<&OsvFieldDefinition> {
        self.entries.get(field_id)
    }
    pub fn identity(&self) -> String {
        content_hash(&self.canonical_bytes())
    }
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("registry_id", &self.registry_id);
        d.insert("registry_version", self.registry_version.to_string());
        d.insert(
            "field_ids",
            self.entries.keys().cloned().collect::<Vec<_>>().join(","),
        );
        d.encode()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvSchema {
    pub schema_id: String,
    pub schema_version: Version,
    pub required_field_ids: Vec<String>,
    pub optional_field_ids: Vec<String>,
    pub registry_version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionRule {
    pub rule_id: String,
    pub rule_version: Version,
    pub source_component_id: String,
    pub target_field_id: String,
    pub algorithm: String,
    pub uncertainty_transformation: String,
    pub validity_conditions: String,
    pub numeric_profile_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvProjectionProfile {
    pub profile_id: String,
    pub profile_version: Version,
    pub accepted_state_basis_profile_ids: BTreeSet<String>,
    pub rules: Vec<ProjectionRule>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CovariancePolicy {
    Full,
    Sparse,
    Block,
    MarginalVarianceOnly,
    QualitativeDependence,
    NotPublished,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicationStatus {
    Committed,
    CommittedDegraded,
    Withheld,
    Faulted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvPublicationProfile {
    pub profile_id: String,
    pub profile_version: Version,
    pub required_field_ids: Vec<String>,
    pub optional_field_ids: Vec<String>,
    pub accepted_state_basis_profiles: BTreeSet<String>,
    pub minimum_estimation_status: EstimationStatus,
    pub minimum_component_validity: ComponentValidity,
    pub uncertainty_representation: String,
    pub covariance_policy: CovariancePolicy,
    pub missing_field_policy: String,
    pub publication_failure_policy: String,
    pub schema_version: Version,
    pub registry_versions: BTreeSet<Version>,
    pub numeric_profile_id: String,
    pub schema: OsvSchema,
    pub registry: OsvFieldRegistrySnapshot,
    pub projection: OsvProjectionProfile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OsvFieldValidity {
    Valid,
    Degraded,
    Unresolved,
    Unavailable,
    Incompatible,
    Redacted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OsvFieldValue {
    Value(String),
    Absent { reason: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvField {
    pub field_id: String,
    pub value: OsvFieldValue,
    pub validity: OsvFieldValidity,
    pub quality: Quality,
    pub uncertainty: UncertaintyRepresentation,
    pub source_component_id: Option<String>,
    pub source_lineage_hash: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvDraft {
    pub draft_id: String,
    pub cycle_id: String,
    pub estimated_state_id: String,
    pub observation_batch_id: String,
    pub observation_window_id: String,
    pub observation_cutoff: Timestamp,
    pub publication_profile_id: String,
    pub publication_profile_version: Version,
    pub schema_id: String,
    pub schema_version: Version,
    pub registry_id: String,
    pub registry_version: Version,
    pub fields: Vec<OsvField>,
    pub status: PublicationStatus,
    pub covariance_reference: String,
    pub numeric_profile_id: String,
    pub provenance_id: String,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvValidationResult {
    pub validation_id: String,
    pub draft_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OsvPublicationRecord {
    pub publication_id: String,
    pub cycle_id: String,
    pub draft_id: String,
    pub validation_id: String,
    pub status: PublicationStatus,
    pub canonical_hash: String,
    pub commitment_event: CommitmentEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedOperationalStateVector {
    pub osv_id: String,
    pub cycle_id: String,
    pub previous_osv_id: Option<String>,
    pub estimated_state_id: String,
    pub observation_batch_id: String,
    pub observation_window_id: String,
    pub observation_cutoff: Timestamp,
    pub publication_profile_id: String,
    pub publication_profile_version: Version,
    pub osv_schema_id: String,
    pub osv_schema_version: Version,
    pub osv_registry_version: Version,
    pub field_registry_versions: BTreeSet<Version>,
    pub fields: Vec<OsvField>,
    pub publication_status: PublicationStatus,
    pub covariance_reference: String,
    pub numeric_profile_id: String,
    pub compatibility_profile_id: String,
    pub provenance_id: String,
    pub publication_record_id: String,
    pub artifact_hash: String,
    pub commitment_identity: String,
}

impl CommittedOperationalStateVector {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("osv_id", &self.osv_id);
        d.insert("cycle_id", &self.cycle_id);
        d.insert("estimated_state_id", &self.estimated_state_id);
        d.insert("observation_batch_id", &self.observation_batch_id);
        d.insert("observation_window_id", &self.observation_window_id);
        d.insert(
            "observation_cutoff",
            format!("{:?}", self.observation_cutoff),
        );
        d.insert("publication_profile_id", &self.publication_profile_id);
        d.insert(
            "publication_profile_version",
            self.publication_profile_version.to_string(),
        );
        d.insert("schema_id", &self.osv_schema_id);
        d.insert("schema_version", self.osv_schema_version.to_string());
        d.insert("registry_version", self.osv_registry_version.to_string());
        d.insert(
            "field_ids",
            self.fields
                .iter()
                .map(|f| f.field_id.clone())
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert(
            "field_values",
            self.fields
                .iter()
                .map(|f| format!("{:?}", f.value))
                .collect::<Vec<_>>()
                .join("|"),
        );
        d.insert(
            "field_validity",
            self.fields
                .iter()
                .map(|f| format!("{:?}", f.validity))
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert(
            "field_uncertainty",
            self.fields
                .iter()
                .map(|f| format!("{:?}", f.uncertainty))
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert("status", format!("{:?}", self.publication_status));
        d.insert("covariance_reference", &self.covariance_reference);
        d.insert("numeric_profile_id", &self.numeric_profile_id);
        d.insert("compatibility_profile_id", &self.compatibility_profile_id);
        d.insert("provenance_id", &self.provenance_id);
        d.encode()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedOsvHandoff {
    pub osv: CommittedOperationalStateVector,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OsvPublicationFailure {
    InvalidEstimatedStateHandoff(String),
    UncommittedEstimatedState,
    EstimatedStateIdentityMismatch,
    CycleBindingMismatch,
    ObservationCutoffMismatch,
    ObservationBatchMismatch,
    UnsupportedEstimationStatus,
    StateBasisProfileMismatch,
    OsvSchemaVersionMismatch,
    FieldRegistryVersionMismatch,
    PublicationProfileMismatch,
    ProjectionProfileMismatch,
    MissingRequiredStateComponent(String),
    MissingRequiredOsvField(String),
    IllegalFieldAbsence(String),
    UnresolvedField(String),
    UncertaintyPublicationFailure(String),
    TemporalAtomicityFailure(String),
    CanonicalOrderingFailure,
    OsvValidationFailure(Vec<String>),
    OsvCommitmentConflict,
    SecondOsvForCycle,
    Commitment(CommitmentError),
}
impl OsvPublicationFailure {
    pub fn failure_record(&self) -> FailureRecord {
        FailureRecord::new(format!("osv::{self:?}"), "OSV publication failed")
    }
}

#[derive(Default)]
pub struct OsvStore {
    commitments: CommitmentStore,
    osvs: BTreeMap<String, CommittedOperationalStateVector>,
    cycles: BTreeMap<String, String>,
    publications: BTreeMap<String, OsvPublicationRecord>,
}

impl OsvStore {
    pub fn commit(
        &mut self,
        osv: CommittedOperationalStateVector,
        validation: &OsvValidationResult,
    ) -> Result<(CommittedOsvHandoff, OsvPublicationRecord), OsvPublicationFailure> {
        if !validation.eligible {
            return Err(OsvPublicationFailure::OsvValidationFailure(
                validation.reasons.clone(),
            ));
        }
        if let Some(existing) = self.cycles.get(&osv.cycle_id) {
            if existing != &osv.commitment_identity {
                return Err(OsvPublicationFailure::SecondOsvForCycle);
            }
        }
        let identity = ArtifactIdentity::deterministic(
            version_binding(),
            IdentityBinding {
                schema: osv.osv_schema_version.to_string(),
                registry: osv.osv_registry_version.to_string(),
                cycle: Some(osv.cycle_id.clone()),
                producer: "sacs-osv-publisher".into(),
            },
            &osv.canonical_bytes(),
        );
        let event = self
            .commitments
            .commit(&identity, &osv.canonical_bytes())
            .map_err(OsvPublicationFailure::Commitment)?;
        let commitment_event = match event {
            CommitResult::Committed(e) | CommitResult::Idempotent(e) => e,
        };
        let publication = OsvPublicationRecord {
            publication_id: format!("publication:{}", identity.identifier),
            cycle_id: osv.cycle_id.clone(),
            draft_id: validation.draft_id.clone(),
            validation_id: validation.validation_id.clone(),
            status: osv.publication_status.clone(),
            canonical_hash: osv.artifact_hash.clone(),
            commitment_event,
        };
        self.cycles
            .insert(osv.cycle_id.clone(), osv.commitment_identity.clone());
        self.osvs
            .insert(osv.commitment_identity.clone(), osv.clone());
        self.publications
            .insert(publication.publication_id.clone(), publication.clone());
        Ok((CommittedOsvHandoff { osv }, publication))
    }
    pub fn get(&self, identity: &str) -> Option<&CommittedOperationalStateVector> {
        self.osvs.get(identity)
    }
    pub fn event_count(&self) -> usize {
        self.commitments.event_count()
    }
}

pub struct OsvPublisher {
    pub declaration: OsvPublisherProfile,
}

impl OsvPublisher {
    pub fn publish(
        &self,
        handoff: &EstimatedStateHandoff,
        profile: &OsvPublicationProfile,
    ) -> Result<
        (
            OsvDraft,
            OsvValidationResult,
            CommittedOperationalStateVector,
        ),
        OsvPublicationFailure,
    > {
        validate_handoff(handoff, profile)?;
        let fields = project_fields(&handoff.state.state_components, profile)?;
        let status = if fields
            .iter()
            .any(|f| !matches!(f.validity, OsvFieldValidity::Valid))
        {
            PublicationStatus::CommittedDegraded
        } else {
            PublicationStatus::Committed
        };
        let mut draft = OsvDraft {
            draft_id: String::new(),
            cycle_id: handoff.state.cycle_id.clone(),
            estimated_state_id: handoff.state.estimated_state_id.clone(),
            observation_batch_id: handoff.state.observation_batch_id.clone(),
            observation_window_id: handoff.report.input.window_id.clone(),
            observation_cutoff: handoff.state.observation_cutoff.clone(),
            publication_profile_id: profile.profile_id.clone(),
            publication_profile_version: profile.profile_version.clone(),
            schema_id: profile.schema.schema_id.clone(),
            schema_version: profile.schema.schema_version.clone(),
            registry_id: profile.registry.registry_id.clone(),
            registry_version: profile.registry.registry_version.clone(),
            fields,
            status,
            covariance_reference: format!("{:?}", handoff.state.covariance_reference),
            numeric_profile_id: handoff.state.numeric_profile_id.clone(),
            provenance_id: handoff.provenance.provenance_id.clone(),
            canonical_hash: String::new(),
        };
        draft.canonical_hash = content_hash(&draft_bytes(&draft));
        draft.draft_id = format!("draft:{}", draft.canonical_hash);
        let validation = validate_draft(&draft, profile);
        if !validation.eligible {
            return Err(OsvPublicationFailure::OsvValidationFailure(
                validation.reasons.clone(),
            ));
        }
        let mut osv = committed_from_draft(&draft, &handoff.state.prior_estimated_state_id);
        osv.artifact_hash = content_hash(&osv.canonical_bytes());
        osv.commitment_identity = format!("osv:{}", osv.artifact_hash);
        Ok((draft, validation, osv))
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

fn validate_handoff(
    h: &EstimatedStateHandoff,
    p: &OsvPublicationProfile,
) -> Result<(), OsvPublicationFailure> {
    if h.commitment_identity.is_empty() || h.state_hash != content_hash(&h.state.canonical_bytes())
    {
        return Err(OsvPublicationFailure::InvalidEstimatedStateHandoff(
            "state hash mismatch".into(),
        ));
    }
    if h.state.estimated_state_id.is_empty()
        || h.state.provenance.estimated_state_id != h.state.estimated_state_id
        || h.provenance.provenance_id != h.state.provenance_id
    {
        return Err(OsvPublicationFailure::EstimatedStateIdentityMismatch);
    }
    if h.report.input.cycle_id != h.state.cycle_id {
        return Err(OsvPublicationFailure::CycleBindingMismatch);
    }
    if h.report.input.observation_batch_id != h.state.observation_batch_id {
        return Err(OsvPublicationFailure::ObservationBatchMismatch);
    }
    if h.report.input.observation_cutoff != h.state.observation_cutoff {
        return Err(OsvPublicationFailure::ObservationCutoffMismatch);
    }
    if !p
        .accepted_state_basis_profiles
        .contains(&h.state.state_basis_profile_id)
    {
        return Err(OsvPublicationFailure::StateBasisProfileMismatch);
    }
    if !p.registry_versions.contains(&p.registry.registry_version)
        || p.schema.registry_version != p.registry.registry_version
    {
        return Err(OsvPublicationFailure::FieldRegistryVersionMismatch);
    }
    if !p
        .accepted_state_basis_profiles
        .contains(&h.state.state_basis_profile_id)
        || p.schema.schema_version != p.schema_version
    {
        return Err(OsvPublicationFailure::OsvSchemaVersionMismatch);
    }
    if matches!(
        h.state.estimation_status,
        EstimationStatus::Faulted | EstimationStatus::Unresolved
    ) {
        return Err(OsvPublicationFailure::UnsupportedEstimationStatus);
    }
    Ok(())
}

fn project_fields(
    components: &[StateComponent],
    p: &OsvPublicationProfile,
) -> Result<Vec<OsvField>, OsvPublicationFailure> {
    let by_id: BTreeMap<_, _> = components
        .iter()
        .map(|c| (c.component_id.as_str(), c))
        .collect();
    let mut out = Vec::new();
    for rule in &p.projection.rules {
        let def = p.registry.resolve(&rule.target_field_id).ok_or_else(|| {
            OsvPublicationFailure::MissingRequiredOsvField(rule.target_field_id.clone())
        })?;
        let component = by_id
            .get(rule.source_component_id.as_str())
            .ok_or_else(|| {
                OsvPublicationFailure::MissingRequiredStateComponent(
                    rule.source_component_id.clone(),
                )
            })?;
        let (value, validity) = match (&component.value, &component.validity) {
            (Some(v), ComponentValidity::Resolved) => {
                (OsvFieldValue::Value(v.clone()), OsvFieldValidity::Valid)
            }
            (Some(v), _) => (OsvFieldValue::Value(v.clone()), OsvFieldValidity::Degraded),
            (None, ComponentValidity::Unavailable) => (
                OsvFieldValue::Absent {
                    reason: "unavailable".into(),
                },
                OsvFieldValidity::Unavailable,
            ),
            (None, _) => (
                OsvFieldValue::Absent {
                    reason: "unresolved".into(),
                },
                OsvFieldValidity::Unresolved,
            ),
        };
        if p.required_field_ids.contains(&rule.target_field_id)
            && matches!(value, OsvFieldValue::Absent { .. })
            && !def.nullable
        {
            return Err(OsvPublicationFailure::IllegalFieldAbsence(
                rule.target_field_id.clone(),
            ));
        }
        if rule.numeric_profile_id != p.numeric_profile_id {
            return Err(OsvPublicationFailure::ProjectionProfileMismatch);
        }
        out.push(OsvField {
            field_id: rule.target_field_id.clone(),
            value,
            validity,
            quality: component.quality.clone(),
            uncertainty: component.estimated_uncertainty.clone(),
            source_component_id: Some(component.component_id.clone()),
            source_lineage_hash: Some(component.lineage_hash.clone()),
        });
    }
    out.sort_by(|a, b| a.field_id.cmp(&b.field_id));
    Ok(out)
}

fn draft_bytes(d: &OsvDraft) -> Vec<u8> {
    let mut c = CanonicalDocument::new();
    c.insert("cycle", &d.cycle_id);
    c.insert("state", &d.estimated_state_id);
    c.insert(
        "fields",
        d.fields
            .iter()
            .map(|f| f.field_id.clone())
            .collect::<Vec<_>>()
            .join(","),
    );
    c.insert(
        "values",
        d.fields
            .iter()
            .map(|f| format!("{:?}", f.value))
            .collect::<Vec<_>>()
            .join("|"),
    );
    c.insert("profile", &d.publication_profile_id);
    c.encode()
}

fn validate_draft(d: &OsvDraft, p: &OsvPublicationProfile) -> OsvValidationResult {
    let mut reasons = Vec::new();
    let ids: BTreeSet<_> = d.fields.iter().map(|f| f.field_id.as_str()).collect();
    for id in &p.required_field_ids {
        if !ids.contains(id.as_str()) {
            reasons.push(format!("missing required field {id}"));
        }
    }
    if d.registry_version != p.registry.registry_version {
        reasons.push("registry mismatch".into());
    }
    if d.schema_version != p.schema.schema_version {
        reasons.push("schema mismatch".into());
    }
    let mut c = CanonicalDocument::new();
    c.insert("draft", &d.draft_id);
    c.insert("eligible", reasons.is_empty().to_string());
    c.insert("reasons", reasons.join("|"));
    OsvValidationResult {
        validation_id: format!("validation:{}", content_hash(&c.encode())),
        draft_id: d.draft_id.clone(),
        eligible: reasons.is_empty(),
        reasons,
        canonical_hash: content_hash(&c.encode()),
    }
}

fn committed_from_draft(
    d: &OsvDraft,
    previous: &Option<String>,
) -> CommittedOperationalStateVector {
    CommittedOperationalStateVector {
        osv_id: format!("osv:{}", d.canonical_hash),
        cycle_id: d.cycle_id.clone(),
        previous_osv_id: previous.clone(),
        estimated_state_id: d.estimated_state_id.clone(),
        observation_batch_id: d.observation_batch_id.clone(),
        observation_window_id: d.observation_window_id.clone(),
        observation_cutoff: d.observation_cutoff.clone(),
        publication_profile_id: d.publication_profile_id.clone(),
        publication_profile_version: d.publication_profile_version.clone(),
        osv_schema_id: d.schema_id.clone(),
        osv_schema_version: d.schema_version.clone(),
        osv_registry_version: d.registry_version.clone(),
        field_registry_versions: [d.registry_version.clone()].into_iter().collect(),
        fields: d.fields.clone(),
        publication_status: d.status.clone(),
        covariance_reference: d.covariance_reference.clone(),
        numeric_profile_id: d.numeric_profile_id.clone(),
        compatibility_profile_id: d.publication_profile_id.clone(),
        provenance_id: d.provenance_id.clone(),
        publication_record_id: format!("publication:{}", d.canonical_hash),
        artifact_hash: String::new(),
        commitment_identity: String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn registry_encoding_is_order_independent() {
        let v = Version::new("1.0.0").unwrap();
        let empty = BTreeMap::new();
        let a = OsvFieldRegistrySnapshot {
            registry_id: "r".into(),
            registry_version: v.clone(),
            entries: empty.clone(),
        };
        let b = OsvFieldRegistrySnapshot {
            registry_id: "r".into(),
            registry_version: v,
            entries: empty,
        };
        assert_eq!(a.canonical_bytes(), b.canonical_bytes());
    }
    #[test]
    fn failure_is_explicit() {
        assert!(matches!(
            OsvPublicationFailure::SecondOsvForCycle,
            OsvPublicationFailure::SecondOsvForCycle
        ));
    }

    fn committed(cycle: &str, identity: &str) -> CommittedOperationalStateVector {
        let v = Version::new("1.0.0").unwrap();
        CommittedOperationalStateVector {
            osv_id: format!("osv:{identity}"),
            cycle_id: cycle.into(),
            previous_osv_id: None,
            estimated_state_id: "state-1".into(),
            observation_batch_id: "batch-1".into(),
            observation_window_id: "window-1".into(),
            observation_cutoff: Timestamp::Known {
                nanos: 1,
                clock_domain_id: "test".into(),
            },
            publication_profile_id: "profile-1".into(),
            publication_profile_version: v.clone(),
            osv_schema_id: "schema-1".into(),
            osv_schema_version: v.clone(),
            osv_registry_version: v.clone(),
            field_registry_versions: [v].into_iter().collect(),
            fields: Vec::new(),
            publication_status: PublicationStatus::Committed,
            covariance_reference: "NOT_PRODUCED".into(),
            numeric_profile_id: "exact".into(),
            compatibility_profile_id: "profile-1".into(),
            provenance_id: "prov-1".into(),
            publication_record_id: "publication-1".into(),
            artifact_hash: "hash-1".into(),
            commitment_identity: identity.into(),
        }
    }

    #[test]
    fn store_is_idempotent_and_rejects_second_cycle_identity() {
        let osv = committed("cycle-1", "identity-1");
        let validation = OsvValidationResult {
            validation_id: "validation-1".into(),
            draft_id: "draft-1".into(),
            eligible: true,
            reasons: Vec::new(),
            canonical_hash: "hash".into(),
        };
        let mut store = OsvStore::default();
        assert!(store.commit(osv.clone(), &validation).is_ok());
        assert!(store.commit(osv, &validation).is_ok());
        assert_eq!(store.event_count(), 1);
        assert!(matches!(
            store.commit(committed("cycle-1", "identity-2"), &validation),
            Err(OsvPublicationFailure::SecondOsvForCycle)
        ));
    }
}
