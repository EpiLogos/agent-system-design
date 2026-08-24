use epilogos_factory::build::{FactoryBuildSelection, FactoryBuildState};
use epilogos_factory::build_cognitive::FactoryBuildCognitiveViewProvider;
use epilogos_factory::core::run::{
    PassageAnchor, Project, ProjectRef, Run, RunRef, RunThought, RunThoughtCommand, RunThoughtId,
    RunThoughtLifecycle, RunThoughtOutcome, ThoughtProducer,
};
use std::str::FromStr;

const PROJECT: &str = "project:01ARZ3NDEKTSV4RRFFQ69G5FAA";
const RUN: &str = "run:01ARZ3NDEKTSV4RRFFQ69G5FAB";
const RELATION: &str = "authored-relation-evidence/t2-challenge";

#[test]
fn live_build_retains_thought_and_next_cognitive_snapshot_sees_it() {
    let project_ref = ProjectRef::from_str(PROJECT).unwrap();
    let run_ref = RunRef::from_str(RUN).unwrap();
    let project = Project::new(project_ref.clone());
    let run = Run::new(
        run_ref.clone(),
        project_ref.clone(),
        "Develop the current Run through retained cognition",
        "factory-test-owner",
    )
    .unwrap();
    let mut state = FactoryBuildState::new(project, run).unwrap();
    let selection = FactoryBuildSelection {
        project_ref,
        run_ref: run_ref.clone(),
    };
    let authority = state.run_mutation_authority(&run_ref).unwrap();
    let provider = FactoryBuildCognitiveViewProvider;

    let before_state_revision = state.revision();
    let before_run_revision = state.run(&run_ref).unwrap().revision();
    let before_topology_revision = state.run(&run_ref).unwrap().map().topology_revision();
    assert_eq!(
        provider
            .snapshot(&state, &selection)
            .unwrap()
            .view
            .thought_count,
        0
    );

    let command = RunThoughtCommand {
        command_id: "retain-live-challenge".into(),
        expected_revision: before_run_revision,
        thought: RunThought {
            id: RunThoughtId::new("live-challenge").unwrap(),
            run_ref: run_ref.clone(),
            anchor_ref: "source/run-thinking.md".into(),
            anchor_revision: Some("sha256:live-source-revision".into()),
            passage: Some(PassageAnchor {
                start_byte: 42,
                end_byte: 88,
                label: Some("active challenge".into()),
            }),
            producer: ThoughtProducer {
                agent_ref: Some("agent/mahamaya".into()),
                agency_ref: Some("agency/factory-run".into()),
                agent_session_ref: Some("agent-session/live-build".into()),
                execution_ref: Some("execution/live-build".into()),
            },
            run_map_subject_refs: vec!["run-map-subject/current-work".into()],
            related_refs: vec!["claim:01ARZ3NDEKTSV4RRFFQ69G5FAD".into()],
            relation_evidence_refs: vec![RELATION.into()],
            lifecycle: RunThoughtLifecycle::Active,
        },
    };

    let outcome = state
        .apply_run_thought_command(&run_ref, &authority, command.clone())
        .unwrap();
    assert!(matches!(outcome, RunThoughtOutcome::Applied { .. }));
    assert!(state.revision() > before_state_revision);
    assert!(state.run(&run_ref).unwrap().revision() > before_run_revision);
    assert_eq!(
        state.run(&run_ref).unwrap().map().topology_revision(),
        before_topology_revision,
        "retaining cognition through Build must leave RunMap topology unchanged"
    );

    let after = provider.snapshot(&state, &selection).unwrap();
    assert_eq!(after.revision, state.revision().get());
    assert_eq!(after.view.thought_count, 1);
    assert_eq!(after.view.thoughts[0].id.as_str(), "live-challenge");
    assert_eq!(
        after.view.thoughts[0].relation_evidence_refs,
        vec![RELATION]
    );

    let after_applied_revision = state.revision();
    let replay = state
        .apply_run_thought_command(&run_ref, &authority, command)
        .unwrap();
    assert!(matches!(replay, RunThoughtOutcome::AlreadyApplied { .. }));
    assert_eq!(
        state.revision(),
        after_applied_revision,
        "idempotent command replay must not advance Factory Build revision"
    );
}
