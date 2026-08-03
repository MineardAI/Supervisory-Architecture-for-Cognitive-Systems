use canonical_identity::ArtifactIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lifecycle {
    Constructed,
    Validated,
    Certified,
    Committed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Artifact<T> {
    pub identity: ArtifactIdentity,
    pub lifecycle: Lifecycle,
    pub payload: T,
}

impl<T> Artifact<T> {
    pub fn constructed(identity: ArtifactIdentity, payload: T) -> Self {
        Self {
            identity,
            lifecycle: Lifecycle::Constructed,
            payload,
        }
    }
    pub fn transition(self, lifecycle: Lifecycle) -> Result<Self, LifecycleError> {
        let valid_step = matches!(
            (self.lifecycle, lifecycle),
            (Lifecycle::Constructed, Lifecycle::Validated)
                | (Lifecycle::Validated, Lifecycle::Certified)
                | (Lifecycle::Certified, Lifecycle::Committed)
        );
        if !valid_step {
            return Err(LifecycleError::Regression);
        }
        Ok(Self { lifecycle, ..self })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleError {
    Regression,
}

#[cfg(test)]
mod tests {
    use super::*;
    use canonical_identity::IdentityBinding;
    use canonical_versioning::{Version, VersionBinding};
    #[test]
    fn lifecycle_requires_each_step() {
        let v = Version::new("1.0.0").unwrap();
        let identity = ArtifactIdentity {
            identifier: "id".into(),
            versions: VersionBinding {
                baseline: v.clone(),
                contract: v.clone(),
                schema: v.clone(),
                registry: v.clone(),
                implementation: v,
            },
            binding: IdentityBinding {
                schema: "schema".into(),
                registry: "registry".into(),
                cycle: None,
                producer: "producer".into(),
            },
            content_hash: "hash".into(),
        };
        let artifact = Artifact::constructed(identity, ());
        assert!(artifact.clone().transition(Lifecycle::Committed).is_err());
        assert!(artifact.transition(Lifecycle::Validated).is_ok());
    }
}
