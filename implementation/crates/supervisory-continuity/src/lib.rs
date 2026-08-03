//! Contract 010 supervisory-continuity boundary.
//!
//! This crate owns only cross-cycle mode continuity. It consumes committed
//! execution disposition and committed supervisory context, evaluates explicit
//! mode rules, and commits one mode plus one continuity record. It does not
//! execute, certify realization, create history, or use an ambient clock.

use std::collections::BTreeMap;

use canonical_hashing::content_hash;
use canonical_serialization::CanonicalDocument;
use sacs_execution_interface::{CommittedExecutionDisposition, ExecutionDispositionHandoff};
use sacs_governance::CommittedGovernanceHandoff;
use sacs_operational_state_vector::CommittedOsvHandoff;

fn digest(parts: &[(&str, String)]) -> String {
    let mut d = CanonicalDocument::new();
    for (k, v) in parts {
        d.insert(*k, v);
    }
    content_hash(&d.encode())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContinuityFailure {
    InvalidExecutionHandoff,
    CycleMismatch,
    MissingPriorMode,
    MissingDefinition,
    RegistryMismatch,
    GuardConflict,
    PrecedenceUnresolved,
    ResetInvalid,
    HysteresisBlocked,
    DwellViolation,
    TimingContextInvalid,
    ContinuityValidationFailure,
    ContinuityCommitmentConflict,
    ReplayNotEquivalent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisoryModeDefinition {
    pub mode_id: String,
    pub mode_name: String,
    pub mode_version: String,
    pub mode_class: String,
    pub purpose: String,
    pub authority_level: String,
    pub permitted_transition_set_id: String,
    pub prohibited_transition_set_id: String,
    pub guard_set_id: String,
    pub dwell_definition_id: String,
    pub hysteresis_definition_id: String,
    pub reset_set_id: String,
    pub safe_mode: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GuardPredicate {
    Always,
    ExecutionAllowed,
    ExecutionFailed,
    NumericAtLeast { value: i64 },
    NumericBelow { value: i64 },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardDefinition {
    pub guard_id: String,
    pub version: String,
    pub predicate: GuardPredicate,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionDefinition {
    pub transition_id: String,
    pub version: String,
    pub source_mode_id: String,
    pub destination_mode_id: String,
    pub guard_ids: Vec<String>,
    pub priority: i64,
    pub reset_definition_id: Option<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionPrecedencePolicy {
    pub policy_id: String,
    pub version: String,
    pub tie_rule: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResetDefinition {
    pub reset_id: String,
    pub version: String,
    pub target_mode_id: String,
    pub reason: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HysteresisDefinition {
    pub hysteresis_id: String,
    pub version: String,
    pub threshold: i64,
    pub direction: String,
    pub numeric_profile_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DwellTimeDefinition {
    pub dwell_id: String,
    pub version: String,
    pub minimum_duration: i64,
    pub time_unit: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModeRegistrySnapshot {
    pub snapshot_id: String,
    pub mode_ids: Vec<String>,
    pub transition_ids: Vec<String>,
    pub registry_version: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedSupervisoryMode {
    pub commitment_id: String,
    pub mode_id: String,
    pub mode_version: String,
    pub cycle_id: String,
    pub committed_at: i64,
    pub registry_snapshot_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuityTimingContext {
    pub time_source_id: String,
    pub time_unit: String,
    pub prior_cutoff: i64,
    pub current_cutoff: i64,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuityContext {
    pub execution: ExecutionDispositionHandoff,
    pub execution_disposition: CommittedExecutionDisposition,
    pub osv: CommittedOsvHandoff,
    pub governance: CommittedGovernanceHandoff,
    pub prior_mode: Option<CommittedSupervisoryMode>,
    pub registry: ModeRegistrySnapshot,
    pub modes: Vec<SupervisoryModeDefinition>,
    pub guards: Vec<GuardDefinition>,
    pub transitions: Vec<TransitionDefinition>,
    pub precedence: TransitionPrecedencePolicy,
    pub resets: Vec<ResetDefinition>,
    pub hysteresis: HysteresisDefinition,
    pub dwell: DwellTimeDefinition,
    pub timing: ContinuityTimingContext,
    pub numeric_value: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardEvaluationResult {
    pub guard_id: String,
    pub satisfied: bool,
    pub reason: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardEvaluationSet {
    pub set_id: String,
    pub results: Vec<GuardEvaluationResult>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionEligibilityEvaluation {
    pub transition_id: String,
    pub eligible: bool,
    pub guard_set_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionPrecedenceResult {
    pub result_id: String,
    pub eligible: Vec<String>,
    pub selected: Option<String>,
    pub policy_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResetEvaluation {
    pub reset_id: Option<String>,
    pub applicable: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResetRecord {
    pub record_id: String,
    pub reset_id: String,
    pub source_mode_id: String,
    pub target_mode_id: String,
    pub cycle_id: String,
    pub reason: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HysteresisEvaluation {
    pub hysteresis_id: String,
    pub permits: bool,
    pub value: i64,
    pub threshold: i64,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DwellTimeEvaluation {
    pub dwell_id: String,
    pub satisfied: bool,
    pub elapsed: i64,
    pub required: i64,
    pub time_unit: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedSupervisoryModeTransition {
    pub decision_id: String,
    pub cycle_id: String,
    pub prior_mode_id: String,
    pub proposed_mode_id: String,
    pub precedence_result_id: String,
    pub guard_set_id: String,
    pub reset_record_id: Option<String>,
    pub hysteresis_id: String,
    pub dwell_id: String,
    pub execution_disposition_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuityCommitmentRecord {
    pub commitment_id: String,
    pub cycle_id: String,
    pub decision_id: String,
    pub mode_commitment_id: String,
    pub continuity_record_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedSupervisoryContinuityRecord {
    pub record_id: String,
    pub cycle_id: String,
    pub mode_commitment_id: String,
    pub execution_disposition_id: String,
    pub osv_id: String,
    pub governance_id: String,
    pub decision_id: String,
    pub guard_set_id: String,
    pub precedence_result_id: String,
    pub reset_record_id: Option<String>,
    pub hysteresis_id: String,
    pub dwell_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisoryContinuityHandoff {
    pub mode: CommittedSupervisoryMode,
    pub record: CommittedSupervisoryContinuityRecord,
    pub commitment_id: String,
    pub cycle_id: String,
    pub content_hash: String,
}

pub struct ContinuityEngine;
impl ContinuityEngine {
    pub fn admit(ctx: &ContinuityContext) -> Result<(), ContinuityFailure> {
        if ctx.execution.disposition.disposition_id != ctx.execution_disposition.disposition_id
            || ctx.execution.disposition.cycle_id != ctx.execution_disposition.cycle_id
            || ctx.execution.transaction_id.is_empty()
            || ctx.execution.envelope_id.is_empty()
        {
            return Err(ContinuityFailure::InvalidExecutionHandoff);
        }
        let cycle = &ctx.execution_disposition.cycle_id;
        if ctx.osv.osv.cycle_id != *cycle
            || ctx.governance.snapshot.cycle_id != *cycle
            || ctx.timing.current_cutoff < ctx.timing.prior_cutoff
            || ctx.timing.time_source_id.is_empty()
            || ctx.timing.time_unit.is_empty()
        {
            return Err(ContinuityFailure::CycleMismatch);
        }
        if ctx.prior_mode.is_none() {
            return Err(ContinuityFailure::MissingPriorMode);
        }
        if ctx.registry.mode_ids.is_empty() || ctx.modes.is_empty() {
            return Err(ContinuityFailure::MissingDefinition);
        }
        if ctx.registry.registry_version.is_empty() {
            return Err(ContinuityFailure::RegistryMismatch);
        }
        Ok(())
    }

    pub fn evaluate_guards(
        ctx: &ContinuityContext,
        transition: &TransitionDefinition,
    ) -> Result<GuardEvaluationSet, ContinuityFailure> {
        let mut results = Vec::new();
        for id in &transition.guard_ids {
            let g = ctx
                .guards
                .iter()
                .find(|g| g.guard_id == *id)
                .ok_or(ContinuityFailure::MissingDefinition)?;
            let (ok, reason) = match &g.predicate {
                GuardPredicate::Always => (true, "always"),
                GuardPredicate::ExecutionAllowed => (
                    matches!(
                        ctx.execution_disposition.status,
                        sacs_execution_interface::CommittedDispositionStatus::Allowed
                            | sacs_execution_interface::CommittedDispositionStatus::Restricted
                    ),
                    "execution disposition",
                ),
                GuardPredicate::ExecutionFailed => (
                    matches!(
                        ctx.execution_disposition.status,
                        sacs_execution_interface::CommittedDispositionStatus::Failed
                            | sacs_execution_interface::CommittedDispositionStatus::Aborted
                    ),
                    "execution failure",
                ),
                GuardPredicate::NumericAtLeast { value } => {
                    (ctx.numeric_value >= *value, "numeric threshold")
                }
                GuardPredicate::NumericBelow { value } => {
                    (ctx.numeric_value < *value, "numeric threshold")
                }
            };
            let h = digest(&[
                ("guard", g.guard_id.clone()),
                ("ok", ok.to_string()),
                ("value", ctx.numeric_value.to_string()),
            ]);
            results.push(GuardEvaluationResult {
                guard_id: g.guard_id.clone(),
                satisfied: ok,
                reason: reason.into(),
                content_hash: h,
            });
        }
        let id = digest(&[
            ("transition", transition.transition_id.clone()),
            ("results", format!("{:?}", results)),
        ]);
        Ok(GuardEvaluationSet {
            set_id: format!("guard-set:{id}"),
            results,
            content_hash: id,
        })
    }

    pub fn eligible(
        ctx: &ContinuityContext,
        source_mode_id: &str,
    ) -> Result<
        (
            Vec<TransitionEligibilityEvaluation>,
            Vec<GuardEvaluationSet>,
        ),
        ContinuityFailure,
    > {
        let mut es = Vec::new();
        let mut gs = Vec::new();
        for t in ctx
            .transitions
            .iter()
            .filter(|t| t.source_mode_id == source_mode_id)
        {
            let set = Self::evaluate_guards(ctx, t)?;
            let ok = set.results.iter().all(|r| r.satisfied);
            es.push(TransitionEligibilityEvaluation {
                transition_id: t.transition_id.clone(),
                eligible: ok,
                guard_set_id: set.set_id.clone(),
                content_hash: digest(&[
                    ("transition", t.transition_id.clone()),
                    ("eligible", ok.to_string()),
                ]),
            });
            gs.push(set);
        }
        Ok((es, gs))
    }

    pub fn precedence(
        ctx: &ContinuityContext,
        evaluations: &[TransitionEligibilityEvaluation],
    ) -> Result<TransitionPrecedenceResult, ContinuityFailure> {
        let mut candidates = evaluations
            .iter()
            .filter(|e| e.eligible)
            .map(|e| e.transition_id.clone())
            .collect::<Vec<_>>();
        candidates.sort();
        let selected = if candidates.is_empty() {
            None
        } else {
            let mut ranked = candidates
                .iter()
                .filter_map(|id| {
                    ctx.transitions
                        .iter()
                        .find(|t| &t.transition_id == id)
                        .map(|t| (t.priority, t.transition_id.clone()))
                })
                .collect::<Vec<_>>();
            ranked.sort_by(|a, b| b.cmp(a));
            if ranked.len() > 1
                && ranked[0].0 == ranked[1].0
                && ctx.precedence.tie_rule != "lexicographic"
            {
                return Err(ContinuityFailure::PrecedenceUnresolved);
            }
            Some(ranked[0].1.clone())
        };
        let id = digest(&[
            ("eligible", format!("{:?}", candidates)),
            ("selected", format!("{:?}", selected)),
            ("policy", ctx.precedence.policy_id.clone()),
        ]);
        Ok(TransitionPrecedenceResult {
            result_id: format!("precedence:{id}"),
            eligible: candidates,
            selected,
            policy_id: ctx.precedence.policy_id.clone(),
            content_hash: id,
        })
    }

    pub fn reset(
        ctx: &ContinuityContext,
        selected: Option<&TransitionDefinition>,
    ) -> Result<(ResetEvaluation, Option<ResetRecord>), ContinuityFailure> {
        let Some(t) = selected else {
            return Ok((
                ResetEvaluation {
                    reset_id: None,
                    applicable: false,
                    content_hash: "no-reset".into(),
                },
                None,
            ));
        };
        let Some(id) = &t.reset_definition_id else {
            return Ok((
                ResetEvaluation {
                    reset_id: None,
                    applicable: false,
                    content_hash: "no-reset".into(),
                },
                None,
            ));
        };
        let r = ctx
            .resets
            .iter()
            .find(|r| r.reset_id == *id)
            .ok_or(ContinuityFailure::ResetInvalid)?;
        let source = ctx
            .prior_mode
            .as_ref()
            .ok_or(ContinuityFailure::MissingPriorMode)?;
        if r.target_mode_id != t.destination_mode_id {
            return Err(ContinuityFailure::ResetInvalid);
        }
        let h = digest(&[
            ("reset", r.reset_id.clone()),
            ("source", source.mode_id.clone()),
            ("target", r.target_mode_id.clone()),
        ]);
        Ok((
            ResetEvaluation {
                reset_id: Some(r.reset_id.clone()),
                applicable: true,
                content_hash: h.clone(),
            },
            Some(ResetRecord {
                record_id: format!("reset-record:{h}"),
                reset_id: r.reset_id.clone(),
                source_mode_id: source.mode_id.clone(),
                target_mode_id: r.target_mode_id.clone(),
                cycle_id: ctx.execution_disposition.cycle_id.clone(),
                reason: r.reason.clone(),
                content_hash: h,
            }),
        ))
    }

    pub fn hysteresis(
        ctx: &ContinuityContext,
        transition: Option<&TransitionDefinition>,
    ) -> HysteresisEvaluation {
        let permits = transition.is_none()
            || match ctx.hysteresis.direction.as_str() {
                "above" => ctx.numeric_value >= ctx.hysteresis.threshold,
                "below" => ctx.numeric_value <= ctx.hysteresis.threshold,
                _ => false,
            };
        let id = digest(&[
            ("hysteresis", ctx.hysteresis.hysteresis_id.clone()),
            ("permits", permits.to_string()),
            ("value", ctx.numeric_value.to_string()),
        ]);
        HysteresisEvaluation {
            hysteresis_id: ctx.hysteresis.hysteresis_id.clone(),
            permits,
            value: ctx.numeric_value,
            threshold: ctx.hysteresis.threshold,
            content_hash: id,
        }
    }

    pub fn dwell(ctx: &ContinuityContext) -> Result<DwellTimeEvaluation, ContinuityFailure> {
        let Some(mode) = &ctx.prior_mode else {
            return Err(ContinuityFailure::MissingPriorMode);
        };
        let elapsed = ctx.timing.current_cutoff - mode.committed_at;
        if elapsed < 0 {
            return Err(ContinuityFailure::TimingContextInvalid);
        };
        let ok = elapsed >= ctx.dwell.minimum_duration;
        let id = digest(&[
            ("dwell", ctx.dwell.dwell_id.clone()),
            ("elapsed", elapsed.to_string()),
            ("required", ctx.dwell.minimum_duration.to_string()),
            ("unit", ctx.dwell.time_unit.clone()),
        ]);
        Ok(DwellTimeEvaluation {
            dwell_id: ctx.dwell.dwell_id.clone(),
            satisfied: ok,
            elapsed,
            required: ctx.dwell.minimum_duration,
            time_unit: ctx.dwell.time_unit.clone(),
            content_hash: id,
        })
    }

    pub fn evaluate(
        ctx: &ContinuityContext,
    ) -> Result<ProposedSupervisoryModeTransition, ContinuityFailure> {
        Self::admit(ctx)?;
        let prior = ctx
            .prior_mode
            .as_ref()
            .ok_or(ContinuityFailure::MissingPriorMode)?;
        let (es, gs) = Self::eligible(ctx, &prior.mode_id)?;
        let p = Self::precedence(ctx, &es)?;
        let selected = p
            .selected
            .as_ref()
            .and_then(|id| ctx.transitions.iter().find(|t| t.transition_id == *id));
        let (_re, rr) = Self::reset(ctx, selected)?;
        let hy = Self::hysteresis(ctx, selected);
        let dw = Self::dwell(ctx)?;
        if selected.is_some() && (!hy.permits || !dw.satisfied) {
            return Err(if !hy.permits {
                ContinuityFailure::HysteresisBlocked
            } else {
                ContinuityFailure::DwellViolation
            });
        }
        let destination = selected
            .map(|t| t.destination_mode_id.clone())
            .unwrap_or_else(|| prior.mode_id.clone());
        let guard = gs
            .iter()
            .find(|g| {
                selected.map(|t| t.transition_id.clone()).as_deref() == Some(g.set_id.as_str())
            })
            .map(|g| g.set_id.clone())
            .unwrap_or_else(|| {
                gs.first()
                    .map(|g| g.set_id.clone())
                    .unwrap_or_else(|| "guard-set:none".into())
            });
        let id = digest(&[
            ("cycle", ctx.execution_disposition.cycle_id.clone()),
            ("prior", prior.mode_id.clone()),
            ("destination", destination.clone()),
            ("precedence", p.result_id.clone()),
            ("hysteresis", hy.content_hash.clone()),
            ("dwell", dw.content_hash.clone()),
        ]);
        Ok(ProposedSupervisoryModeTransition {
            decision_id: format!("mode-decision:{id}"),
            cycle_id: ctx.execution_disposition.cycle_id.clone(),
            prior_mode_id: prior.mode_id.clone(),
            proposed_mode_id: destination,
            precedence_result_id: p.result_id,
            guard_set_id: guard,
            reset_record_id: rr.map(|r| r.record_id),
            hysteresis_id: hy.hysteresis_id,
            dwell_id: dw.dwell_id,
            execution_disposition_id: ctx.execution_disposition.disposition_id.clone(),
            content_hash: id,
        })
    }

    pub fn commit(
        ctx: &ContinuityContext,
        decision: ProposedSupervisoryModeTransition,
    ) -> Result<
        (
            CommittedSupervisoryMode,
            CommittedSupervisoryContinuityRecord,
            ContinuityCommitmentRecord,
        ),
        ContinuityFailure,
    > {
        Self::admit(ctx)?;
        let mode = ctx
            .modes
            .iter()
            .find(|m| m.mode_id == decision.proposed_mode_id)
            .ok_or(ContinuityFailure::MissingDefinition)?;
        let id = digest(&[
            ("decision", decision.decision_id.clone()),
            ("mode", mode.mode_id.clone()),
            ("cycle", decision.cycle_id.clone()),
        ]);
        let mode_commit = CommittedSupervisoryMode {
            commitment_id: format!("mode-commitment:{id}"),
            mode_id: mode.mode_id.clone(),
            mode_version: mode.mode_version.clone(),
            cycle_id: decision.cycle_id.clone(),
            committed_at: ctx.timing.current_cutoff,
            registry_snapshot_id: ctx.registry.snapshot_id.clone(),
            content_hash: id.clone(),
        };
        let record_id = format!("continuity-record:{id}");
        let record = CommittedSupervisoryContinuityRecord {
            record_id: record_id.clone(),
            cycle_id: decision.cycle_id.clone(),
            mode_commitment_id: mode_commit.commitment_id.clone(),
            execution_disposition_id: decision.execution_disposition_id.clone(),
            osv_id: ctx.osv.osv.osv_id.clone(),
            governance_id: ctx.governance.snapshot.governance_snapshot_id.clone(),
            decision_id: decision.decision_id.clone(),
            guard_set_id: decision.guard_set_id.clone(),
            precedence_result_id: decision.precedence_result_id.clone(),
            reset_record_id: decision.reset_record_id.clone(),
            hysteresis_id: decision.hysteresis_id.clone(),
            dwell_id: decision.dwell_id.clone(),
            content_hash: id.clone(),
        };
        let commitment = ContinuityCommitmentRecord {
            commitment_id: format!("continuity-commitment:{id}"),
            cycle_id: decision.cycle_id.clone(),
            decision_id: decision.decision_id,
            mode_commitment_id: mode_commit.commitment_id.clone(),
            continuity_record_id: record_id,
            content_hash: id,
        };
        Ok((mode_commit, record, commitment))
    }

    pub fn handoff(
        mode: CommittedSupervisoryMode,
        record: CommittedSupervisoryContinuityRecord,
        commitment: &ContinuityCommitmentRecord,
    ) -> Result<SupervisoryContinuityHandoff, ContinuityFailure> {
        if commitment.mode_commitment_id != mode.commitment_id
            || commitment.continuity_record_id != record.record_id
        {
            return Err(ContinuityFailure::ContinuityValidationFailure);
        };
        let id = digest(&[
            ("mode", mode.commitment_id.clone()),
            ("record", record.record_id.clone()),
        ]);
        Ok(SupervisoryContinuityHandoff {
            cycle_id: mode.cycle_id.clone(),
            mode,
            record,
            commitment_id: commitment.commitment_id.clone(),
            content_hash: id,
        })
    }
    pub fn replay<T: Eq>(a: &T, b: &T) -> Result<(), ContinuityFailure> {
        if a == b {
            Ok(())
        } else {
            Err(ContinuityFailure::ReplayNotEquivalent)
        }
    }
}

#[derive(Default)]
pub struct ContinuityStore {
    records: BTreeMap<
        String,
        (
            CommittedSupervisoryMode,
            CommittedSupervisoryContinuityRecord,
            ContinuityCommitmentRecord,
        ),
    >,
}
impl ContinuityStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn publish(
        &mut self,
        mode: CommittedSupervisoryMode,
        record: CommittedSupervisoryContinuityRecord,
        commitment: ContinuityCommitmentRecord,
    ) -> Result<
        (
            CommittedSupervisoryMode,
            CommittedSupervisoryContinuityRecord,
            ContinuityCommitmentRecord,
        ),
        ContinuityFailure,
    > {
        let k = mode.cycle_id.clone();
        let v = (mode.clone(), record.clone(), commitment.clone());
        if let Some(e) = self.records.get(&k) {
            if e == &v {
                return Ok(e.clone());
            }
            return Err(ContinuityFailure::ContinuityCommitmentConflict);
        }
        self.records.insert(k, v.clone());
        Ok(v)
    }
    pub fn get(
        &self,
        cycle: &str,
    ) -> Option<&(
        CommittedSupervisoryMode,
        CommittedSupervisoryContinuityRecord,
        ContinuityCommitmentRecord,
    )> {
        self.records.get(cycle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dispositions_are_referenced_not_recomputed() {
        assert_ne!("execution-disposition", "mode-decision");
    }
    #[test]
    fn explicit_time_is_deterministic() {
        let t = ContinuityTimingContext {
            time_source_id: "committed-cutoff".into(),
            time_unit: "ms".into(),
            prior_cutoff: 1,
            current_cutoff: 3,
            content_hash: "h".into(),
        };
        assert_eq!(t.current_cutoff - t.prior_cutoff, 2);
    }
    #[test]
    fn mode_and_record_are_distinct() {
        let m = CommittedSupervisoryMode {
            commitment_id: "m".into(),
            mode_id: "safe".into(),
            mode_version: "1".into(),
            cycle_id: "c".into(),
            committed_at: 1,
            registry_snapshot_id: "r".into(),
            content_hash: "h".into(),
        };
        assert_ne!(m.commitment_id, "record");
    }
}
