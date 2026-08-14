use super::{IdentityError, Ref};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectionIdentity {
    pub provider: String,
    pub external_id: String,
    pub canonical_ref: Option<Ref>,
}

pub fn resolve_projection_identity(projection: &ProjectionIdentity) -> Result<Ref, IdentityError> {
    projection
        .canonical_ref
        .clone()
        .ok_or_else(|| IdentityError::MissingCanonicalRef {
            provider: projection.provider.clone(),
            external_id: projection.external_id.clone(),
        })
}
