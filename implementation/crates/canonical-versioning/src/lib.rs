use std::{fmt, str::FromStr};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Version(String);

impl Version {
    pub fn new(value: impl Into<String>) -> Result<Self, VersionError> {
        let value = value.into();
        let parts: Vec<_> = value.split('.').collect();
        if parts.len() != 3
            || parts
                .iter()
                .any(|part| part.is_empty() || part.parse::<u64>().is_err())
        {
            return Err(VersionError::Invalid(value));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl FromStr for Version {
    type Err = VersionError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VersionError {
    Invalid(String),
    Unsupported { kind: String, value: String },
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(v) => write!(f, "invalid version: {v}"),
            Self::Unsupported { kind, value } => write!(f, "unsupported {kind} version: {value}"),
        }
    }
}
impl std::error::Error for VersionError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionBinding {
    pub baseline: Version,
    pub contract: Version,
    pub schema: Version,
    pub registry: Version,
    pub implementation: Version,
}

impl VersionBinding {
    pub fn validate(&self, supported: &VersionBinding) -> Result<(), VersionError> {
        for (kind, actual, expected) in [
            ("baseline", &self.baseline, &supported.baseline),
            ("contract", &self.contract, &supported.contract),
            ("schema", &self.schema, &supported.schema),
            ("registry", &self.registry, &supported.registry),
            (
                "implementation",
                &self.implementation,
                &supported.implementation,
            ),
        ] {
            if actual != expected {
                return Err(VersionError::Unsupported {
                    kind: kind.into(),
                    value: actual.to_string(),
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_non_semver() {
        assert!(Version::new("1").is_err());
    }
    #[test]
    fn rejects_mismatch_without_downgrade() {
        let v = Version::new("1.0.0").unwrap();
        let other = Version::new("1.0.1").unwrap();
        let a = VersionBinding {
            baseline: v.clone(),
            contract: v.clone(),
            schema: v.clone(),
            registry: v.clone(),
            implementation: v,
        };
        let b = VersionBinding {
            baseline: other.clone(),
            contract: other.clone(),
            schema: other.clone(),
            registry: other.clone(),
            implementation: other,
        };
        assert!(matches!(
            a.validate(&b),
            Err(VersionError::Unsupported { .. })
        ));
    }
}
