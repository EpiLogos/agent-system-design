//! Factory-side optional QL/MEF client boundary.
//!
//! The standalone QL/MEF module owns QL semantics. Factory owns only the client
//! subject identity it passes across this boundary, provider binding/health policy,
//! and the decision to treat QL as disabled, optional, or required. The subject Ref
//! is deliberately opaque here: this adapter must not translate the live CR-001
//! `factory:<kind>:<id>` identity into the older Rust foundation `kind:ULID` type.

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const QL_PROVENANCE_SCHEMA_VERSION: &str = "1.1.0";
pub const QL_MEF_REGISTRY_VERSION: &str = "1.0.0-q2";
pub const QL_OUTPUT_SCHEMA_VERSION: &str = "ql-contract/1.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum QlMode {
    Disabled,
    Optional,
    Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderState {
    Absent,
    Available,
    Degraded,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderHealth {
    pub state: ProviderState,
    pub detail: Option<String>,
}

impl ProviderHealth {
    pub fn absent() -> Self {
        Self {
            state: ProviderState::Absent,
            detail: Some("QL provider not supplied".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRef {
    pub provider: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub provider: ProviderRef,
    pub health: ProviderHealth,
    pub semantic_refraction: bool,
    pub refract: bool,
    pub supported_lenses: Vec<String>,
    pub output_schema_versions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactorySubject {
    pub reference: String,
    pub revision: Option<String>,
}

impl FactorySubject {
    pub fn new(reference: impl Into<String>, revision: Option<String>) -> Result<Self, QlError> {
        let reference = reference.into();
        if reference.trim().is_empty() {
            return Err(QlError::InvalidSubject("subject Ref cannot be empty".into()));
        }
        Ok(Self {
            reference,
            revision,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactoryRecord<T> {
    pub subject: FactorySubject,
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputRefRevision {
    pub reference: String,
    pub revision: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QlProvenance {
    pub schema_version: String,
    pub mef_registry_version: String,
    pub provider: ProviderRef,
    pub operation: String,
    pub input_refs: Vec<InputRefRevision>,
    pub model: Option<String>,
    pub config_ref: Option<String>,
    pub result_class: String,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QlReading {
    pub target_ref: String,
    pub lens_ref: String,
    pub value: Value,
    pub provenance: QlProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderFailure {
    pub code: String,
    pub message: String,
}

impl ProviderFailure {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

pub trait QlProviderClient {
    fn capabilities(&self) -> ProviderCapabilities;

    fn refract(
        &self,
        subject: &FactorySubject,
        lens_ref: &str,
    ) -> Result<QlReading, ProviderFailure>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum QlAttachment {
    Disabled,
    Reading {
        health: ProviderHealth,
        reading: Box<QlReading>,
    },
    Unavailable {
        health: ProviderHealth,
        reason: String,
    },
    Failed {
        health: Option<ProviderHealth>,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FactoryQlResult<T> {
    pub client: FactoryRecord<T>,
    pub attachment: QlAttachment,
    pub capabilities: Option<ProviderCapabilities>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QlError {
    InvalidSubject(String),
    InvalidLens(String),
    RequiredUnavailable(String),
    RequiredProviderFailure(String),
    ProviderContractViolation(String),
}

impl std::fmt::Display for QlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSubject(message)
            | Self::InvalidLens(message)
            | Self::RequiredUnavailable(message)
            | Self::RequiredProviderFailure(message)
            | Self::ProviderContractViolation(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for QlError {}

pub struct FactoryQlAdapter<'a> {
    provider: Option<&'a dyn QlProviderClient>,
    mode: QlMode,
}

impl<'a> FactoryQlAdapter<'a> {
    pub const fn new(provider: Option<&'a dyn QlProviderClient>, mode: QlMode) -> Self {
        Self { provider, mode }
    }

    pub fn refract<T>(
        &self,
        client: FactoryRecord<T>,
        lens_ref: &str,
    ) -> Result<FactoryQlResult<T>, QlError> {
        validate_lens(lens_ref)?;

        if self.mode == QlMode::Disabled {
            return Ok(FactoryQlResult {
                client,
                attachment: QlAttachment::Disabled,
                capabilities: None,
            });
        }

        let Some(provider) = self.provider else {
            let health = ProviderHealth::absent();
            if self.mode == QlMode::Required {
                return Err(QlError::RequiredUnavailable(
                    "QL provider is required but not supplied".into(),
                ));
            }
            return Ok(FactoryQlResult {
                client,
                attachment: QlAttachment::Unavailable {
                    health,
                    reason: "QL provider not supplied".into(),
                },
                capabilities: None,
            });
        };

        let capabilities = provider.capabilities();
        let health = capabilities.health.clone();
        let unavailable_reason = provider_unavailable_reason(&capabilities, lens_ref);
        if let Some(reason) = unavailable_reason {
            if self.mode == QlMode::Required {
                return Err(QlError::RequiredUnavailable(reason));
            }
            return Ok(FactoryQlResult {
                client,
                attachment: QlAttachment::Unavailable {
                    health,
                    reason,
                },
                capabilities: Some(capabilities),
            });
        }

        match provider.refract(&client.subject, lens_ref) {
            Ok(reading) => match validate_reading(&reading, &client.subject, &capabilities) {
                Ok(()) => Ok(FactoryQlResult {
                    client,
                    attachment: QlAttachment::Reading {
                        health,
                        reading: Box::new(reading),
                    },
                    capabilities: Some(capabilities),
                }),
                Err(error) if self.mode == QlMode::Required => Err(error),
                Err(error) => Ok(FactoryQlResult {
                    client,
                    attachment: QlAttachment::Failed {
                        health: Some(health),
                        message: error.to_string(),
                    },
                    capabilities: Some(capabilities),
                }),
            },
            Err(failure) if self.mode == QlMode::Required => {
                Err(QlError::RequiredProviderFailure(format!(
                    "{}: {}",
                    failure.code, failure.message
                )))
            }
            Err(failure) => Ok(FactoryQlResult {
                client,
                attachment: QlAttachment::Failed {
                    health: Some(health),
                    message: format!("{}: {}", failure.code, failure.message),
                },
                capabilities: Some(capabilities),
            }),
        }
    }
}

fn provider_unavailable_reason(
    capabilities: &ProviderCapabilities,
    lens_ref: &str,
) -> Option<String> {
    match capabilities.health.state {
        ProviderState::Absent => return Some("QL provider reports absent".into()),
        ProviderState::Incompatible => return Some("QL provider reports incompatible".into()),
        ProviderState::Available | ProviderState::Degraded => {}
    }
    if !capabilities.semantic_refraction {
        return Some("provider does not advertise semantic-refraction capability".into());
    }
    if !capabilities.refract {
        return Some("provider does not advertise refract".into());
    }
    if !capabilities
        .supported_lenses
        .iter()
        .any(|lens| lens == lens_ref)
    {
        return Some(format!("provider does not advertise lens {lens_ref}"));
    }
    if !capabilities
        .output_schema_versions
        .iter()
        .any(|version| version == QL_OUTPUT_SCHEMA_VERSION)
    {
        return Some(format!(
            "provider does not advertise output schema {QL_OUTPUT_SCHEMA_VERSION}"
        ));
    }
    None
}

fn validate_reading(
    reading: &QlReading,
    subject: &FactorySubject,
    capabilities: &ProviderCapabilities,
) -> Result<(), QlError> {
    if reading.target_ref != subject.reference {
        return Err(QlError::ProviderContractViolation(
            "QL reading target Ref differs from Factory subject Ref".into(),
        ));
    }
    if reading.provenance.schema_version != QL_PROVENANCE_SCHEMA_VERSION {
        return Err(QlError::ProviderContractViolation(
            "QL provenance schema version mismatch".into(),
        ));
    }
    if reading.provenance.mef_registry_version != QL_MEF_REGISTRY_VERSION {
        return Err(QlError::ProviderContractViolation(
            "QL MEF registry version mismatch".into(),
        ));
    }
    if reading.provenance.provider != capabilities.provider {
        return Err(QlError::ProviderContractViolation(
            "QL provenance provider/version differs from discovery".into(),
        ));
    }
    if reading.provenance.operation != "refract" {
        return Err(QlError::ProviderContractViolation(
            "QL provenance operation is not refract".into(),
        ));
    }
    let exact_input = reading.provenance.input_refs.iter().any(|input| {
        input.reference == subject.reference && input.revision == subject.revision
    });
    if !exact_input {
        return Err(QlError::ProviderContractViolation(
            "QL provenance does not preserve Factory subject Ref/revision".into(),
        ));
    }
    Ok(())
}

fn validate_lens(value: &str) -> Result<(), QlError> {
    let Some(body) = value
        .strip_prefix("mef:lens:L")
        .and_then(|value| value.strip_suffix("@1"))
    else {
        return Err(QlError::InvalidLens(format!(
            "{value} is not a canonical QL-MEF LensRef"
        )));
    };
    let bytes = body.as_bytes();
    let valid = match bytes {
        [digit] => (b'0'..=b'5').contains(digit),
        [digit, b'\''] => (b'0'..=b'5').contains(digit),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(QlError::InvalidLens(format!(
            "{value} is not a canonical QL-MEF LensRef"
        )))
    }
}
