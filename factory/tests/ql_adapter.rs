use std::cell::Cell;

use epilogos_factory::ql::{
    FactoryQlAdapter, FactoryRecord, FactorySubject, InputRefRevision, ProviderCapabilities,
    ProviderFailure, ProviderHealth, ProviderRef, ProviderState, QlAttachment, QlError, QlMode,
    QlProvenance, QlProviderClient, QlReading, QL_MEF_REGISTRY_VERSION, QL_OUTPUT_SCHEMA_VERSION,
    QL_PROVENANCE_SCHEMA_VERSION,
};
use serde_json::json;

struct FixtureQl {
    state: ProviderState,
    fail: bool,
    drift_target: bool,
    drift_revision: bool,
    calls: Cell<usize>,
}

impl FixtureQl {
    fn new(state: ProviderState) -> Self {
        Self {
            state,
            fail: false,
            drift_target: false,
            drift_revision: false,
            calls: Cell::new(0),
        }
    }

    fn failing() -> Self {
        Self {
            fail: true,
            ..Self::new(ProviderState::Available)
        }
    }

    fn drift_target() -> Self {
        Self {
            drift_target: true,
            ..Self::new(ProviderState::Available)
        }
    }

    fn drift_revision() -> Self {
        Self {
            drift_revision: true,
            ..Self::new(ProviderState::Available)
        }
    }

    fn calls(&self) -> usize {
        self.calls.get()
    }

    fn provider_ref() -> ProviderRef {
        ProviderRef {
            provider: "fixture-ql".into(),
            version: "0.1.0".into(),
        }
    }
}

impl QlProviderClient for FixtureQl {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            provider: Self::provider_ref(),
            health: ProviderHealth {
                state: self.state,
                detail: (self.state == ProviderState::Degraded)
                    .then(|| "one semantic source unavailable".into()),
            },
            semantic_refraction: true,
            refract: true,
            supported_lenses: vec!["mef:lens:L3@1".into()],
            output_schema_versions: vec![QL_OUTPUT_SCHEMA_VERSION.into()],
        }
    }

    fn refract(
        &self,
        subject: &FactorySubject,
        lens_ref: &str,
    ) -> Result<QlReading, ProviderFailure> {
        self.calls.set(self.calls.get() + 1);
        if self.fail {
            return Err(ProviderFailure::new(
                "fixture.semantic_failure",
                "fixture semantic failure",
            ));
        }

        Ok(QlReading {
            target_ref: if self.drift_target {
                "factory:claim:different".into()
            } else {
                subject.reference.clone()
            },
            lens_ref: lens_ref.into(),
            value: json!({"text":"fixture semantic disclosure","status":"partial"}),
            provenance: QlProvenance {
                schema_version: QL_PROVENANCE_SCHEMA_VERSION.into(),
                mef_registry_version: QL_MEF_REGISTRY_VERSION.into(),
                provider: Self::provider_ref(),
                operation: "refract".into(),
                input_refs: vec![InputRefRevision {
                    reference: subject.reference.clone(),
                    revision: if self.drift_revision {
                        Some("sha256:different-revision".into())
                    } else {
                        subject.revision.clone()
                    },
                }],
                model: Some("fixture-semantic-model".into()),
                config_ref: Some("fixture:config:q4".into()),
                result_class: "semantic-stochastic".into(),
                warnings: Vec::new(),
            },
        })
    }
}

fn record() -> FactoryRecord<String> {
    FactoryRecord {
        subject: FactorySubject::new("factory:claim:c-1", Some("sha256:claim-c-1-r1".into()))
            .unwrap(),
        value: "ordinary Factory payload after mutation".into(),
    }
}

#[test]
fn disabled_ql_is_exact_noql_parity_and_does_not_call_provider() {
    let original = record();
    let provider = FixtureQl::new(ProviderState::Available);
    let result = FactoryQlAdapter::new(Some(&provider), QlMode::Disabled)
        .refract(original.clone(), "mef:lens:L3@1")
        .unwrap();

    assert_eq!(result.client, original);
    assert_eq!(provider.calls(), 0);
    assert!(result.capabilities.is_none());
    assert!(matches!(result.attachment, QlAttachment::Disabled));
}

#[test]
fn optional_no_provider_preserves_ordinary_factory_result_and_reports_absence() {
    let original = record();
    let result = FactoryQlAdapter::new(None, QlMode::Optional)
        .refract(original.clone(), "mef:lens:L3@1")
        .unwrap();

    assert_eq!(result.client, original);
    assert!(result.capabilities.is_none());
    assert!(matches!(
        result.attachment,
        QlAttachment::Unavailable {
            health: ProviderHealth {
                state: ProviderState::Absent,
                ..
            },
            ..
        }
    ));
}

#[test]
fn degraded_fixture_provider_enriches_same_ref_and_revision_with_inspectable_version() {
    let original = record();
    let provider = FixtureQl::new(ProviderState::Degraded);
    let result = FactoryQlAdapter::new(Some(&provider), QlMode::Optional)
        .refract(original.clone(), "mef:lens:L3@1")
        .unwrap();

    assert_eq!(result.client, original);
    assert_eq!(provider.calls(), 1);
    let capabilities = result.capabilities.as_ref().unwrap();
    assert_eq!(capabilities.provider.provider, "fixture-ql");
    assert_eq!(capabilities.provider.version, "0.1.0");
    assert_eq!(capabilities.health.state, ProviderState::Degraded);

    match result.attachment {
        QlAttachment::Reading { health, reading } => {
            assert_eq!(health.state, ProviderState::Degraded);
            assert_eq!(reading.target_ref, "factory:claim:c-1");
            assert_eq!(reading.lens_ref, "mef:lens:L3@1");
            assert_eq!(reading.provenance.provider.provider, "fixture-ql");
            assert_eq!(reading.provenance.provider.version, "0.1.0");
            assert_eq!(
                reading.provenance.input_refs[0].reference,
                "factory:claim:c-1"
            );
            assert_eq!(
                reading.provenance.input_refs[0].revision.as_deref(),
                Some("sha256:claim-c-1-r1")
            );
            assert_eq!(
                reading.provenance.model.as_deref(),
                Some("fixture-semantic-model")
            );
        }
        other => panic!("expected QL reading, got {other:?}"),
    }
}

#[test]
fn incompatible_provider_is_nonfatal_in_optional_mode_and_not_executed() {
    let original = record();
    let provider = FixtureQl::new(ProviderState::Incompatible);
    let result = FactoryQlAdapter::new(Some(&provider), QlMode::Optional)
        .refract(original.clone(), "mef:lens:L3@1")
        .unwrap();

    assert_eq!(result.client, original);
    assert_eq!(provider.calls(), 0);
    assert!(matches!(
        result.attachment,
        QlAttachment::Unavailable { .. }
    ));
}

#[test]
fn required_mode_fails_for_absence_and_provider_failure() {
    let absent = FactoryQlAdapter::new(None, QlMode::Required)
        .refract(record(), "mef:lens:L3@1")
        .unwrap_err();
    assert!(matches!(absent, QlError::RequiredUnavailable(_)));

    let provider = FixtureQl::failing();
    let failure = FactoryQlAdapter::new(Some(&provider), QlMode::Required)
        .refract(record(), "mef:lens:L3@1")
        .unwrap_err();
    assert!(matches!(failure, QlError::RequiredProviderFailure(_)));
    assert_eq!(provider.calls(), 1);
}

#[test]
fn provider_target_or_revision_drift_is_failed_instead_of_translated() {
    for provider in [FixtureQl::drift_target(), FixtureQl::drift_revision()] {
        let original = record();
        let result = FactoryQlAdapter::new(Some(&provider), QlMode::Optional)
            .refract(original.clone(), "mef:lens:L3@1")
            .unwrap();
        assert_eq!(result.client, original);
        assert!(matches!(result.attachment, QlAttachment::Failed { .. }));
    }
}

#[test]
fn legacy_lens_string_is_rejected_before_provider_execution() {
    let provider = FixtureQl::new(ProviderState::Available);
    let error = FactoryQlAdapter::new(Some(&provider), QlMode::Optional)
        .refract(record(), "lens:L3")
        .unwrap_err();

    assert!(matches!(error, QlError::InvalidLens(_)));
    assert_eq!(provider.calls(), 0);
}
