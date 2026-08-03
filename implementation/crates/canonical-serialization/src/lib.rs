use std::{collections::BTreeMap, fmt};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CanonicalDocument(BTreeMap<String, String>);

impl CanonicalDocument {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (key, value) in &self.0 {
            write_field(&mut out, key);
            write_field(&mut out, value);
        }
        out
    }
    pub fn decode(bytes: &[u8]) -> Result<Self, SerializationError> {
        let mut cursor = 0;
        let mut document = Self::new();
        while cursor < bytes.len() {
            let key = read_field(bytes, &mut cursor)?;
            let value = read_field(bytes, &mut cursor)?;
            if document.0.insert(key.clone(), value).is_some() {
                return Err(SerializationError::DuplicateKey(key));
            }
        }
        Ok(document)
    }
}

fn write_field(out: &mut Vec<u8>, value: &str) {
    out.extend_from_slice(&(value.len() as u64).to_be_bytes());
    out.extend_from_slice(value.as_bytes());
}
fn read_field(bytes: &[u8], cursor: &mut usize) -> Result<String, SerializationError> {
    if bytes.len().saturating_sub(*cursor) < 8 {
        return Err(SerializationError::Truncated);
    }
    let mut length = [0; 8];
    length.copy_from_slice(&bytes[*cursor..*cursor + 8]);
    *cursor += 8;
    let length = u64::from_be_bytes(length) as usize;
    if bytes.len().saturating_sub(*cursor) < length {
        return Err(SerializationError::Truncated);
    }
    let value = std::str::from_utf8(&bytes[*cursor..*cursor + length])
        .map_err(|_| SerializationError::InvalidUtf8)?
        .to_owned();
    *cursor += length;
    Ok(value)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SerializationError {
    Truncated,
    InvalidUtf8,
    DuplicateKey(String),
}
impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for SerializationError {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordering_is_stable() {
        let mut d = CanonicalDocument::new();
        d.insert("z", "2");
        d.insert("a", "1");
        assert_eq!(d.encode(), {
            let mut e = CanonicalDocument::new();
            e.insert("a", "1");
            e.insert("z", "2");
            e.encode()
        });
    }
    #[test]
    fn round_trip() {
        let mut d = CanonicalDocument::new();
        d.insert("a", "1");
        assert_eq!(CanonicalDocument::decode(&d.encode()).unwrap(), d);
    }
}
