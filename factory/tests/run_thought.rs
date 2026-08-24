use epilogos_factory::core::identity::Revision;
use epilogos_factory::core::run::{
    PassageAnchor, ProjectRef, Run, RunContractError, RunRef, RunRegistry, RunThought,
    RunThoughtCommand, RunThoughtId, RunThoughtLifecycle, RunThoughtOutcome, ThoughtFieldError,
    ThoughtProducer,
};
use serde_json::Value;

fn run_ref() -> RunRef {
    "run:01ARZ3NDEKTSV4RRFFQ69G5FAV".parse().unwrap()
}

fn other_run_ref() -> RunRef {
    "run:01ARZ3NDEKTSV4RRFFQ69G5FAW".parse().unwrap()
}

fn project_ref() -> ProjectRef {
    "project:01ARZ3NDEKTSV4RRFFQ69G5FAY".parse().unwrap()
}

fn run() -> Run {
    Run::new(run_ref(), project_ref(), "Develop retained cognition", "host-alpha").unwrap()
}

fn thought(
    id: &str,
    run_ref: RunRef,
    agent_session_ref: &str,
    anchor_ref: &str,
) -> RunThought {
    RunThought {
        id: RunThoughtId::new(id).unwrap(),
        run_ref,
        anchor_ref: anchor_ref.to_owned(),
        anchor_revision: Some("rev-7".to_owned()),
        passage: Some(PassageAnchor {
            start_byte: 24,
            end_byte: 80,
            label: Some("retained finding".to_owned()),
        }),
        producer: ThoughtProducer {
            agent_ref: Some("agent:aletheia-candidate".to_owned()),
            agency_ref: Some("agency:run-analysis".to_owned()),
            agent_session_ref: Some(agent_session_ref.to_owned()),
            execution_ref: Some(format!("execution:{id}")),
        },
        run_map_subject_refs: vec!["work:inspect-current-owner".to_owned()],
        related_refs: vec!["evidence:current-run-contract".to_owned()],
        relation_evidence_refs: vec!["knowledge-relation:authored-t3".to_owned()],
        lifecycle: RunThoughtLifecycle::Active,
    }
}

fn retain(run: &mut Run, command_id: &str, thought: RunThought) -> RunThoughtOutcome {
    let authority = run.mutation_authority();
    run.apply_thought_command(
        &authority,
        RunThoughtCommand {
            command_id: command_id.to_owned(),
            expected_revision: run.revision(),
            thought,
        },
    )
    .unwrap()
}

#[test]
fn one_run_holds_cognition_from_several_agent_sessions() {
    let mut run = run();
    retain(
        &mut run,
        "retain-question",
        thought("question", run_ref(), "session:a", "source:run-notes-a"),
    );
    retain(
        &mut run,
        "retain-pattern",
        thought("pattern", run_ref(), "session:b", "source:run-notes-b"),
    );

    assert_eq!(run.thought_field().thoughts().len(), 2);
    assert_eq!(
        run.thought_field()
            .for_run_map_subject("work:inspect-current-owner")
            .len(),
        2
    );
    assert_eq!(run.thought_field().by_producer_ref("session:a").len(), 1);
    assert_eq!(run.thought_field().by_producer_ref("session:b").len(), 1);
    assert_eq!(
        run.thought_field()
            .related_to("evidence:current-run-contract")
            .len(),
        2
    );
}

#[test]
fn retaining_thought_advances_run_revision_without_mutating_run_map() {
    let mut run = run();
    let topology_revision = run.map().topology_revision();
    let outcome = retain(
        &mut run,
        "retain-insight",
        thought("insight", run_ref(), "session:a", "artifact:inspection"),
    );

    assert_eq!(
        outcome,
        RunThoughtOutcome::Applied {
            revision: Revision::new(2).unwrap()
        }
    );
    assert_eq!(run.map().topology_revision(), topology_revision);
    assert_eq!(run.thought_field().thoughts().len(), 1);
}

#[test]
fn thought_command_is_idempotent_without_duplicate_retention() {
    let mut run = run();
    let thought = thought("trace", run_ref(), "session:a", "source:trace");
    let authority = run.mutation_authority();
    let command = RunThoughtCommand {
        command_id: "retain-trace".to_owned(),
        expected_revision: run.revision(),
        thought: thought.clone(),
    };
    let first = run.apply_thought_command(&authority, command.clone()).unwrap();
    assert!(matches!(first, RunThoughtOutcome::Applied { .. }));

    let replay = run.apply_thought_command(&authority, command).unwrap();
    assert_eq!(
        replay,
        RunThoughtOutcome::AlreadyApplied {
            revision: run.revision()
        }
    );
    assert_eq!(run.thought_field().thoughts().len(), 1);
}

#[test]
fn cross_run_thought_is_rejected_atomically() {
    let mut run = run();
    let before = run.clone();
    let authority = run.mutation_authority();
    let result = run.apply_thought_command(
        &authority,
        RunThoughtCommand {
            command_id: "wrong-run".to_owned(),
            expected_revision: run.revision(),
            thought: thought("question", other_run_ref(), "session:a", "source:wrong"),
        },
    );

    assert!(matches!(
        result,
        Err(RunContractError::Thought(ThoughtFieldError::WrongRun { .. }))
    ));
    assert_eq!(run, before);
}

#[test]
fn duplicate_run_local_thought_id_is_rejected() {
    let mut run = run();
    retain(
        &mut run,
        "first-question",
        thought("question", run_ref(), "session:a", "source:first"),
    );
    let before = run.clone();
    let authority = run.mutation_authority();
    let result = run.apply_thought_command(
        &authority,
        RunThoughtCommand {
            command_id: "second-question".to_owned(),
            expected_revision: run.revision(),
            thought: thought("question", run_ref(), "session:b", "source:second"),
        },
    );

    assert!(matches!(
        result,
        Err(RunContractError::Thought(ThoughtFieldError::DuplicateThought(_)))
    ));
    assert_eq!(run, before);
}

#[test]
fn invalid_passage_anchor_is_rejected() {
    let mut run = run();
    let mut invalid = thought("lacuna", run_ref(), "session:a", "source:review");
    invalid.passage = Some(PassageAnchor {
        start_byte: 80,
        end_byte: 24,
        label: None,
    });
    let authority = run.mutation_authority();
    let result = run.apply_thought_command(
        &authority,
        RunThoughtCommand {
            command_id: "bad-range".to_owned(),
            expected_revision: run.revision(),
            thought: invalid,
        },
    );

    assert!(matches!(
        result,
        Err(RunContractError::Thought(
            ThoughtFieldError::InvalidPassageRange { .. }
        ))
    ));
}

#[test]
fn host_replacement_preserves_run_thought_identity_and_content() {
    let mut run = run();
    retain(
        &mut run,
        "retain-discovery",
        thought("discovery", run_ref(), "session:a", "source:discovery"),
    );
    let thought_before = run
        .thought_field()
        .get(&RunThoughtId::new("discovery").unwrap())
        .unwrap()
        .clone();
    let old_authority = run.mutation_authority();
    run.transfer_write_authority(&old_authority, run.revision(), "host-beta")
        .unwrap();

    assert_eq!(
        run.thought_field()
            .get(&RunThoughtId::new("discovery").unwrap()),
        Some(&thought_before)
    );
    assert_eq!(run.reference(), &run_ref());
}

#[test]
fn registry_round_trip_preserves_source_backed_thought_field() {
    let mut run = run();
    retain(
        &mut run,
        "retain-integration",
        thought(
            "integration",
            run_ref(),
            "session:retrospective",
            "source:run-review",
        ),
    );
    let mut registry = RunRegistry::default();
    registry.insert(run.clone()).unwrap();

    let serialized = registry.to_json().unwrap();
    assert!(serialized.contains("thoughtField"));
    assert!(serialized.contains("relationEvidenceRefs"));
    let restored = RunRegistry::from_json(&serialized).unwrap();
    assert_eq!(restored.get(run.reference()), Some(&run));
}

#[test]
fn historical_run_json_without_thought_field_remains_readable() {
    let run = run();
    let mut registry = RunRegistry::default();
    registry.insert(run.clone()).unwrap();
    let mut value: Value = serde_json::from_str(&registry.to_json().unwrap()).unwrap();
    let runs = value["runs"].as_object_mut().unwrap();
    let stored_run = runs.get_mut(&run_ref().to_string()).unwrap().as_object_mut().unwrap();
    stored_run.remove("thoughtField");

    let restored = RunRegistry::from_json(&serde_json::to_string(&value).unwrap()).unwrap();
    let restored_run = restored.get(&run_ref()).unwrap();
    assert!(restored_run.thought_field().thoughts().is_empty());
    assert_eq!(restored_run.map(), run.map());
}
