use epilogos_factory::core::identity::{
    resolve_projection_identity, IdentityError, IdentityStore, ProjectionIdentity, Ref, Revision,
};
use serde_json::{json, Value};

const FIXTURE: &str = include_str!("../../contracts/factory/fixtures/ref-cases.json");
const SCHEMA: &str = include_str!("../../contracts/factory/ref.schema.json");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("Ref fixture must be valid JSON")
}

fn canonical_ref() -> Ref {
    fixture()["canonicalRef"]
        .as_str()
        .expect("canonicalRef must be a string")
        .parse()
        .expect("canonicalRef must parse")
}

fn revision(value: u64) -> Revision {
    Revision::new(value).expect("fixture revision must be positive")
}

fn store_at_revision_three() -> (IdentityStore, Ref) {
    let reference = canonical_ref();
    let mut store = IdentityStore::new();
    store
        .create(reference.clone(), json!({"source": "initial"}))
        .expect("create must succeed");
    store
        .add_alias(&reference, Revision::INITIAL, "epi-logos")
        .expect("alias mutation must succeed");
    store
        .update_payload(&reference, revision(2), json!({"source": "renamed"}))
        .expect("payload mutation must succeed");
    assert_eq!(
        store
            .get(&reference)
            .expect("record must exist")
            .revision
            .get(),
        3
    );
    (store, reference)
}

#[test]
fn canonical_ref_round_trips_without_projection_material() {
    let value = fixture();
    let reference = canonical_ref();
    assert_eq!(reference.kind(), "project");
    assert_eq!(
        reference.to_string(),
        value["canonicalRef"].as_str().unwrap()
    );
    let serialized = serde_json::to_string(&reference).expect("Ref must serialize");
    let round_trip: Ref = serde_json::from_str(&serialized).expect("Ref must deserialize");
    assert_eq!(round_trip, reference);

    for anti in value["antiFixtures"]
        .as_object()
        .expect("antiFixtures object")
        .values()
    {
        let invalid = anti["value"].as_str().expect("anti-fixture value string");
        assert!(invalid.parse::<Ref>().is_err(), "must reject {invalid}");
    }
}

#[test]
fn identity_survives_provider_path_and_projection_change() {
    let value = fixture();
    let expected = canonical_ref();
    for projection in value["projectionChanges"]
        .as_array()
        .expect("projectionChanges array")
    {
        let projection: ProjectionIdentity = serde_json::from_value(projection.clone())
            .expect("projection fixture must deserialize");
        assert_eq!(resolve_projection_identity(&projection).unwrap(), expected);
    }
}

#[test]
fn projection_loss_is_rejected_instead_of_guessed() {
    let value = fixture();
    let projection: ProjectionIdentity = serde_json::from_value(value["projectionLoss"].clone())
        .expect("projection loss fixture must deserialize");
    assert!(matches!(
        resolve_projection_identity(&projection),
        Err(IdentityError::MissingCanonicalRef { .. })
    ));
}

#[test]
fn stale_write_is_rejected_without_mutation() {
    let value = fixture();
    let (mut store, reference) = store_at_revision_three();
    let stale = value["staleWrite"]["attemptedExpectedRevision"]
        .as_u64()
        .expect("stale revision integer");
    let before = store.get(&reference).unwrap().clone();
    let result = store.update_payload(&reference, revision(stale), json!({"source": "stale"}));
    assert!(matches!(
        result,
        Err(IdentityError::RevisionMismatch { actual, .. }) if actual.get() == 3
    ));
    assert_eq!(store.get(&reference).unwrap(), &before);
}

#[test]
fn aliases_advance_revision_and_cannot_cross_identity() {
    let value = fixture();
    let reference = canonical_ref();
    let competing: Ref = value["competingRef"].as_str().unwrap().parse().unwrap();
    let alias = value["alias"].as_str().unwrap();
    let mut store = IdentityStore::new();
    store.create(reference.clone(), json!({})).unwrap();
    let aliased = store
        .add_alias(&reference, Revision::INITIAL, alias)
        .unwrap();
    assert_eq!(aliased.revision.get(), 2);
    assert_eq!(store.resolve_alias(alias).unwrap().reference, reference);

    store.create(competing.clone(), json!({})).unwrap();
    assert!(matches!(
        store.add_alias(&competing, Revision::INITIAL, alias),
        Err(IdentityError::AliasConflict { existing, .. }) if existing == reference
    ));
}

#[test]
fn tombstone_is_terminal_resolvable_and_prevents_id_reuse() {
    let value = fixture();
    let (mut store, reference) = store_at_revision_three();
    let before = value["tombstone"]["revisionBefore"].as_u64().unwrap();
    let after = value["tombstone"]["revisionAfter"].as_u64().unwrap();
    let tombstoned = store.tombstone(&reference, revision(before)).unwrap();
    assert!(tombstoned.tombstoned);
    assert_eq!(tombstoned.revision.get(), after);
    assert!(store.get(&reference).is_some());
    assert!(matches!(
        store.update_payload(&reference, revision(after), json!({"source": "revived"})),
        Err(IdentityError::Tombstoned(found)) if found == reference
    ));
    assert!(matches!(
        store.create(reference.clone(), json!({"source": "reused"})),
        Err(IdentityError::Retired(found)) if found == reference
    ));
}

#[test]
fn serialization_preserves_revision_alias_tombstone_and_no_reuse() {
    let (mut store, reference) = store_at_revision_three();
    store.tombstone(&reference, revision(3)).unwrap();
    let serialized = store.to_json().expect("identity store must serialize");
    let restored = IdentityStore::from_json(&serialized).expect("identity store must restore");
    assert_eq!(restored, store);
    assert_eq!(
        restored.resolve_alias("epi-logos").unwrap().reference,
        reference
    );
    assert!(restored.get(&reference).unwrap().tombstoned);
}

#[test]
fn schema_exposes_ref_revision_tombstone_and_no_projection_identity_contract() {
    let schema: Value = serde_json::from_str(SCHEMA).expect("Ref schema must be JSON");
    assert_eq!(schema["$id"], json!("factory.ref.schema/v1"));
    assert_eq!(
        schema["$defs"]["ref"]["pattern"],
        json!("^[a-z][a-z0-9-]*:[0-7][0-9A-HJKMNP-TV-Z]{25}$")
    );
    assert_eq!(schema["$defs"]["revision"]["minimum"], json!(1));
    let required = schema["$defs"]["identityRecord"]["required"]
        .as_array()
        .expect("identityRecord required array");
    for field in ["ref", "revision", "aliases", "tombstoned", "payload"] {
        assert!(required.iter().any(|candidate| candidate == field));
    }
}
