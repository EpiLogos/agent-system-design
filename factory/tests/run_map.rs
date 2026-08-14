use epilogos_factory::core::identity::{Ref, Revision};
use epilogos_factory::core::run::{
    resolve_run_projection, EdgeKind, NodeId, NodeKind, NodeState, Project, ProjectRef, Run,
    RunContractError, RunProjectionIdentity, RunRef, RunRegistry, RunTopologyCommand, TopologyEdge,
    TopologyError, TopologyMutation, TopologyNode,
};
use serde_json::{json, Value};

const FIXTURE: &str = include_str!("../../contracts/factory/fixtures/run-map-cases.json");
const SCHEMA: &str = include_str!("../../contracts/factory/run-map.schema.json");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("Run Map fixture must be JSON")
}

fn project_ref() -> ProjectRef {
    fixture()["projectRef"].as_str().unwrap().parse().unwrap()
}

fn run_ref() -> RunRef {
    fixture()["runRef"].as_str().unwrap().parse().unwrap()
}

fn node_id(value: &str) -> NodeId {
    NodeId::new(value).unwrap()
}

fn work_node(id: &str, label: &str, state: NodeState) -> TopologyNode {
    TopologyNode {
        id: node_id(id),
        kind: NodeKind::Work,
        label: label.to_owned(),
        state: Some(state),
        semantic_ref: None,
    }
}

fn edge(from: &str, to: &str, relation: EdgeKind) -> TopologyEdge {
    TopologyEdge {
        from: node_id(from),
        to: node_id(to),
        relation,
    }
}

fn run() -> Run {
    let value = fixture();
    Run::new(
        run_ref(),
        project_ref(),
        value["destination"].as_str().unwrap(),
        value["initialWriteOwner"].as_str().unwrap(),
    )
    .unwrap()
}

fn apply(run: &mut Run, command_id: &str, mutation: TopologyMutation) {
    let authority = run.mutation_authority();
    run.apply_topology_command(
        &authority,
        RunTopologyCommand {
            command_id: command_id.to_owned(),
            expected_revision: run.revision(),
            mutation,
        },
    )
    .unwrap();
}

fn add_two_work_nodes(run: &mut Run) {
    apply(
        run,
        "add-work-nodes",
        TopologyMutation::Batch {
            mutations: vec![
                TopologyMutation::AddNode {
                    node: work_node("work-a", "Establish topology", NodeState::Ready),
                },
                TopologyMutation::AddNode {
                    node: work_node("work-b", "Verify topology", NodeState::Planned),
                },
                TopologyMutation::AddEdge {
                    edge: edge("destination", "work-a", EdgeKind::BranchesTo),
                },
                TopologyMutation::AddEdge {
                    edge: edge("work-a", "work-b", EdgeKind::BranchesTo),
                },
            ],
        },
    );
}

#[test]
fn project_and_run_refs_are_typed_and_project_is_not_a_repository() {
    let project = Project::new(project_ref());
    assert_eq!(
        project.reference().to_string(),
        fixture()["projectRef"].as_str().unwrap()
    );
    assert_eq!(project.revision(), Revision::INITIAL);
    assert!(fixture()["projectRef"]
        .as_str()
        .unwrap()
        .parse::<RunRef>()
        .is_err());
    assert!(fixture()["runRef"]
        .as_str()
        .unwrap()
        .parse::<ProjectRef>()
        .is_err());
}

#[test]
fn one_run_has_one_derived_map_address_without_independent_map_ref() {
    let run = run();
    assert_eq!(run.map().run_ref(), run.reference());
    assert_eq!(
        run.map().address().to_string(),
        fixture()["mapAddress"].as_str().unwrap()
    );
    assert_eq!(run.map().topology_revision(), Revision::INITIAL);
    assert_eq!(
        run.map()
            .nodes()
            .values()
            .filter(|node| node.kind == NodeKind::Destination)
            .count(),
        1
    );
}

#[test]
fn run_survives_host_and_session_replacement_without_identity_change() {
    let value = fixture();
    let mut run = run();
    let original_ref = run.reference().clone();
    let original_map_address = run.map().address();
    let original_topology_revision = run.map().topology_revision();
    let old_authority = run.mutation_authority();
    let new_authority = run
        .transfer_write_authority(
            &old_authority,
            run.revision(),
            value["replacementWriteOwner"].as_str().unwrap(),
        )
        .unwrap();

    assert_eq!(run.reference(), &original_ref);
    assert_eq!(run.map().address(), original_map_address);
    assert_eq!(run.map().topology_revision(), original_topology_revision);
    assert_eq!(run.write_authority().owner(), "host-beta");
    assert_eq!(run.write_authority().epoch(), 2);

    let serialized = serde_json::to_string(&run).unwrap();
    for session in value["sessionReplacements"].as_array().unwrap() {
        assert!(!serialized.contains(session.as_str().unwrap()));
    }

    let stale_authority_result = run.apply_topology_command(
        &old_authority,
        RunTopologyCommand {
            command_id: "old-host-command".to_owned(),
            expected_revision: run.revision(),
            mutation: TopologyMutation::SetNodeState {
                node_id: node_id("destination"),
                state: NodeState::Ready,
            },
        },
    );
    assert!(matches!(
        stale_authority_result,
        Err(RunContractError::InvalidMutationAuthority)
    ));
    assert_eq!(new_authority, run.mutation_authority());
}

#[test]
fn registry_rejects_second_canonical_map_for_same_run() {
    let run = run();
    let mut registry = RunRegistry::default();
    registry.insert(run.clone()).unwrap();
    assert!(matches!(
        registry.insert(run),
        Err(RunContractError::DuplicateCanonicalRunMap(found)) if found == run_ref()
    ));
}

#[test]
fn stale_revision_rejects_topology_mutation_without_change() {
    let mut run = run();
    add_two_work_nodes(&mut run);
    let before = run.clone();
    let authority = run.mutation_authority();
    let result = run.apply_topology_command(
        &authority,
        RunTopologyCommand {
            command_id: "stale-write".to_owned(),
            expected_revision: Revision::INITIAL,
            mutation: TopologyMutation::SetNodeState {
                node_id: node_id("work-a"),
                state: NodeState::Active,
            },
        },
    );
    assert!(matches!(
        result,
        Err(RunContractError::RevisionConflict { .. })
    ));
    assert_eq!(run, before);
}

#[test]
fn invalid_topology_batch_is_atomic_and_requires_cycles_are_rejected() {
    let mut run = run();
    add_two_work_nodes(&mut run);
    let before = run.clone();
    let authority = run.mutation_authority();
    let result = run.apply_topology_command(
        &authority,
        RunTopologyCommand {
            command_id: "requires-cycle".to_owned(),
            expected_revision: run.revision(),
            mutation: TopologyMutation::Batch {
                mutations: vec![
                    TopologyMutation::AddEdge {
                        edge: edge("work-a", "work-b", EdgeKind::Requires),
                    },
                    TopologyMutation::AddEdge {
                        edge: edge("work-b", "work-a", EdgeKind::Requires),
                    },
                ],
            },
        },
    );
    assert!(matches!(
        result,
        Err(RunContractError::Topology(TopologyError::RequiresCycle))
    ));
    assert_eq!(run, before);
}

#[test]
fn returns_to_is_the_legal_back_edge_without_becoming_dependency_cycle() {
    let mut run = run();
    add_two_work_nodes(&mut run);
    let prior_topology_revision = run.map().topology_revision();
    apply(
        &mut run,
        "return-to-work-a",
        TopologyMutation::AddEdge {
            edge: edge("work-b", "work-a", EdgeKind::ReturnsTo),
        },
    );
    assert!(run
        .map()
        .edges()
        .iter()
        .any(|edge| edge.relation == EdgeKind::ReturnsTo));
    assert_eq!(
        run.map().topology_revision().get(),
        prior_topology_revision.get() + 1
    );
}

#[test]
fn invalid_dangling_and_second_destination_mutations_are_rejected() {
    let mut run = run();
    let initial = run.clone();
    let authority = run.mutation_authority();
    let dangling = run.apply_topology_command(
        &authority,
        RunTopologyCommand {
            command_id: "dangling".to_owned(),
            expected_revision: run.revision(),
            mutation: TopologyMutation::AddEdge {
                edge: edge("destination", "missing", EdgeKind::BranchesTo),
            },
        },
    );
    assert!(matches!(
        dangling,
        Err(RunContractError::Topology(TopologyError::MissingNode(_)))
    ));
    assert_eq!(run, initial);

    let authority = run.mutation_authority();
    let duplicate_destination = run.apply_topology_command(
        &authority,
        RunTopologyCommand {
            command_id: "second-destination".to_owned(),
            expected_revision: run.revision(),
            mutation: TopologyMutation::Batch {
                mutations: vec![
                    TopologyMutation::AddNode {
                        node: TopologyNode {
                            id: node_id("destination-two"),
                            kind: NodeKind::Destination,
                            label: "Competing destination".to_owned(),
                            state: None,
                            semantic_ref: None,
                        },
                    },
                    TopologyMutation::AddEdge {
                        edge: edge("destination", "destination-two", EdgeKind::BranchesTo),
                    },
                ],
            },
        },
    );
    assert!(matches!(
        duplicate_destination,
        Err(RunContractError::Topology(TopologyError::DestinationCount(
            2
        )))
    ));
    assert_eq!(run, initial);
}

#[test]
fn semantic_topology_nodes_keep_later_ticket_objects_as_refs() {
    let mut run = run();
    let decision_ref: Ref = "decision:01ARZ3NDEKTSV4RRFFQ69G5FAY".parse().unwrap();
    apply(
        &mut run,
        "decision-node",
        TopologyMutation::Batch {
            mutations: vec![
                TopologyMutation::AddNode {
                    node: TopologyNode {
                        id: node_id("decision-a"),
                        kind: NodeKind::Decision,
                        label: "Choose route".to_owned(),
                        state: None,
                        semantic_ref: Some(decision_ref),
                    },
                },
                TopologyMutation::AddEdge {
                    edge: edge("destination", "decision-a", EdgeKind::BranchesTo),
                },
            ],
        },
    );
    assert_eq!(
        run.map().nodes().get(&node_id("decision-a")).unwrap().kind,
        NodeKind::Decision
    );
}

#[test]
fn projection_change_preserves_run_ref_and_projection_loss_is_not_guessed() {
    let value = fixture();
    let expected = run_ref();
    for projection in value["projectionChanges"].as_array().unwrap() {
        let projection: RunProjectionIdentity = serde_json::from_value(projection.clone()).unwrap();
        assert_eq!(resolve_run_projection(&projection).unwrap(), expected);
    }
    let lost: RunProjectionIdentity =
        serde_json::from_value(value["projectionLoss"].clone()).unwrap();
    assert!(matches!(
        resolve_run_projection(&lost),
        Err(RunContractError::MissingCanonicalRunRef { .. })
    ));
}

#[test]
fn registry_round_trip_preserves_run_map_and_revision_authority() {
    let mut run = run();
    add_two_work_nodes(&mut run);
    let mut registry = RunRegistry::default();
    registry.insert(run.clone()).unwrap();
    let serialized = registry.to_json().unwrap();
    let restored = RunRegistry::from_json(&serialized).unwrap();
    assert_eq!(restored.get(run.reference()), Some(&run));
}

#[test]
fn schema_has_singular_map_no_independent_map_id_and_no_stored_frontier() {
    let schema: Value = serde_json::from_str(SCHEMA).unwrap();
    assert_eq!(schema["$id"], json!("factory.run-map.schema/v1"));
    let run_properties = schema["$defs"]["run"]["properties"].as_object().unwrap();
    assert!(run_properties.contains_key("map"));
    assert!(!run_properties.contains_key("maps"));
    let map_properties = schema["$defs"]["runMap"]["properties"].as_object().unwrap();
    assert!(!map_properties.contains_key("id"));
    assert!(!map_properties.contains_key("frontier"));
    assert_eq!(
        schema["$defs"]["mapAddress"]["pattern"],
        json!("^run:[0-7][0-9A-HJKMNP-TV-Z]{25}/map$")
    );
}
