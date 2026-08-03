use canonical_hashing::content_hash;
use canonical_identity::ArtifactIdentity;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitmentEvent {
    pub artifact_id: String,
    pub content_hash: String,
    pub sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommitResult {
    Committed(CommitmentEvent),
    Idempotent(CommitmentEvent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommitmentError {
    Conflict {
        artifact_id: String,
        existing_hash: String,
        incoming_hash: String,
    },
}

#[derive(Default)]
pub struct CommitmentStore {
    artifacts: BTreeMap<String, String>,
    events: Vec<CommitmentEvent>,
}

impl CommitmentStore {
    pub fn commit(
        &mut self,
        identity: &ArtifactIdentity,
        canonical_bytes: &[u8],
    ) -> Result<CommitResult, CommitmentError> {
        let hash = content_hash(canonical_bytes);
        if let Some(existing) = self.artifacts.get(&identity.identifier) {
            if existing == &hash {
                return Ok(CommitResult::Idempotent(
                    self.events
                        .iter()
                        .find(|e| e.artifact_id == identity.identifier)
                        .cloned()
                        .expect("event exists"),
                ));
            }
            return Err(CommitmentError::Conflict {
                artifact_id: identity.identifier.clone(),
                existing_hash: existing.clone(),
                incoming_hash: hash,
            });
        }
        let event = CommitmentEvent {
            artifact_id: identity.identifier.clone(),
            content_hash: hash.clone(),
            sequence: self.events.len() as u64,
        };
        self.artifacts.insert(identity.identifier.clone(), hash);
        self.events.push(event.clone());
        Ok(CommitResult::Committed(event))
    }
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use canonical_versioning::{Version, VersionBinding};
    fn identity() -> ArtifactIdentity {
        let v = Version::new("1.0.0").unwrap();
        ArtifactIdentity::deterministic(
            VersionBinding {
                baseline: v.clone(),
                contract: v.clone(),
                schema: v.clone(),
                registry: v.clone(),
                implementation: v,
            },
            canonical_identity::IdentityBinding {
                schema: "s".into(),
                registry: "r".into(),
                cycle: None,
                producer: "p".into(),
            },
            b"seed",
        )
    }
    #[test]
    fn duplicate_is_idempotent_and_conflict_is_rejected() {
        let mut s = CommitmentStore::default();
        let identity = identity();
        let first = s.commit(&identity, b"a").unwrap();
        assert!(matches!(first, CommitResult::Committed(_)));
        assert!(matches!(
            s.commit(&identity, b"a"),
            Ok(CommitResult::Idempotent(_))
        ));
        assert!(matches!(
            s.commit(&identity, b"b"),
            Err(CommitmentError::Conflict { .. })
        ));
    }
}
