use canonical_serialization::CanonicalDocument;
use sha2::{Digest, Sha256};

pub fn content_hash(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
pub fn canonical_hash(document: &CanonicalDocument) -> String {
    content_hash(&document.encode())
}
pub fn identity_hash(identity_bytes: &[u8]) -> String {
    content_hash(identity_bytes)
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable_sha256() {
        assert_eq!(
            content_hash(b"SACS"),
            "07889eb79a3581eff7395cd0eab5d5814a5bc72a8c4b07eb0e3c7e5b70256ad9"
        );
    }
    #[test]
    fn canonical_content_hashes_equal() {
        let mut a = CanonicalDocument::new();
        a.insert("b", "2");
        a.insert("a", "1");
        let mut b = CanonicalDocument::new();
        b.insert("a", "1");
        b.insert("b", "2");
        assert_eq!(canonical_hash(&a), canonical_hash(&b));
    }
}
