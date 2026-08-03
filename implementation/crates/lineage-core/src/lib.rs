use canonical_identity::ArtifactIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageEdge {
    pub parent: String,
    pub child: String,
    pub cycle: Option<String>,
    pub correlation: String,
    pub provenance: Vec<String>,
}

impl LineageEdge {
    pub fn new(
        parent: &ArtifactIdentity,
        child: &ArtifactIdentity,
        correlation: impl Into<String>,
    ) -> Self {
        Self {
            parent: parent.identifier.clone(),
            child: child.identifier.clone(),
            cycle: child.binding.cycle.clone(),
            correlation: correlation.into(),
            provenance: vec![parent.content_hash.clone(), child.content_hash.clone()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edge_is_explicit() {
        let edge = LineageEdge {
            parent: "p".into(),
            child: "c".into(),
            cycle: None,
            correlation: "x".into(),
            provenance: vec![],
        };
        assert_eq!(edge.parent, "p");
    }
}
