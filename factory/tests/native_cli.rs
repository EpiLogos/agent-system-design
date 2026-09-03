use epilogos_factory::action_projection::FACTORY_ACTION_PROJECTION_CONTRACT;
use epilogos_factory::build::{
    CandidateRecord, FactoryBuildSelection, FactoryBuildState, FACTORY_NATIVE_OWNER,
    REQUEST_MORE_EVIDENCE_ACTION_REF, REQUEST_MORE_EVIDENCE_CAPABILITY_REF,
};
use epilogos_factory::build_provider::FactoryBuildFileProvider;
use epilogos_factory::cli::execute_cli;
use epilogos_factory::core::run::{Project, ProjectRef, Run, RunRef};
use serde_json::{json, Value};
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

const PROJECT: &str = "project:01ARZ3NDEKTSV4RRFFQ69G5FCA";
const RUN: &str = "run:01ARZ3NDEKTSV4RRFFQ69G5FCB";
const CANDIDATE: &str = "candidate:01ARZ3NDEKTSV4RRFFQ69G5FCC";

#[test]
fn cli_snapshot_refresh_and_action_list_are_the_native_provider_reading() {
    let fixture = fixture("reading");
    let snapshot = cli_json(&[
        "build",
        "snapshot",
        fixture.path.to_str().unwrap(),
        PROJECT,
        RUN,
        "--json",
    ]);
    let refresh = cli_json(&[
        "build",
        "refresh",
        fixture.path.to_str().unwrap(),
        PROJECT,
        RUN,
        "--json",
    ]);
    assert_eq!(snapshot["contract"], "factory.build-view/v1");
    assert_eq!(snapshot["providerContract"], "factory.build-view-provider/v1");
    assert_eq!(snapshot, refresh);

    let actions = cli_json(&[
        "action",
        "list",
        fixture.path.to_str().unwrap(),
        PROJECT,
        RUN,
        "--json",
    ]);
    assert_eq!(actions, snapshot["view"]["actions"]);
    assert_eq!(actions[0]["actionRef"], REQUEST_MORE_EVIDENCE_ACTION_REF);

    fixture.remove();
}

#[test]
fn cli_invoke_matches_headless_projection_and_fresh_provider_observes_the_mutation() {
    let cli_fixture = fixture("cli");
    let headless_fixture = fixture("headless");
    let request = projected_request();
    let request_json = serde_json::to_string(&request).unwrap();

    let cli_receipt = execute_cli(
        &strings(&[
            "action",
            "invoke",
            cli_fixture.path.to_str().unwrap(),
            PROJECT,
            RUN,
            "-",
            "--json",
        ]),
        Some(&request_json),
    )
    .unwrap();
    let cli_receipt: Value = serde_json::from_str(&cli_receipt).unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_factory-action-headless"))
        .args([
            headless_fixture.path.to_str().unwrap(),
            PROJECT,
            RUN,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(request_json.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "headless projection failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let headless_receipt: Value = serde_json::from_slice(&output.stdout).unwrap();

    assert_eq!(cli_receipt, headless_receipt);
    assert_eq!(cli_receipt["caller"]["callerRef"], "agent:factory-cli-acceptance");
    assert_eq!(
        cli_receipt["caller"]["lineage"],
        json!(["agency:acceptance", "agent:factory-cli-acceptance"])
    );
    assert_eq!(cli_receipt["authorityRef"], "authority:factory-cli-acceptance");
    assert_eq!(cli_receipt["nativeResult"]["actionRef"], REQUEST_MORE_EVIDENCE_ACTION_REF);
    assert!(
        cli_receipt["nativeResult"]["nextRevision"].as_u64().unwrap()
            > cli_receipt["nativeResult"]["previousRevision"].as_u64().unwrap()
    );

    let selection = selection();
    let reopened = FactoryBuildFileProvider::open(&cli_fixture.path, selection).unwrap();
    let persisted = reopened.snapshot().unwrap();
    assert_eq!(persisted.view.human_requests.len(), 1);
    assert_eq!(
        persisted.revision,
        cli_receipt["nativeResult"]["nextRevision"].as_u64().unwrap()
    );
    assert_eq!(
        persisted.view.human_requests[0].human_request_ref,
        cli_receipt["nativeResult"]["createdHumanRequestRef"]
            .as_str()
            .unwrap()
    );

    cli_fixture.remove();
    headless_fixture.remove();
}

#[test]
fn cli_retains_native_projection_and_provider_errors() {
    let fixture = fixture("errors");
    let mut bad_lineage = projected_request();
    bad_lineage["caller"]["lineage"] = json!(["agency:acceptance", "agent:different"]);
    let error = execute_cli(
        &strings(&[
            "action",
            "invoke",
            fixture.path.to_str().unwrap(),
            PROJECT,
            RUN,
            "-",
            "--json",
        ]),
        Some(&serde_json::to_string(&bad_lineage).unwrap()),
    )
    .unwrap_err();
    assert!(error.to_string().contains("caller lineage must terminate at caller_ref"));

    let mut wrong_owner = projected_request();
    wrong_owner["authority"]["nativeOwner"] = json!("not-factory");
    let error = execute_cli(
        &strings(&[
            "action",
            "invoke",
            fixture.path.to_str().unwrap(),
            PROJECT,
            RUN,
            "-",
            "--json",
        ]),
        Some(&serde_json::to_string(&wrong_owner).unwrap()),
    )
    .unwrap_err();
    assert!(error.to_string().contains("cannot substitute native owner"));

    let error = execute_cli(
        &strings(&[
            "build",
            "snapshot",
            fixture.path.to_str().unwrap(),
            "project:01ARZ3NDEKTSV4RRFFQ69G5FZZ",
            RUN,
            "--json",
        ]),
        None,
    )
    .unwrap_err();
    assert!(error.to_string().contains("project"));

    fixture.remove();
}

#[test]
fn cli_json_is_deterministic_for_unchanged_native_state() {
    let fixture = fixture("deterministic");
    let args = strings(&[
        "build",
        "snapshot",
        fixture.path.to_str().unwrap(),
        PROJECT,
        RUN,
        "--json",
    ]);
    let first = execute_cli(&args, None).unwrap();
    let second = execute_cli(&args, None).unwrap();
    assert_eq!(first, second);

    let capabilities = strings(&["capabilities", "--json"]);
    assert_eq!(
        execute_cli(&capabilities, None).unwrap(),
        execute_cli(&capabilities, None).unwrap()
    );
    fixture.remove();
}

fn projected_request() -> Value {
    json!({
        "contract": FACTORY_ACTION_PROJECTION_CONTRACT,
        "projectionRef": "projection:factory-cli-acceptance",
        "caller": {
            "callerRef": "agent:factory-cli-acceptance",
            "projectionKind": "headless",
            "lineage": ["agency:acceptance", "agent:factory-cli-acceptance"]
        },
        "actionRef": REQUEST_MORE_EVIDENCE_ACTION_REF,
        "subjectRef": CANDIDATE,
        "runRef": RUN,
        "authority": {
            "authorityRef": "authority:factory-cli-acceptance",
            "nativeOwner": FACTORY_NATIVE_OWNER,
            "capabilityRef": REQUEST_MORE_EVIDENCE_CAPABILITY_REF,
            "capabilityGranted": true,
            "actionAuthorised": true
        }
    })
}

fn cli_json(args: &[&str]) -> Value {
    let output = execute_cli(&strings(args), None).unwrap();
    serde_json::from_str(&output).unwrap()
}

fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|value| (*value).to_owned()).collect()
}

fn selection() -> FactoryBuildSelection {
    FactoryBuildSelection {
        project_ref: ProjectRef::from_str(PROJECT).unwrap(),
        run_ref: RunRef::from_str(RUN).unwrap(),
    }
}

struct Fixture {
    path: std::path::PathBuf,
}

impl Fixture {
    fn remove(self) {
        fs::remove_file(self.path).unwrap();
    }
}

fn fixture(label: &str) -> Fixture {
    let project_ref = ProjectRef::from_str(PROJECT).unwrap();
    let run_ref = RunRef::from_str(RUN).unwrap();
    let project = Project::new(project_ref.clone());
    let run = Run::new(
        run_ref.clone(),
        project_ref,
        "Prove the Factory native CLI over canonical state",
        "factory-native-cli-test",
    )
    .unwrap();
    let mut state = FactoryBuildState::new(project, run).unwrap();
    state
        .insert_candidate(CandidateRecord {
            run_ref,
            candidate_ref: CANDIDATE.into(),
            revision: 1,
            label: "CLI Candidate".into(),
            status: "ready".into(),
            producing_execution_refs: Vec::new(),
            claim_refs: Vec::new(),
            evidence_refs: Vec::new(),
            artifact_refs: Vec::new(),
            preview_ref: None,
            tradeoffs: Vec::new(),
        })
        .unwrap();

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "epilogos-factory-native-cli-{label}-{}-{nonce}.json",
        std::process::id()
    ));
    FactoryBuildFileProvider::create(&path, selection(), state).unwrap();
    Fixture { path }
}
