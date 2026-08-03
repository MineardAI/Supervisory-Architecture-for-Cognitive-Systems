use canonical_hashing::identity_hash;
use canonical_versioning::VersionBinding;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityBinding {
    pub schema: String,
    pub registry: String,
    pub cycle: Option<String>,
    pub producer: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIdentity {
    pub identifier: String,
    pub versions: VersionBinding,
    pub binding: IdentityBinding,
    pub content_hash: String,
}

impl ArtifactIdentity {
    pub fn deterministic(
        versions: VersionBinding,
        binding: IdentityBinding,
        canonical_content: &[u8],
    ) -> Self {
        let seed = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            versions.baseline,
            versions.contract,
            versions.schema,
            versions.registry,
            versions.implementation,
            binding.schema,
            binding.registry,
            binding.producer,
            binding.cycle.as_deref().unwrap_or("<absent>")
        );
        let identifier = identity_hash(
            format!("{seed}|{}", String::from_utf8_lossy(canonical_content)).as_bytes(),
        );
        let content_hash = identity_hash(canonical_content);
        Self {
            identifier,
            versions,
            binding,
            content_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use canonical_versioning::Version;
    fn v() -> VersionBinding {
        let x = Version::new("1.0.0").unwrap();
        VersionBinding {
            baseline: x.clone(),
            contract: x.clone(),
            schema: x.clone(),
            registry: x.clone(),
            implementation: x,
        }
    }
    #[test]
    fn same_input_same_identity() {
        let b = IdentityBinding {
            schema: "s".into(),
            registry: "r".into(),
            cycle: None,
            producer: "p".into(),
        };
        assert_eq!(
            ArtifactIdentity::deterministic(v(), b.clone(), b"x"),
            ArtifactIdentity::deterministic(v(), b, b"x")
        );
    }
}
