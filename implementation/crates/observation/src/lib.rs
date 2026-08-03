//! Contract 001 observation admission.
//!
//! This crate owns source admission, observation-window policy, evidence
//! validation, deterministic ordering, batch commitment, and the immutable
//! handoff shape. It does not infer state or assign supervisory meaning.

use std::collections::{BTreeMap, BTreeSet};

use canonical_hashing::canonical_hash;
use canonical_identity::{ArtifactIdentity, IdentityBinding};
use canonical_serialization::CanonicalDocument;
use canonical_versioning::{Version, VersionBinding};
use commitment_engine::{CommitResult, CommitmentError, CommitmentStore};
use deterministic_ordering::stable_sort_by_key;
use failure_model::FailureRecord;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDeclaration {
    pub source_id: String,
    pub source_type: String,
    pub source_version: Version,
    pub producer_implementation_id: String,
    pub producer_implementation_version: Version,
    pub capability_manifest_id: String,
    pub clock_domain_id: String,
    pub trust_domain_id: String,
    pub registry_version: Version,
    pub supported_observation_types: BTreeSet<String>,
    pub supported_observation_classes: BTreeSet<ObservationClass>,
    pub supported_schema_versions: BTreeSet<Version>,
    pub available: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObservationClass {
    Scalar,
    Categorical,
    Boolean,
    Vector,
    Event,
    Aggregate,
    Structured,
    Availability,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservationValue {
    Present(String),
    Absent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Availability {
    Available,
    Unavailable,
    Invalid,
    Stale,
    Unsupported,
    NotApplicable,
    Redacted,
    Pending,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Quality {
    Nominal,
    Degraded,
    Suspect,
    Failed,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Timestamp {
    Known {
        nanos: i128,
        clock_domain_id: String,
    },
    Unknown {
        reason: String,
        clock_domain_id: Option<String>,
    },
}

impl Timestamp {
    fn sort_key(&self) -> (u8, i128, String) {
        match self {
            Self::Known { nanos, .. } => (0, *nanos, String::new()),
            Self::Unknown { reason, .. } => (1, 0, reason.clone()),
        }
    }
    fn clock_domain(&self) -> Option<&str> {
        match self {
            Self::Known {
                clock_domain_id, ..
            } => Some(clock_domain_id),
            Self::Unknown {
                clock_domain_id, ..
            } => clock_domain_id.as_deref(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidityInterval {
    pub start: Timestamp,
    pub end: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizationRecord {
    pub adapter_id: String,
    pub adapter_version: Version,
    pub input_schema_version: Version,
    pub output_schema_version: Version,
    pub normalization_rule_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedObservationManifest {
    pub manifest_id: String,
    pub required_observation_types: BTreeSet<String>,
    pub optional_observation_types: BTreeSet<String>,
    pub conditional_observation_types: BTreeSet<String>,
    pub source_constraints: BTreeSet<String>,
    pub freshness_constraints: BTreeMap<String, i128>,
    pub minimum_quality_constraints: BTreeMap<String, Quality>,
    pub minimum_confidence_constraints: BTreeMap<String, String>,
    pub window_policy_id: String,
    pub registry_versions: BTreeSet<Version>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterDeclaration {
    pub adapter_id: String,
    pub adapter_version: Version,
    pub supported_source_types: BTreeSet<String>,
    pub supported_input_schemas: BTreeSet<Version>,
    pub supported_output_observation_types: BTreeSet<String>,
    pub normalization_rules: BTreeSet<String>,
    pub error_mapping: String,
    pub availability_mapping: String,
    pub clock_mapping: String,
    pub compatibility_profile: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationProvenance {
    pub provenance_id: String,
    pub observation_id: String,
    pub source_id: String,
    pub capability_manifest_id: String,
    pub adapter_id: Option<String>,
    pub adapter_version: Option<Version>,
    pub window_id: String,
    pub collection_path: String,
    pub validation_result: String,
    pub normalization_result: Option<NormalizationRecord>,
    pub registry_version: Version,
    pub schema_version: Version,
    pub parent_observation_ids: Vec<String>,
    pub created_at: Timestamp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationRecord {
    pub observation_id: String,
    pub observation_type_id: String,
    pub observation_class: ObservationClass,
    pub observation_form: String,
    pub source_id: String,
    pub source_sequence: u64,
    pub source_event_time: Timestamp,
    pub receipt_time: Timestamp,
    pub window_id: String,
    pub value: ObservationValue,
    pub unit: Option<String>,
    pub availability_state: Availability,
    pub quality_state: Quality,
    pub confidence: Option<String>,
    pub uncertainty: Option<String>,
    pub age_nanos: Option<i128>,
    pub validity_interval: Option<ValidityInterval>,
    pub schema_version: Version,
    pub registry_version: Version,
    pub provenance_id: String,
    pub normalization: Option<NormalizationRecord>,
}

impl ObservationRecord {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert(
            "availability_state",
            format!("{:?}", self.availability_state),
        );
        d.insert("observation_class", format!("{:?}", self.observation_class));
        d.insert("observation_form", &self.observation_form);
        d.insert("observation_id", &self.observation_id);
        d.insert("observation_type_id", &self.observation_type_id);
        d.insert("provenance_id", &self.provenance_id);
        d.insert("receipt_time", format!("{:?}", self.receipt_time));
        d.insert("schema_version", self.schema_version.to_string());
        d.insert("source_event_time", format!("{:?}", self.source_event_time));
        d.insert("source_id", &self.source_id);
        d.insert("source_sequence", self.source_sequence.to_string());
        d.insert(
            "confidence",
            self.confidence.as_deref().unwrap_or("<absent>"),
        );
        d.insert(
            "uncertainty",
            self.uncertainty.as_deref().unwrap_or("<absent>"),
        );
        d.insert(
            "age_nanos",
            self.age_nanos
                .map(|age| age.to_string())
                .unwrap_or_else(|| "<absent>".into()),
        );
        d.insert("validity_interval", format!("{:?}", self.validity_interval));
        d.insert("normalization", format!("{:?}", self.normalization));
        d.insert("unit", self.unit.as_deref().unwrap_or("<absent>"));
        d.insert(
            "value",
            match &self.value {
                ObservationValue::Present(value) => value,
                ObservationValue::Absent => "<absent>",
            },
        );
        d.insert("window_id", &self.window_id);
        d.encode()
    }

    pub fn identity(
        &self,
        versions: VersionBinding,
        cycle_id: impl Into<String>,
        producer: impl Into<String>,
    ) -> ArtifactIdentity {
        ArtifactIdentity::deterministic(
            versions,
            IdentityBinding {
                schema: self.schema_version.to_string(),
                registry: self.registry_version.to_string(),
                cycle: Some(cycle_id.into()),
                producer: producer.into(),
            },
            &self.canonical_bytes(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowStatus {
    Open,
    Closed,
    Faulted,
    Incomplete,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LateDataPolicy {
    DeferToNextWindow,
    RecordAsLateOnly,
    ReopenProhibited,
    FaultWindow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationWindowConfig {
    pub window_id: String,
    pub cycle_id: String,
    pub open_time: Timestamp,
    pub scheduled_close_time: Timestamp,
    pub observation_cutoff: Timestamp,
    pub window_policy_id: String,
    pub clock_domain_id: String,
    pub late_data_policy: LateDataPolicy,
    pub expected_manifest: ExpectedObservationManifest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationWindow {
    pub window_id: String,
    pub cycle_id: String,
    pub open_time: Timestamp,
    pub scheduled_close_time: Timestamp,
    pub actual_close_time: Option<Timestamp>,
    pub observation_cutoff: Timestamp,
    pub window_policy_id: String,
    pub clock_domain_id: String,
    pub status: WindowStatus,
    pub late_data_policy: LateDataPolicy,
    pub expected_manifest: ExpectedObservationManifest,
    records: BTreeMap<String, ObservationRecord>,
    sources: BTreeMap<String, SourceDeclaration>,
}

impl ObservationWindow {
    pub fn open(config: ObservationWindowConfig) -> Result<Self, ObservationError> {
        let window = Self {
            window_id: config.window_id,
            cycle_id: config.cycle_id,
            open_time: config.open_time,
            scheduled_close_time: config.scheduled_close_time,
            actual_close_time: None,
            observation_cutoff: config.observation_cutoff,
            window_policy_id: config.window_policy_id,
            clock_domain_id: config.clock_domain_id,
            status: WindowStatus::Open,
            late_data_policy: config.late_data_policy,
            expected_manifest: config.expected_manifest,
            records: BTreeMap::new(),
            sources: BTreeMap::new(),
        };
        window.validate_times()?;
        Ok(window)
    }

    pub fn register_source(&mut self, source: SourceDeclaration) -> Result<(), ObservationError> {
        if self.status != WindowStatus::Open {
            return Err(ObservationError::WindowClosed);
        }
        if source.clock_domain_id != self.clock_domain_id {
            return Err(ObservationError::ClockDomainMismatch);
        }
        if self
            .sources
            .insert(source.source_id.clone(), source)
            .is_some()
        {
            return Err(ObservationError::DuplicateSource);
        }
        Ok(())
    }

    pub fn admit(&mut self, record: ObservationRecord) -> Result<(), ObservationError> {
        if self.status != WindowStatus::Open {
            return Err(ObservationError::WindowClosed);
        }
        let source = self
            .sources
            .get(&record.source_id)
            .ok_or_else(|| ObservationError::UnknownSource(record.source_id.clone()))?;
        if record.window_id != self.window_id {
            return Err(ObservationError::WrongWindow);
        }
        if !source
            .supported_observation_types
            .contains(&record.observation_type_id)
        {
            return Err(ObservationError::UnsupportedObservationType);
        }
        if !source
            .supported_observation_classes
            .contains(&record.observation_class)
        {
            return Err(ObservationError::UnsupportedObservationClass);
        }
        if !source
            .supported_schema_versions
            .contains(&record.schema_version)
        {
            return Err(ObservationError::VersionMismatch);
        }
        if record.registry_version != source.registry_version {
            return Err(ObservationError::VersionMismatch);
        }
        if !source.available && matches!(record.availability_state, Availability::Available) {
            return Err(ObservationError::SourceUnavailable);
        }
        validate_timestamp(&record.source_event_time, &self.clock_domain_id)?;
        validate_timestamp(&record.receipt_time, &self.clock_domain_id)?;
        if record.receipt_time.sort_key() > self.observation_cutoff.sort_key() {
            return match self.late_data_policy {
                LateDataPolicy::FaultWindow => {
                    self.status = WindowStatus::Faulted;
                    Err(ObservationError::WindowFaulted)
                }
                LateDataPolicy::ReopenProhibited => Err(ObservationError::OutsideCutoff),
                policy => Err(ObservationError::LateDataRejected(policy)),
            };
        }
        if matches!(record.availability_state, Availability::Available)
            && matches!(record.value, ObservationValue::Absent)
        {
            return Err(ObservationError::MissingAvailableValue);
        }
        if let Some(existing) = self.records.get(&record.observation_id) {
            if existing.canonical_bytes() == record.canonical_bytes() {
                return Ok(());
            }
            return Err(ObservationError::ConflictingObservation(
                record.observation_id,
            ));
        }
        if self.records.values().any(|existing| {
            existing.source_id == record.source_id
                && existing.source_sequence == record.source_sequence
                && existing.observation_id != record.observation_id
        }) {
            return Err(ObservationError::SequenceConflict(record.source_sequence));
        }
        self.records.insert(record.observation_id.clone(), record);
        Ok(())
    }

    pub fn close(
        self,
        actual_close_time: Timestamp,
    ) -> Result<ClosedObservationWindow, ObservationError> {
        self.close_as(actual_close_time, WindowStatus::Closed)
    }

    pub fn close_as(
        mut self,
        actual_close_time: Timestamp,
        status: WindowStatus,
    ) -> Result<ClosedObservationWindow, ObservationError> {
        if self.status != WindowStatus::Open {
            return Err(ObservationError::WindowClosed);
        }
        if !matches!(
            status,
            WindowStatus::Closed | WindowStatus::Faulted | WindowStatus::Incomplete
        ) {
            return Err(ObservationError::InvalidFinalStatus);
        }
        validate_timestamp(&actual_close_time, &self.clock_domain_id)?;
        self.actual_close_time = Some(actual_close_time);
        self.status = status;
        Ok(ClosedObservationWindow(self))
    }

    fn validate_times(&self) -> Result<(), ObservationError> {
        validate_timestamp(&self.open_time, &self.clock_domain_id)?;
        validate_timestamp(&self.scheduled_close_time, &self.clock_domain_id)?;
        validate_timestamp(&self.observation_cutoff, &self.clock_domain_id)
    }
}

fn validate_timestamp(
    timestamp: &Timestamp,
    expected_domain: &str,
) -> Result<(), ObservationError> {
    if let Some(domain) = timestamp.clock_domain() {
        if domain != expected_domain {
            return Err(ObservationError::ClockDomainMismatch);
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClosedObservationWindow(ObservationWindow);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BatchStatus {
    Complete,
    Degraded,
    Incomplete,
    Faulted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationBatch {
    pub observation_batch_id: String,
    pub window_id: String,
    pub cycle_id: String,
    pub observation_cutoff: Timestamp,
    pub ordered_observation_ids: Vec<String>,
    pub observations: Vec<ObservationRecord>,
    pub expected_observation_manifest_id: String,
    pub missing_observation_summary: String,
    pub invalid_observation_summary: String,
    pub late_observation_summary: String,
    pub registry_versions: BTreeSet<Version>,
    pub schema_versions: BTreeSet<Version>,
    pub batch_hash: String,
    pub batch_status: BatchStatus,
    pub provenance_id: String,
    pub provenance: Vec<ObservationProvenance>,
}

impl ClosedObservationWindow {
    pub fn build_batch(
        self,
        batch_id: impl Into<String>,
        provenance_id: impl Into<String>,
        batch_status: BatchStatus,
    ) -> Result<ObservationBatch, ObservationError> {
        let window = self.0;
        let mut observations: Vec<_> = window.records.into_values().collect();
        stable_sort_by_key(&mut observations, |record| {
            (
                record.source_event_time.sort_key(),
                record.receipt_time.sort_key(),
                record.source_id.clone(),
                record.source_sequence,
                record.observation_type_id.clone(),
                record.observation_id.clone(),
            )
        });
        let ids = observations
            .iter()
            .map(|record| record.observation_id.clone())
            .collect::<Vec<_>>();
        let observed_types: BTreeSet<_> = observations
            .iter()
            .map(|record| record.observation_type_id.clone())
            .collect();
        let missing: Vec<_> = window
            .expected_manifest
            .required_observation_types
            .difference(&observed_types)
            .cloned()
            .collect();
        if !missing.is_empty() && matches!(batch_status, BatchStatus::Complete) {
            return Err(ObservationError::MissingRequiredObservation(missing));
        }
        let mut d = CanonicalDocument::new();
        d.insert("cycle_id", &window.cycle_id);
        d.insert(
            "observation_cutoff",
            format!("{:?}", window.observation_cutoff),
        );
        d.insert("observation_ids", ids.join(","));
        d.insert(
            "observation_hashes",
            observations
                .iter()
                .map(|record| {
                    canonical_hash(
                        &CanonicalDocument::decode(&record.canonical_bytes())
                            .expect("record encoding"),
                    )
                    .to_string()
                })
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert("window_id", &window.window_id);
        let batch_hash = canonical_hash(&d);
        let registry_versions = observations
            .iter()
            .map(|record| record.registry_version.clone())
            .collect();
        let schema_versions = observations
            .iter()
            .map(|record| record.schema_version.clone())
            .collect();
        let batch_status = if missing.is_empty() {
            batch_status
        } else {
            BatchStatus::Incomplete
        };
        let created_at = window
            .actual_close_time
            .clone()
            .ok_or(ObservationError::MissingCloseTime)?;
        let provenance = observations
            .iter()
            .map(|record| {
                let source = window
                    .sources
                    .get(&record.source_id)
                    .ok_or_else(|| ObservationError::UnknownSource(record.source_id.clone()))?;
                Ok(ObservationProvenance {
                    provenance_id: record.provenance_id.clone(),
                    observation_id: record.observation_id.clone(),
                    source_id: record.source_id.clone(),
                    capability_manifest_id: source.capability_manifest_id.clone(),
                    adapter_id: record
                        .normalization
                        .as_ref()
                        .map(|normalization| normalization.adapter_id.clone()),
                    adapter_version: record
                        .normalization
                        .as_ref()
                        .map(|normalization| normalization.adapter_version.clone()),
                    window_id: record.window_id.clone(),
                    collection_path: format!("window/{}", record.window_id),
                    validation_result: "admitted".into(),
                    normalization_result: record.normalization.clone(),
                    registry_version: record.registry_version.clone(),
                    schema_version: record.schema_version.clone(),
                    parent_observation_ids: Vec::new(),
                    created_at: created_at.clone(),
                })
            })
            .collect::<Result<Vec<_>, ObservationError>>()?;
        Ok(ObservationBatch {
            observation_batch_id: batch_id.into(),
            window_id: window.window_id,
            cycle_id: window.cycle_id,
            observation_cutoff: window.observation_cutoff,
            ordered_observation_ids: ids,
            observations,
            expected_observation_manifest_id: window.expected_manifest.manifest_id,
            missing_observation_summary: if missing.is_empty() {
                "<none>".into()
            } else {
                missing.join(",")
            },
            invalid_observation_summary: "<none-recorded>".into(),
            late_observation_summary: "<none-recorded>".into(),
            registry_versions,
            schema_versions,
            batch_hash,
            batch_status,
            provenance_id: provenance_id.into(),
            provenance,
        })
    }
}

impl ObservationBatch {
    pub fn recompute_hash(&self) -> String {
        let mut d = CanonicalDocument::new();
        d.insert("cycle_id", &self.cycle_id);
        d.insert(
            "observation_cutoff",
            format!("{:?}", self.observation_cutoff),
        );
        d.insert("observation_ids", self.ordered_observation_ids.join(","));
        d.insert(
            "observation_hashes",
            self.observations
                .iter()
                .map(|record| {
                    canonical_hash(
                        &CanonicalDocument::decode(&record.canonical_bytes())
                            .expect("record encoding"),
                    )
                })
                .collect::<Vec<_>>()
                .join(","),
        );
        d.insert("window_id", &self.window_id);
        canonical_hash(&d)
    }

    pub fn verify_integrity(&self) -> bool {
        self.recompute_hash() == self.batch_hash
    }

    pub fn identity(
        &self,
        versions: VersionBinding,
        producer: impl Into<String>,
    ) -> ArtifactIdentity {
        let mut d = CanonicalDocument::new();
        d.insert("batch_hash", &self.batch_hash);
        d.insert("batch_id", &self.observation_batch_id);
        d.insert("cycle_id", &self.cycle_id);
        d.insert("window_id", &self.window_id);
        ArtifactIdentity::deterministic(
            versions,
            IdentityBinding {
                schema: "observation-batch".into(),
                registry: self
                    .registry_versions
                    .iter()
                    .next()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "<absent>".into()),
                cycle: Some(self.cycle_id.clone()),
                producer: producer.into(),
            },
            &d.encode(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationHandoff {
    pub batch: ObservationBatch,
    pub commitment_identity: String,
    pub batch_hash: String,
}

#[derive(Default)]
pub struct ObservationStore {
    commitments: CommitmentStore,
    batches: BTreeMap<String, ObservationBatch>,
}

impl ObservationStore {
    pub fn commit_for_handoff(
        &mut self,
        batch: ObservationBatch,
        versions: VersionBinding,
        producer: impl Into<String>,
    ) -> Result<ObservationHandoff, ObservationError> {
        let identity = batch.identity(versions, producer);
        let bytes = batch.canonical_bytes();
        let batch_hash = batch.batch_hash.clone();
        let result = self
            .commitments
            .commit(&identity, &bytes)
            .map_err(ObservationError::Commitment)?;
        match result {
            CommitResult::Committed(_) | CommitResult::Idempotent(_) => {
                self.batches
                    .entry(identity.identifier.clone())
                    .or_insert_with(|| batch.clone());
                Ok(ObservationHandoff {
                    batch,
                    commitment_identity: identity.identifier,
                    batch_hash,
                })
            }
        }
    }
    pub fn get(&self, identity: &str) -> Option<&ObservationBatch> {
        self.batches.get(identity)
    }
}

impl ObservationBatch {
    fn canonical_bytes(&self) -> Vec<u8> {
        let mut d = CanonicalDocument::new();
        d.insert("batch_hash", &self.batch_hash);
        d.insert("batch_id", &self.observation_batch_id);
        d.insert("ordered_ids", self.ordered_observation_ids.join(","));
        d.insert("status", format!("{:?}", self.batch_status));
        d.encode()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservationError {
    UnknownSource(String),
    DuplicateSource,
    UnsupportedObservationType,
    UnsupportedObservationClass,
    VersionMismatch,
    ClockDomainMismatch,
    SourceUnavailable,
    WindowClosed,
    WindowFaulted,
    InvalidFinalStatus,
    WrongWindow,
    OutsideCutoff,
    LateDataRejected(LateDataPolicy),
    SequenceConflict(u64),
    MissingRequiredObservation(Vec<String>),
    MissingCloseTime,
    MissingAvailableValue,
    ConflictingObservation(String),
    Commitment(CommitmentError),
}

impl ObservationError {
    pub fn failure_record(&self) -> FailureRecord {
        FailureRecord::new(
            format!("observation::{self:?}"),
            "observation admission failed",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version() -> Version {
        Version::new("1.0.0").unwrap()
    }
    fn source() -> SourceDeclaration {
        SourceDeclaration {
            source_id: "source-a".into(),
            source_type: "sensor".into(),
            source_version: version(),
            producer_implementation_id: "imp".into(),
            producer_implementation_version: version(),
            capability_manifest_id: "cap".into(),
            clock_domain_id: "clock".into(),
            trust_domain_id: "trust".into(),
            registry_version: version(),
            supported_observation_types: ["temperature".into()].into_iter().collect(),
            supported_observation_classes: [ObservationClass::Scalar].into_iter().collect(),
            supported_schema_versions: [version()].into_iter().collect(),
            available: true,
        }
    }
    fn timestamp(nanos: i128) -> Timestamp {
        Timestamp::Known {
            nanos,
            clock_domain_id: "clock".into(),
        }
    }
    fn manifest() -> ExpectedObservationManifest {
        ExpectedObservationManifest {
            manifest_id: "manifest-1".into(),
            required_observation_types: BTreeSet::new(),
            optional_observation_types: BTreeSet::new(),
            conditional_observation_types: BTreeSet::new(),
            source_constraints: BTreeSet::new(),
            freshness_constraints: BTreeMap::new(),
            minimum_quality_constraints: BTreeMap::new(),
            minimum_confidence_constraints: BTreeMap::new(),
            window_policy_id: "policy-1".into(),
            registry_versions: [version()].into_iter().collect(),
        }
    }
    fn window() -> ObservationWindow {
        ObservationWindow::open(ObservationWindowConfig {
            window_id: "window-1".into(),
            cycle_id: "cycle-1".into(),
            open_time: timestamp(0),
            scheduled_close_time: timestamp(100),
            observation_cutoff: timestamp(90),
            window_policy_id: "policy-1".into(),
            clock_domain_id: "clock".into(),
            late_data_policy: LateDataPolicy::ReopenProhibited,
            expected_manifest: manifest(),
        })
        .unwrap()
    }
    fn record(id: &str, sequence: u64, value: ObservationValue) -> ObservationRecord {
        ObservationRecord {
            observation_id: id.into(),
            observation_type_id: "temperature".into(),
            observation_class: ObservationClass::Scalar,
            observation_form: "raw".into(),
            source_id: "source-a".into(),
            source_sequence: sequence,
            source_event_time: timestamp(sequence as i128),
            receipt_time: timestamp(sequence as i128),
            window_id: "window-1".into(),
            value,
            unit: Some("C".into()),
            availability_state: Availability::Available,
            quality_state: Quality::Nominal,
            confidence: None,
            uncertainty: None,
            age_nanos: None,
            validity_interval: None,
            schema_version: version(),
            registry_version: version(),
            provenance_id: format!("prov-{id}"),
            normalization: None,
        }
    }
    fn versions() -> VersionBinding {
        let v = version();
        VersionBinding {
            baseline: v.clone(),
            contract: v.clone(),
            schema: v.clone(),
            registry: v.clone(),
            implementation: v,
        }
    }

    #[test]
    fn observation_identity_binds_source_content_and_cycle() {
        let record = record("o-1", 1, ObservationValue::Present("20".into()));
        let identity = record.identity(versions(), "cycle-1", "sacs-observation");
        assert_eq!(identity.binding.cycle.as_deref(), Some("cycle-1"));
        assert_eq!(identity.binding.producer, "sacs-observation");
        assert_eq!(
            identity.content_hash,
            canonical_hash(&CanonicalDocument::decode(&record.canonical_bytes()).unwrap())
        );
    }

    #[test]
    fn registered_source_and_valid_record_are_admitted() {
        let mut w = window();
        w.register_source(source()).unwrap();
        w.admit(record("o-1", 1, ObservationValue::Present("20".into())))
            .unwrap();
    }
    #[test]
    fn unknown_source_and_unsupported_type_fail() {
        let mut w = window();
        let mut s = source();
        s.supported_observation_types.clear();
        w.register_source(s).unwrap();
        assert!(matches!(
            w.admit(record("o", 1, ObservationValue::Present("1".into()))),
            Err(ObservationError::UnsupportedObservationType)
        ));
    }
    #[test]
    fn close_prevents_post_close_admission() {
        let mut w = window();
        w.register_source(source()).unwrap();
        let mut closed = w.close(timestamp(80)).unwrap();
        assert!(matches!(
            closed
                .0
                .admit(record("o", 1, ObservationValue::Present("1".into()))),
            Err(ObservationError::WindowClosed)
        ));
    }
    #[test]
    fn cutoff_and_missing_available_value_are_explicit_failures() {
        let mut w = window();
        w.register_source(source()).unwrap();
        let mut late = record("late", 1, ObservationValue::Present("1".into()));
        late.receipt_time = timestamp(91);
        assert!(matches!(
            w.admit(late),
            Err(ObservationError::OutsideCutoff)
        ));
        assert!(matches!(
            w.admit(record("missing", 1, ObservationValue::Absent)),
            Err(ObservationError::MissingAvailableValue)
        ));
    }
    #[test]
    fn ordering_is_independent_of_insertion_order() {
        let mut a = window();
        a.register_source(source()).unwrap();
        a.admit(record("o-2", 2, ObservationValue::Present("2".into())))
            .unwrap();
        a.admit(record("o-1", 1, ObservationValue::Present("1".into())))
            .unwrap();
        let batch = a
            .close(timestamp(80))
            .unwrap()
            .build_batch("batch", "prov", BatchStatus::Complete)
            .unwrap();
        assert_eq!(batch.ordered_observation_ids, vec!["o-1", "o-2"]);
    }
    #[test]
    fn identical_duplicate_is_idempotent_and_conflict_is_rejected() {
        let mut w = window();
        w.register_source(source()).unwrap();
        w.admit(record("o", 1, ObservationValue::Present("1".into())))
            .unwrap();
        assert!(w
            .admit(record("o", 1, ObservationValue::Present("1".into())))
            .is_ok());
        assert!(matches!(
            w.admit(record("o", 1, ObservationValue::Present("2".into()))),
            Err(ObservationError::ConflictingObservation(_))
        ));
    }
    #[test]
    fn identical_batch_retry_is_idempotent_and_conflicting_content_conflicts() {
        let mut w = window();
        w.register_source(source()).unwrap();
        w.admit(record("o", 1, ObservationValue::Present("1".into())))
            .unwrap();
        let batch = w
            .close(timestamp(80))
            .unwrap()
            .build_batch("batch", "prov", BatchStatus::Complete)
            .unwrap();
        let mut store = ObservationStore::default();
        let first = store
            .commit_for_handoff(batch.clone(), versions(), "sacs-observation")
            .unwrap();
        let second = store
            .commit_for_handoff(batch, versions(), "sacs-observation")
            .unwrap();
        assert_eq!(first.commitment_identity, second.commitment_identity);
    }
    #[test]
    fn explicit_availability_is_preserved() {
        let mut w = window();
        let mut s = source();
        s.supported_observation_types.insert("health".into());
        w.register_source(s).unwrap();
        let mut r = record("unavailable", 1, ObservationValue::Absent);
        r.observation_type_id = "health".into();
        r.availability_state = Availability::Unavailable;
        w.admit(r).unwrap();
    }

    #[test]
    fn unavailable_source_cannot_emit_available_evidence() {
        let mut w = window();
        let mut s = source();
        s.available = false;
        w.register_source(s).unwrap();
        assert!(matches!(
            w.admit(record(
                "unavailable-source",
                1,
                ObservationValue::Present("1".into())
            )),
            Err(ObservationError::SourceUnavailable)
        ));
    }

    #[test]
    fn required_manifest_gap_cannot_claim_complete_batch() {
        let mut w = window();
        w.expected_manifest
            .required_observation_types
            .insert("temperature".into());
        w.register_source(source()).unwrap();
        let closed = w.close(timestamp(80)).unwrap();
        assert!(matches!(
            closed.build_batch("batch", "prov", BatchStatus::Complete),
            Err(ObservationError::MissingRequiredObservation(_))
        ));
    }
}
