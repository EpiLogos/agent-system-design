use super::{Ref, Revision};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

const STORE_SCHEMA_VERSION: &str = "factory.identity-store/v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityRecord {
    #[serde(rename = "ref")]
    pub reference: Ref,
    pub revision: Revision,
    pub aliases: BTreeSet<String>,
    pub tombstoned: bool,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityStore {
    schema_version: String,
    records: BTreeMap<Ref, IdentityRecord>,
    retired_refs: BTreeSet<Ref>,
}

impl Default for IdentityStore {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityStore {
    pub fn new() -> Self {
        Self {
            schema_version: STORE_SCHEMA_VERSION.to_owned(),
            records: BTreeMap::new(),
            retired_refs: BTreeSet::new(),
        }
    }

    pub fn create(
        &mut self,
        reference: Ref,
        payload: Value,
    ) -> Result<&IdentityRecord, IdentityError> {
        if self.retired_refs.contains(&reference) {
            return Err(IdentityError::Retired(reference));
        }
        if self.records.contains_key(&reference) {
            return Err(IdentityError::AlreadyExists(reference));
        }
        let record = IdentityRecord {
            reference: reference.clone(),
            revision: Revision::INITIAL,
            aliases: BTreeSet::new(),
            tombstoned: false,
            payload,
        };
        self.records.insert(reference.clone(), record);
        self.records
            .get(&reference)
            .ok_or(IdentityError::NotFound(reference))
    }

    pub fn get(&self, reference: &Ref) -> Option<&IdentityRecord> {
        self.records.get(reference)
    }

    pub fn resolve_alias(&self, alias: &str) -> Option<&IdentityRecord> {
        self.records
            .values()
            .find(|record| record.aliases.contains(alias))
    }

    pub fn update_payload(
        &mut self,
        reference: &Ref,
        expected_revision: Revision,
        payload: Value,
    ) -> Result<&IdentityRecord, IdentityError> {
        let record = self.record_for_mutation(reference, expected_revision)?;
        record.payload = payload;
        record.revision = record
            .revision
            .next()
            .ok_or(IdentityError::RevisionOverflow)?;
        Ok(record)
    }

    pub fn add_alias(
        &mut self,
        reference: &Ref,
        expected_revision: Revision,
        alias: impl Into<String>,
    ) -> Result<&IdentityRecord, IdentityError> {
        let alias = alias.into();
        if alias.trim().is_empty() {
            return Err(IdentityError::InvalidAlias);
        }
        if let Some(existing) = self
            .resolve_alias(&alias)
            .map(|record| record.reference.clone())
        {
            if &existing != reference {
                return Err(IdentityError::AliasConflict { alias, existing });
            }
        }

        let record = self.record_for_mutation(reference, expected_revision)?;
        if record.aliases.insert(alias) {
            record.revision = record
                .revision
                .next()
                .ok_or(IdentityError::RevisionOverflow)?;
        }
        Ok(record)
    }

    pub fn tombstone(
        &mut self,
        reference: &Ref,
        expected_revision: Revision,
    ) -> Result<&IdentityRecord, IdentityError> {
        {
            let record = self.record_for_mutation(reference, expected_revision)?;
            record.tombstoned = true;
            record.revision = record
                .revision
                .next()
                .ok_or(IdentityError::RevisionOverflow)?;
        }
        self.retired_refs.insert(reference.clone());
        self.records
            .get(reference)
            .ok_or_else(|| IdentityError::NotFound(reference.clone()))
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(serialized: &str) -> Result<Self, IdentityError> {
        let store: Self = serde_json::from_str(serialized)
            .map_err(|error| IdentityError::CorruptStore(error.to_string()))?;
        store.validate_integrity()?;
        Ok(store)
    }

    fn record_for_mutation(
        &mut self,
        reference: &Ref,
        expected_revision: Revision,
    ) -> Result<&mut IdentityRecord, IdentityError> {
        let record = self
            .records
            .get_mut(reference)
            .ok_or_else(|| IdentityError::NotFound(reference.clone()))?;
        if record.tombstoned {
            return Err(IdentityError::Tombstoned(reference.clone()));
        }
        if record.revision != expected_revision {
            return Err(IdentityError::RevisionMismatch {
                reference: reference.clone(),
                expected: expected_revision,
                actual: record.revision,
            });
        }
        Ok(record)
    }

    fn validate_integrity(&self) -> Result<(), IdentityError> {
        if self.schema_version != STORE_SCHEMA_VERSION {
            return Err(IdentityError::CorruptStore(
                "unsupported identity store schemaVersion".to_owned(),
            ));
        }
        let mut aliases = BTreeMap::<&str, &Ref>::new();
        for (key, record) in &self.records {
            if key != &record.reference || record.revision.get() == 0 {
                return Err(IdentityError::CorruptStore(
                    "record key/ref or revision invariant violated".to_owned(),
                ));
            }
            if record.tombstoned && !self.retired_refs.contains(key) {
                return Err(IdentityError::CorruptStore(
                    "tombstoned Ref missing from retiredRefs".to_owned(),
                ));
            }
            for alias in &record.aliases {
                if alias.trim().is_empty() {
                    return Err(IdentityError::CorruptStore(
                        "empty alias in identity record".to_owned(),
                    ));
                }
                if let Some(existing) = aliases.insert(alias, key) {
                    if existing != key {
                        return Err(IdentityError::CorruptStore(format!(
                            "alias {alias} resolves to multiple Refs"
                        )));
                    }
                }
            }
        }
        for reference in &self.retired_refs {
            match self.records.get(reference) {
                Some(record) if record.tombstoned => {}
                _ => {
                    return Err(IdentityError::CorruptStore(
                        "retired Ref is not retained as a tombstone".to_owned(),
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IdentityError {
    InvalidAlias,
    AlreadyExists(Ref),
    Retired(Ref),
    NotFound(Ref),
    RevisionMismatch {
        reference: Ref,
        expected: Revision,
        actual: Revision,
    },
    Tombstoned(Ref),
    AliasConflict {
        alias: String,
        existing: Ref,
    },
    RevisionOverflow,
    MissingCanonicalRef {
        provider: String,
        external_id: String,
    },
    CorruptStore(String),
}

impl Display for IdentityError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAlias => write!(formatter, "alias must not be empty"),
            Self::AlreadyExists(reference) => write!(formatter, "Ref already exists: {reference}"),
            Self::Retired(reference) => {
                write!(
                    formatter,
                    "Ref is retired and may not be reused: {reference}"
                )
            }
            Self::NotFound(reference) => write!(formatter, "Ref not found: {reference}"),
            Self::RevisionMismatch {
                reference,
                expected,
                actual,
            } => write!(
                formatter,
                "revision mismatch for {reference}: expected {}, actual {}",
                expected.get(),
                actual.get()
            ),
            Self::Tombstoned(reference) => write!(formatter, "Ref is tombstoned: {reference}"),
            Self::AliasConflict { alias, existing } => {
                write!(formatter, "alias {alias} already resolves to {existing}")
            }
            Self::RevisionOverflow => write!(formatter, "revision overflow"),
            Self::MissingCanonicalRef {
                provider,
                external_id,
            } => write!(
                formatter,
                "projection {provider}:{external_id} has no canonical Ref"
            ),
            Self::CorruptStore(message) => write!(formatter, "corrupt identity store: {message}"),
        }
    }
}

impl Error for IdentityError {}
