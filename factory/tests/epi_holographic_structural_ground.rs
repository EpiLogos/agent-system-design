use epilogos_factory::core::run::RunRef;
use epilogos_factory::structural_ground::{
    verify_structural_ground, StructuralBinding, StructuralFidelityObservation, StructuralGap,
    StructuralGround, StructuralRelation, StructuralSourceRef, STRUCTURAL_GROUND_VERSION,
};
use std::str::FromStr;

const EPI_REVISION: &str = "daa660cbc1b8c5da83828698665a753852cb0287";
const QL_CODE_REVISION: &str = "de7d50c9f7dcfec33cfa0fd5f8a8a1068b4fbe84";
const BIMBA_PARENT: &str = "#1-4";
const BIMBA_PREDECESSOR: &str = "#1-4.1";
const BIMBA_INVERSION: &str = "#1-4.2";
const RUN_REF: &str = "run:01ARZ3NDEKTSV4RRFFQ69G5FAV";

fn source(reference: &str, revision: &str) -> StructuralSourceRef {
    StructuralSourceRef {
        reference: reference.into(),
        revision: Some(revision.into()),
    }
}

fn implementation_binding(revision: &str) -> StructuralBinding {
    StructuralBinding {
        identity: BIMBA_INVERSION.into(),
        implementation_ref: "github:EpiLogos/QL-MEF:c/src/primitive.c#ql_position_invert".into(),
        relation: "implemented-by".into(),
        implementation_revision: Some(revision.into()),
    }
}

fn source_relations() -> Vec<StructuralRelation> {
    vec![
        StructuralRelation {
            from_identity: BIMBA_PARENT.into(),
            to_identity: BIMBA_INVERSION.into(),
            relation: "CONTAINS_LOGIC_STAGE".into(),
        },
        StructuralRelation {
            from_identity: BIMBA_PREDECESSOR.into(),
            to_identity: BIMBA_INVERSION.into(),
            relation: "INVERTS_INTO".into(),
        },
    ]
}

fn ground() -> StructuralGround {
    StructuralGround {
        version: STRUCTURAL_GROUND_VERSION.into(),
        id: "epi-holographic-specimen/position-inversion/v1".into(),
        source_refs: vec![
            source("github:EpiLogos/Epi-Logos-C-Experiments", EPI_REVISION),
            source("github:EpiLogos/QL-MEF", QL_CODE_REVISION),
        ],
        structural_manifest_refs: vec![
            "github:EpiLogos/Epi-Logos-C-Experiments:Idea/Bimba/Map/datasets/low-detail/nodes_paramasiva.json".into(),
            "github:EpiLogos/QL-MEF:docs/integrations/epi-logos/EPI-HOLOGRAPHIC-KERNEL-MANIFEST.json".into(),
        ],
        relation_manifest_refs: vec![
            "github:EpiLogos/Epi-Logos-C-Experiments:Idea/Bimba/Map/datasets/low-detail/relations_paramasiva.json".into(),
        ],
        in_scope_identities: vec![
            BIMBA_PARENT.into(),
            BIMBA_PREDECESSOR.into(),
            BIMBA_INVERSION.into(),
        ],
        constitutive_relations: source_relations(),
        existing_implementation_bindings: vec![implementation_binding(QL_CODE_REVISION)],
        unresolved_source_gaps: vec![StructuralGap {
            id: "bimba-live".into(),
            detail: "the exact frozen Map identity is recovered, but live Bimba MCP/Neo4j verification is not available in this execution environment".into(),
            owner_ref: Some("github:EpiLogos/Epi-Logos-C-Experiments".into()),
        }],
        return_or_mutation_law: Some(
            "returned implementation reality may revise Bimba semantic ground only through an explicit Epi source-owner change".into(),
        ),
    }
}

#[test]
fn exact_bimba_inversion_identity_is_structurally_bound_to_the_ql_c_symbol() {
    let ground = ground();
    let observed = StructuralFidelityObservation {
        source_refs: ground.source_refs.clone(),
        identities: ground.in_scope_identities.clone(),
        implementation_bindings: ground.existing_implementation_bindings.clone(),
        constitutive_relations: source_relations(),
    };

    let evidence = verify_structural_ground(RunRef::from_str(RUN_REF).unwrap(), &ground, &observed)
        .expect("source-ground fixture is valid");

    assert!(evidence.passed, "structural issues: {:?}", evidence.issues);
    assert_eq!(
        ground.existing_implementation_bindings[0].identity,
        BIMBA_INVERSION
    );
    assert_eq!(
        ground.constitutive_relations[0].relation,
        "CONTAINS_LOGIC_STAGE"
    );
    assert_eq!(ground.constitutive_relations[1].relation, "INVERTS_INTO");
    assert_eq!(evidence.unresolved_source_gaps[0].id, "bimba-live");
}

#[test]
fn exact_bimba_binding_still_rejects_a_stale_ql_code_revision() {
    let ground = ground();
    let mut observed_binding = implementation_binding("stale-ql-revision");
    observed_binding.identity = BIMBA_INVERSION.into();
    let observed = StructuralFidelityObservation {
        source_refs: ground.source_refs.clone(),
        identities: ground.in_scope_identities.clone(),
        implementation_bindings: vec![observed_binding],
        constitutive_relations: source_relations(),
    };

    let evidence = verify_structural_ground(RunRef::from_str(RUN_REF).unwrap(), &ground, &observed)
        .expect("source-ground fixture is valid");

    assert!(!evidence.passed);
    assert!(evidence
        .issues
        .iter()
        .any(|issue| format!("{:?}", issue.kind) == "StaleBinding"));
}
