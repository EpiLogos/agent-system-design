use epilogos_factory::core::run::RunRef;
use epilogos_factory::project_development::{
    BoundedIntentCondition, BoundedIntentReturn, IntentCriterionEvaluation, IntentCriterionState,
    ProjectDevelopmentLedger,
};
use epilogos_factory::project_development_store::{
    FileProjectDevelopmentStore, ProjectDevelopmentStore,
};
use std::fs;
use ulid::Ulid;

fn run_ref() -> RunRef {
    "run:01ARZ3NDEKTSV4RRFFQ69G5FAV"
        .parse()
        .expect("valid deterministic RunRef")
}

#[test]
fn developmental_context_survives_owner_store_reload() {
    let run_ref = run_ref();
    let mut ledger = ProjectDevelopmentLedger::new(run_ref.clone());
    ledger
        .set_intent(BoundedIntentCondition {
            run_ref: run_ref.clone(),
            condition_ref: "condition:bounded-intent".to_owned(),
            intent_source_ref: "source:intent".to_owned(),
            focus_ref: Some("focus:current".to_owned()),
            success_condition_refs: vec!["success:context-returned".to_owned()],
            constraint_refs: vec!["constraint:preserve-owner-refs".to_owned()],
            context_resolution_ref: "context-resolution:project-a".to_owned(),
        })
        .expect("bounded Intent is valid");
    ledger
        .set_intent_return(BoundedIntentReturn {
            run_ref: run_ref.clone(),
            return_ref: "return:recognized".to_owned(),
            intent_source_ref: "source:intent".to_owned(),
            context_resolution_ref: "context-resolution:project-a".to_owned(),
            artifact_refs: vec!["artifact:one".to_owned()],
            claim_refs: vec!["claim:one".to_owned()],
            evidence_refs: vec!["evidence:whole".to_owned()],
            criterion_evaluations: vec![IntentCriterionEvaluation {
                criterion_ref: "success:context-returned".to_owned(),
                state: IntentCriterionState::Satisfied,
                evidence_refs: vec!["evidence:criterion".to_owned()],
            }],
        })
        .expect("returned reality matches the bounded Intent");

    let root = std::env::temp_dir().join(format!(
        "epilogos-factory-project-development-{}",
        Ulid::new()
    ));
    let first_process = FileProjectDevelopmentStore::new(&root);
    first_process
        .save(&ledger)
        .expect("owner store persists ledger");
    drop(first_process);

    let reloaded_process = FileProjectDevelopmentStore::new(&root);
    let reloaded = reloaded_process
        .load(&run_ref)
        .expect("owner store reload succeeds")
        .expect("run-scoped ledger exists");

    assert_eq!(reloaded, ledger);
    assert_eq!(
        reloaded.intent.as_ref().unwrap().context_resolution_ref,
        "context-resolution:project-a"
    );
    assert_eq!(
        reloaded.intent_return.as_ref().unwrap().return_ref,
        "return:recognized"
    );

    fs::remove_dir_all(root).expect("test store cleanup");
}

#[test]
fn missing_run_is_not_fabricated() {
    let root = std::env::temp_dir().join(format!(
        "epilogos-factory-project-development-missing-{}",
        Ulid::new()
    ));
    let store = FileProjectDevelopmentStore::new(root);
    assert!(store
        .load(&run_ref())
        .expect("missing provider state is a valid observation")
        .is_none());
}
