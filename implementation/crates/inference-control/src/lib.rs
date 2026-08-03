//! Contract 008 semantic-preserving inference-control translation boundary.
//!
//! This crate translates one committed Contract 007 command into one explicit
//! runtime representation. It never calls a runtime, selects a provider,
//! creates an execution plan, or decides execution disposition.

use std::collections::BTreeMap;

use canonical_hashing::content_hash;
use canonical_serialization::CanonicalDocument;
use sacs_controller::CommittedControlCommandHandoff;

fn digest(parts: &[(&str, String)]) -> String {
    let mut d = CanonicalDocument::new();
    for (key, value) in parts {
        d.insert(*key, value);
    }
    content_hash(&d.encode())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeAvailability {
    Unknown,
    Available,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeCapabilityProfile {
    pub runtime_id: String,
    pub runtime_family: String,
    pub capability_profile_id: String,
    pub version: String,
    pub supported_command_type: String,
    pub dimensions: Vec<String>,
    pub units: Vec<String>,
    pub lower: Vec<i64>,
    pub upper: Vec<i64>,
    pub precision: u32,
    pub ordering_supported: bool,
    pub atomicity_supported: bool,
    pub restriction_preservation_supported: bool,
    pub availability: RuntimeAvailability,
    pub schema_version: String,
    pub registry_version: String,
    pub content_hash: String,
    pub provenance_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeCapabilityRegistryEntry {
    pub entry_id: String,
    pub profile: RuntimeCapabilityProfile,
    pub entry_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeCapabilityRegistrySnapshot {
    pub snapshot_id: String,
    pub runtime_id: String,
    pub entries: Vec<String>,
    pub registry_version: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationDefinition {
    pub definition_id: String,
    pub version: String,
    pub kind: String,
    pub content_hash: String,
    pub provenance_id: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationDefinitionRegistryEntry {
    pub entry_id: String,
    pub definition: TranslationDefinition,
    pub entry_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationDefinitionRegistrySnapshot {
    pub snapshot_id: String,
    pub entries: Vec<String>,
    pub registry_version: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizationRule {
    pub rule_id: String,
    pub version: String,
    pub field_name_policy: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRepresentationProfile {
    pub profile_id: String,
    pub version: String,
    pub representation: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationNumericRule {
    pub rule_id: String,
    pub version: String,
    pub scale_numerator: i64,
    pub scale_denominator: i64,
    pub precision: u32,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationOrderingRule {
    pub rule_id: String,
    pub version: String,
    pub dimension_order: Vec<String>,
    pub atomic_groups: Vec<Vec<String>>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationValidationRule {
    pub rule_id: String,
    pub version: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationTransactionRule {
    pub rule_id: String,
    pub version: String,
    pub atomic: bool,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationProfile {
    pub profile_id: String,
    pub version: String,
    pub runtime_family: String,
    pub command_semantics: String,
    pub required_definition_ids: Vec<String>,
    pub normalization_rule_id: String,
    pub representation_profile_id: String,
    pub numeric_rule_id: String,
    pub ordering_rule_id: String,
    pub validation_rule_id: String,
    pub transaction_rule_id: String,
    pub content_hash: String,
    pub provenance_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedTranslationConfiguration {
    pub configuration_id: String,
    pub cycle_id: String,
    pub committed_command_id: String,
    pub target_runtime_id: String,
    pub translation_profile: TranslationProfile,
    pub definitions: Vec<TranslationDefinition>,
    pub normalization: NormalizationRule,
    pub representation: CanonicalRepresentationProfile,
    pub numeric: TranslationNumericRule,
    pub ordering: TranslationOrderingRule,
    pub validation: TranslationValidationRule,
    pub transaction: TranslationTransactionRule,
    pub capability_snapshot_id: String,
    pub definition_snapshot_id: String,
    pub schema_version: String,
    pub registry_version: String,
    pub replay_required: bool,
    pub content_hash: String,
    pub provenance_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationRequirementSet {
    pub requirement_id: String,
    pub cycle_id: String,
    pub command_id: String,
    pub dimensions: Vec<String>,
    pub values: Vec<i64>,
    pub lower: Vec<i64>,
    pub upper: Vec<i64>,
    pub mandatory_restrictions: Vec<String>,
    pub atomic: bool,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedTranslationBinding {
    pub binding_id: String,
    pub cycle_id: String,
    pub command_id: String,
    pub controller_commitment_id: String,
    pub configuration_id: String,
    pub runtime_id: String,
    pub capability_profile_id: String,
    pub requirement_id: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InferenceCommand {
    pub dimension: String,
    pub value: i64,
    pub unit: String,
    pub lower: i64,
    pub upper: i64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedInferenceCommandSet {
    pub set_id: String,
    pub cycle_id: String,
    pub source_command_id: String,
    pub binding_id: String,
    pub runtime_id: String,
    pub commands: Vec<InferenceCommand>,
    pub atomic_groups: Vec<Vec<String>>,
    pub numeric_rule_id: String,
    pub representation_profile_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationValidationReport {
    pub report_id: String,
    pub set_id: String,
    pub valid: bool,
    pub reasons: Vec<String>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedInferenceCommandSet {
    pub set_id: String,
    pub cycle_id: String,
    pub source_command_id: String,
    pub binding_id: String,
    pub validation_report_id: String,
    pub runtime_id: String,
    pub commands: Vec<InferenceCommand>,
    pub atomic_groups: Vec<Vec<String>>,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedTranslationTransaction {
    pub transaction_id: String,
    pub cycle_id: String,
    pub set_id: String,
    pub source_command_id: String,
    pub configuration_id: String,
    pub runtime_id: String,
    pub binding_id: String,
    pub validation_report_id: String,
    pub atomic: bool,
    pub ordering_rule_id: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationCommitmentRecord {
    pub commitment_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub set_id: String,
    pub configuration_id: String,
    pub runtime_id: String,
    pub status: String,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedTranslationTransaction {
    pub transaction_id: String,
    pub cycle_id: String,
    pub commitment_id: String,
    pub set_id: String,
    pub source_command_id: String,
    pub configuration_id: String,
    pub runtime_id: String,
    pub ordering_rule_id: String,
    pub atomic: bool,
    pub content_hash: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionInterfaceHandoffEnvelope {
    pub envelope_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub set_id: String,
    pub source_command_id: String,
    pub commitment_id: String,
    pub runtime_id: String,
    pub validation_report_id: String,
    pub atomic: bool,
    pub ordering_rule_id: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferenceTranslationFailure {
    InvalidControlCommandHandoff,
    ControllerCommitmentMismatch,
    TranslationConfigurationIncomplete,
    TranslationConfigurationMismatch,
    RuntimeCapabilityMismatch(String),
    RuntimeUnavailable,
    RequirementResolutionFailure(String),
    SemanticUnsupported,
    SemanticLossDetected,
    AuthorityExpansionDetected,
    MandatoryRestrictionLoss,
    DimensionUnsupported(String),
    NumericRangeUnrepresentable(String),
    PrecisionInsufficient,
    OrderingUnrepresentable,
    NormalizationFailure,
    TranslationValidationFailure(String),
    TranslationTransactionValidationFailure(String),
    TranslationCommitmentConflict,
    ReplayNotEquivalent,
}

pub struct TranslationEngine;
impl TranslationEngine {
    pub fn admit(
        handoff: &CommittedControlCommandHandoff,
    ) -> Result<(), InferenceTranslationFailure> {
        if handoff.command.command_id.is_empty()
            || handoff.commitment_id != handoff.command.commitment_id
            || handoff.certificate_id != handoff.command.certificate_id
        {
            return Err(InferenceTranslationFailure::InvalidControlCommandHandoff);
        }
        if handoff.command.dimensions.len() != handoff.command.values.len() {
            return Err(InferenceTranslationFailure::InvalidControlCommandHandoff);
        }
        Ok(())
    }

    pub fn requirements(
        handoff: &CommittedControlCommandHandoff,
        cycle_id: &str,
        config: &CommittedTranslationConfiguration,
    ) -> Result<TranslationRequirementSet, InferenceTranslationFailure> {
        Self::admit(handoff)?;
        if config.cycle_id != cycle_id || config.committed_command_id != handoff.command.command_id
        {
            return Err(InferenceTranslationFailure::TranslationConfigurationMismatch);
        }
        let id = digest(&[
            ("cycle", cycle_id.into()),
            ("command", handoff.command.command_id.clone()),
            ("dimensions", format!("{:?}", handoff.command.dimensions)),
            ("values", format!("{:?}", handoff.command.values)),
        ]);
        Ok(TranslationRequirementSet {
            requirement_id: format!("requirements:{id}"),
            cycle_id: cycle_id.into(),
            command_id: handoff.command.command_id.clone(),
            dimensions: handoff.command.dimensions.clone(),
            values: handoff.command.values.clone(),
            lower: vec![i64::MIN; handoff.command.values.len()],
            upper: vec![i64::MAX; handoff.command.values.len()],
            mandatory_restrictions: vec!["preserve-controller-authority".into()],
            atomic: true,
            content_hash: id,
        })
    }

    pub fn bind(
        handoff: &CommittedControlCommandHandoff,
        config: &CommittedTranslationConfiguration,
        capability: &RuntimeCapabilityProfile,
        requirement: &TranslationRequirementSet,
    ) -> Result<ValidatedTranslationBinding, InferenceTranslationFailure> {
        Self::admit(handoff)?;
        if config.target_runtime_id != capability.runtime_id
            || config.cycle_id != requirement.cycle_id
            || config.committed_command_id != handoff.command.command_id
            || capability.runtime_family != config.translation_profile.runtime_family
        {
            return Err(InferenceTranslationFailure::TranslationConfigurationMismatch);
        }
        if capability.dimensions != requirement.dimensions {
            return Err(InferenceTranslationFailure::DimensionUnsupported(
                "command dimensions".into(),
            ));
        }
        if !capability.ordering_supported {
            return Err(InferenceTranslationFailure::OrderingUnrepresentable);
        }
        if !capability.atomicity_supported {
            return Err(InferenceTranslationFailure::RuntimeCapabilityMismatch(
                "atomicity".into(),
            ));
        }
        let id = digest(&[
            ("command", handoff.command.command_id.clone()),
            ("configuration", config.configuration_id.clone()),
            ("runtime", capability.runtime_id.clone()),
            ("requirements", requirement.requirement_id.clone()),
        ]);
        Ok(ValidatedTranslationBinding {
            binding_id: format!("translation-binding:{id}"),
            cycle_id: requirement.cycle_id.clone(),
            command_id: handoff.command.command_id.clone(),
            controller_commitment_id: handoff.commitment_id.clone(),
            configuration_id: config.configuration_id.clone(),
            runtime_id: capability.runtime_id.clone(),
            capability_profile_id: capability.capability_profile_id.clone(),
            requirement_id: requirement.requirement_id.clone(),
            content_hash: id,
        })
    }

    pub fn translate(
        binding: &ValidatedTranslationBinding,
        handoff: &CommittedControlCommandHandoff,
        config: &CommittedTranslationConfiguration,
        capability: &RuntimeCapabilityProfile,
        requirement: &TranslationRequirementSet,
    ) -> Result<ProposedInferenceCommandSet, InferenceTranslationFailure> {
        if binding.command_id != handoff.command.command_id
            || binding.configuration_id != config.configuration_id
            || binding.runtime_id != capability.runtime_id
            || binding.requirement_id != requirement.requirement_id
        {
            return Err(InferenceTranslationFailure::TranslationConfigurationMismatch);
        }
        if config.numeric.scale_denominator == 0 {
            return Err(InferenceTranslationFailure::NormalizationFailure);
        }
        let mut commands = Vec::new();
        for (i, dimension) in handoff.command.dimensions.iter().enumerate() {
            let raw = handoff.command.values[i];
            let scaled = raw.checked_mul(config.numeric.scale_numerator).ok_or(
                InferenceTranslationFailure::NumericRangeUnrepresentable(dimension.clone()),
            )? / config.numeric.scale_denominator;
            if scaled < capability.lower[i] || scaled > capability.upper[i] {
                return Err(InferenceTranslationFailure::NumericRangeUnrepresentable(
                    dimension.clone(),
                ));
            }
            commands.push(InferenceCommand {
                dimension: dimension.clone(),
                value: scaled,
                unit: "unitless".into(),
                lower: capability.lower[i],
                upper: capability.upper[i],
            });
        }
        let order = commands
            .iter()
            .map(|c| c.dimension.clone())
            .collect::<Vec<_>>();
        if order != config.ordering.dimension_order {
            return Err(InferenceTranslationFailure::OrderingUnrepresentable);
        }
        let id = digest(&[
            ("binding", binding.binding_id.clone()),
            ("commands", format!("{:?}", commands)),
            ("representation", config.representation.profile_id.clone()),
        ]);
        Ok(ProposedInferenceCommandSet {
            set_id: format!("proposed-inference-set:{id}"),
            cycle_id: binding.cycle_id.clone(),
            source_command_id: handoff.command.command_id.clone(),
            binding_id: binding.binding_id.clone(),
            runtime_id: capability.runtime_id.clone(),
            commands,
            atomic_groups: config.ordering.atomic_groups.clone(),
            numeric_rule_id: config.numeric.rule_id.clone(),
            representation_profile_id: config.representation.profile_id.clone(),
            content_hash: id,
        })
    }

    pub fn validate(
        set: &ProposedInferenceCommandSet,
        binding: &ValidatedTranslationBinding,
        capability: &RuntimeCapabilityProfile,
    ) -> TranslationValidationReport {
        let mut reasons = Vec::new();
        if set.binding_id != binding.binding_id {
            reasons.push("binding mismatch".into());
        }
        if set.runtime_id != capability.runtime_id {
            reasons.push("runtime mismatch".into());
        }
        if !capability.restriction_preservation_supported {
            reasons.push("restriction preservation unsupported".into());
        }
        for (i, c) in set.commands.iter().enumerate() {
            if c.value < capability.lower[i] || c.value > capability.upper[i] {
                reasons.push(format!("command {i} outside capability"));
            }
        }
        let valid = reasons.is_empty();
        let id = digest(&[
            ("set", set.set_id.clone()),
            ("valid", valid.to_string()),
            ("reasons", format!("{:?}", reasons)),
        ]);
        TranslationValidationReport {
            report_id: format!("translation-validation:{id}"),
            set_id: set.set_id.clone(),
            valid,
            reasons,
            content_hash: id,
        }
    }

    pub fn commit_set(
        set: ProposedInferenceCommandSet,
        report: TranslationValidationReport,
    ) -> Result<CommittedInferenceCommandSet, InferenceTranslationFailure> {
        if !report.valid || report.set_id != set.set_id {
            return Err(InferenceTranslationFailure::TranslationValidationFailure(
                report.reasons.join(","),
            ));
        }
        Ok(CommittedInferenceCommandSet {
            set_id: set.set_id,
            cycle_id: set.cycle_id,
            source_command_id: set.source_command_id,
            binding_id: set.binding_id,
            validation_report_id: report.report_id,
            runtime_id: set.runtime_id,
            commands: set.commands,
            atomic_groups: set.atomic_groups,
            content_hash: set.content_hash,
        })
    }

    pub fn construct_transaction(
        set: &CommittedInferenceCommandSet,
        config: &CommittedTranslationConfiguration,
        binding: &ValidatedTranslationBinding,
        report: &TranslationValidationReport,
    ) -> Result<ConstructedTranslationTransaction, InferenceTranslationFailure> {
        if !report.valid || report.set_id != set.set_id || binding.binding_id != set.binding_id {
            return Err(
                InferenceTranslationFailure::TranslationTransactionValidationFailure(
                    "incomplete transaction context".into(),
                ),
            );
        }
        let id = digest(&[
            ("set", set.set_id.clone()),
            ("config", config.configuration_id.clone()),
            ("binding", binding.binding_id.clone()),
            ("runtime", set.runtime_id.clone()),
        ]);
        Ok(ConstructedTranslationTransaction {
            transaction_id: format!("constructed-translation:{id}"),
            cycle_id: set.cycle_id.clone(),
            set_id: set.set_id.clone(),
            source_command_id: set.source_command_id.clone(),
            configuration_id: config.configuration_id.clone(),
            runtime_id: set.runtime_id.clone(),
            binding_id: binding.binding_id.clone(),
            validation_report_id: report.report_id.clone(),
            atomic: config.transaction.atomic,
            ordering_rule_id: config.ordering.rule_id.clone(),
            content_hash: id,
        })
    }

    pub fn commit_transaction(
        tx: ConstructedTranslationTransaction,
    ) -> Result<
        (CommittedTranslationTransaction, TranslationCommitmentRecord),
        InferenceTranslationFailure,
    > {
        let id = digest(&[
            ("transaction", tx.transaction_id.clone()),
            ("set", tx.set_id.clone()),
            ("cycle", tx.cycle_id.clone()),
        ]);
        let record = TranslationCommitmentRecord {
            commitment_id: format!("translation-commitment:{id}"),
            cycle_id: tx.cycle_id.clone(),
            transaction_id: tx.transaction_id.clone(),
            set_id: tx.set_id.clone(),
            configuration_id: tx.configuration_id.clone(),
            runtime_id: tx.runtime_id.clone(),
            status: "COMMITTED".into(),
            content_hash: id.clone(),
        };
        Ok((
            CommittedTranslationTransaction {
                transaction_id: tx.transaction_id,
                cycle_id: tx.cycle_id,
                commitment_id: record.commitment_id.clone(),
                set_id: tx.set_id,
                source_command_id: tx.source_command_id,
                configuration_id: tx.configuration_id,
                runtime_id: tx.runtime_id,
                ordering_rule_id: tx.ordering_rule_id,
                atomic: tx.atomic,
                content_hash: id,
            },
            record,
        ))
    }

    pub fn envelope(
        transaction: &CommittedTranslationTransaction,
        set: &CommittedInferenceCommandSet,
        report: &TranslationValidationReport,
        record: &TranslationCommitmentRecord,
    ) -> Result<ExecutionInterfaceHandoffEnvelope, InferenceTranslationFailure> {
        if transaction.transaction_id.is_empty()
            || transaction.set_id != set.set_id
            || record.transaction_id != transaction.transaction_id
            || !report.valid
        {
            return Err(
                InferenceTranslationFailure::TranslationTransactionValidationFailure(
                    "invalid handoff envelope context".into(),
                ),
            );
        }
        let id = digest(&[
            ("transaction", transaction.transaction_id.clone()),
            ("set", set.set_id.clone()),
            ("record", record.commitment_id.clone()),
        ]);
        Ok(ExecutionInterfaceHandoffEnvelope {
            envelope_id: format!("execution-handoff:{id}"),
            cycle_id: transaction.cycle_id.clone(),
            transaction_id: transaction.transaction_id.clone(),
            set_id: set.set_id.clone(),
            source_command_id: set.source_command_id.clone(),
            commitment_id: record.commitment_id.clone(),
            runtime_id: transaction.runtime_id.clone(),
            validation_report_id: report.report_id.clone(),
            atomic: transaction.atomic,
            ordering_rule_id: transaction.ordering_rule_id.clone(),
            content_hash: id,
        })
    }

    pub fn replay<T: Eq>(original: &T, replayed: &T) -> Result<(), InferenceTranslationFailure> {
        if original == replayed {
            Ok(())
        } else {
            Err(InferenceTranslationFailure::ReplayNotEquivalent)
        }
    }
}

#[derive(Default)]
pub struct TranslationStore {
    transactions: BTreeMap<String, (CommittedTranslationTransaction, TranslationCommitmentRecord)>,
}
impl TranslationStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn publish(
        &mut self,
        transaction: CommittedTranslationTransaction,
        record: TranslationCommitmentRecord,
    ) -> Result<
        (CommittedTranslationTransaction, TranslationCommitmentRecord),
        InferenceTranslationFailure,
    > {
        let key = transaction.cycle_id.clone();
        if let Some(existing) = self.transactions.get(&key) {
            if existing == &(transaction.clone(), record.clone()) {
                return Ok(existing.clone());
            }
            return Err(InferenceTranslationFailure::TranslationCommitmentConflict);
        }
        self.transactions
            .insert(key, (transaction.clone(), record.clone()));
        Ok((transaction, record))
    }
    pub fn get(
        &self,
        cycle_id: &str,
    ) -> Option<&(CommittedTranslationTransaction, TranslationCommitmentRecord)> {
        self.transactions.get(cycle_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capability_is_not_availability_or_permission() {
        let p = RuntimeCapabilityProfile {
            runtime_id: "r".into(),
            runtime_family: "reference".into(),
            capability_profile_id: "p".into(),
            version: "1".into(),
            supported_command_type: "control".into(),
            dimensions: vec!["u".into()],
            units: vec!["unitless".into()],
            lower: vec![-1],
            upper: vec![1],
            precision: 0,
            ordering_supported: true,
            atomicity_supported: true,
            restriction_preservation_supported: true,
            availability: RuntimeAvailability::Unavailable,
            schema_version: "1".into(),
            registry_version: "1".into(),
            content_hash: "h".into(),
            provenance_id: "x".into(),
        };
        assert_eq!(p.availability, RuntimeAvailability::Unavailable);
        assert!(p.ordering_supported);
    }
    #[test]
    fn unavailable_is_distinct_value() {
        assert_ne!(
            RuntimeAvailability::Unavailable,
            RuntimeAvailability::Available
        );
    }
}
