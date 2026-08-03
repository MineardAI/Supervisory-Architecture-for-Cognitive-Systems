//! Contract 012 external-verification profile and evidence interface.
//!
//! SACS constructs and preserves a bounded verification exchange. It never
//! issues an external judgment, invokes a verifier, changes Contract 011
//! history, or treats internal conformance as external verification.

use canonical_hashing::content_hash;
use canonical_serialization::CanonicalDocument;
use sacs_constitutional_history::Contract012Handoff;
use std::collections::BTreeMap;

fn digest(parts: &[(&str, String)]) -> String {
    let mut d = CanonicalDocument::new();
    for (k, v) in parts {
        d.insert(*k, v);
    }
    content_hash(&d.encode())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationFailure {
    InvalidCycleSubject,
    ExchangeIdentityConflict,
    ClaimInvalid,
    ProfileBindingFailure,
    RequirementBindingFailure,
    MethodBindingFailure,
    ApplicabilityFailure,
    EvidenceResolutionFailure(String),
    EvidenceValidationFailure,
    SubmissionConstructionFailure,
    SubmissionCommitmentConflict,
    CustodyTransferFailure,
    ExternalResultCorrelationFailure,
    ExternalResultIntegrityFailure,
    ResultAdmissionFailure,
    ResultConstructionFailure,
    ResultCertificationFailure,
    ResultCommitmentConflict,
    ExchangeConstructionFailure,
    ExchangeCommitmentFailure,
    TerminalOutcomeConflict,
    ReplayNotEquivalent,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationExchangeIdentity {
    pub exchange_id: String,
    pub cycle_record_id: String,
    pub cycle_id: String,
    pub verifier_id: String,
    pub profile_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationClaim {
    pub claim_id: String,
    pub version: String,
    pub scope: String,
    pub subject_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiableClaimSet {
    pub claim_set_id: String,
    pub exchange_id: String,
    pub claims: Vec<VerificationClaim>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationMethodDefinition {
    pub method_id: String,
    pub version: String,
    pub method_name: String,
    pub limitations: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedVerificationProfileBinding {
    pub profile_id: String,
    pub version: String,
    pub verifier_id: String,
    pub scope: String,
    pub method_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedVerificationRequirementSet {
    pub requirement_id: String,
    pub version: String,
    pub profile_id: String,
    pub required_evidence_roles: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApplicabilityStatus {
    Applicable,
    Inapplicable,
    Unsupported,
    Prohibited,
    Unresolved,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationApplicabilityResult {
    pub result_id: String,
    pub exchange_id: String,
    pub status: ApplicabilityStatus,
    pub reasons: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationEvidenceDefinition {
    pub role: String,
    pub version: String,
    pub required: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationEvidenceReference {
    pub evidence_id: String,
    pub role: String,
    pub subject_id: String,
    pub content_hash: String,
    pub schema_version: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceResolutionResult {
    pub result_id: String,
    pub resolved: Vec<String>,
    pub missing: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceValidationResult {
    pub result_id: String,
    pub valid: bool,
    pub reasons: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationEvidencePackage {
    pub package_id: String,
    pub exchange_id: String,
    pub references: Vec<VerificationEvidenceReference>,
    pub resolution_id: String,
    pub validation_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedVerificationSubmission {
    pub submission_id: String,
    pub exchange_id: String,
    pub claim_set_id: String,
    pub profile_id: String,
    pub requirement_id: String,
    pub package_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationSubmissionCommitmentRecord {
    pub commitment_id: String,
    pub submission_id: String,
    pub exchange_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedVerificationSubmission {
    pub submission_id: String,
    pub exchange_id: String,
    pub commitment_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CustodyStatus {
    Prepared,
    Transferred,
    Acknowledged,
    Returned,
    Failed,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustodyTransferRecord {
    pub transfer_id: String,
    pub exchange_id: String,
    pub artifact_id: String,
    pub sender: String,
    pub receiver: String,
    pub status: CustodyStatus,
    pub sequence: i64,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalVerificationResult {
    pub result_id: String,
    pub exchange_id: String,
    pub verifier_id: String,
    pub submission_id: String,
    pub claim_set_id: String,
    pub method_id: String,
    pub judgment: String,
    pub limitations: String,
    pub payload: String,
    pub payload_hash: String,
    pub externally_issued: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationResultAdmissionRecord {
    pub admission_id: String,
    pub result_id: String,
    pub exchange_id: String,
    pub valid: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedVerificationResult {
    pub constructed_id: String,
    pub external_result_id: String,
    pub exchange_id: String,
    pub judgment: String,
    pub payload_hash: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationResultCertificationRecord {
    pub certification_id: String,
    pub constructed_id: String,
    pub preserves_external_payload: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedVerificationResult {
    pub result_id: String,
    pub exchange_id: String,
    pub external_result_id: String,
    pub judgment: String,
    pub payload_hash: String,
    pub certification_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedVerificationExchangeRecord {
    pub constructed_id: String,
    pub exchange_id: String,
    pub submission_id: String,
    pub custody_transfer_id: String,
    pub result_id: String,
    pub committed_result_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationExchangeCommitmentRecord {
    pub commitment_id: String,
    pub exchange_id: String,
    pub constructed_id: String,
    pub committed_result_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedVerificationExchangeRecord {
    pub exchange_id: String,
    pub commitment_id: String,
    pub submission_id: String,
    pub committed_result_id: String,
    pub judgment: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedVerificationExchangeFailure {
    pub exchange_id: String,
    pub failure_id: String,
    pub reason: VerificationFailure,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationExchangeReplayResult {
    pub replay_id: String,
    pub exchange_id: String,
    pub equivalent: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationExchangeAuditRepresentation {
    pub audit_id: String,
    pub exchange_id: String,
    pub findings: Vec<String>,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationContext {
    pub subject: Contract012Handoff,
    pub verifier_id: String,
    pub profile: CommittedVerificationProfileBinding,
    pub requirements: CommittedVerificationRequirementSet,
    pub method: VerificationMethodDefinition,
    pub claims: Vec<VerificationClaim>,
    pub evidence_definitions: Vec<VerificationEvidenceDefinition>,
    pub evidence: Vec<VerificationEvidenceReference>,
    pub schema_version: String,
    pub supported_schema_version: String,
}

pub trait ExternalVerifierAdapter {
    fn identity(&self) -> (&str, &str);
    fn transmit(&self, submission: &CommittedVerificationSubmission) -> CustodyTransferRecord;
    fn receive(&self, submission: &CommittedVerificationSubmission) -> ExternalVerificationResult;
}

pub struct VerificationEngine;
impl VerificationEngine {
    pub fn admit_committed_cycle(
        ctx: &VerificationContext,
    ) -> Result<VerificationExchangeIdentity, VerificationFailure> {
        let c = &ctx.subject.cycle_record;
        if c.cycle_record_id.is_empty()
            || c.cycle_id.is_empty()
            || ctx.verifier_id.is_empty()
            || ctx.schema_version != ctx.supported_schema_version
        {
            return Err(VerificationFailure::InvalidCycleSubject);
        }
        let id = digest(&[
            ("cycle-record", c.cycle_record_id.clone()),
            ("cycle", c.cycle_id.clone()),
            ("verifier", ctx.verifier_id.clone()),
            ("profile", ctx.profile.profile_id.clone()),
        ]);
        Ok(VerificationExchangeIdentity {
            exchange_id: format!("verification-exchange:{id}"),
            cycle_record_id: c.cycle_record_id.clone(),
            cycle_id: c.cycle_id.clone(),
            verifier_id: ctx.verifier_id.clone(),
            profile_id: ctx.profile.profile_id.clone(),
            content_hash: id,
        })
    }
    pub fn construct_claim_set(
        id: &VerificationExchangeIdentity,
        claims: &[VerificationClaim],
    ) -> Result<VerifiableClaimSet, VerificationFailure> {
        let mut c = claims.to_vec();
        c.sort_by(|a, b| a.claim_id.cmp(&b.claim_id));
        if c.iter().any(|x| {
            x.subject_id != id.cycle_record_id || x.claim_id.is_empty() || x.version.is_empty()
        }) {
            return Err(VerificationFailure::ClaimInvalid);
        }
        if c.windows(2).any(|w| w[0].claim_id == w[1].claim_id) {
            return Err(VerificationFailure::ClaimInvalid);
        }
        let h = digest(&[
            ("exchange", id.exchange_id.clone()),
            ("claims", format!("{:?}", c)),
        ]);
        Ok(VerifiableClaimSet {
            claim_set_id: format!("claim-set:{h}"),
            exchange_id: id.exchange_id.clone(),
            claims: c,
            content_hash: h,
        })
    }
    pub fn bind_profile(
        id: &VerificationExchangeIdentity,
        profile: CommittedVerificationProfileBinding,
        method: &VerificationMethodDefinition,
    ) -> Result<CommittedVerificationProfileBinding, VerificationFailure> {
        if profile.profile_id != id.profile_id
            || profile.verifier_id != id.verifier_id
            || profile.method_id != method.method_id
            || profile.version.is_empty()
        {
            return Err(VerificationFailure::ProfileBindingFailure);
        }
        Ok(profile)
    }
    pub fn bind_requirements(
        id: &VerificationExchangeIdentity,
        profile: &CommittedVerificationProfileBinding,
        requirements: CommittedVerificationRequirementSet,
    ) -> Result<CommittedVerificationRequirementSet, VerificationFailure> {
        if requirements.profile_id != profile.profile_id || requirements.version.is_empty() {
            return Err(VerificationFailure::RequirementBindingFailure);
        }
        let _ = id;
        Ok(requirements)
    }
    pub fn applicability(
        id: &VerificationExchangeIdentity,
        claims: &VerifiableClaimSet,
        profile: &CommittedVerificationProfileBinding,
    ) -> VerificationApplicabilityResult {
        let ok = claims.exchange_id == id.exchange_id
            && profile.profile_id == id.profile_id
            && !claims.claims.is_empty();
        let status = if ok {
            ApplicabilityStatus::Applicable
        } else {
            ApplicabilityStatus::Unresolved
        };
        let reasons = if ok {
            Vec::new()
        } else {
            vec!["claim/profile binding unresolved".into()]
        };
        let h = digest(&[
            ("exchange", id.exchange_id.clone()),
            ("status", format!("{:?}", status)),
            ("reasons", format!("{:?}", reasons)),
        ]);
        VerificationApplicabilityResult {
            result_id: format!("applicability:{h}"),
            exchange_id: id.exchange_id.clone(),
            status,
            reasons,
            content_hash: h,
        }
    }
    pub fn resolve_evidence(
        id: &VerificationExchangeIdentity,
        requirements: &CommittedVerificationRequirementSet,
        evidence: &[VerificationEvidenceReference],
    ) -> EvidenceResolutionResult {
        let mut resolved = Vec::new();
        let mut missing = Vec::new();
        for role in &requirements.required_evidence_roles {
            if let Some(e) = evidence
                .iter()
                .find(|e| e.role == *role && e.subject_id == id.cycle_record_id)
            {
                resolved.push(e.evidence_id.clone())
            } else {
                missing.push(role.clone())
            }
        }
        let h = digest(&[
            ("exchange", id.exchange_id.clone()),
            ("resolved", format!("{:?}", resolved)),
            ("missing", format!("{:?}", missing)),
        ]);
        EvidenceResolutionResult {
            result_id: format!("evidence-resolution:{h}"),
            resolved,
            missing,
            content_hash: h,
        }
    }
    pub fn validate_evidence(
        resolution: &EvidenceResolutionResult,
        evidence: &[VerificationEvidenceReference],
    ) -> EvidenceValidationResult {
        let mut reasons = Vec::new();
        if !resolution.missing.is_empty() {
            reasons.push("required evidence missing".into())
        }
        if evidence
            .iter()
            .any(|e| e.evidence_id.is_empty() || e.content_hash.is_empty())
        {
            reasons.push("evidence identity/hash missing".into())
        }
        let valid = reasons.is_empty();
        let h = digest(&[
            ("resolution", resolution.result_id.clone()),
            ("valid", valid.to_string()),
            ("reasons", format!("{:?}", reasons)),
        ]);
        EvidenceValidationResult {
            result_id: format!("evidence-validation:{h}"),
            valid,
            reasons,
            content_hash: h,
        }
    }
    pub fn construct_package(
        id: &VerificationExchangeIdentity,
        evidence: &[VerificationEvidenceReference],
        resolution: &EvidenceResolutionResult,
        validation: &EvidenceValidationResult,
    ) -> Result<VerificationEvidencePackage, VerificationFailure> {
        if !validation.valid {
            return Err(VerificationFailure::EvidenceValidationFailure);
        }
        let h = digest(&[
            ("exchange", id.exchange_id.clone()),
            ("resolution", resolution.result_id.clone()),
            ("validation", validation.result_id.clone()),
            ("evidence", format!("{:?}", evidence)),
        ]);
        Ok(VerificationEvidencePackage {
            package_id: format!("evidence-package:{h}"),
            exchange_id: id.exchange_id.clone(),
            references: evidence.to_vec(),
            resolution_id: resolution.result_id.clone(),
            validation_id: validation.result_id.clone(),
            content_hash: h,
        })
    }
    pub fn construct_submission(
        id: &VerificationExchangeIdentity,
        claims: &VerifiableClaimSet,
        profile: &CommittedVerificationProfileBinding,
        requirements: &CommittedVerificationRequirementSet,
        package: &VerificationEvidencePackage,
    ) -> Result<ConstructedVerificationSubmission, VerificationFailure> {
        let h = digest(&[
            ("exchange", id.exchange_id.clone()),
            ("claims", claims.claim_set_id.clone()),
            ("profile", profile.profile_id.clone()),
            ("requirements", requirements.requirement_id.clone()),
            ("package", package.package_id.clone()),
        ]);
        Ok(ConstructedVerificationSubmission {
            submission_id: format!("constructed-submission:{h}"),
            exchange_id: id.exchange_id.clone(),
            claim_set_id: claims.claim_set_id.clone(),
            profile_id: profile.profile_id.clone(),
            requirement_id: requirements.requirement_id.clone(),
            package_id: package.package_id.clone(),
            content_hash: h,
        })
    }
    pub fn commit_submission(
        s: ConstructedVerificationSubmission,
    ) -> (
        CommittedVerificationSubmission,
        VerificationSubmissionCommitmentRecord,
    ) {
        let h = digest(&[
            ("submission", s.submission_id.clone()),
            ("exchange", s.exchange_id.clone()),
        ]);
        let c = VerificationSubmissionCommitmentRecord {
            commitment_id: format!("submission-commitment:{h}"),
            submission_id: s.submission_id.clone(),
            exchange_id: s.exchange_id.clone(),
            content_hash: h.clone(),
        };
        (
            CommittedVerificationSubmission {
                submission_id: s.submission_id,
                exchange_id: s.exchange_id,
                commitment_id: c.commitment_id.clone(),
                content_hash: h,
            },
            c,
        )
    }
    pub fn custody(
        id: &VerificationExchangeIdentity,
        submission: &CommittedVerificationSubmission,
        sender: &str,
        receiver: &str,
        sequence: i64,
    ) -> Result<CustodyTransferRecord, VerificationFailure> {
        if submission.exchange_id != id.exchange_id || sender.is_empty() || receiver.is_empty() {
            return Err(VerificationFailure::CustodyTransferFailure);
        }
        let h = digest(&[
            ("exchange", id.exchange_id.clone()),
            ("submission", submission.submission_id.clone()),
            ("sender", sender.into()),
            ("receiver", receiver.into()),
            ("sequence", sequence.to_string()),
        ]);
        Ok(CustodyTransferRecord {
            transfer_id: format!("custody:{h}"),
            exchange_id: id.exchange_id.clone(),
            artifact_id: submission.submission_id.clone(),
            sender: sender.into(),
            receiver: receiver.into(),
            status: CustodyStatus::Transferred,
            sequence,
            content_hash: h,
        })
    }
    pub fn admit_result(
        id: &VerificationExchangeIdentity,
        submission: &CommittedVerificationSubmission,
        result: &ExternalVerificationResult,
    ) -> Result<VerificationResultAdmissionRecord, VerificationFailure> {
        if !result.externally_issued
            || result.exchange_id != id.exchange_id
            || result.submission_id != submission.submission_id
            || result.payload_hash != content_hash(result.payload.as_bytes())
        {
            return Err(VerificationFailure::ExternalResultIntegrityFailure);
        }
        let h = digest(&[
            ("exchange", id.exchange_id.clone()),
            ("result", result.result_id.clone()),
            ("payload", result.payload_hash.clone()),
        ]);
        Ok(VerificationResultAdmissionRecord {
            admission_id: format!("result-admission:{h}"),
            result_id: result.result_id.clone(),
            exchange_id: id.exchange_id.clone(),
            valid: true,
            content_hash: h,
        })
    }
    pub fn construct_result(
        result: &ExternalVerificationResult,
        admission: &VerificationResultAdmissionRecord,
    ) -> Result<ConstructedVerificationResult, VerificationFailure> {
        if !admission.valid || admission.result_id != result.result_id {
            return Err(VerificationFailure::ResultAdmissionFailure);
        }
        let h = digest(&[
            ("result", result.result_id.clone()),
            ("payload", result.payload_hash.clone()),
            ("judgment", result.judgment.clone()),
        ]);
        Ok(ConstructedVerificationResult {
            constructed_id: format!("constructed-result:{h}"),
            external_result_id: result.result_id.clone(),
            exchange_id: result.exchange_id.clone(),
            judgment: result.judgment.clone(),
            payload_hash: result.payload_hash.clone(),
            content_hash: h,
        })
    }
    pub fn certify_result(
        r: &ConstructedVerificationResult,
    ) -> VerificationResultCertificationRecord {
        let h = digest(&[
            ("constructed", r.constructed_id.clone()),
            ("payload", r.payload_hash.clone()),
        ]);
        VerificationResultCertificationRecord {
            certification_id: format!("result-certification:{h}"),
            constructed_id: r.constructed_id.clone(),
            preserves_external_payload: true,
            content_hash: h,
        }
    }
    pub fn commit_result(
        r: ConstructedVerificationResult,
        c: VerificationResultCertificationRecord,
    ) -> Result<CommittedVerificationResult, VerificationFailure> {
        if c.constructed_id != r.constructed_id || !c.preserves_external_payload {
            return Err(VerificationFailure::ResultCertificationFailure);
        }
        let h = digest(&[
            ("constructed", r.constructed_id.clone()),
            ("certification", c.certification_id.clone()),
        ]);
        Ok(CommittedVerificationResult {
            result_id: format!("committed-result:{h}"),
            exchange_id: r.exchange_id,
            external_result_id: r.external_result_id,
            judgment: r.judgment,
            payload_hash: r.payload_hash,
            certification_id: c.certification_id,
            content_hash: h,
        })
    }
    pub fn construct_exchange(
        id: &VerificationExchangeIdentity,
        submission: &CommittedVerificationSubmission,
        custody: &CustodyTransferRecord,
        result: &CommittedVerificationResult,
    ) -> Result<ConstructedVerificationExchangeRecord, VerificationFailure> {
        if submission.exchange_id != id.exchange_id
            || custody.exchange_id != id.exchange_id
            || result.exchange_id != id.exchange_id
        {
            return Err(VerificationFailure::ExchangeConstructionFailure);
        }
        let h = digest(&[
            ("exchange", id.exchange_id.clone()),
            ("submission", submission.submission_id.clone()),
            ("custody", custody.transfer_id.clone()),
            ("result", result.result_id.clone()),
        ]);
        Ok(ConstructedVerificationExchangeRecord {
            constructed_id: format!("constructed-exchange:{h}"),
            exchange_id: id.exchange_id.clone(),
            submission_id: submission.submission_id.clone(),
            custody_transfer_id: custody.transfer_id.clone(),
            result_id: result.external_result_id.clone(),
            committed_result_id: result.result_id.clone(),
            content_hash: h,
        })
    }
    pub fn commit_exchange(
        e: ConstructedVerificationExchangeRecord,
    ) -> Result<
        (
            CommittedVerificationExchangeRecord,
            VerificationExchangeCommitmentRecord,
        ),
        VerificationFailure,
    > {
        let h = digest(&[
            ("constructed", e.constructed_id.clone()),
            ("exchange", e.exchange_id.clone()),
            ("result", e.committed_result_id.clone()),
        ]);
        let c = VerificationExchangeCommitmentRecord {
            commitment_id: format!("exchange-commitment:{h}"),
            exchange_id: e.exchange_id.clone(),
            constructed_id: e.constructed_id,
            committed_result_id: e.committed_result_id.clone(),
            content_hash: h.clone(),
        };
        Ok((
            CommittedVerificationExchangeRecord {
                exchange_id: e.exchange_id,
                commitment_id: c.commitment_id.clone(),
                submission_id: e.submission_id,
                committed_result_id: e.committed_result_id,
                judgment: "preserved-external-judgment".into(),
                content_hash: h,
            },
            c,
        ))
    }
    pub fn replay<T: Eq>(a: &T, b: &T) -> VerificationExchangeReplayResult {
        let eq = a == b;
        let h = digest(&[("equivalent", eq.to_string())]);
        VerificationExchangeReplayResult {
            replay_id: format!("exchange-replay:{h}"),
            exchange_id: "replay-only".into(),
            equivalent: eq,
            content_hash: h,
        }
    }
    pub fn audit<T: std::fmt::Debug>(
        exchange: &str,
        terminal: &T,
    ) -> VerificationExchangeAuditRepresentation {
        let h = digest(&[
            ("exchange", exchange.into()),
            ("terminal", format!("{:?}", terminal)),
        ]);
        VerificationExchangeAuditRepresentation {
            audit_id: format!("exchange-audit:{h}"),
            exchange_id: exchange.into(),
            findings: vec!["non-constitutive audit".into()],
            content_hash: h,
        }
    }
}

#[derive(Default)]
pub struct VerificationStore {
    exchanges: BTreeMap<String, CommittedVerificationExchangeRecord>,
}
impl VerificationStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn publish(
        &mut self,
        e: CommittedVerificationExchangeRecord,
    ) -> Result<CommittedVerificationExchangeRecord, VerificationFailure> {
        if let Some(x) = self.exchanges.get(&e.exchange_id) {
            if x == &e {
                return Ok(x.clone());
            }
            return Err(VerificationFailure::TerminalOutcomeConflict);
        }
        self.exchanges.insert(e.exchange_id.clone(), e.clone());
        Ok(e)
    }
    pub fn get(&self, id: &str) -> Option<&CommittedVerificationExchangeRecord> {
        self.exchanges.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn external_judgment_is_not_internal() {
        let r = ExternalVerificationResult {
            result_id: "r".into(),
            exchange_id: "e".into(),
            verifier_id: "v".into(),
            submission_id: "s".into(),
            claim_set_id: "c".into(),
            method_id: "m".into(),
            judgment: "UNFAVORABLE".into(),
            limitations: "limited".into(),
            payload: "opaque".into(),
            payload_hash: content_hash(b"opaque"),
            externally_issued: true,
        };
        assert_eq!(r.judgment, "UNFAVORABLE");
    }
    #[test]
    fn custody_is_not_ownership() {
        assert_ne!(CustodyStatus::Transferred, CustodyStatus::Acknowledged);
    }
    #[test]
    fn replay_is_nonconstitutive() {
        assert!(VerificationEngine::replay(&"x", &"x").equivalent);
    }
}
