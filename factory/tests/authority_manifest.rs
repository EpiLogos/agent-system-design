use epilogos_factory::authority::{parse_manifest, render_markdown, validate_manifest};
use serde_json::{json, Value};

const MANIFEST: &str = include_str!("../../contracts/factory/authority-manifest.json");
const SCHEMA: &str = include_str!("../../contracts/factory/authority-manifest.schema.json");
const INVALID_PROMOTION: &str =
    include_str!("../../contracts/factory/fixtures/authority-invalid-promotion.json");

fn manifest_value() -> Value {
    serde_json::from_str(MANIFEST).expect("authority manifest must be JSON")
}

fn validate_value(value: Value) -> Vec<String> {
    let manifest = serde_json::from_value(value).expect("mutated fixture must retain manifest shape");
    validate_manifest(&manifest)
}

#[test]
fn manifest_is_valid() {
    let manifest = parse_manifest(MANIFEST).expect("manifest must deserialize");
    assert_eq!(Vec::<String>::new(), validate_manifest(&manifest));
}

#[test]
fn provenance_round_trip_is_lossless() {
    let value = manifest_value();
    let encoded = serde_json::to_string(&value).expect("manifest must serialize");
    let decoded: Value = serde_json::from_str(&encoded).expect("manifest must deserialize");
    assert_eq!(value, decoded);
}

#[test]
fn invalid_promotion_fixture_is_rejected() {
    let fixture: Value = serde_json::from_str(INVALID_PROMOTION).expect("fixture must be JSON");
    let mut value = manifest_value();
    let id = fixture["retrievedReferenceId"]
        .as_str()
        .expect("fixture id must be string");
    let reference = value["retrievedReferences"]
        .as_array()
        .expect("retrievedReferences must be array")
        .iter()
        .find(|entry| entry["id"] == id)
        .expect("fixture reference must exist")
        .clone();
    let mut promoted = reference;
    promoted["precedence"] = fixture["precedence"].clone();
    promoted["scope"] = fixture["scope"].clone();
    promoted["governs"] = json!(true);
    value["sources"]
        .as_array_mut()
        .expect("sources must be array")
        .push(promoted);

    let errors = validate_value(value);
    let expected = fixture["expectedError"]
        .as_str()
        .expect("expectedError must be string");
    assert!(errors.iter().any(|error| error.contains(expected)));
}

#[test]
fn duplicate_source_identity_is_rejected() {
    let mut value = manifest_value();
    let duplicate = value["sources"][0].clone();
    value["sources"]
        .as_array_mut()
        .expect("sources must be array")
        .push(duplicate);
    let errors = validate_value(value);
    assert!(errors.iter().any(|error| error == "duplicate source id"));
    assert!(errors.iter().any(|error| error == "duplicate source path"));
}

#[test]
fn constitutional_index_has_unique_highest_precedence() {
    let mut value = manifest_value();
    value["sources"][1]["precedence"] = json!(100);
    let errors = validate_value(value);
    assert!(errors
        .iter()
        .any(|error| error == "constitutional index must have unique highest precedence"));
}

#[test]
fn determination_statuses_are_exact() {
    let mut value = manifest_value();
    value["determinationStatuses"]
        .as_array_mut()
        .expect("determinationStatuses must be array")
        .push(json!("CONSENSUS"));
    let errors = validate_value(value);
    assert!(errors
        .iter()
        .any(|error| error == "determination status set differs from root programme"));
}

#[test]
fn schema_surface_is_static_and_matches_manifest_version() {
    let schema: Value = serde_json::from_str(SCHEMA).expect("schema must be JSON");
    assert_eq!(schema["$schema"], json!("https://json-schema.org/draft/2020-12/schema"));
    assert_eq!(schema["$id"], json!("factory.authority-manifest.schema/v1"));
    assert_eq!(schema["properties"]["schemaVersion"]["const"], json!("factory.authority-manifest/v1"));
    let required = schema["required"].as_array().expect("required must be array");
    for field in [
        "schemaVersion",
        "repository",
        "determinationStatuses",
        "documentStatuses",
        "sources",
        "promotionRules",
        "invariants",
    ] {
        assert!(required.iter().any(|candidate| candidate == field));
    }
}

#[test]
fn generated_markdown_is_derived_from_manifest() {
    let manifest = parse_manifest(MANIFEST).expect("manifest must deserialize");
    let rendered = render_markdown(&manifest);
    assert!(rendered.contains("QL-SOFTWARE-FACTORY-CONSTITUTIONAL-INDEX.md"));
    assert!(rendered.contains("RUN-CLOSURE-VERIFICATION-ALIGNMENT.md"));
    assert!(rendered.contains("Generated from `contracts/factory/authority-manifest.json`"));
}
