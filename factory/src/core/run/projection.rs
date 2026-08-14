use super::{RunContractError, RunRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunProjectionIdentity {
    pub provider: String,
    pub external_id: String,
    pub canonical_run_ref: Option<RunRef>,
}

pub fn resolve_run_projection(projection: &RunProjectionIdentity) -> Result<RunRef, RunContractError> {
    projection
        .canonical_run_ref
        .clone()
        .ok_or_else(|| RunContractError::MissingCanonicalRunRef {
            provider: projection.provider.clone(),
            external_id: projection.external_id.clone(),
        })
}
