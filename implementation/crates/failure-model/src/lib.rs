use canonical_hashing::content_hash;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FailureRecord {
    pub failure_id: String,
    pub class: String,
    pub detail: String,
    pub immutable_hash: String,
}

impl FailureRecord {
    pub fn new(class: impl Into<String>, detail: impl Into<String>) -> Self {
        let class = class.into();
        let detail = detail.into();
        let immutable_hash = content_hash(format!("{class}\0{detail}").as_bytes());
        Self {
            failure_id: immutable_hash.clone(),
            class,
            detail,
            immutable_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn failure_is_not_null_and_is_deterministic() {
        let a = FailureRecord::new("unsupported-version", "1.0.0");
        assert_eq!(a, FailureRecord::new("unsupported-version", "1.0.0"));
        assert!(!a.failure_id.is_empty());
    }
}
