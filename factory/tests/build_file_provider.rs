use epilogos_factory::build::{
    CandidateRecord, FactoryActionAuthority, FactoryActionInvocation, FactoryBuildSelection,
    FactoryBuildState, FACTORY_NATIVE_OWNER, REQUEST_MORE_EVIDENCE_ACTION_REF,
    REQUEST_MORE_EVIDENCE_CAPABILITY_REF,
};
use epilogos_factory::build_provider::FactoryBuildFileProvider;
use epilogos_factory::core::run::{Project, ProjectRef, Run, RunRef};
use std::fs;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

const PROJECT: &str = "project:01ARZ3NDEKTSV4RRFFQ69G5FCA";
const RUN: &str = "run:01ARZ3NDEKTSV4RRFFQ69G5FCB";
const CANDIDATE: &str = "candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC";

#[test]
fn local_provider_persists_canonical_mutation_and_reopens_at_new_revision() {
    let project_ref = ProjectRef::from_str(PROJECT).unwrap();
    let run_ref = RunRef::from_str(RUN).unwrap();
    let project = Project::new(project_ref.clone());
    let run = Run::new(
        run_ref.clone(),
        project_ref.clone(),
        "Persist the Factory-owned local Build provider",
        "factory-provider-test",
    )
    .unwrap();
    let mut state = FactoryBuildState::new(project, run).unwrap();
    state
        .insert_candidate(CandidateRecord {
            run_ref: run_ref.clone(),
            candidate_ref: CANDIDATE.into(),
            revision: 1,
            label: "Persistent Candidate".into(),
            status: "ready".into(),
            producing_execution_refs: Vec::new(),
            claim_refs: Vec::new(),
            evidence_refs: Vec::new(),
            artifact_refs: Vec::new(),
            preview_ref: None,
            tradeoffs: Vec::new(),
        })
        .unwrap();
    let selection = FactoryBuildSelection {
        project_ref,
        run_ref: run_ref.clone(),
    };
    let path = unique_state_path();

    let mut provider = FactoryBuildFileProvider::create(&path, selection.clone(), state).unwrap();
    let before = provider.snapshot().unwrap();
    assert!(before.view.human_requests.is_empty());

    let receipt = provider
        .execute_action(
            &FactoryActionInvocation {
                action_ref: REQUEST_MORE_EVIDENCE_ACTION_REF.into(),
                subject_ref: CANDIDATE.into(),
                run_ref,
            },
            &FactoryActionAuthority {
                authority_ref: "authority/provider-test".into(),
                native_owner: FACTORY_NATIVE_OWNER.into(),
                capability_ref: Some(REQUEST_MORE_EVIDENCE_CAPABILITY_REF.into()),
                capability_granted: true,
                action_authorised: true,
            },
        )
        .unwrap();
    let after = provider.snapshot().unwrap();
    assert!(after.revision > before.revision);
    assert_eq!(receipt.next_revision, after.revision);
    assert_eq!(after.view.human_requests.len(), 1);

    drop(provider);
    let reopened = FactoryBuildFileProvider::open(&path, selection).unwrap();
    let persisted = reopened.snapshot().unwrap();
    assert_eq!(persisted.revision, after.revision);
    assert_eq!(persisted.view.human_requests, after.view.human_requests);

    fs::remove_file(path).unwrap();
}

fn unique_state_path() -> std::path::PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "epilogos-factory-build-provider-{}-{nonce}.json",
        std::process::id()
    ))
}
