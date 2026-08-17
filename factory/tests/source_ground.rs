use serde_json::{json, Value};

const GROUND: &str = include_str!("../../contracts/factory/source-ground.json");

fn ground() -> Value {
    serde_json::from_str(GROUND).expect("source Ground must be valid JSON")
}

#[test]
fn rust_ground_is_pinned_at_the_right_determination_levels() {
    let ground = ground();
    assert_eq!(ground["repository"]["baselineRef"], json!("main"));
    assert_eq!(ground["repository"]["determination"], json!("OBSERVED"));
    assert_eq!(
        ground["authorialDeterminations"]["implementationLanguage"]["value"],
        json!("Rust")
    );
    assert_eq!(
        ground["authorialDeterminations"]["implementationLanguage"]["determination"],
        json!("GENUINE HUMAN AUTHORSHIP")
    );
    assert_eq!(
        ground["materializedImplementation"]["packageManifest"],
        json!("factory/Cargo.toml")
    );
    assert_eq!(ground["blocked"], json!(false));
}

#[test]
fn product_and_runtime_islands_remain_outside_factory_core() {
    let ground = ground();
    assert_eq!(
        ground["productBoundaries"]["aikit"]["factoryOwnsInternals"],
        json!(false)
    );
    assert_eq!(
        ground["productBoundaries"]["workcell"]["factoryOwnsProviderInternals"],
        json!(false)
    );
    assert_eq!(
        ground["productBoundaries"]["qlMef"]["factoryOwnsKernel"],
        json!(false)
    );
    let islands = ground["sourceIslands"]
        .as_array()
        .expect("sourceIslands must be an array");
    assert!(islands
        .iter()
        .all(|island| island["canonicalFactoryCore"] == json!(false)));
}
