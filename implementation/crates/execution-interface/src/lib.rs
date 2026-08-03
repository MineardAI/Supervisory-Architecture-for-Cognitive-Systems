//! Contract 009 execution-interface boundary.
//!
//! This crate admits only the committed Contract 008 transaction/envelope,
//! evaluates explicit eligibility, optionally invokes an explicitly supplied
//! transport-neutral adapter, and atomically commits one execution disposition.
//! It does not implement providers, networking, shell execution, continuity,
//! history, or realization certification.

use std::collections::BTreeMap;

use canonical_hashing::content_hash;
use canonical_serialization::CanonicalDocument;
use sacs_inference_control::{CommittedTranslationTransaction, ExecutionInterfaceHandoffEnvelope};

fn digest(parts: &[(&str, String)]) -> String {
    let mut d = CanonicalDocument::new();
    for (key, value) in parts {
        d.insert(*key, value);
    }
    content_hash(&d.encode())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionInterfaceFailure {
    InvalidHandoff,
    UncommittedTranslationInput,
    TransactionEnvelopeMismatch,
    UnsupportedVersion,
    MissingLineage,
    ExecutionIneligible,
    ExecutionRefused,
    ExecutionWithheld,
    ExecutionDeferred,
    AdapterUnavailable,
    AdapterRejected,
    AdapterFailure,
    MalformedAdapterResult,
    PlanConstructionFailure,
    OutcomeMismatch,
    OutcomeValidationFailure,
    MissingRequiredOutcome,
    DispositionValidationFailure(String),
    DispositionCommitmentConflict,
    ReplayNotEquivalent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreExecutionDisposition {
    Allow,
    Restrict,
    Defer,
    Refuse,
    Abort,
    Fail,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionOutcomeStatus {
    SuccessfulApplication,
    PartialApplication,
    RefusedApplication,
    DeferredApplication,
    InterruptedApplication,
    FailedApplication,
    CompensatedApplication,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommittedDispositionStatus {
    Allowed,
    Restricted,
    Deferred,
    Refused,
    Aborted,
    Failed,
    Withheld,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionInterfaceConfiguration {
    pub configuration_id: String,
    pub cycle_id: String,
    pub runtime_id: String,
    pub schema_version: String,
    pub registry_version: String,
    pub supported_schema_version: String,
    pub execution_permitted: bool,
    pub runtime_available: bool,
    pub mandatory_restrictions_preserved: bool,
    pub adapter_id: String,
    pub adapter_version: String,
    pub content_hash: String,
    pub provenance_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionHandoffAdmissionRecord {
    pub admission_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub envelope_id: String,
    pub valid: bool,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptAndEligibilityReport {
    pub report_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub envelope_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreExecutionDispositionRecord {
    pub disposition_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub disposition: PreExecutionDisposition,
    pub restrictions_preserved: bool,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPlan {
    pub plan_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub envelope_id: String,
    pub runtime_id: String,
    pub adapter_id: String,
    pub adapter_version: String,
    pub ordering_rule_id: String,
    pub atomic: bool,
    pub restrictions_preserved: bool,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionAdapterRequest {
    pub request_id: String,
    pub plan_id: String,
    pub transaction_id: String,
    pub runtime_id: String,
    pub adapter_id: String,
    pub adapter_version: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionAdapterResult {
    pub result_id: String,
    pub request_id: String,
    pub adapter_id: String,
    pub adapter_version: String,
    pub status: ExecutionOutcomeStatus,
    pub evidence: String,
    pub evidence_hash: String,
}

pub trait ExecutionAdapter {
    fn identity(&self) -> (&str, &str);
    fn apply(&self, request: &ExecutionAdapterRequest) -> ExecutionAdapterResult;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionOutcome {
    pub outcome_id: String,
    pub plan_id: String,
    pub transaction_id: String,
    pub request_id: String,
    pub result_id: String,
    pub adapter_id: String,
    pub adapter_version: String,
    pub status: ExecutionOutcomeStatus,
    pub evidence_hash: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionDispositionValidationReport {
    pub report_id: String,
    pub valid: bool,
    pub reasons: Vec<String>,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionCommitmentRecord {
    pub commitment_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub envelope_id: String,
    pub admission_id: String,
    pub eligibility_report_id: String,
    pub pre_execution_disposition_id: String,
    pub plan_id: Option<String>,
    pub outcome_id: Option<String>,
    pub disposition_id: String,
    pub status: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedExecutionDisposition {
    pub disposition_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub envelope_id: String,
    pub commitment_id: String,
    pub status: CommittedDispositionStatus,
    pub plan_id: Option<String>,
    pub outcome_id: Option<String>,
    pub adapter_evidence_hash: Option<String>,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionDispositionHandoff {
    pub disposition: CommittedExecutionDisposition,
    pub commitment_id: String,
    pub transaction_id: String,
    pub envelope_id: String,
    pub cycle_id: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedExecutionDisposition {
    pub admission: ExecutionHandoffAdmissionRecord,
    pub eligibility: ReceiptAndEligibilityReport,
    pub pre_execution: PreExecutionDispositionRecord,
    pub plan: Option<ExecutionPlan>,
    pub request: Option<ExecutionAdapterRequest>,
    pub outcome: Option<ExecutionOutcome>,
    pub validation: ExecutionDispositionValidationReport,
}

pub struct ExecutionEngine;
impl ExecutionEngine {
    pub fn admit(
        transaction: &CommittedTranslationTransaction,
        envelope: &ExecutionInterfaceHandoffEnvelope,
        config: &ExecutionInterfaceConfiguration,
    ) -> Result<ExecutionHandoffAdmissionRecord, ExecutionInterfaceFailure> {
        if transaction.transaction_id.is_empty()
            || envelope.envelope_id.is_empty()
            || transaction.cycle_id != envelope.cycle_id
            || transaction.transaction_id != envelope.transaction_id
            || transaction.set_id != envelope.set_id
            || transaction.source_command_id != envelope.source_command_id
            || transaction.runtime_id != envelope.runtime_id
            || config.cycle_id != transaction.cycle_id
            || config.runtime_id != transaction.runtime_id
            || config.schema_version != config.supported_schema_version
        {
            return Err(ExecutionInterfaceFailure::InvalidHandoff);
        }
        if transaction.commitment_id.is_empty()
            || envelope.commitment_id.is_empty()
            || envelope.ordering_rule_id != transaction.ordering_rule_id
            || !envelope.atomic
        {
            return Err(ExecutionInterfaceFailure::MissingLineage);
        }
        let id = digest(&[
            ("cycle", transaction.cycle_id.clone()),
            ("transaction", transaction.transaction_id.clone()),
            ("envelope", envelope.envelope_id.clone()),
            ("set", transaction.set_id.clone()),
        ]);
        Ok(ExecutionHandoffAdmissionRecord {
            admission_id: format!("execution-admission:{id}"),
            cycle_id: transaction.cycle_id.clone(),
            transaction_id: transaction.transaction_id.clone(),
            envelope_id: envelope.envelope_id.clone(),
            valid: true,
            content_hash: id,
        })
    }

    pub fn evaluate_eligibility(
        admission: &ExecutionHandoffAdmissionRecord,
        config: &ExecutionInterfaceConfiguration,
    ) -> ReceiptAndEligibilityReport {
        let mut reasons = Vec::new();
        if !admission.valid {
            reasons.push("invalid admission".into());
        }
        if !config.runtime_available {
            reasons.push("runtime unavailable".into());
        }
        if !config.execution_permitted {
            reasons.push("execution permission absent".into());
        }
        if !config.mandatory_restrictions_preserved {
            reasons.push("mandatory restrictions not preserved".into());
        }
        let eligible = reasons.is_empty();
        let id = digest(&[
            ("admission", admission.admission_id.clone()),
            ("eligible", eligible.to_string()),
            ("reasons", format!("{:?}", reasons)),
        ]);
        ReceiptAndEligibilityReport {
            report_id: format!("eligibility:{id}"),
            cycle_id: admission.cycle_id.clone(),
            transaction_id: admission.transaction_id.clone(),
            envelope_id: admission.envelope_id.clone(),
            eligible,
            reasons,
            content_hash: id,
        }
    }

    pub fn pre_execute(
        report: &ReceiptAndEligibilityReport,
        config: &ExecutionInterfaceConfiguration,
    ) -> PreExecutionDispositionRecord {
        let disposition = if report.eligible {
            PreExecutionDisposition::Allow
        } else if !config.runtime_available {
            PreExecutionDisposition::Defer
        } else if !config.execution_permitted {
            PreExecutionDisposition::Refuse
        } else if !config.mandatory_restrictions_preserved {
            PreExecutionDisposition::Restrict
        } else {
            PreExecutionDisposition::Fail
        };
        let id = digest(&[
            ("report", report.report_id.clone()),
            ("disposition", format!("{:?}", disposition)),
        ]);
        PreExecutionDispositionRecord {
            disposition_id: format!("pre-execution:{id}"),
            cycle_id: report.cycle_id.clone(),
            transaction_id: report.transaction_id.clone(),
            disposition,
            restrictions_preserved: config.mandatory_restrictions_preserved,
            content_hash: id,
        }
    }

    pub fn plan(
        transaction: &CommittedTranslationTransaction,
        envelope: &ExecutionInterfaceHandoffEnvelope,
        pre: &PreExecutionDispositionRecord,
        config: &ExecutionInterfaceConfiguration,
    ) -> Result<(ExecutionPlan, ExecutionAdapterRequest), ExecutionInterfaceFailure> {
        if !matches!(
            pre.disposition,
            PreExecutionDisposition::Allow | PreExecutionDisposition::Restrict
        ) {
            return Err(ExecutionInterfaceFailure::ExecutionIneligible);
        }
        if !pre.restrictions_preserved {
            return Err(ExecutionInterfaceFailure::PlanConstructionFailure);
        }
        let id = digest(&[
            ("transaction", transaction.transaction_id.clone()),
            ("envelope", envelope.envelope_id.clone()),
            ("adapter", config.adapter_id.clone()),
            ("version", config.adapter_version.clone()),
        ]);
        let plan = ExecutionPlan {
            plan_id: format!("execution-plan:{id}"),
            cycle_id: transaction.cycle_id.clone(),
            transaction_id: transaction.transaction_id.clone(),
            envelope_id: envelope.envelope_id.clone(),
            runtime_id: transaction.runtime_id.clone(),
            adapter_id: config.adapter_id.clone(),
            adapter_version: config.adapter_version.clone(),
            ordering_rule_id: transaction.ordering_rule_id.clone(),
            atomic: transaction.atomic,
            restrictions_preserved: pre.restrictions_preserved,
            content_hash: id.clone(),
        };
        let request_hash = digest(&[
            ("plan", plan.plan_id.clone()),
            ("transaction", transaction.transaction_id.clone()),
        ]);
        let request = ExecutionAdapterRequest {
            request_id: format!("adapter-request:{request_hash}"),
            plan_id: plan.plan_id.clone(),
            transaction_id: transaction.transaction_id.clone(),
            runtime_id: transaction.runtime_id.clone(),
            adapter_id: config.adapter_id.clone(),
            adapter_version: config.adapter_version.clone(),
            content_hash: request_hash,
        };
        Ok((plan, request))
    }

    pub fn capture(
        plan: &ExecutionPlan,
        request: &ExecutionAdapterRequest,
        result: ExecutionAdapterResult,
    ) -> Result<ExecutionOutcome, ExecutionInterfaceFailure> {
        if result.request_id != request.request_id
            || result.adapter_id != plan.adapter_id
            || result.adapter_version != plan.adapter_version
            || result.evidence_hash != content_hash(result.evidence.as_bytes())
        {
            return Err(ExecutionInterfaceFailure::MalformedAdapterResult);
        }
        let id = digest(&[
            ("plan", plan.plan_id.clone()),
            ("request", request.request_id.clone()),
            ("result", result.result_id.clone()),
            ("evidence", result.evidence_hash.clone()),
        ]);
        Ok(ExecutionOutcome {
            outcome_id: format!("execution-outcome:{id}"),
            plan_id: plan.plan_id.clone(),
            transaction_id: plan.transaction_id.clone(),
            request_id: request.request_id.clone(),
            result_id: result.result_id,
            adapter_id: result.adapter_id,
            adapter_version: result.adapter_version,
            status: result.status,
            evidence_hash: result.evidence_hash,
            content_hash: id,
        })
    }

    pub fn validate(
        prepared: &PreparedExecutionDisposition,
    ) -> ExecutionDispositionValidationReport {
        let mut reasons = Vec::new();
        if !prepared.admission.valid {
            reasons.push("admission invalid".into());
        }
        if prepared.eligibility.eligible && prepared.plan.is_none() {
            reasons.push("eligible disposition has no plan".into());
        }
        if prepared.outcome.is_some() != prepared.plan.is_some() {
            reasons.push("outcome/plan mismatch".into());
        }
        if let (Some(plan), Some(outcome)) = (&prepared.plan, &prepared.outcome) {
            if plan.plan_id != outcome.plan_id {
                reasons.push("outcome plan mismatch".into());
            }
        }
        let valid = reasons.is_empty();
        let id = digest(&[
            ("admission", prepared.admission.admission_id.clone()),
            ("pre", prepared.pre_execution.disposition_id.clone()),
            ("valid", valid.to_string()),
            ("reasons", format!("{:?}", reasons)),
        ]);
        ExecutionDispositionValidationReport {
            report_id: format!("disposition-validation:{id}"),
            valid,
            reasons,
            content_hash: id,
        }
    }

    pub fn prepare(
        transaction: &CommittedTranslationTransaction,
        envelope: &ExecutionInterfaceHandoffEnvelope,
        config: &ExecutionInterfaceConfiguration,
        adapter: Option<&dyn ExecutionAdapter>,
    ) -> Result<PreparedExecutionDisposition, ExecutionInterfaceFailure> {
        let admission = Self::admit(transaction, envelope, config)?;
        let eligibility = Self::evaluate_eligibility(&admission, config);
        let pre = Self::pre_execute(&eligibility, config);
        let (plan, request, outcome) = if matches!(
            pre.disposition,
            PreExecutionDisposition::Allow | PreExecutionDisposition::Restrict
        ) {
            let (plan, request) = Self::plan(transaction, envelope, &pre, config)?;
            let adapter = adapter.ok_or(ExecutionInterfaceFailure::AdapterUnavailable)?;
            if adapter.identity() != (config.adapter_id.as_str(), config.adapter_version.as_str()) {
                return Err(ExecutionInterfaceFailure::AdapterUnavailable);
            }
            let result = adapter.apply(&request);
            let outcome = Self::capture(&plan, &request, result)?;
            (Some(plan), Some(request), Some(outcome))
        } else {
            (None, None, None)
        };
        let mut prepared = PreparedExecutionDisposition {
            admission,
            eligibility,
            pre_execution: pre,
            plan,
            request,
            outcome,
            validation: ExecutionDispositionValidationReport {
                report_id: String::new(),
                valid: false,
                reasons: Vec::new(),
                content_hash: String::new(),
            },
        };
        prepared.validation = Self::validate(&prepared);
        if !prepared.validation.valid {
            return Err(ExecutionInterfaceFailure::DispositionValidationFailure(
                prepared.validation.reasons.join(","),
            ));
        }
        Ok(prepared)
    }

    pub fn commit(
        prepared: PreparedExecutionDisposition,
    ) -> Result<(CommittedExecutionDisposition, ExecutionCommitmentRecord), ExecutionInterfaceFailure>
    {
        if !prepared.validation.valid {
            return Err(ExecutionInterfaceFailure::DispositionValidationFailure(
                "invalid prepared disposition".into(),
            ));
        }
        let status = match prepared.pre_execution.disposition {
            PreExecutionDisposition::Allow => CommittedDispositionStatus::Allowed,
            PreExecutionDisposition::Restrict => CommittedDispositionStatus::Restricted,
            PreExecutionDisposition::Defer => CommittedDispositionStatus::Deferred,
            PreExecutionDisposition::Refuse => CommittedDispositionStatus::Refused,
            PreExecutionDisposition::Abort => CommittedDispositionStatus::Aborted,
            PreExecutionDisposition::Fail => CommittedDispositionStatus::Failed,
        };
        let id = digest(&[
            ("cycle", prepared.admission.cycle_id.clone()),
            ("transaction", prepared.admission.transaction_id.clone()),
            ("pre", prepared.pre_execution.disposition_id.clone()),
            (
                "outcome",
                prepared
                    .outcome
                    .as_ref()
                    .map(|o| o.outcome_id.clone())
                    .unwrap_or_default(),
            ),
        ]);
        let disposition_id = format!("committed-disposition:{id}");
        let record = ExecutionCommitmentRecord {
            commitment_id: format!("execution-commitment:{id}"),
            cycle_id: prepared.admission.cycle_id.clone(),
            transaction_id: prepared.admission.transaction_id.clone(),
            envelope_id: prepared.admission.envelope_id.clone(),
            admission_id: prepared.admission.admission_id.clone(),
            eligibility_report_id: prepared.eligibility.report_id.clone(),
            pre_execution_disposition_id: prepared.pre_execution.disposition_id.clone(),
            plan_id: prepared.plan.as_ref().map(|p| p.plan_id.clone()),
            outcome_id: prepared.outcome.as_ref().map(|o| o.outcome_id.clone()),
            disposition_id: disposition_id.clone(),
            status: "COMMITTED".into(),
            content_hash: id.clone(),
        };
        Ok((
            CommittedExecutionDisposition {
                disposition_id,
                cycle_id: record.cycle_id.clone(),
                transaction_id: record.transaction_id.clone(),
                envelope_id: record.envelope_id.clone(),
                commitment_id: record.commitment_id.clone(),
                status,
                plan_id: record.plan_id.clone(),
                outcome_id: record.outcome_id.clone(),
                adapter_evidence_hash: prepared.outcome.as_ref().map(|o| o.evidence_hash.clone()),
                content_hash: id,
            },
            record,
        ))
    }

    pub fn handoff(
        disposition: &CommittedExecutionDisposition,
        record: &ExecutionCommitmentRecord,
    ) -> Result<ExecutionDispositionHandoff, ExecutionInterfaceFailure> {
        if disposition.commitment_id != record.commitment_id
            || disposition.disposition_id != record.disposition_id
        {
            return Err(ExecutionInterfaceFailure::InvalidHandoff);
        }
        let id = digest(&[
            ("disposition", disposition.disposition_id.clone()),
            ("commitment", record.commitment_id.clone()),
            ("cycle", disposition.cycle_id.clone()),
        ]);
        Ok(ExecutionDispositionHandoff {
            disposition: disposition.clone(),
            commitment_id: record.commitment_id.clone(),
            transaction_id: disposition.transaction_id.clone(),
            envelope_id: disposition.envelope_id.clone(),
            cycle_id: disposition.cycle_id.clone(),
            content_hash: id,
        })
    }

    pub fn replay<T: Eq>(original: &T, replayed: &T) -> Result<(), ExecutionInterfaceFailure> {
        if original == replayed {
            Ok(())
        } else {
            Err(ExecutionInterfaceFailure::ReplayNotEquivalent)
        }
    }
}

#[derive(Default)]
pub struct ExecutionStore {
    dispositions: BTreeMap<String, (CommittedExecutionDisposition, ExecutionCommitmentRecord)>,
}
impl ExecutionStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn publish(
        &mut self,
        disposition: CommittedExecutionDisposition,
        record: ExecutionCommitmentRecord,
    ) -> Result<(CommittedExecutionDisposition, ExecutionCommitmentRecord), ExecutionInterfaceFailure>
    {
        let key = disposition.cycle_id.clone();
        if let Some(existing) = self.dispositions.get(&key) {
            if existing == &(disposition.clone(), record.clone()) {
                return Ok(existing.clone());
            }
            return Err(ExecutionInterfaceFailure::DispositionCommitmentConflict);
        }
        self.dispositions
            .insert(key, (disposition.clone(), record.clone()));
        Ok((disposition, record))
    }
    pub fn get(
        &self,
        cycle_id: &str,
    ) -> Option<&(CommittedExecutionDisposition, ExecutionCommitmentRecord)> {
        self.dispositions.get(cycle_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct FixtureAdapter;
    impl ExecutionAdapter for FixtureAdapter {
        fn identity(&self) -> (&str, &str) {
            ("fixture", "1")
        }
        fn apply(&self, r: &ExecutionAdapterRequest) -> ExecutionAdapterResult {
            let e = String::from("applied");
            ExecutionAdapterResult {
                result_id: "result:1".into(),
                request_id: r.request_id.clone(),
                adapter_id: "fixture".into(),
                adapter_version: "1".into(),
                status: ExecutionOutcomeStatus::SuccessfulApplication,
                evidence_hash: content_hash(e.as_bytes()),
                evidence: e,
            }
        }
    }
    #[test]
    fn disposition_vocabulary_is_distinct() {
        assert_ne!(
            PreExecutionDisposition::Allow,
            PreExecutionDisposition::Refuse
        );
        assert_ne!(
            CommittedDispositionStatus::Allowed,
            CommittedDispositionStatus::Withheld
        );
    }
    #[test]
    fn adapter_result_is_correlated() {
        let a = FixtureAdapter;
        let request = ExecutionAdapterRequest {
            request_id: "r".into(),
            plan_id: "p".into(),
            transaction_id: "t".into(),
            runtime_id: "rt".into(),
            adapter_id: "fixture".into(),
            adapter_version: "1".into(),
            content_hash: "h".into(),
        };
        let plan = ExecutionPlan {
            plan_id: "p".into(),
            cycle_id: "c".into(),
            transaction_id: "t".into(),
            envelope_id: "e".into(),
            runtime_id: "rt".into(),
            adapter_id: "fixture".into(),
            adapter_version: "1".into(),
            ordering_rule_id: "o".into(),
            atomic: true,
            restrictions_preserved: true,
            content_hash: "h".into(),
        };
        let out = ExecutionEngine::capture(&plan, &request, a.apply(&request)).unwrap();
        assert_eq!(out.request_id, "r");
    }
}
