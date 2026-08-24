use epilogos_factory::build::{FactoryBuildSelection, FactoryBuildState, FACTORY_NATIVE_OWNER};
use epilogos_factory::build_cognitive::{
    FactoryBuildCognitiveFocus, FactoryBuildCognitiveViewProvider,
    FACTORY_BUILD_COGNITIVE_PROVIDER_CONTRACT, FACTORY_BUILD_COGNITIVE_VIEW_CONTRACT,
};
use epilogos_factory::core::run::{
    PassageAnchor, Project, ProjectRef, Run, RunRef, RunThought, RunThoughtCommand, RunThoughtId,
    RunThoughtLifecycle, ThoughtProducer,
};
use std::str::FromStr;

const PROJECT: &str = "project:01ARZ3NDEKTSV4RRFFQ69G5FAA";
const RUN: &str = "run:01ARZ3NDEKTSV4RRFFQ69G5FAB";
const SUBJECT: &str = "run-map-subject/cognitive-frontier";
const CLAIM: &str = "claim:01ARZ3NDEKTSV4RRFFQ69G5FAD";
const EVIDENCE: &str = "evidence:01ARZ3NDEKTSV4RRFFQ69G5FAE";
const RELATION_PATTERN: &str = "authored-relation-evidence/t3-pattern";
const RELATION_ANOMALY: &str = "authored-relation-evidence/t3-prime-anomaly";

fn thought(
    id: &str,
    run_ref: &RunRef,
    anchor_ref: &str,
    session_ref: &str,
    execution_ref: &str,
    related_ref: &str,
    relation_evidence_ref: &str,
    lifecycle: RunThoughtLifecycle,
) -> RunThought {
    RunThought {
        id: RunThoughtId::new(id).unwrap(),
        run_ref: run_ref.clone(),
        anchor_ref: anchor_ref.into(),
        anchor_revision: Some("sha256:source-revision".into()),
        passage: Some(PassageAnchor {
            start_byte: 120,
            end_byte: 196,
            label: Some("retained finding".into()),
        }),
        producer: ThoughtProducer {
            agent_ref: Some("agent/mahamaya".into()),
            agency_ref: Some("agency/factory-run".into()),
            agent_session_ref: Some(session_ref.into()),
            execution_ref: Some(execution_ref.into()),
        },
        run_map_subject_refs: vec![SUBJECT.into()],
        related_refs: vec![related_ref.into()],
        relation_evidence_refs: vec![relation_evidence_ref.into()],
        lifecycle,
    }
}

fn canonical_state() -> (FactoryBuildState, FactoryBuildSelection) {
    let project_ref = ProjectRef::from_str(PROJECT).unwrap();
    let run_ref = RunRef::from_str(RUN).unwrap();
    let project = Project::new(project_ref.clone());
    let mut run = Run::new(
        run_ref.clone(),
        project_ref.clone(),
        "Retain and re-enter Run cognition",
        "factory-test-owner",
    )
    .unwrap();
    let authority = run.mutation_authority();
    let topology_revision = run.map().topology_revision();

    run.apply_thought_command(
        &authority,
        RunThoughtCommand {
            command_id: "retain-pattern".into(),
            expected_revision: run.revision(),
            thought: thought(
                "pattern-reading",
                &run_ref,
                "source/run-thinking.md",
                "agent-session/one",
                "execution/one",
                CLAIM,
                RELATION_PATTERN,
                RunThoughtLifecycle::Active,
            ),
        },
    )
    .unwrap();
    run.apply_thought_command(
        &authority,
        RunThoughtCommand {
            command_id: "retain-anomaly".into(),
            expected_revision: run.revision(),
            thought: thought(
                "anomaly-reading",
                &run_ref,
                "artifact/test-report.md",
                "agent-session/two",
                "execution/two",
                EVIDENCE,
                RELATION_ANOMALY,
                RunThoughtLifecycle::Integrated,
            ),
        },
    )
    .unwrap();

    assert_eq!(
        run.map().topology_revision(),
        topology_revision,
        "retaining cognition must not mutate RunMap topology"
    );

    let state = FactoryBuildState::new(project, run).unwrap();
    (
        state,
        FactoryBuildSelection {
            project_ref,
            run_ref,
        },
    )
}

#[test]
fn cognitive_optic_materialises_one_run_field_across_agent_sessions() {
    let (state, selection) = canonical_state();
    let provider = FactoryBuildCognitiveViewProvider;
    let before = serde_json::to_string(&state).unwrap();
    let snapshot = provider.snapshot(&state, &selection).unwrap();
    let after = serde_json::to_string(&state).unwrap();

    assert_eq!(
        before, after,
        "cognitive reads must not mutate Factory state"
    );
    assert_eq!(snapshot.contract, FACTORY_BUILD_COGNITIVE_VIEW_CONTRACT);
    assert_eq!(
        snapshot.provider_contract,
        FACTORY_BUILD_COGNITIVE_PROVIDER_CONTRACT
    );
    assert_eq!(snapshot.provenance.owner, FACTORY_NATIVE_OWNER);
    assert_eq!(snapshot.view.run_ref, RUN);
    assert_eq!(snapshot.view.thought_count, 2);
    assert_eq!(snapshot.view.thoughts.len(), 2);
    assert_eq!(snapshot.view.by_run_map_subject[SUBJECT].len(), 2);
    assert_eq!(
        snapshot.view.by_producer_ref["agent-session/one"],
        vec![RunThoughtId::new("pattern-reading").unwrap()]
    );
    assert_eq!(
        snapshot.view.by_producer_ref["agent-session/two"],
        vec![RunThoughtId::new("anomaly-reading").unwrap()]
    );
    assert_eq!(
        snapshot.view.by_anchor_ref["source/run-thinking.md"],
        vec![RunThoughtId::new("pattern-reading").unwrap()]
    );
    assert_eq!(
        snapshot.view.by_relation_evidence_ref[RELATION_ANOMALY],
        vec![RunThoughtId::new("anomaly-reading").unwrap()]
    );
    assert_eq!(snapshot.view.by_lifecycle["active"].len(), 1);
    assert_eq!(snapshot.view.by_lifecycle["integrated"].len(), 1);

    let serialised = snapshot.to_json().unwrap();
    assert!(serialised.contains(RELATION_PATTERN));
    assert!(serialised.contains(RELATION_ANOMALY));
    assert!(serialised.contains("startByte\":120"));
    assert!(serialised.contains("agent-session/one"));
    assert!(serialised.contains("agent-session/two"));
}

#[test]
fn exact_focus_returns_smallest_factory_owned_cognitive_selection() {
    let (state, selection) = canonical_state();
    let provider = FactoryBuildCognitiveViewProvider;
    let focus = FactoryBuildCognitiveFocus {
        run_map_subject_ref: Some(SUBJECT.into()),
        producer_ref: Some("agent-session/two".into()),
        relation_evidence_ref: Some(RELATION_ANOMALY.into()),
        ..FactoryBuildCognitiveFocus::default()
    };

    let snapshot = provider
        .focused_snapshot(&state, &selection, &focus)
        .unwrap();

    assert_eq!(snapshot.view.focus.as_ref(), Some(&focus));
    assert_eq!(snapshot.view.thought_count, 1);
    assert_eq!(snapshot.view.thoughts[0].id.as_str(), "anomaly-reading");
    assert_eq!(
        snapshot.view.thoughts[0].anchor_ref,
        "artifact/test-report.md"
    );
    assert_eq!(snapshot.view.thoughts[0].related_refs, vec![EVIDENCE]);
    assert_eq!(
        snapshot.view.thoughts[0].relation_evidence_refs,
        vec![RELATION_ANOMALY]
    );
}
