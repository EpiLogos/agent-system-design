use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct Revision(u64);

impl Revision {
    pub const INITIAL: Self = Self(1);

    pub fn new(value: u64) -> Option<Self> {
        (value > 0).then_some(Self(value))
    }

    pub fn get(self) -> u64 {
        self.0
    }

    pub(crate) fn next(self) -> Option<Self> {
        self.0.checked_add(1).map(Self)
    }
}

impl<'de> Deserialize<'de> for Revision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).ok_or_else(|| serde::de::Error::custom("revision must be >= 1"))
    }
}
