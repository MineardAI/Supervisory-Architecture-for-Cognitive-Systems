//! Contract 011 realization certification, supervisory provenance, and audit.
//!
//! The two commitment stages are intentionally separate. Stage One certifies
//! one committed execution/continuity pair. Stage Two resolves committed
//! artifacts and relationships before committing one whole-cycle record.
//! Replay and audit are evidence only; Contract 012 receives only the committed
//! cycle record handoff.

use std::collections::BTreeMap;

use canonical_hashing::content_hash;
use canonical_serialization::CanonicalDocument;
use sacs_execution_interface::ExecutionDispositionHandoff;
use sacs_supervisory_continuity::SupervisoryContinuityHandoff;

fn digest(parts: &[(&str, String)]) -> String {
    let mut d = CanonicalDocument::new();
    for (k, v) in parts {
        d.insert(*k, v);
    }
    content_hash(&d.encode())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HistoryFailure {
    InvalidStageOneInput,
    ExecutionContinuityMismatch,
    CycleMismatch,
    MissingEvidence,
    UnsupportedVersion,
    RealizationValidationFailure,
    RealizationCommitmentConflict,
    InvalidStageTwoInput,
    MissingArtifact(String),
    UncommittedArtifact(String),
    ArtifactHashMismatch(String),
    ArtifactVersionMismatch(String),
    ArtifactCycleMismatch(String),
    RelationshipMissing(String),
    RelationshipContradictory(String),
    LineageIncomplete,
    LineageIntegrityInvalid,
    CycleRecordValidationFailure,
    CycleCommitmentConflict,
    ProvenanceFailure(String),
    ReplayNotEquivalent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageOneAdmissionRequest {
    pub execution: ExecutionDispositionHandoff,
    pub continuity: SupervisoryContinuityHandoff,
    pub correlation_id: String,
    pub schema_version: String,
    pub supported_schema_version: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StageOneAdmissionResult {
    pub admission_id: String,
    pub cycle_id: String,
    pub execution_id: String,
    pub continuity_id: String,
    pub valid: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedRealizationRecord {
    pub realization_id: String,
    pub cycle_id: String,
    pub execution_disposition_id: String,
    pub continuity_record_id: String,
    pub mode_id: String,
    pub outcome_id: Option<String>,
    pub evidence_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationValidationResult {
    pub validation_id: String,
    pub realization_id: String,
    pub valid: bool,
    pub reasons: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationCommitmentRecord {
    pub commitment_id: String,
    pub realization_id: String,
    pub cycle_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedRealizationRecord {
    pub realization_id: String,
    pub cycle_id: String,
    pub execution_disposition_id: String,
    pub continuity_record_id: String,
    pub mode_id: String,
    pub outcome_id: Option<String>,
    pub commitment_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RealizationCertificationFailure {
    pub failure_id: String,
    pub cycle_id: String,
    pub reason: HistoryFailure,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalArtifactRequirement {
    pub role: String,
    pub owning_contract: String,
    pub required: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalArtifactReference {
    pub role: String,
    pub artifact_id: String,
    pub content_hash: String,
    pub owning_contract: String,
    pub cycle_id: String,
    pub schema_version: String,
    pub committed: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalArtifactResolutionResult {
    pub role: String,
    pub resolved: bool,
    pub reason: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalArtifactResolutionSet {
    pub set_id: String,
    pub references: Vec<CanonicalArtifactReference>,
    pub results: Vec<CanonicalArtifactResolutionResult>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstitutionalRelationshipDefinition {
    pub relationship_id: String,
    pub relationship_type: String,
    pub source_role: String,
    pub target_role: String,
    pub owning_contract: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedConstitutionalRelationship {
    pub relationship_id: String,
    pub relationship_type: String,
    pub source_id: String,
    pub target_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedConstitutionalRelationshipSet {
    pub set_id: String,
    pub relationships: Vec<ResolvedConstitutionalRelationship>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedSupervisoryLineage {
    pub lineage_id: String,
    pub cycle_id: String,
    pub realization_id: String,
    pub artifact_set_id: String,
    pub relationship_set_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvaluationStatus {
    Complete,
    Incomplete,
    IntegrityValid,
    IntegrityInvalid,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageCompletenessEvaluationResult {
    pub evaluation_id: String,
    pub lineage_id: String,
    pub status: EvaluationStatus,
    pub missing: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageIntegrityEvaluationResult {
    pub evaluation_id: String,
    pub lineage_id: String,
    pub status: EvaluationStatus,
    pub reasons: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedSupervisoryCycleRecord {
    pub cycle_record_id: String,
    pub cycle_id: String,
    pub realization_id: String,
    pub artifact_set_id: String,
    pub relationship_set_id: String,
    pub lineage_id: String,
    pub completeness_id: String,
    pub integrity_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisoryCycleCommitmentRecord {
    pub commitment_id: String,
    pub cycle_record_id: String,
    pub cycle_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedSupervisoryCycleRecord {
    pub cycle_record_id: String,
    pub cycle_id: String,
    pub commitment_id: String,
    pub lineage_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedSupervisoryProvenanceFailure {
    pub failure_id: String,
    pub cycle_id: String,
    pub reason: HistoryFailure,
    pub evidence_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayVerificationResult {
    pub replay_id: String,
    pub terminal_id: String,
    pub equivalent: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditRepresentation {
    pub audit_id: String,
    pub terminal_id: String,
    pub findings: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Contract012Handoff {
    pub cycle_record: CommittedSupervisoryCycleRecord,
    pub content_hash: String,
}

pub struct HistoryEngine;
impl HistoryEngine {
    pub fn admit_stage_one(
        request: &StageOneAdmissionRequest,
    ) -> Result<StageOneAdmissionResult, HistoryFailure> {
        let e = &request.execution.disposition;
        let c = &request.continuity.record;
        if request.schema_version != request.supported_schema_version
            || request.correlation_id.is_empty()
            || e.cycle_id != c.cycle_id
            || c.execution_disposition_id != e.disposition_id
            || request.execution.cycle_id != request.continuity.cycle_id
        {
            return Err(HistoryFailure::InvalidStageOneInput);
        }
        let id = digest(&[
            ("cycle", e.cycle_id.clone()),
            ("execution", e.disposition_id.clone()),
            ("continuity", c.record_id.clone()),
            ("correlation", request.correlation_id.clone()),
        ]);
        Ok(StageOneAdmissionResult {
            admission_id: format!("stage-one-admission:{id}"),
            cycle_id: e.cycle_id.clone(),
            execution_id: e.disposition_id.clone(),
            continuity_id: c.record_id.clone(),
            valid: true,
            content_hash: id,
        })
    }

    pub fn construct_realization(
        request: &StageOneAdmissionRequest,
        admission: &StageOneAdmissionResult,
    ) -> Result<ConstructedRealizationRecord, HistoryFailure> {
        if !admission.valid {
            return Err(HistoryFailure::InvalidStageOneInput);
        };
        let e = &request.execution.disposition;
        let c = &request.continuity.record;
        let id = digest(&[
            ("admission", admission.admission_id.clone()),
            ("execution", e.disposition_id.clone()),
            ("continuity", c.record_id.clone()),
        ]);
        Ok(ConstructedRealizationRecord {
            realization_id: format!("constructed-realization:{id}"),
            cycle_id: e.cycle_id.clone(),
            execution_disposition_id: e.disposition_id.clone(),
            continuity_record_id: c.record_id.clone(),
            mode_id: request.continuity.mode.mode_id.clone(),
            outcome_id: e.outcome_id.clone(),
            evidence_id: request.execution.content_hash.clone(),
            content_hash: id,
        })
    }

    pub fn validate_realization(
        record: &ConstructedRealizationRecord,
    ) -> RealizationValidationResult {
        let valid = !record.evidence_id.is_empty() && !record.mode_id.is_empty();
        let reasons = if valid {
            Vec::new()
        } else {
            vec!["mandatory realization evidence missing".into()]
        };
        let id = digest(&[
            ("realization", record.realization_id.clone()),
            ("valid", valid.to_string()),
            ("reasons", format!("{:?}", reasons)),
        ]);
        RealizationValidationResult {
            validation_id: format!("realization-validation:{id}"),
            realization_id: record.realization_id.clone(),
            valid,
            reasons,
            content_hash: id,
        }
    }

    pub fn commit_realization(
        record: ConstructedRealizationRecord,
        validation: RealizationValidationResult,
    ) -> Result<(CommittedRealizationRecord, RealizationCommitmentRecord), HistoryFailure> {
        if !validation.valid || validation.realization_id != record.realization_id {
            return Err(HistoryFailure::RealizationValidationFailure);
        }
        let id = digest(&[
            ("realization", record.realization_id.clone()),
            ("validation", validation.validation_id.clone()),
        ]);
        let commitment = RealizationCommitmentRecord {
            commitment_id: format!("realization-commitment:{id}"),
            realization_id: record.realization_id.clone(),
            cycle_id: record.cycle_id.clone(),
            content_hash: id.clone(),
        };
        Ok((
            CommittedRealizationRecord {
                realization_id: record.realization_id,
                cycle_id: record.cycle_id,
                execution_disposition_id: record.execution_disposition_id,
                continuity_record_id: record.continuity_record_id,
                mode_id: record.mode_id,
                outcome_id: record.outcome_id,
                commitment_id: commitment.commitment_id.clone(),
                content_hash: id,
            },
            commitment,
        ))
    }

    pub fn resolve_artifacts(
        realization: &CommittedRealizationRecord,
        requirements: &[CanonicalArtifactRequirement],
        references: &[CanonicalArtifactReference],
    ) -> Result<CanonicalArtifactResolutionSet, HistoryFailure> {
        let mut refs = references.to_vec();
        refs.sort_by(|a, b| a.role.cmp(&b.role));
        let mut results = Vec::new();
        for req in requirements {
            let found = refs.iter().find(|r| r.role == req.role);
            if found.is_none() && req.required {
                return Err(HistoryFailure::MissingArtifact(req.role.clone()));
            }
            if let Some(r) = found {
                if !r.committed {
                    return Err(HistoryFailure::UncommittedArtifact(r.role.clone()));
                }
                if r.cycle_id != realization.cycle_id {
                    return Err(HistoryFailure::ArtifactCycleMismatch(r.role.clone()));
                }
                results.push(CanonicalArtifactResolutionResult {
                    role: r.role.clone(),
                    resolved: true,
                    reason: "exact committed reference".into(),
                    content_hash: digest(&[
                        ("role", r.role.clone()),
                        ("id", r.artifact_id.clone()),
                        ("hash", r.content_hash.clone()),
                    ]),
                });
            }
        }
        let id = digest(&[
            ("realization", realization.realization_id.clone()),
            ("references", format!("{:?}", refs)),
            ("results", format!("{:?}", results)),
        ]);
        Ok(CanonicalArtifactResolutionSet {
            set_id: format!("artifact-resolution:{id}"),
            references: refs,
            results,
            content_hash: id,
        })
    }

    pub fn resolve_relationships(
        realization: &CommittedRealizationRecord,
        definitions: &[ConstitutionalRelationshipDefinition],
        artifacts: &CanonicalArtifactResolutionSet,
    ) -> Result<ResolvedConstitutionalRelationshipSet, HistoryFailure> {
        let mut out = Vec::new();
        for d in definitions {
            let s = artifacts
                .references
                .iter()
                .find(|r| r.role == d.source_role)
                .ok_or_else(|| HistoryFailure::RelationshipMissing(d.source_role.clone()))?;
            let t = artifacts
                .references
                .iter()
                .find(|r| r.role == d.target_role)
                .ok_or_else(|| HistoryFailure::RelationshipMissing(d.target_role.clone()))?;
            out.push(ResolvedConstitutionalRelationship {
                relationship_id: d.relationship_id.clone(),
                relationship_type: d.relationship_type.clone(),
                source_id: s.artifact_id.clone(),
                target_id: t.artifact_id.clone(),
                content_hash: digest(&[
                    ("relationship", d.relationship_id.clone()),
                    ("source", s.artifact_id.clone()),
                    ("target", t.artifact_id.clone()),
                ]),
            });
        }
        let id = digest(&[
            ("realization", realization.realization_id.clone()),
            ("relationships", format!("{:?}", out)),
        ]);
        Ok(ResolvedConstitutionalRelationshipSet {
            set_id: format!("relationship-resolution:{id}"),
            relationships: out,
            content_hash: id,
        })
    }

    pub fn lineage(
        realization: &CommittedRealizationRecord,
        artifacts: &CanonicalArtifactResolutionSet,
        relationships: &ResolvedConstitutionalRelationshipSet,
    ) -> ConstructedSupervisoryLineage {
        let id = digest(&[
            ("realization", realization.realization_id.clone()),
            ("artifacts", artifacts.set_id.clone()),
            ("relationships", relationships.set_id.clone()),
        ]);
        ConstructedSupervisoryLineage {
            lineage_id: format!("lineage:{id}"),
            cycle_id: realization.cycle_id.clone(),
            realization_id: realization.realization_id.clone(),
            artifact_set_id: artifacts.set_id.clone(),
            relationship_set_id: relationships.set_id.clone(),
            content_hash: id,
        }
    }

    pub fn completeness(
        lineage: &ConstructedSupervisoryLineage,
        artifacts: &CanonicalArtifactResolutionSet,
        relationships: &ResolvedConstitutionalRelationshipSet,
        requirements: &[CanonicalArtifactRequirement],
    ) -> LineageCompletenessEvaluationResult {
        let missing = requirements
            .iter()
            .filter(|r| r.required && !artifacts.references.iter().any(|a| a.role == r.role))
            .map(|r| r.role.clone())
            .collect::<Vec<_>>();
        let status = if missing.is_empty()
            && (!relationships.relationships.is_empty() || requirements.is_empty())
        {
            EvaluationStatus::Complete
        } else {
            EvaluationStatus::Incomplete
        };
        let id = digest(&[
            ("lineage", lineage.lineage_id.clone()),
            ("status", format!("{:?}", status)),
            ("missing", format!("{:?}", missing)),
        ]);
        LineageCompletenessEvaluationResult {
            evaluation_id: format!("completeness:{id}"),
            lineage_id: lineage.lineage_id.clone(),
            status,
            missing,
            content_hash: id,
        }
    }

    pub fn integrity(
        lineage: &ConstructedSupervisoryLineage,
        artifacts: &CanonicalArtifactResolutionSet,
        relationships: &ResolvedConstitutionalRelationshipSet,
        completeness: &LineageCompletenessEvaluationResult,
    ) -> LineageIntegrityEvaluationResult {
        let mut reasons = Vec::new();
        if !matches!(completeness.status, EvaluationStatus::Complete) {
            reasons.push("lineage incomplete".into())
        }
        if artifacts
            .references
            .iter()
            .any(|a| a.artifact_id.is_empty() || a.content_hash.is_empty())
        {
            reasons.push("artifact identity/hash missing".into())
        }
        if relationships
            .relationships
            .iter()
            .any(|r| r.source_id == r.target_id)
        {
            reasons.push("self relationship".into())
        }
        let status = if reasons.is_empty() {
            EvaluationStatus::IntegrityValid
        } else {
            EvaluationStatus::IntegrityInvalid
        };
        let id = digest(&[
            ("lineage", lineage.lineage_id.clone()),
            ("status", format!("{:?}", status)),
            ("reasons", format!("{:?}", reasons)),
        ]);
        LineageIntegrityEvaluationResult {
            evaluation_id: format!("integrity:{id}"),
            lineage_id: lineage.lineage_id.clone(),
            status,
            reasons,
            content_hash: id,
        }
    }

    pub fn construct_cycle(
        lineage: &ConstructedSupervisoryLineage,
        complete: &LineageCompletenessEvaluationResult,
        integrity: &LineageIntegrityEvaluationResult,
    ) -> Result<ConstructedSupervisoryCycleRecord, HistoryFailure> {
        if !matches!(complete.status, EvaluationStatus::Complete)
            || !matches!(integrity.status, EvaluationStatus::IntegrityValid)
        {
            return Err(HistoryFailure::CycleRecordValidationFailure);
        }
        let id = digest(&[
            ("lineage", lineage.lineage_id.clone()),
            ("complete", complete.evaluation_id.clone()),
            ("integrity", integrity.evaluation_id.clone()),
        ]);
        Ok(ConstructedSupervisoryCycleRecord {
            cycle_record_id: format!("constructed-cycle:{id}"),
            cycle_id: lineage.cycle_id.clone(),
            realization_id: lineage.realization_id.clone(),
            artifact_set_id: lineage.artifact_set_id.clone(),
            relationship_set_id: lineage.relationship_set_id.clone(),
            lineage_id: lineage.lineage_id.clone(),
            completeness_id: complete.evaluation_id.clone(),
            integrity_id: integrity.evaluation_id.clone(),
            content_hash: id,
        })
    }

    pub fn commit_cycle(
        record: ConstructedSupervisoryCycleRecord,
    ) -> Result<
        (
            CommittedSupervisoryCycleRecord,
            SupervisoryCycleCommitmentRecord,
        ),
        HistoryFailure,
    > {
        let id = digest(&[
            ("cycle-record", record.cycle_record_id.clone()),
            ("lineage", record.lineage_id.clone()),
        ]);
        let commitment = SupervisoryCycleCommitmentRecord {
            commitment_id: format!("cycle-commitment:{id}"),
            cycle_record_id: record.cycle_record_id.clone(),
            cycle_id: record.cycle_id.clone(),
            content_hash: id.clone(),
        };
        Ok((
            CommittedSupervisoryCycleRecord {
                cycle_record_id: record.cycle_record_id,
                cycle_id: record.cycle_id,
                lineage_id: record.lineage_id,
                commitment_id: commitment.commitment_id.clone(),
                content_hash: id,
            },
            commitment,
        ))
    }

    pub fn replay<T: Eq>(a: &T, b: &T) -> ReplayVerificationResult {
        let eq = a == b;
        let id = digest(&[("equivalent", eq.to_string())]);
        ReplayVerificationResult {
            replay_id: format!("replay:{id}"),
            terminal_id: id.clone(),
            equivalent: eq,
            content_hash: id,
        }
    }
    pub fn audit<T: std::fmt::Debug>(terminal_id: &str, terminal: &T) -> AuditRepresentation {
        let id = digest(&[
            ("terminal", terminal_id.into()),
            ("terminal-debug", format!("{:?}", terminal)),
        ]);
        AuditRepresentation {
            audit_id: format!("audit:{id}"),
            terminal_id: terminal_id.into(),
            findings: vec!["non-constitutive audit representation".into()],
            content_hash: id,
        }
    }
    pub fn handoff(record: CommittedSupervisoryCycleRecord) -> Contract012Handoff {
        let id = digest(&[
            ("cycle", record.cycle_record_id.clone()),
            ("hash", record.content_hash.clone()),
        ]);
        Contract012Handoff {
            cycle_record: record,
            content_hash: id,
        }
    }
}

#[derive(Default)]
pub struct HistoryStore {
    realizations: BTreeMap<String, (CommittedRealizationRecord, RealizationCommitmentRecord)>,
    cycles: BTreeMap<
        String,
        (
            CommittedSupervisoryCycleRecord,
            SupervisoryCycleCommitmentRecord,
        ),
    >,
}
impl HistoryStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn publish_realization(
        &mut self,
        r: CommittedRealizationRecord,
        c: RealizationCommitmentRecord,
    ) -> Result<(CommittedRealizationRecord, RealizationCommitmentRecord), HistoryFailure> {
        let k = r.realization_id.clone();
        let v = (r.clone(), c.clone());
        if let Some(e) = self.realizations.get(&k) {
            if e == &v {
                return Ok(e.clone());
            }
            return Err(HistoryFailure::RealizationCommitmentConflict);
        }
        self.realizations.insert(k, v.clone());
        Ok(v)
    }
    pub fn publish_cycle(
        &mut self,
        r: CommittedSupervisoryCycleRecord,
        c: SupervisoryCycleCommitmentRecord,
    ) -> Result<
        (
            CommittedSupervisoryCycleRecord,
            SupervisoryCycleCommitmentRecord,
        ),
        HistoryFailure,
    > {
        let k = r.cycle_id.clone();
        let v = (r.clone(), c.clone());
        if let Some(e) = self.cycles.get(&k) {
            if e == &v {
                return Ok(e.clone());
            }
            return Err(HistoryFailure::CycleCommitmentConflict);
        }
        self.cycles.insert(k, v.clone());
        Ok(v)
    }
    pub fn get_cycle(
        &self,
        cycle: &str,
    ) -> Option<&(
        CommittedSupervisoryCycleRecord,
        SupervisoryCycleCommitmentRecord,
    )> {
        self.cycles.get(cycle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stages_are_distinct() {
        assert_ne!(
            std::any::type_name::<ConstructedRealizationRecord>(),
            std::any::type_name::<CommittedRealizationRecord>()
        );
        assert_ne!(
            std::any::type_name::<ConstructedSupervisoryCycleRecord>(),
            std::any::type_name::<CommittedSupervisoryCycleRecord>()
        );
    }
    #[test]
    fn replay_is_non_mutating() {
        let value = "a";
        let r = HistoryEngine::replay(&value, &value);
        assert!(r.equivalent);
    }
    #[test]
    fn audit_is_non_constitutive() {
        let a = HistoryEngine::audit("t", &"terminal");
        assert!(a.findings[0].contains("non-constitutive"));
    }
}
