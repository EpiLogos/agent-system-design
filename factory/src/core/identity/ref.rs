use super::RefParseError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use ulid::Ulid;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ref {
    kind: String,
    id: Ulid,
}

impl Ref {
    pub fn new(kind: impl Into<String>, id: Ulid) -> Result<Self, RefParseError> {
        let kind = kind.into();
        validate_kind(&kind)?;
        Ok(Self { kind, id })
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn id(&self) -> Ulid {
        self.id
    }
}

impl Display for Ref {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}:{}", self.kind, self.id)
    }
}

impl FromStr for Ref {
    type Err = RefParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (kind, id) = value.split_once(':').ok_or(RefParseError::InvalidShape)?;
        validate_kind(kind)?;
        if id.contains(':') || id.is_empty() {
            return Err(RefParseError::InvalidShape);
        }
        let id = Ulid::from_string(id).map_err(|_| RefParseError::InvalidId)?;
        Self::new(kind, id)
    }
}

impl Serialize for Ref {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Ref {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

fn validate_kind(kind: &str) -> Result<(), RefParseError> {
    let mut chars = kind.chars();
    let Some(first) = chars.next() else {
        return Err(RefParseError::InvalidKind);
    };
    if !first.is_ascii_lowercase()
        || !chars.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return Err(RefParseError::InvalidKind);
    }
    Ok(())
}
