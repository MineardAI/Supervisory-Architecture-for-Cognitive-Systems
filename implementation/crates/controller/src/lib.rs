//! Contract 007 controller boundary.
//!
//! This is a bounded reference realization.  It consumes committed upstream
//! handoffs, evaluates an explicit integer proportional composition, validates
//! the proposed artifacts, and publishes one atomic canonical bundle.  It is
//! not translation, execution, continuity, or external verification.

use std::collections::{BTreeMap, BTreeSet};

use canonical_hashing::content_hash;
use canonical_serialization::CanonicalDocument;
use sacs_governance::CommittedGovernanceHandoff;
use sacs_operational_profiles::ControllerConfigurationHandoff;
use sacs_operational_state_vector::CommittedOsvHandoff;
use sacs_supervisory_objectives::CommittedObjectiveSetHandoff;

fn hash(parts: &[(&str, String)]) -> String {
    let mut d = CanonicalDocument::new();
    for (k, v) in parts {
        d.insert(*k, v);
    }
    content_hash(&d.encode())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpstreamContext {
    pub osv: CommittedOsvHandoff,
    pub governance: CommittedGovernanceHandoff,
    pub objectives: CommittedObjectiveSetHandoff,
    pub configuration: ControllerConfigurationHandoff,
}

impl UpstreamContext {
    pub fn validate(&self) -> Result<(), ControllerFailure> {
        let c = &self.osv.osv.cycle_id;
        if self.governance.snapshot.cycle_id != *c
            || self.objectives.objective_set.cycle_id != *c
            || self.configuration.profile.cycle_id != *c
            || self.configuration.reference_state.cycle_id != *c
        {
            return Err(ControllerFailure::CycleBindingMismatch);
        }
        if self.configuration.profile.osv_id != self.osv.osv.osv_id
            || self.configuration.reference_state.osv_id != self.osv.osv.osv_id
            || self.configuration.profile.governance_snapshot_id
                != self.governance.snapshot.governance_snapshot_id
            || self.configuration.reference_state.governance_snapshot_id
                != self.governance.snapshot.governance_snapshot_id
            || self.configuration.profile.objective_set_id
                != self.objectives.objective_set.objective_set_id
            || self.configuration.reference_state.objective_set_id
                != self.objectives.objective_set.objective_set_id
        {
            return Err(ControllerFailure::LineageMismatch);
        }
        if self.configuration.profile.numeric_profile_id
            != self.configuration.reference_state.numeric_profile_id
            || self.configuration.profile.numeric_profile_id.is_empty()
        {
            return Err(ControllerFailure::NumericProfileMismatch);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControllerFailure {
    CycleBindingMismatch,
    LineageMismatch,
    NumericProfileMismatch,
    InvalidInput(String),
    DefinitionHashMismatch,
    ConfigurationUnavailable,
    ConfigurationMismatch,
    StateMissing,
    StatePartitionMissing(String),
    StateTransitionInvalid(String),
    ErrorEvaluationFailure(String),
    ConstraintBindingFailure(String),
    AuthorityExpansionDetected,
    CompositionCompatibilityFailure(String),
    ControllerEvaluationFailure(String),
    CommandValidationFailure(String),
    AuthorityCertificationFailure(String),
    CommitmentFailure(String),
    CommitmentConflict,
    ReplayNotEquivalent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryClass {
    Stateless,
    Stateful,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerDefinition {
    pub definition_id: String,
    pub version: String,
    pub schema_version: String,
    pub registry_version: String,
    pub memory_class: MemoryClass,
    pub error_metric_id: String,
    pub numeric_profile_id: String,
    pub implementation_id: String,
    pub content_hash: String,
    pub provenance_id: String,
}
impl ControllerDefinition {
    pub fn calculated_hash(&self) -> String {
        hash(&[
            ("id", self.definition_id.clone()),
            ("version", self.version.clone()),
            ("metric", self.error_metric_id.clone()),
            ("numeric", self.numeric_profile_id.clone()),
            ("implementation", self.implementation_id.clone()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerStateDefinition {
    pub definition_id: String,
    pub version: String,
    pub owner: String,
    pub schema_version: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerStatePartition {
    pub partition_id: String,
    pub owner: String,
    pub definition_id: String,
    pub cycle_id: String,
    pub value: i64,
    pub lifecycle: StateLifecycle,
    pub prior_state_id: Option<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StateLifecycle {
    Stateless,
    Initialized,
    Active,
    Retained,
    Reset,
    Migrated,
    Faulted,
    Retired,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedControllerStateSet {
    pub state_set_id: String,
    pub cycle_id: String,
    pub composition_id: String,
    pub partitions: Vec<ControllerStatePartition>,
    pub numeric_profile_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatelessControllerInputDisposition {
    pub cycle_id: String,
    pub policy_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControllerStateInput {
    Committed(CommittedControllerStateSet),
    Stateless(StatelessControllerInputDisposition),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposedControllerStateOutcome {
    Next(CommittedControllerStateSet),
    Stateless {
        cycle_id: String,
        policy_id: String,
        content_hash: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorRepresentationDefinition {
    pub definition_id: String,
    pub version: String,
    pub dimensions: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorMetricDefinition {
    pub metric_id: String,
    pub version: String,
    pub representation_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedControlError {
    pub error_id: String,
    pub cycle_id: String,
    pub osv_id: String,
    pub reference_state_id: String,
    pub configuration_id: String,
    pub metric_id: String,
    pub representation_id: String,
    pub dimensions: Vec<String>,
    pub values: Vec<i64>,
    pub numeric_profile_id: String,
    pub content_hash: String,
}
impl CommittedControlError {
    pub fn canonical_bytes(&self) -> Vec<u8> {
        format!(
            "{}|{}|{}|{}|{:?}",
            self.error_id, self.cycle_id, self.metric_id, self.representation_id, self.values
        )
        .into_bytes()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlConstraintDefinition {
    pub definition_id: String,
    pub version: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionSpaceDefinition {
    pub action_space_id: String,
    pub dimensions: Vec<String>,
    pub lower: Vec<i64>,
    pub upper: Vec<i64>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedControlConstraintSet {
    pub constraint_set_id: String,
    pub cycle_id: String,
    pub governance_id: String,
    pub profile_id: String,
    pub configuration_id: String,
    pub action_space: ActionSpaceDefinition,
    pub mandatory: BTreeSet<String>,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedControllerConfiguration {
    pub configuration_id: String,
    pub cycle_id: String,
    pub definition: ControllerDefinition,
    pub error_metric: ErrorMetricDefinition,
    pub error_representation: ErrorRepresentationDefinition,
    pub action_space: ActionSpaceDefinition,
    pub gain: i64,
    pub deterministic_order: Vec<String>,
    pub numeric_profile_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedValidatedControllerComposition {
    pub composition_id: String,
    pub cycle_id: String,
    pub configuration_id: String,
    pub state_set_id: String,
    pub error_id: String,
    pub constraint_set_id: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedControlCommand {
    pub command_id: String,
    pub cycle_id: String,
    pub composition_id: String,
    pub error_id: String,
    pub constraint_set_id: String,
    pub dimensions: Vec<String>,
    pub values: Vec<i64>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerEvaluationResult {
    pub result_id: String,
    pub cycle_id: String,
    pub composition_id: String,
    pub command: ProposedControlCommand,
    pub state_outcome: ProposedControllerStateOutcome,
    pub evidence: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandValidationResult {
    pub validation_id: String,
    pub command_id: String,
    pub valid: bool,
    pub reasons: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerStateTransitionValidationResult {
    pub validation_id: String,
    pub valid: bool,
    pub reasons: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityCertificate {
    pub certificate_id: String,
    pub command_id: String,
    pub governance_id: String,
    pub profile_id: String,
    pub constraint_set_id: String,
    pub valid: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerCommitmentLineage {
    pub osv_id: String,
    pub governance_id: String,
    pub objective_set_id: String,
    pub profile_id: String,
    pub reference_state_id: String,
    pub configuration_id: String,
    pub error_id: String,
    pub constraint_set_id: String,
    pub composition_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerCommitmentRecord {
    pub commitment_id: String,
    pub cycle_id: String,
    pub command_id: String,
    pub state_outcome_id: String,
    pub certificate_id: String,
    pub status: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedControlCommand {
    pub command_id: String,
    pub cycle_id: String,
    pub commitment_id: String,
    pub composition_id: String,
    pub error_id: String,
    pub constraint_set_id: String,
    pub action_space_id: String,
    pub dimensions: Vec<String>,
    pub values: Vec<i64>,
    pub certificate_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerCommitmentBundle {
    pub command: CommittedControlCommand,
    pub state_outcome: ProposedControllerStateOutcome,
    pub certificate: AuthorityCertificate,
    pub command_validation: CommandValidationResult,
    pub state_validation: ControllerStateTransitionValidationResult,
    pub record: ControllerCommitmentRecord,
    pub lineage: ControllerCommitmentLineage,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedControllerCommitmentBundle {
    pub evaluation: ControllerEvaluationResult,
    pub command_validation: CommandValidationResult,
    pub state_validation: ControllerStateTransitionValidationResult,
    pub certificate: AuthorityCertificate,
    pub lineage: ControllerCommitmentLineage,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedControlCommandHandoff {
    pub command: CommittedControlCommand,
    pub commitment_id: String,
    pub action_space_id: String,
    pub numeric_profile_id: String,
    pub certificate_id: String,
    pub lineage: ControllerCommitmentLineage,
}

pub struct ControllerEvaluator;
impl ControllerEvaluator {
    pub fn control_error(
        ctx: &UpstreamContext,
        config: &CommittedControllerConfiguration,
        representation: &ErrorRepresentationDefinition,
        metric: &ErrorMetricDefinition,
    ) -> Result<CommittedControlError, ControllerFailure> {
        ctx.validate()?;
        if metric.representation_id != representation.definition_id
            || config.numeric_profile_id != ctx.configuration.profile.numeric_profile_id
        {
            return Err(ControllerFailure::ConfigurationMismatch);
        }
        let mut values = Vec::new();
        let mut dims = Vec::new();
        for c in &ctx.configuration.reference_state.components {
            if let Ok(target) = c.target_value.parse::<i64>() {
                if let Some(f) = ctx
                    .osv
                    .osv
                    .fields
                    .iter()
                    .find(|f| f.field_id == c.component_id)
                {
                    if let sacs_operational_state_vector::OsvFieldValue::Value(v) = &f.value {
                        if let Ok(v) = v.parse::<i64>() {
                            dims.push(c.component_id.clone());
                            values.push(v - target);
                        }
                    }
                }
            }
        }
        if values.is_empty() {
            return Err(ControllerFailure::ErrorEvaluationFailure(
                "no exact integer reference dimensions".into(),
            ));
        }
        let id = hash(&[
            ("cycle", ctx.osv.osv.cycle_id.clone()),
            ("osv", ctx.osv.osv.osv_id.clone()),
            (
                "reference",
                ctx.configuration.reference_state.reference_state_id.clone(),
            ),
            ("values", format!("{:?}", values)),
        ]);
        Ok(CommittedControlError {
            error_id: format!("control-error:{id}"),
            cycle_id: ctx.osv.osv.cycle_id.clone(),
            osv_id: ctx.osv.osv.osv_id.clone(),
            reference_state_id: ctx.configuration.reference_state.reference_state_id.clone(),
            configuration_id: config.configuration_id.clone(),
            metric_id: metric.metric_id.clone(),
            representation_id: representation.definition_id.clone(),
            dimensions: dims,
            values,
            numeric_profile_id: config.numeric_profile_id.clone(),
            content_hash: id,
        })
    }

    pub fn bind_constraints(
        ctx: &UpstreamContext,
        config: &CommittedControllerConfiguration,
    ) -> Result<CommittedControlConstraintSet, ControllerFailure> {
        ctx.validate()?;
        let mut mandatory = BTreeSet::new();
        for p in &ctx.configuration.profile.constraints {
            mandatory.insert(p.clone());
        }
        if config.action_space.dimensions.is_empty()
            || config.action_space.dimensions.len() != config.action_space.lower.len()
            || config.action_space.dimensions.len() != config.action_space.upper.len()
        {
            return Err(ControllerFailure::ConstraintBindingFailure(
                "incomplete action space".into(),
            ));
        }
        let id = hash(&[
            ("cycle", ctx.osv.osv.cycle_id.clone()),
            (
                "governance",
                ctx.governance.snapshot.governance_snapshot_id.clone(),
            ),
            ("profile", ctx.configuration.profile.profile_id.clone()),
            ("action", config.action_space.action_space_id.clone()),
            ("mandatory", format!("{:?}", mandatory)),
        ]);
        Ok(CommittedControlConstraintSet {
            constraint_set_id: format!("constraints:{id}"),
            cycle_id: ctx.osv.osv.cycle_id.clone(),
            governance_id: ctx.governance.snapshot.governance_snapshot_id.clone(),
            profile_id: ctx.configuration.profile.profile_id.clone(),
            configuration_id: config.configuration_id.clone(),
            action_space: config.action_space.clone(),
            mandatory,
            content_hash: id,
        })
    }

    pub fn bind_composition(
        ctx: &UpstreamContext,
        config: &CommittedControllerConfiguration,
        state: &ControllerStateInput,
        error: &CommittedControlError,
        constraints: &CommittedControlConstraintSet,
    ) -> Result<CommittedValidatedControllerComposition, ControllerFailure> {
        ctx.validate()?;
        if error.cycle_id != config.cycle_id || constraints.cycle_id != config.cycle_id {
            return Err(ControllerFailure::CycleBindingMismatch);
        }
        let sid = match state {
            ControllerStateInput::Committed(s) => {
                if s.cycle_id != config.cycle_id {
                    return Err(ControllerFailure::StateMissing);
                };
                s.state_set_id.clone()
            }
            ControllerStateInput::Stateless(s) => {
                if s.cycle_id != config.cycle_id {
                    return Err(ControllerFailure::StateMissing);
                };
                format!("stateless:{}", s.policy_id)
            }
        };
        let id = hash(&[
            ("cycle", config.cycle_id.clone()),
            ("configuration", config.configuration_id.clone()),
            ("state", sid.clone()),
            ("error", error.error_id.clone()),
            ("constraints", constraints.constraint_set_id.clone()),
        ]);
        Ok(CommittedValidatedControllerComposition {
            composition_id: format!("composition:{id}"),
            cycle_id: config.cycle_id.clone(),
            configuration_id: config.configuration_id.clone(),
            state_set_id: sid,
            error_id: error.error_id.clone(),
            constraint_set_id: constraints.constraint_set_id.clone(),
            content_hash: id,
        })
    }

    pub fn evaluate(
        composition: &CommittedValidatedControllerComposition,
        error: &CommittedControlError,
        constraints: &CommittedControlConstraintSet,
        config: &CommittedControllerConfiguration,
        state: &ControllerStateInput,
    ) -> Result<ControllerEvaluationResult, ControllerFailure> {
        if composition.error_id != error.error_id
            || composition.constraint_set_id != constraints.constraint_set_id
        {
            return Err(ControllerFailure::ControllerEvaluationFailure(
                "binding mismatch".into(),
            ));
        }
        let values = error
            .values
            .iter()
            .map(|v| v.saturating_mul(config.gain))
            .enumerate()
            .map(|(i, v)| {
                v.clamp(
                    constraints.action_space.lower[i],
                    constraints.action_space.upper[i],
                )
            })
            .collect::<Vec<_>>();
        let id = hash(&[
            ("composition", composition.composition_id.clone()),
            ("values", format!("{:?}", values)),
        ]);
        let command = ProposedControlCommand {
            command_id: format!("proposed-command:{id}"),
            cycle_id: composition.cycle_id.clone(),
            composition_id: composition.composition_id.clone(),
            error_id: error.error_id.clone(),
            constraint_set_id: constraints.constraint_set_id.clone(),
            dimensions: constraints.action_space.dimensions.clone(),
            values,
            content_hash: id.clone(),
        };
        let outcome = match state {
            ControllerStateInput::Stateless(s) => ProposedControllerStateOutcome::Stateless {
                cycle_id: s.cycle_id.clone(),
                policy_id: s.policy_id.clone(),
                content_hash: hash(&[
                    ("cycle", s.cycle_id.clone()),
                    ("policy", s.policy_id.clone()),
                ]),
            },
            ControllerStateInput::Committed(s) => ProposedControllerStateOutcome::Next(s.clone()),
        };
        Ok(ControllerEvaluationResult {
            result_id: format!("evaluation:{id}"),
            cycle_id: composition.cycle_id.clone(),
            composition_id: composition.composition_id.clone(),
            command,
            state_outcome: outcome,
            evidence: vec!["integer-proportional-reference-controller".into()],
            content_hash: id,
        })
    }

    pub fn validate_command(
        command: &ProposedControlCommand,
        constraints: &CommittedControlConstraintSet,
    ) -> CommandValidationResult {
        let mut reasons = Vec::new();
        if command.dimensions != constraints.action_space.dimensions {
            reasons.push("dimension mismatch".into());
        }
        if command.values.len() != constraints.action_space.lower.len() {
            reasons.push("value dimension mismatch".into());
        }
        for (i, value) in command.values.iter().enumerate() {
            if let (Some(lo), Some(hi)) = (
                constraints.action_space.lower.get(i),
                constraints.action_space.upper.get(i),
            ) {
                if value < lo || value > hi {
                    reasons.push(format!("value {i} outside action space"));
                }
            }
        }
        let valid = reasons.is_empty();
        let content_hash = hash(&[
            ("command", command.command_id.clone()),
            ("valid", valid.to_string()),
            ("reasons", format!("{:?}", reasons)),
        ]);
        CommandValidationResult {
            validation_id: format!("command-validation:{content_hash}"),
            command_id: command.command_id.clone(),
            valid,
            reasons,
            content_hash,
        }
    }

    pub fn validate_state(
        state: &ControllerStateInput,
        cycle_id: &str,
        definition: &ControllerDefinition,
    ) -> ControllerStateTransitionValidationResult {
        let mut reasons = Vec::new();
        match state {
            ControllerStateInput::Stateless(s) => {
                if s.cycle_id != cycle_id {
                    reasons.push("stateless cycle mismatch".into());
                }
            }
            ControllerStateInput::Committed(s) => {
                if s.cycle_id != cycle_id {
                    reasons.push("state cycle mismatch".into());
                }
                if s.numeric_profile_id != definition.numeric_profile_id {
                    reasons.push("state numeric profile mismatch".into());
                }
                if s.partitions.is_empty() {
                    reasons.push("missing state partition".into());
                }
            }
        }
        let valid = reasons.is_empty();
        let content_hash = hash(&[
            ("cycle", cycle_id.into()),
            ("valid", valid.to_string()),
            ("reasons", format!("{:?}", reasons)),
        ]);
        ControllerStateTransitionValidationResult {
            validation_id: format!("state-validation:{content_hash}"),
            valid,
            reasons,
            content_hash,
        }
    }

    pub fn certify(
        ctx: &UpstreamContext,
        evaluation: &ControllerEvaluationResult,
        constraints: &CommittedControlConstraintSet,
        validation: &CommandValidationResult,
    ) -> Result<AuthorityCertificate, ControllerFailure> {
        if !validation.valid {
            return Err(ControllerFailure::AuthorityCertificationFailure(
                "command validation failed".into(),
            ));
        }
        ctx.validate()?;
        let id = hash(&[
            ("command", evaluation.command.command_id.clone()),
            ("constraints", constraints.constraint_set_id.clone()),
            (
                "governance",
                ctx.governance.snapshot.governance_snapshot_id.clone(),
            ),
        ]);
        Ok(AuthorityCertificate {
            certificate_id: format!("authority-certificate:{id}"),
            command_id: evaluation.command.command_id.clone(),
            governance_id: ctx.governance.snapshot.governance_snapshot_id.clone(),
            profile_id: ctx.configuration.profile.profile_id.clone(),
            constraint_set_id: constraints.constraint_set_id.clone(),
            valid: true,
            content_hash: id,
        })
    }

    pub fn prepare(
        ctx: &UpstreamContext,
        evaluation: ControllerEvaluationResult,
        constraints: &CommittedControlConstraintSet,
        config: &CommittedControllerConfiguration,
        state: &ControllerStateInput,
    ) -> Result<PreparedControllerCommitmentBundle, ControllerFailure> {
        let cv = Self::validate_command(&evaluation.command, constraints);
        let sv = Self::validate_state(state, &config.cycle_id, &config.definition);
        if !sv.valid {
            return Err(ControllerFailure::StateTransitionInvalid(
                sv.reasons.join(", "),
            ));
        }
        let cert = Self::certify(ctx, &evaluation, constraints, &cv)?;
        let lineage = ControllerCommitmentLineage {
            osv_id: ctx.osv.osv.osv_id.clone(),
            governance_id: ctx.governance.snapshot.governance_snapshot_id.clone(),
            objective_set_id: ctx.objectives.objective_set.objective_set_id.clone(),
            profile_id: ctx.configuration.profile.profile_id.clone(),
            reference_state_id: ctx.configuration.reference_state.reference_state_id.clone(),
            configuration_id: config.configuration_id.clone(),
            error_id: evaluation.command.error_id.clone(),
            constraint_set_id: constraints.constraint_set_id.clone(),
            composition_id: evaluation.composition_id.clone(),
            content_hash: String::new(),
        };
        let mut lineage = lineage;
        lineage.content_hash = hash(&[
            ("osv", lineage.osv_id.clone()),
            ("command", evaluation.command.command_id.clone()),
            ("composition", lineage.composition_id.clone()),
        ]);
        Ok(PreparedControllerCommitmentBundle {
            evaluation,
            command_validation: cv,
            state_validation: sv,
            certificate: cert,
            lineage,
        })
    }

    pub fn commit(
        prepared: PreparedControllerCommitmentBundle,
    ) -> Result<ControllerCommitmentBundle, ControllerFailure> {
        if !prepared.command_validation.valid
            || !prepared.state_validation.valid
            || !prepared.certificate.valid
        {
            return Err(ControllerFailure::CommitmentFailure(
                "incomplete prepared bundle".into(),
            ));
        }
        let commitment_id = hash(&[
            ("cycle", prepared.evaluation.cycle_id.clone()),
            ("command", prepared.evaluation.command.command_id.clone()),
            ("lineage", prepared.lineage.content_hash.clone()),
        ]);
        let record = ControllerCommitmentRecord {
            commitment_id: format!("controller-commitment:{commitment_id}"),
            cycle_id: prepared.evaluation.cycle_id.clone(),
            command_id: prepared.evaluation.command.command_id.clone(),
            state_outcome_id: hash(&[(
                "outcome",
                format!("{:?}", prepared.evaluation.state_outcome),
            )]),
            certificate_id: prepared.certificate.certificate_id.clone(),
            status: "COMMITTED".into(),
            content_hash: commitment_id.clone(),
        };
        let command = CommittedControlCommand {
            command_id: format!("committed:{}", prepared.evaluation.command.command_id),
            cycle_id: prepared.evaluation.cycle_id.clone(),
            commitment_id: record.commitment_id.clone(),
            composition_id: prepared.evaluation.composition_id.clone(),
            error_id: prepared.evaluation.command.error_id.clone(),
            constraint_set_id: prepared.evaluation.command.constraint_set_id.clone(),
            action_space_id: prepared.lineage.constraint_set_id.clone(),
            dimensions: prepared.evaluation.command.dimensions.clone(),
            values: prepared.evaluation.command.values.clone(),
            certificate_id: prepared.certificate.certificate_id.clone(),
            content_hash: commitment_id,
        };
        Ok(ControllerCommitmentBundle {
            command,
            state_outcome: prepared.evaluation.state_outcome,
            certificate: prepared.certificate,
            command_validation: prepared.command_validation,
            state_validation: prepared.state_validation,
            record,
            lineage: prepared.lineage,
        })
    }

    pub fn handoff(
        bundle: &ControllerCommitmentBundle,
        numeric_profile_id: String,
    ) -> CommittedControlCommandHandoff {
        CommittedControlCommandHandoff {
            command: bundle.command.clone(),
            commitment_id: bundle.record.commitment_id.clone(),
            action_space_id: bundle.command.action_space_id.clone(),
            numeric_profile_id,
            certificate_id: bundle.certificate.certificate_id.clone(),
            lineage: bundle.lineage.clone(),
        }
    }

    pub fn replay(
        original: &ControllerCommitmentBundle,
        replayed: &ControllerCommitmentBundle,
    ) -> Result<(), ControllerFailure> {
        if original.command == replayed.command
            && original.state_outcome == replayed.state_outcome
            && original.lineage == replayed.lineage
        {
            Ok(())
        } else {
            Err(ControllerFailure::ReplayNotEquivalent)
        }
    }
}

#[derive(Default)]
pub struct ControllerStore {
    bundles: BTreeMap<String, ControllerCommitmentBundle>,
}
impl ControllerStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn publish(
        &mut self,
        bundle: ControllerCommitmentBundle,
    ) -> Result<ControllerCommitmentBundle, ControllerFailure> {
        let key = bundle.record.cycle_id.clone();
        if let Some(existing) = self.bundles.get(&key) {
            if existing == &bundle {
                return Ok(existing.clone());
            }
            return Err(ControllerFailure::CommitmentConflict);
        }
        self.bundles.insert(key, bundle.clone());
        Ok(bundle)
    }
    pub fn get(&self, cycle_id: &str) -> Option<&ControllerCommitmentBundle> {
        self.bundles.get(cycle_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn definition_hash_is_stable() {
        let d = ControllerDefinition {
            definition_id: "d".into(),
            version: "1".into(),
            schema_version: "1".into(),
            registry_version: "1".into(),
            memory_class: MemoryClass::Stateless,
            error_metric_id: "m".into(),
            numeric_profile_id: "n".into(),
            implementation_id: "reference-proportional".into(),
            content_hash: String::new(),
            provenance_id: "p".into(),
        };
        assert_eq!(d.calculated_hash(), d.calculated_hash());
    }
    #[test]
    fn stateless_state_is_explicit() {
        let s = ControllerStateInput::Stateless(StatelessControllerInputDisposition {
            cycle_id: "c".into(),
            policy_id: "explicit-stateless".into(),
            content_hash: "h".into(),
        });
        let d = ControllerDefinition {
            definition_id: "d".into(),
            version: "1".into(),
            schema_version: "1".into(),
            registry_version: "1".into(),
            memory_class: MemoryClass::Stateless,
            error_metric_id: "m".into(),
            numeric_profile_id: "n".into(),
            implementation_id: "i".into(),
            content_hash: "h".into(),
            provenance_id: "p".into(),
        };
        assert!(ControllerEvaluator::validate_state(&s, "c", &d).valid);
    }
    #[test]
    fn bounded_command_validation_rejects_expansion() {
        let c = CommittedControlConstraintSet {
            constraint_set_id: "c".into(),
            cycle_id: "x".into(),
            governance_id: "g".into(),
            profile_id: "p".into(),
            configuration_id: "q".into(),
            action_space: ActionSpaceDefinition {
                action_space_id: "a".into(),
                dimensions: vec!["u".into()],
                lower: vec![-1],
                upper: vec![1],
                content_hash: "h".into(),
            },
            mandatory: BTreeSet::new(),
            content_hash: "h".into(),
        };
        let p = ProposedControlCommand {
            command_id: "p".into(),
            cycle_id: "x".into(),
            composition_id: "q".into(),
            error_id: "e".into(),
            constraint_set_id: "c".into(),
            dimensions: vec!["u".into()],
            values: vec![2],
            content_hash: "h".into(),
        };
        assert!(!ControllerEvaluator::validate_command(&p, &c).valid);
    }
}
