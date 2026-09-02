use epilogos_factory::action_projection::{
    execute_projected_factory_action, FactoryActionCaller, FactoryActionProjectionKind,
    FactoryActionProjectionReceipt, FactoryActionProjectionRequest,
    ProjectedFactoryActionAuthority, FACTORY_ACTION_PROJECTION_CONTRACT,
};
use epilogos_factory::build::{
    CandidateRecord, FactoryBuildSelection, FactoryBuildState, REQUEST_MORE_EVIDENCE_ACTION_REF,
    REQUEST_MORE_EVIDENCE_CAPABILITY_REF,
};
use epilogos_factory::build_provider::FactoryBuildFileProvider;
use epilogos_factory::core::run::{Project, ProjectRef, Run, RunRef};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;

const PROJECT: &str = "project:01ARZ3NDEKTSV4RRFFQ69G5FCA";
const RUN: &str = "run:01ARZ3NDEKTSV4RRFFQ69G5FCB";
const CANDIDATE: &str = "candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC";
const AUTHORITY: &str = "authority/factory-action-parity";

fn state() -> (FactoryBuildState, FactoryBuildSelection) {
    let project_ref = ProjectRef::from_str(PROJECT).unwrap();
    let run_ref = RunRef::from_str(RUN).unwrap();
    let project = Project::new(project_ref.clone());
    let run = Run::new(
        run_ref.clone(),
        project_ref.clone(),
        "Prove one Factory Action across projections",
        "factory-owner",
    )
    .unwrap();
    let mut state = FactoryBuildState::new(project, run).unwrap();
    state
        .insert_candidate(CandidateRecord {
            run_ref: run_ref.clone(),
            candidate_ref: CANDIDATE.into(),
            revision: 1,
            label: "Projection parity Candidate".into(),
            status: "ready".into(),
            producing_execution_refs: Vec::new(),
            claim_refs: Vec::new(),
            evidence_refs: vec!["evidence:projection-parity".into()],
            artifact_refs: Vec::new(),
            preview_ref: None,
            tradeoffs: Vec::new(),
        })
        .unwrap();
    (
        state,
        FactoryBuildSelection {
            project_ref,
            run_ref,
        },
    )
}

fn request(
    kind: FactoryActionProjectionKind,
    projection_ref: &str,
    caller_ref: &str,
    lineage: Vec<&str>,
) -> FactoryActionProjectionRequest {
    FactoryActionProjectionRequest {
        contract: FACTORY_ACTION_PROJECTION_CONTRACT.into(),
        projection_ref: projection_ref.into(),
        caller: FactoryActionCaller {
            caller_ref: caller_ref.into(),
            projection_kind: kind,
            lineage: lineage.into_iter().map(str::to_owned).collect(),
        },
        action_ref: REQUEST_MORE_EVIDENCE_ACTION_REF.into(),
        subject_ref: CANDIDATE.into(),
        run_ref: RUN.into(),
        authority: ProjectedFactoryActionAuthority {
            authority_ref: AUTHORITY.into(),
            native_owner: "factory".into(),
            capability_ref: Some(REQUEST_MORE_EVIDENCE_CAPABILITY_REF.into()),
            capability_granted: true,
            action_authorised: true,
        },
    }
}

fn temp_state_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "factory-action-parity-{}-{label}.json",
        std::process::id()
    ))
}

fn provider(label: &str) -> (FactoryBuildFileProvider, PathBuf) {
    let (state, selection) = state();
    let path = temp_state_path(label);
    let _ = fs::remove_file(&path);
    let provider = FactoryBuildFileProvider::create(&path, selection, state).unwrap();
    (provider, path)
}

fn assert_same_native_operation(receipts: &[FactoryActionProjectionReceipt]) {
    assert_eq!(receipts.len(), 3);
    for receipt in receipts {
        assert_eq!(receipt.contract, FACTORY_ACTION_PROJECTION_CONTRACT);
        assert_eq!(receipt.action_ref, REQUEST_MORE_EVIDENCE_ACTION_REF);
        assert_eq!(receipt.subject_ref, CANDIDATE);
        assert_eq!(receipt.run_ref, RUN);
        assert_eq!(receipt.authority_ref, AUTHORITY);
        assert_eq!(
            receipt.native_result.action_ref,
            REQUEST_MORE_EVIDENCE_ACTION_REF
        );
        assert_eq!(receipt.native_result.subject_ref, CANDIDATE);
        assert_eq!(receipt.native_result.authority_ref, AUTHORITY);
        assert_eq!(
            receipt.native_result.created_human_request_ref,
            format!("human-request/request-evidence/{CANDIDATE}")
        );
    }
    let first = &receipts[0].native_result;
    for receipt in &receipts[1..] {
        assert_eq!(
            receipt.native_result.previous_revision,
            first.previous_revision
        );
        assert_eq!(receipt.native_result.next_revision, first.next_revision);
        assert_eq!(
            receipt.native_result.created_human_request_ref,
            first.created_human_request_ref
        );
    }
}

#[test]
fn desktop_agent_and_headless_projections_share_one_action_handler_and_result_semantics() {
    let (mut desktop_provider, desktop_path) = provider("desktop");
    let (mut agent_provider, agent_path) = provider("agent");
    let (_headless_provider, headless_path) = provider("headless");

    let desktop = execute_projected_factory_action(
        &mut desktop_provider,
        &request(
            FactoryActionProjectionKind::DesktopHuman,
            "projection:desktop/factory-build",
            "human:operator",
            vec!["human:operator"],
        ),
    )
    .unwrap();
    let agent = execute_projected_factory_action(
        &mut agent_provider,
        &request(
            FactoryActionProjectionKind::SituatedAgent,
            "projection:agent/factory-build",
            "agent:mahamaya",
            vec!["human:operator", "agent:mahamaya"],
        ),
    )
    .unwrap();

    let headless_request = request(
        FactoryActionProjectionKind::Headless,
        "projection:headless/factory-build",
        "automation:parity-fixture",
        vec!["automation:parity-fixture"],
    );
    let input = serde_json::to_vec(&headless_request).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_factory-action-headless"))
        .arg(&headless_path)
        .arg(PROJECT)
        .arg(RUN)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&input).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "headless projection failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let headless: FactoryActionProjectionReceipt = serde_json::from_slice(&output.stdout).unwrap();

    assert_eq!(
        desktop.caller.projection_kind,
        FactoryActionProjectionKind::DesktopHuman
    );
    assert_eq!(desktop.caller.lineage, vec!["human:operator"]);
    assert_eq!(
        agent.caller.projection_kind,
        FactoryActionProjectionKind::SituatedAgent
    );
    assert_eq!(
        agent.caller.lineage,
        vec!["human:operator", "agent:mahamaya"]
    );
    assert_eq!(
        headless.caller.projection_kind,
        FactoryActionProjectionKind::Headless
    );
    assert_eq!(headless.caller.lineage, vec!["automation:parity-fixture"]);

    assert_same_native_operation(&[desktop, agent, headless]);

    for path in [desktop_path, agent_path, headless_path] {
        let _ = fs::remove_file(path);
    }
}

#[test]
fn projection_cannot_erase_caller_lineage_or_substitute_native_owner() {
    let (mut provider, path) = provider("rejected");

    let mut erased = request(
        FactoryActionProjectionKind::SituatedAgent,
        "projection:agent/factory-build",
        "agent:mahamaya",
        vec!["human:operator"],
    );
    let error = execute_projected_factory_action(&mut provider, &erased).unwrap_err();
    assert!(error.to_string().contains("terminate at caller_ref"));

    erased.caller.lineage = vec!["human:operator".into(), "agent:mahamaya".into()];
    erased.authority.native_owner = "projection".into();
    let error = execute_projected_factory_action(&mut provider, &erased).unwrap_err();
    assert!(error.to_string().contains("cannot substitute native owner"));

    // Rejections happened before native mutation; the valid call still starts
    // from the same canonical revision and succeeds once.
    erased.authority.native_owner = "factory".into();
    let receipt = execute_projected_factory_action(&mut provider, &erased).unwrap();
    assert_eq!(
        receipt.native_result.action_ref,
        REQUEST_MORE_EVIDENCE_ACTION_REF
    );

    let _ = fs::remove_file(path);
}
