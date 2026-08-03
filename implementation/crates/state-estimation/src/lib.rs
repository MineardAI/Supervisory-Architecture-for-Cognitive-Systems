//! Contract 002 state estimation.
//!
//! The estimator consumes only the committed Contract 001 ObservationHandoff.
//! It produces Estimated State and an immutable Contract 003-facing handoff;
//! it does not construct or publish an Operational State Vector.

use std::collections::{BTreeMap, BTreeSet};

use canonical_hashing::{canonical_hash, content_hash};
use canonical_identity::{ArtifactIdentity, IdentityBinding};
use canonical_serialization::CanonicalDocument;
use canonical_versioning::{Version, VersionBinding};
use commitment_engine::{CommitResult, CommitmentError, CommitmentStore};
use failure_model::FailureRecord;
use sacs_observation::{
    Availability, BatchStatus, ObservationHandoff, ObservationRecord, Quality, Timestamp,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatorDefinition {
    pub estimator_id: String,
    pub estimator_version: Version,
    pub algorithm_class: String,
    pub algorithm_version: Version,
    pub numeric_profile_ids: BTreeSet<String>,
    pub determinism_claim: String,
    pub stateful_or_stateless: String,
    pub implementation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatorRegistrySnapshot {
    pub registry_version: Version,
    pub entries: BTreeMap<String, EstimatorDefinition>,
}

impl EstimatorRegistrySnapshot {
    pub fn resolve(&self, estimator_id: &str) -> Option<&EstimatorDefinition> {
        self.entries.get(estimator_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateComponentDefinition {
    pub component_id: String,
    pub observation_type_ids: BTreeSet<String>,
    pub semantics: String,
    pub component_type: String,
    pub unit: Option<String>,
    pub schema_version: Version,
    pub registry_version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateBasisProfile {
    pub state_basis_profile_id: String,
    pub state_basis_profile_version: Version,
    pub components: BTreeMap<String, StateComponentDefinition>,
    pub uncertainty_model: String,
    pub covariance_ordering: Vec<String>,
    pub initialization_rules: String,
    pub update_rules: String,
    pub compatibility_rules: String,
    pub schema_version: Version,
    pub registry_version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PriorStatePolicy {
    Forbidden,
    Explicit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissingDataPolicy {
    FailRequired,
    UnresolvedComponent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StaleDataPolicy {
    Reject,
    AcceptWithAge,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConflictPolicy {
    Fail,
    PreserveUnresolved,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UncertaintyPolicy {
    Unknown,
    QualitativeClass,
    PreserveSource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatorProfile {
    pub estimator_profile_id: String,
    pub estimator_profile_version: Version,
    pub accepted_observation_batch_versions: BTreeSet<Version>,
    pub required_observation_types: BTreeSet<String>,
    pub optional_observation_types: BTreeSet<String>,
    pub state_basis_profile_id: String,
    pub prior_state_policy: PriorStatePolicy,
    pub missing_data_policy: MissingDataPolicy,
    pub stale_data_policy: StaleDataPolicy,
    pub conflict_policy: ConflictPolicy,
    pub uncertainty_policy: UncertaintyPolicy,
    pub numeric_profile_id: String,
    pub initialization_policy: String,
    pub reset_policy: String,
    pub failure_policy: String,
    pub output_schema_version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatorConfiguration {
    pub estimator: EstimatorDefinition,
    pub profile: EstimatorProfile,
    pub basis: StateBasisProfile,
    pub registry_snapshot: EstimatorRegistrySnapshot,
    pub observation_registry_versions: BTreeSet<Version>,
    pub observation_schema_versions: BTreeSet<Version>,
    pub numeric_profile_id: String,
    pub prior_estimated_state_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatorInputBinding {
    pub observation_handoff_identity: String,
    pub observation_batch_id: String,
    pub cycle_id: String,
    pub window_id: String,
    pub observation_cutoff: Timestamp,
    pub ordered_observation_ids: Vec<String>,
    pub expected_observation_manifest_id: String,
    pub availability_summary: String,
    pub quality_summary: String,
    pub late_data_summary: String,
    pub registry_versions: BTreeSet<Version>,
    pub schema_versions: BTreeSet<Version>,
    pub provenance_reference: String,
    pub commitment_identity: String,
    pub batch_hash: String,
}

impl EstimatorInputBinding {
    pub fn from_handoff(handoff: &ObservationHandoff) -> Self {
        let batch = &handoff.batch;
        Self {
            observation_handoff_identity: handoff.commitment_identity.clone(),
            observation_batch_id: batch.observation_batch_id.clone(),
            cycle_id: batch.cycle_id.clone(),
            window_id: batch.window_id.clone(),
            observation_cutoff: batch.observation_cutoff.clone(),
            ordered_observation_ids: batch.ordered_observation_ids.clone(),
            expected_observation_manifest_id: batch.expected_observation_manifest_id.clone(),
            availability_summary: batch.missing_observation_summary.clone(),
            quality_summary: batch.invalid_observation_summary.clone(),
            late_data_summary: batch.late_observation_summary.clone(),
            registry_versions: batch.registry_versions.clone(),
            schema_versions: batch.schema_versions.clone(),
            provenance_reference: batch.provenance_id.clone(),
            commitment_identity: handoff.commitment_identity.clone(),
            batch_hash: batch.batch_hash.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TreatmentStatus {
    Accepted,
    Excluded,
    Missing,
    Stale,
    Invalid,
    Unavailable,
    Unsupported,
    Redacted,
    Pending,
    Conflicting,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationTreatment {
    pub observation_id: Option<String>,
    pub observation_type_id: String,
    pub status: TreatmentStatus,
    pub reason: String,
    pub rule_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UncertaintyRepresentation {
    BoundedInterval,
    Variance,
    Covariance,
    FiniteDistribution,
    QualitativeClass,
    Unknown,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentValidity {
    Resolved,
    Unresolved,
    Unavailable,
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateComponent {
    pub component_id: String,
    pub value: Option<String>,
    pub contributing_observation_ids: Vec<String>,
    pub source_confidences: Vec<Option<String>>,
    pub source_uncertainties: Vec<Option<String>>,
    pub estimator_confidence: Option<String>,
    pub estimated_uncertainty: UncertaintyRepresentation,
    pub validity: ComponentValidity,
    pub quality: Quality,
    pub lineage_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CovarianceReference {
    pub covariance_status: String,
    pub covariance_id: Option<String>,
    pub component_order: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EstimationStatus {
    Complete,
    Partial,
    Degraded,
    Unresolved,
    Faulted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EstimatorStateLifecycle {
    Uninitialized,
    Initializing,
    Active,
    Degraded,
    ResetPending,
    Faulted,
    Retired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatorProvenance {
    pub provenance_id: String,
    pub estimator_id: String,
    pub estimator_version: Version,
    pub estimator_configuration_id: String,
    pub observation_handoff_identity: String,
    pub observation_batch_id: String,
    pub contributing_observation_ids: Vec<String>,
    pub numeric_profile_id: String,
    pub missing_data_rule_id: String,
    pub stale_data_rule_id: String,
    pub conflict_rule_id: String,
    pub prior_estimated_state_id: Option<String>,
    pub estimated_state_id: String,
    pub schema_version: Version,
    pub registry_versions: BTreeSet<Version>,
    pub creation_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatorReport {
    pub report_id: String,
    pub input: EstimatorInputBinding,
    pub configuration_id: String,
    pub treatments: Vec<ObservationTreatment>,
    pub output_component_ids: Vec<String>,
    pub unresolved_component_ids: Vec<String>,
    pub uncertainty_representation: UncertaintyRepresentation,
    pub outcome: EstimationStatus,
    pub algorithm_identity: String,
    pub canonical_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatedState {
    pub estimated_state_id: String,
    pub cycle_id: String,
    pub observation_batch_id: String,
    pub observation_cutoff: Timestamp,
    pub estimator_profile_id: String,
    pub estimator_profile_version: Version,
    pub estimator_implementation_ids: Vec<String>,
    pub state_basis_profile_id: String,
    pub state_components: Vec<StateComponent>,
    pub uncertainty_representation: UncertaintyRepresentation,
    pub covariance_reference: CovarianceReference,
    pub component_validity: BTreeMap<String, ComponentValidity>,
    pub component_quality: BTreeMap<String, Quality>,
    pub input_coverage: Vec<ObservationTreatment>,
    pub estimation_status: EstimationStatus,
    pub prior_estimated_state_id: Option<String>,
    pub schema_version: Version,
    pub registry_versions: BTreeSet<Version>,
    pub numeric_profile_id: String,
    pub provenance_id: String,
    pub created_at: Timestamp,
    pub provenance: EstimatorProvenance,
}

impl EstimatedState {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("cycle_id", &self.cycle_id);
        d.insert("estimation_status", format!("{:?}", self.estimation_status));
        d.insert("estimated_state_id", &self.estimated_state_id);
        d.insert("observation_batch_id", &self.observation_batch_id);
        d.insert(
            "observation_cutoff",
            format!("{:?}", self.observation_cutoff),
        );
        d.insert("profile_id", &self.estimator_profile_id);
        d.insert("basis_id", &self.state_basis_profile_id);
        d.insert(
            "component_ids",
            self.state_components
                .iter()
                .map(|component| component.component_id.clone())
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert(
            "component_values",
            self.state_components
                .iter()
                .map(|component| component.value.clone().unwrap_or_else(|| "<absent>".into()))
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert(
            "uncertainty",
            format!("{:?}", self.uncertainty_representation),
        );
        d.insert("numeric_profile_id", &self.numeric_profile_id);
        d.encode()
    }

    pub fn identity(
        &self,
        versions: VersionBinding,
        producer: impl Into<String>,
    ) -> ArtifactIdentity {
        ArtifactIdentity::deterministic(
            versions,
            IdentityBinding {
                schema: self.schema_version.to_string(),
                registry: self
                    .registry_versions
                    .iter()
                    .next()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "<absent>".into()),
                cycle: Some(self.cycle_id.clone()),
                producer: producer.into(),
            },
            &self.canonical_bytes(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EstimatedStateHandoff {
    pub state: EstimatedState,
    pub report: EstimatorReport,
    pub provenance: EstimatorProvenance,
    pub commitment_identity: String,
    pub state_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EstimationFailure {
    InvalidObservationHandoff,
    ObservationIdentityMismatch,
    CycleBindingMismatch,
    UnsupportedEstimator,
    EstimatorDefinitionUnavailable,
    EstimatorConfigurationInvalid,
    EstimatorVersionMismatch,
    NumericProfileMismatch,
    SchemaVersionMismatch,
    RegistryVersionMismatch,
    UncommittedObservationInput,
    MissingRequiredObservation(String),
    DisallowedStaleObservation(String),
    UnresolvedObservationConflict(String),
    InvalidObservation(String),
    UnsupportedUncertaintyRepresentation,
    PriorStateUnavailable,
    DeterministicEvaluationFailure(String),
    EstimatedStateCommitmentConflict,
    Commitment(CommitmentError),
}

impl EstimationFailure {
    pub fn failure_record(&self) -> FailureRecord {
        FailureRecord::new(format!("estimation::{self:?}"), "state estimation failed")
    }
}

#[derive(Default)]
pub struct StateEstimationStore {
    commitments: CommitmentStore,
    states: BTreeMap<String, EstimatedState>,
}

pub struct ReferenceEstimator {
    pub implementation_id: String,
}

impl Default for ReferenceEstimator {
    fn default() -> Self {
        Self {
            implementation_id: "sacs-reference-exact-rational-mean".into(),
        }
    }
}

impl ReferenceEstimator {
    pub fn estimate(
        &self,
        handoff: &ObservationHandoff,
        config: &EstimatorConfiguration,
    ) -> Result<(EstimatedState, EstimatorReport), EstimationFailure> {
        self.validate_input(handoff, config)?;
        let input = EstimatorInputBinding::from_handoff(handoff);
        let mut treatments = Vec::new();
        let mut components = Vec::new();
        let mut unresolved = Vec::new();
        for definition in config.basis.components.values() {
            let relevant: Vec<&ObservationRecord> = handoff
                .batch
                .observations
                .iter()
                .filter(|record| {
                    definition
                        .observation_type_ids
                        .contains(&record.observation_type_id)
                })
                .collect();
            if relevant.is_empty() {
                if config
                    .profile
                    .required_observation_types
                    .iter()
                    .any(|kind| definition.observation_type_ids.contains(kind))
                    && matches!(
                        config.profile.missing_data_policy,
                        MissingDataPolicy::FailRequired
                    )
                {
                    return Err(EstimationFailure::MissingRequiredObservation(
                        definition.component_id.clone(),
                    ));
                }
                unresolved.push(definition.component_id.clone());
                treatments.push(ObservationTreatment {
                    observation_id: None,
                    observation_type_id: definition.component_id.clone(),
                    status: TreatmentStatus::Missing,
                    reason: "no admitted observation".into(),
                    rule_id: "missing-data-policy".into(),
                });
                continue;
            }
            if relevant.len() > 1
                && matches!(config.profile.conflict_policy, ConflictPolicy::Fail)
                && distinct_values(&relevant)
            {
                return Err(EstimationFailure::UnresolvedObservationConflict(
                    definition.component_id.clone(),
                ));
            }
            let mut values = Vec::new();
            let mut ids = Vec::new();
            let mut confidences = Vec::new();
            let mut uncertainties = Vec::new();
            let mut quality = Quality::Nominal;
            for record in relevant {
                let treatment = treatment_for(record, &config.profile)?;
                if matches!(
                    treatment.status,
                    TreatmentStatus::Accepted | TreatmentStatus::Stale
                ) {
                    if let Some(value) = numeric_value(record) {
                        values.push(value);
                        ids.push(record.observation_id.clone());
                        confidences.push(record.confidence.clone());
                        uncertainties.push(record.uncertainty.clone());
                        quality = record.quality_state.clone();
                    } else {
                        return Err(EstimationFailure::InvalidObservation(
                            record.observation_id.clone(),
                        ));
                    }
                }
                treatments.push(treatment);
            }
            if values.is_empty() {
                unresolved.push(definition.component_id.clone());
                continue;
            }
            let sum: i128 = values.iter().sum();
            let value = if values.len() == 1 {
                sum.to_string()
            } else {
                format!("{sum}/{}", values.len())
            };
            let lineage_hash = canonical_hash(&document_for(&ids));
            components.push(StateComponent {
                component_id: definition.component_id.clone(),
                value: Some(value),
                contributing_observation_ids: ids,
                source_confidences: confidences,
                source_uncertainties: uncertainties,
                estimator_confidence: None,
                estimated_uncertainty: uncertainty_for(&config.profile.uncertainty_policy),
                validity: ComponentValidity::Resolved,
                quality,
                lineage_hash,
            });
        }
        let status = if unresolved.is_empty() {
            EstimationStatus::Complete
        } else {
            EstimationStatus::Partial
        };
        let mut state = EstimatedState {
            estimated_state_id: String::new(),
            cycle_id: handoff.batch.cycle_id.clone(),
            observation_batch_id: handoff.batch.observation_batch_id.clone(),
            observation_cutoff: handoff.batch.observation_cutoff.clone(),
            estimator_profile_id: config.profile.estimator_profile_id.clone(),
            estimator_profile_version: config.profile.estimator_profile_version.clone(),
            estimator_implementation_ids: vec![
                self.implementation_id.clone(),
                config.estimator.implementation_hash.clone(),
            ],
            state_basis_profile_id: config.basis.state_basis_profile_id.clone(),
            state_components: components,
            uncertainty_representation: uncertainty_for(&config.profile.uncertainty_policy),
            covariance_reference: CovarianceReference {
                covariance_status: "NOT_PRODUCED".into(),
                covariance_id: None,
                component_order: config.basis.covariance_ordering.clone(),
            },
            component_validity: BTreeMap::new(),
            component_quality: BTreeMap::new(),
            input_coverage: treatments.clone(),
            estimation_status: status.clone(),
            prior_estimated_state_id: config.prior_estimated_state_id.clone(),
            schema_version: config.profile.output_schema_version.clone(),
            registry_versions: config.observation_registry_versions.clone(),
            numeric_profile_id: config.numeric_profile_id.clone(),
            provenance_id: format!("provenance:{}", handoff.commitment_identity),
            created_at: handoff.batch.observation_cutoff.clone(),
            provenance: EstimatorProvenance {
                provenance_id: format!("provenance:{}", handoff.commitment_identity),
                estimator_id: config.estimator.estimator_id.clone(),
                estimator_version: config.estimator.estimator_version.clone(),
                estimator_configuration_id: configuration_id(config),
                observation_handoff_identity: handoff.commitment_identity.clone(),
                observation_batch_id: handoff.batch.observation_batch_id.clone(),
                contributing_observation_ids: treatments
                    .iter()
                    .filter_map(|t| t.observation_id.clone())
                    .collect(),
                numeric_profile_id: config.numeric_profile_id.clone(),
                missing_data_rule_id: format!("{:?}", config.profile.missing_data_policy),
                stale_data_rule_id: format!("{:?}", config.profile.stale_data_policy),
                conflict_rule_id: format!("{:?}", config.profile.conflict_policy),
                prior_estimated_state_id: config.prior_estimated_state_id.clone(),
                estimated_state_id: String::new(),
                schema_version: config.profile.output_schema_version.clone(),
                registry_versions: config.observation_registry_versions.clone(),
                creation_identity: handoff.commitment_identity.clone(),
            },
        };
        for component in &state.state_components {
            state
                .component_validity
                .insert(component.component_id.clone(), component.validity.clone());
            state
                .component_quality
                .insert(component.component_id.clone(), component.quality.clone());
        }
        state.estimated_state_id = content_hash(&state.canonical_bytes());
        state.provenance.estimated_state_id = state.estimated_state_id.clone();
        let report = EstimatorReport {
            report_id: format!("report:{}", state.estimated_state_id),
            input,
            configuration_id: configuration_id(config),
            treatments,
            output_component_ids: state
                .state_components
                .iter()
                .map(|component| component.component_id.clone())
                .collect(),
            unresolved_component_ids: unresolved,
            uncertainty_representation: state.uncertainty_representation.clone(),
            outcome: status,
            algorithm_identity: self.implementation_id.clone(),
            canonical_hash: content_hash(&state.canonical_bytes()),
        };
        Ok((state, report))
    }

    fn validate_input(
        &self,
        handoff: &ObservationHandoff,
        config: &EstimatorConfiguration,
    ) -> Result<(), EstimationFailure> {
        if !handoff.batch.verify_integrity()
            || handoff.batch.batch_hash != handoff.batch_hash
            || handoff
                .batch
                .observations
                .iter()
                .any(|record| record.window_id != handoff.batch.window_id)
        {
            return Err(EstimationFailure::InvalidObservationHandoff);
        }
        if config.profile.prior_state_policy == PriorStatePolicy::Forbidden
            && config.prior_estimated_state_id.is_some()
        {
            return Err(EstimationFailure::PriorStateUnavailable);
        }
        if config
            .registry_snapshot
            .resolve(&config.estimator.estimator_id)
            .is_none()
        {
            return Err(EstimationFailure::EstimatorDefinitionUnavailable);
        }
        if config
            .registry_snapshot
            .resolve(&config.estimator.estimator_id)
            != Some(&config.estimator)
        {
            return Err(EstimationFailure::EstimatorVersionMismatch);
        }
        if !config
            .estimator
            .numeric_profile_ids
            .contains(&config.numeric_profile_id)
            || config.profile.numeric_profile_id != config.numeric_profile_id
        {
            return Err(EstimationFailure::NumericProfileMismatch);
        }
        if !config
            .profile
            .accepted_observation_batch_versions
            .contains(&Version::new("1.0.0").expect("static"))
        {
            return Err(EstimationFailure::SchemaVersionMismatch);
        }
        if matches!(handoff.batch.batch_status, BatchStatus::Faulted) {
            return Err(EstimationFailure::InvalidObservationHandoff);
        }
        Ok(())
    }
}

fn configuration_id(config: &EstimatorConfiguration) -> String {
    let mut d = CanonicalDocument::new();
    d.insert("estimator", &config.estimator.estimator_id);
    d.insert(
        "estimator_version",
        config.estimator.estimator_version.to_string(),
    );
    d.insert("profile", &config.profile.estimator_profile_id);
    d.insert("basis", &config.basis.state_basis_profile_id);
    d.insert("numeric", &config.numeric_profile_id);
    canonical_hash(&d)
}
fn document_for(ids: &[String]) -> CanonicalDocument {
    let mut d = CanonicalDocument::new();
    d.insert("observation_ids", ids.join(","));
    d
}
fn distinct_values(records: &[&ObservationRecord]) -> bool {
    let mut values = BTreeSet::new();
    for record in records {
        if let Some(value) = numeric_value(record) {
            values.insert(value);
        }
    }
    values.len() > 1
}
fn numeric_value(record: &ObservationRecord) -> Option<i128> {
    match &record.value {
        sacs_observation::ObservationValue::Present(value) => value.parse().ok(),
        sacs_observation::ObservationValue::Absent => None,
    }
}
fn uncertainty_for(policy: &UncertaintyPolicy) -> UncertaintyRepresentation {
    match policy {
        UncertaintyPolicy::Unknown => UncertaintyRepresentation::Unknown,
        UncertaintyPolicy::QualitativeClass => UncertaintyRepresentation::QualitativeClass,
        UncertaintyPolicy::PreserveSource => UncertaintyRepresentation::Unknown,
    }
}
fn treatment_for(
    record: &ObservationRecord,
    profile: &EstimatorProfile,
) -> Result<ObservationTreatment, EstimationFailure> {
    let required = profile
        .required_observation_types
        .contains(&record.observation_type_id);
    let (status, reason) = match record.availability_state {
        Availability::Available => (TreatmentStatus::Accepted, "available"),
        Availability::Stale => {
            if matches!(profile.stale_data_policy, StaleDataPolicy::Reject) {
                return Err(EstimationFailure::DisallowedStaleObservation(
                    record.observation_id.clone(),
                ));
            }
            (TreatmentStatus::Stale, "stale accepted with preserved age")
        }
        Availability::Unavailable => (TreatmentStatus::Unavailable, "unavailable"),
        Availability::Invalid => (TreatmentStatus::Invalid, "invalid"),
        Availability::Unsupported => (TreatmentStatus::Unsupported, "unsupported"),
        Availability::Redacted => (TreatmentStatus::Redacted, "redacted"),
        Availability::Pending => (TreatmentStatus::Pending, "pending"),
        Availability::NotApplicable => (TreatmentStatus::Excluded, "not applicable"),
    };
    if required && !matches!(status, TreatmentStatus::Accepted | TreatmentStatus::Stale) {
        return Err(EstimationFailure::MissingRequiredObservation(
            record.observation_type_id.clone(),
        ));
    }
    Ok(ObservationTreatment {
        observation_id: Some(record.observation_id.clone()),
        observation_type_id: record.observation_type_id.clone(),
        status,
        reason: reason.into(),
        rule_id: format!("required={required}"),
    })
}

impl StateEstimationStore {
    pub fn commit(
        &mut self,
        state: EstimatedState,
        report: EstimatorReport,
        versions: VersionBinding,
        producer: impl Into<String>,
    ) -> Result<EstimatedStateHandoff, EstimationFailure> {
        let identity = state.identity(versions, producer);
        let bytes = state.canonical_bytes();
        let result = self
            .commitments
            .commit(&identity, &bytes)
            .map_err(EstimationFailure::Commitment)?;
        match result {
            CommitResult::Committed(_) | CommitResult::Idempotent(_) => {
                self.states
                    .entry(identity.identifier.clone())
                    .or_insert_with(|| state.clone());
                Ok(EstimatedStateHandoff {
                    state: state.clone(),
                    report,
                    provenance: state.provenance.clone(),
                    commitment_identity: identity.identifier,
                    state_hash: content_hash(&state.canonical_bytes()),
                })
            }
        }
    }
    pub fn get(&self, identity: &str) -> Option<&EstimatedState> {
        self.states.get(identity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sacs_observation::{
        ExpectedObservationManifest, LateDataPolicy, ObservationClass, ObservationValue,
        ObservationWindow, ObservationWindowConfig, SourceDeclaration,
    };

    fn v() -> Version {
        Version::new("1.0.0").unwrap()
    }
    fn versions() -> VersionBinding {
        let x = v();
        VersionBinding {
            baseline: x.clone(),
            contract: x.clone(),
            schema: x.clone(),
            registry: x.clone(),
            implementation: x,
        }
    }
    fn source() -> SourceDeclaration {
        SourceDeclaration {
            source_id: "source".into(),
            source_type: "sensor".into(),
            source_version: v(),
            producer_implementation_id: "imp".into(),
            producer_implementation_version: v(),
            capability_manifest_id: "cap".into(),
            clock_domain_id: "clock".into(),
            trust_domain_id: "trust".into(),
            registry_version: v(),
            supported_observation_types: ["temperature".into()].into_iter().collect(),
            supported_observation_classes: [ObservationClass::Scalar].into_iter().collect(),
            supported_schema_versions: [v()].into_iter().collect(),
            available: true,
        }
    }
    fn ts(n: i128) -> Timestamp {
        Timestamp::Known {
            nanos: n,
            clock_domain_id: "clock".into(),
        }
    }
    fn handoff(value: &str, availability: Availability) -> ObservationHandoff {
        let mut w = ObservationWindow::open(ObservationWindowConfig {
            window_id: "w".into(),
            cycle_id: "c".into(),
            open_time: ts(0),
            scheduled_close_time: ts(100),
            observation_cutoff: ts(90),
            window_policy_id: "p".into(),
            clock_domain_id: "clock".into(),
            late_data_policy: LateDataPolicy::ReopenProhibited,
            expected_manifest: ExpectedObservationManifest {
                manifest_id: "m".into(),
                required_observation_types: BTreeSet::new(),
                optional_observation_types: BTreeSet::new(),
                conditional_observation_types: BTreeSet::new(),
                source_constraints: BTreeSet::new(),
                freshness_constraints: BTreeMap::new(),
                minimum_quality_constraints: BTreeMap::new(),
                minimum_confidence_constraints: BTreeMap::new(),
                window_policy_id: "p".into(),
                registry_versions: [v()].into_iter().collect(),
            },
        })
        .unwrap();
        w.register_source(source()).unwrap();
        let r = sacs_observation::ObservationRecord {
            observation_id: "o".into(),
            observation_type_id: "temperature".into(),
            observation_class: ObservationClass::Scalar,
            observation_form: "raw".into(),
            source_id: "source".into(),
            source_sequence: 1,
            source_event_time: ts(1),
            receipt_time: ts(1),
            window_id: "w".into(),
            value: if matches!(availability, Availability::Available) {
                ObservationValue::Present(value.into())
            } else {
                ObservationValue::Absent
            },
            unit: None,
            availability_state: availability,
            quality_state: Quality::Nominal,
            confidence: Some("source-confidence".into()),
            uncertainty: Some("source-uncertainty".into()),
            age_nanos: None,
            validity_interval: None,
            schema_version: v(),
            registry_version: v(),
            provenance_id: "prov".into(),
            normalization: None,
        };
        w.admit(r).unwrap();
        let b = w
            .close(ts(80))
            .unwrap()
            .build_batch("b", "bp", BatchStatus::Complete)
            .unwrap();
        let mut store = sacs_observation::ObservationStore::default();
        store
            .commit_for_handoff(b, versions(), "observation")
            .unwrap()
    }
    fn config() -> EstimatorConfiguration {
        let definition = EstimatorDefinition {
            estimator_id: "estimator".into(),
            estimator_version: v(),
            algorithm_class: "exact-rational-mean".into(),
            algorithm_version: v(),
            numeric_profile_ids: ["exact-integer".into()].into_iter().collect(),
            determinism_claim: "deterministic".into(),
            stateful_or_stateless: "stateless".into(),
            implementation_hash: "reference".into(),
        };
        let profile = EstimatorProfile {
            estimator_profile_id: "profile".into(),
            estimator_profile_version: v(),
            accepted_observation_batch_versions: [v()].into_iter().collect(),
            required_observation_types: ["temperature".into()].into_iter().collect(),
            optional_observation_types: BTreeSet::new(),
            state_basis_profile_id: "basis".into(),
            prior_state_policy: PriorStatePolicy::Forbidden,
            missing_data_policy: MissingDataPolicy::FailRequired,
            stale_data_policy: StaleDataPolicy::Reject,
            conflict_policy: ConflictPolicy::Fail,
            uncertainty_policy: UncertaintyPolicy::PreserveSource,
            numeric_profile_id: "exact-integer".into(),
            initialization_policy: "stateless".into(),
            reset_policy: "not-applicable".into(),
            failure_policy: "explicit".into(),
            output_schema_version: v(),
        };
        let component = StateComponentDefinition {
            component_id: "temperature-state".into(),
            observation_type_ids: ["temperature".into()].into_iter().collect(),
            semantics: "reference numeric component".into(),
            component_type: "integer-rational".into(),
            unit: Some("C".into()),
            schema_version: v(),
            registry_version: v(),
        };
        let basis = StateBasisProfile {
            state_basis_profile_id: "basis".into(),
            state_basis_profile_version: v(),
            components: [("temperature-state".into(), component)]
                .into_iter()
                .collect(),
            uncertainty_model: "unknown-preserved".into(),
            covariance_ordering: vec!["temperature-state".into()],
            initialization_rules: "no prior state".into(),
            update_rules: "exact rational mean".into(),
            compatibility_rules: "explicit".into(),
            schema_version: v(),
            registry_version: v(),
        };
        let mut entries = BTreeMap::new();
        entries.insert("estimator".into(), definition.clone());
        EstimatorConfiguration {
            estimator: definition,
            profile,
            basis,
            registry_snapshot: EstimatorRegistrySnapshot {
                registry_version: v(),
                entries,
            },
            observation_registry_versions: [v()].into_iter().collect(),
            observation_schema_versions: [v()].into_iter().collect(),
            numeric_profile_id: "exact-integer".into(),
            prior_estimated_state_id: None,
        }
    }
    #[test]
    fn committed_handoff_produces_deterministic_estimated_state() {
        let h = handoff("4", Availability::Available);
        let (a, _) = ReferenceEstimator::default()
            .estimate(&h, &config())
            .unwrap();
        let (b, _) = ReferenceEstimator::default()
            .estimate(&h, &config())
            .unwrap();
        assert_eq!(a.canonical_bytes(), b.canonical_bytes());
        assert_ne!(a.estimated_state_id, "");
        assert_eq!(a.state_components[0].value.as_deref(), Some("4"));
    }
    #[test]
    fn missing_required_input_fails_explicitly() {
        let h = handoff("", Availability::Unavailable);
        assert!(matches!(
            ReferenceEstimator::default().estimate(&h, &config()),
            Err(EstimationFailure::MissingRequiredObservation(_))
        ));
    }
    #[test]
    fn source_confidence_and_uncertainty_remain_distinct() {
        let h = handoff("4", Availability::Available);
        let (state, _) = ReferenceEstimator::default()
            .estimate(&h, &config())
            .unwrap();
        let c = &state.state_components[0];
        assert_eq!(
            c.source_confidences[0].as_deref(),
            Some("source-confidence")
        );
        assert_eq!(
            c.source_uncertainties[0].as_deref(),
            Some("source-uncertainty")
        );
        assert!(c.estimator_confidence.is_none());
    }
    #[test]
    fn stale_input_is_rejected_by_profile() {
        let h = handoff("4", Availability::Stale);
        assert!(matches!(
            ReferenceEstimator::default().estimate(&h, &config()),
            Err(EstimationFailure::DisallowedStaleObservation(_))
        ));
    }
    #[test]
    fn conflict_is_not_silently_resolved() {
        let mut h = handoff("4", Availability::Available);
        let mut conflict = h.batch.observations[0].clone();
        conflict.observation_id = "o-2".into();
        conflict.source_sequence = 2;
        conflict.value = ObservationValue::Present("9".into());
        h.batch.observations.push(conflict);
        h.batch.ordered_observation_ids.push("o-2".into());
        h.batch.batch_hash = h.batch.recompute_hash();
        h.batch_hash = h.batch.batch_hash.clone();
        assert!(matches!(
            ReferenceEstimator::default().estimate(&h, &config()),
            Err(EstimationFailure::UnresolvedObservationConflict(_))
        ));
    }

    #[test]
    fn modified_handoff_payload_fails_integrity_validation() {
        let mut h = handoff("4", Availability::Available);
        h.batch.observations[0].value = ObservationValue::Present("5".into());
        assert!(matches!(
            ReferenceEstimator::default().estimate(&h, &config()),
            Err(EstimationFailure::InvalidObservationHandoff)
        ));
    }
    #[test]
    fn state_is_not_osv() {
        let h = handoff("4", Availability::Available);
        let (state, _) = ReferenceEstimator::default()
            .estimate(&h, &config())
            .unwrap();
        assert!(!state.canonical_bytes().windows(3).any(|w| w == b"OSV"));
    }

    #[test]
    fn state_commit_retry_is_idempotent() {
        let h = handoff("4", Availability::Available);
        let (state, report) = ReferenceEstimator::default()
            .estimate(&h, &config())
            .unwrap();
        let mut store = StateEstimationStore::default();
        let first = store
            .commit(
                state.clone(),
                report.clone(),
                versions(),
                "sacs-state-estimation",
            )
            .unwrap();
        let second = store
            .commit(state, report, versions(), "sacs-state-estimation")
            .unwrap();
        assert_eq!(first.commitment_identity, second.commitment_identity);
        assert_eq!(first.state_hash, second.state_hash);
    }
}
