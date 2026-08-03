use canonical_versioning::{Version, VersionBinding};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityProfile {
    pub implementation_identity: String,
    pub implementation_version: Version,
    pub baseline_version: Version,
    pub supported_contracts: Vec<u16>,
    pub supported_registry_versions: Vec<Version>,
    pub numeric_profile: String,
    pub declared_exclusions: Vec<String>,
}

impl CapabilityProfile {
    pub fn validate(
        &self,
        contract_number: u16,
        requested: &VersionBinding,
        supported_contract: &Version,
        supported_registry: &Version,
        supported_numeric: &str,
    ) -> Result<(), ConformanceError> {
        if !self.supported_contracts.contains(&contract_number)
            || &requested.contract != supported_contract
        {
            return Err(ConformanceError::UnsupportedContract(
                requested.contract.clone(),
            ));
        }
        if !self
            .supported_registry_versions
            .contains(&requested.registry)
            || &requested.registry != supported_registry
        {
            return Err(ConformanceError::UnsupportedRegistry(
                requested.registry.clone(),
            ));
        }
        if self.numeric_profile != supported_numeric {
            return Err(ConformanceError::UnsupportedNumeric(
                self.numeric_profile.clone(),
            ));
        }
        Ok(())
    }
    pub fn release_binding(&self, schema: Version) -> Result<VersionBinding, ConformanceError> {
        let Some(registry) = self.supported_registry_versions.first() else {
            return Err(ConformanceError::InvalidProfile);
        };
        Ok(VersionBinding {
            baseline: self.baseline_version.clone(),
            contract: Version::new("1.0.0").expect("static"),
            schema,
            registry: registry.clone(),
            implementation: self.implementation_version.clone(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConformanceError {
    UnsupportedContract(Version),
    UnsupportedRegistry(Version),
    UnsupportedNumeric(String),
    InvalidProfile,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsupported_contract_fails() {
        let v = Version::new("1.0.0").unwrap();
        let p = CapabilityProfile {
            implementation_identity: "test".into(),
            implementation_version: v.clone(),
            baseline_version: v.clone(),
            supported_contracts: vec![0],
            supported_registry_versions: vec![v.clone()],
            numeric_profile: "exact".into(),
            declared_exclusions: vec![],
        };
        let requested = VersionBinding {
            baseline: v.clone(),
            contract: v.clone(),
            schema: v.clone(),
            registry: v.clone(),
            implementation: v,
        };
        assert!(matches!(
            p.validate(
                1,
                &requested,
                &Version::new("1.0.0").unwrap(),
                &Version::new("1.0.0").unwrap(),
                "exact"
            ),
            Err(ConformanceError::UnsupportedContract(_))
        ));
    }
}
