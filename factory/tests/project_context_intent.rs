use epilogos_factory::core::run::RunRef;
use epilogos_factory::project_development::{
    BoundedIntentCondition, BoundedIntentReturn, BoundedIntentReturnState,
    IntentCriterionEvaluation, IntentCriterionState, PraxisCondition, ProjectDevelopmentError,
    ProjectDevelopmentLedger,
};
use std::str::FromStr;

const RUN: &str = "run:01ARZ3NDEKTSV4RRFFQ69G5FB0";
const INTENT_SOURCE: &str = "context-source:project/intent/current";
const CONTEXT_RESOLUTION: &str = "aikit:context-resolution/run-context-174";
const CRITERION_TESTS: &str = "intent-criterion:tests-green";
const CRITERION_AUTHORITY: &str = "intent-criterion:authority-preserved";
const EVIDENCE_TESTS: &str = "evidence:tests-green-174";
const EVIDENCE_AUTHORITY: &str = "evidence:authority-preserved-174";

fn run_ref() -> RunRef {
    RunRef::from_str(RUN).unwrap()
}

fn intent() -> BoundedIntentCondition {
    BoundedIntentCondition {
        run_ref: run_ref(),
        condition_ref: "project-context/intent-174".into(),
        intent_source_ref: INTENT_SOURCE.into(),
        focus_ref: Some("focus/project-context-intent-return".into()),
        success_condition_refs: vec![CRITERION_TESTS.into(), CRITERION_AUTHORITY.into()],
        constraint_refs: vec![
            "constraint:no-intent-store".into(),
            "constraint:no-second-context-resolver".into(),
        ],
        context_resolution_ref: CONTEXT_RESOLUTION.into(),
    }
}

fn praxis() -> PraxisCondition {
    PraxisCondition {
        run_ref: run_ref(),
        condition_ref: "praxis/project-context-174".into(),
        focus_ref: Some("focus/project-context-intent-return".into()),
        method_ref: None,
        skill_refs: Vec::new(),
        skill_set_refs: Vec::new(),
        profile_ref: Some("aikit:profile/code".into()),
        usage_overlay_refs: Vec::new(),
        context_source_refs: vec![INTENT_SOURCE.into()],
        action_refs: vec!["aikit:action/develop".into()],
        model_ref: None,
        harness_ref: Some("harness:code".into()),
        harness_composition_ref: None,
        agency_ref: Some("actuation:agency/factory-developer".into()),
        material_condition_refs: Vec::new(),
        resolution_ref: Some(CONTEXT_RESOLUTION.into()),
        provider_ref: "github:EpiLogos/ai-kit".into(),
        provider_revision: "04858d2997a2269c4e1c2b9ae957807af3e49bf1".into(),
    }
}

#[test]
fn bounded_intent_context_run_return_is_one_inspectable_vertical() {
    let mut ledger = ProjectDevelopmentLedger::new(run_ref());
    ledger.set_intent(intent()).unwrap();
    ledger.set_praxis(praxis()).unwrap();
    ledger
        .set_intent_return(BoundedIntentReturn {
            run_ref: run_ref(),
            return_ref: "project-context/return-174".into(),
            intent_source_ref: INTENT_SOURCE.into(),
            context_resolution_ref: CONTEXT_RESOLUTION.into(),
            artifact_refs: vec!["artifact:factory-project-context-contract".into()],
            claim_refs: vec!["claim:bounded-intent-return-is-source-preserving".into()],
            evidence_refs: vec![EVIDENCE_TESTS.into(), EVIDENCE_AUTHORITY.into()],
            criterion_evaluations: vec![
                IntentCriterionEvaluation {
                    criterion_ref: CRITERION_TESTS.into(),
                    state: IntentCriterionState::Satisfied,
                    evidence_refs: vec![EVIDENCE_TESTS.into()],
                },
                IntentCriterionEvaluation {
                    criterion_ref: CRITERION_AUTHORITY.into(),
                    state: IntentCriterionState::Satisfied,
                    evidence_refs: vec![EVIDENCE_AUTHORITY.into()],
                },
            ],
        })
        .unwrap();

    assert_eq!(
        ledger.intent_return_state(),
        Some(BoundedIntentReturnState::Satisfied)
    );
    assert_eq!(
        ledger.intent.as_ref().unwrap().intent_source_ref,
        INTENT_SOURCE
    );
    assert_eq!(
        ledger.praxis.as_ref().unwrap().resolution_ref.as_deref(),
        Some(CONTEXT_RESOLUTION)
    );

    let review = ledger.human_review();
    assert!(review
        .intention_and_ground_refs
        .contains(&INTENT_SOURCE.to_owned()));
    assert!(review.evidence_refs.contains(&EVIDENCE_TESTS.to_owned()));
    assert!(review
        .evidence_refs
        .contains(&EVIDENCE_AUTHORITY.to_owned()));
}

#[test]
fn incomplete_or_negative_return_remains_truthful() {
    let mut ledger = ProjectDevelopmentLedger::new(run_ref());
    ledger.set_intent(intent()).unwrap();

    ledger
        .set_intent_return(BoundedIntentReturn {
            run_ref: run_ref(),
            return_ref: "project-context/return-partial".into(),
            intent_source_ref: INTENT_SOURCE.into(),
            context_resolution_ref: CONTEXT_RESOLUTION.into(),
            artifact_refs: Vec::new(),
            claim_refs: Vec::new(),
            evidence_refs: vec![EVIDENCE_TESTS.into()],
            criterion_evaluations: vec![IntentCriterionEvaluation {
                criterion_ref: CRITERION_TESTS.into(),
                state: IntentCriterionState::Satisfied,
                evidence_refs: vec![EVIDENCE_TESTS.into()],
            }],
        })
        .unwrap();
    assert_eq!(
        ledger.intent_return_state(),
        Some(BoundedIntentReturnState::Indeterminate)
    );

    ledger
        .set_intent_return(BoundedIntentReturn {
            run_ref: run_ref(),
            return_ref: "project-context/return-negative".into(),
            intent_source_ref: INTENT_SOURCE.into(),
            context_resolution_ref: CONTEXT_RESOLUTION.into(),
            artifact_refs: Vec::new(),
            claim_refs: Vec::new(),
            evidence_refs: vec![EVIDENCE_TESTS.into(), EVIDENCE_AUTHORITY.into()],
            criterion_evaluations: vec![
                IntentCriterionEvaluation {
                    criterion_ref: CRITERION_TESTS.into(),
                    state: IntentCriterionState::Satisfied,
                    evidence_refs: vec![EVIDENCE_TESTS.into()],
                },
                IntentCriterionEvaluation {
                    criterion_ref: CRITERION_AUTHORITY.into(),
                    state: IntentCriterionState::Unsatisfied,
                    evidence_refs: vec![EVIDENCE_AUTHORITY.into()],
                },
            ],
        })
        .unwrap();
    assert_eq!(
        ledger.intent_return_state(),
        Some(BoundedIntentReturnState::Unsatisfied)
    );
}

#[test]
fn return_cannot_change_source_resolution_or_claim_success_without_evidence() {
    let mut ledger = ProjectDevelopmentLedger::new(run_ref());
    ledger.set_intent(intent()).unwrap();

    let wrong_resolution = ledger.set_intent_return(BoundedIntentReturn {
        run_ref: run_ref(),
        return_ref: "project-context/return-wrong-resolution".into(),
        intent_source_ref: INTENT_SOURCE.into(),
        context_resolution_ref: "aikit:context-resolution/other".into(),
        artifact_refs: Vec::new(),
        claim_refs: Vec::new(),
        evidence_refs: Vec::new(),
        criterion_evaluations: Vec::new(),
    });
    assert!(matches!(
        wrong_resolution,
        Err(ProjectDevelopmentError::ContextResolutionMismatch { .. })
    ));

    let unknown_criterion = ledger.set_intent_return(BoundedIntentReturn {
        run_ref: run_ref(),
        return_ref: "project-context/return-unknown-criterion".into(),
        intent_source_ref: INTENT_SOURCE.into(),
        context_resolution_ref: CONTEXT_RESOLUTION.into(),
        artifact_refs: Vec::new(),
        claim_refs: Vec::new(),
        evidence_refs: Vec::new(),
        criterion_evaluations: vec![IntentCriterionEvaluation {
            criterion_ref: "intent-criterion:not-from-source".into(),
            state: IntentCriterionState::Indeterminate,
            evidence_refs: Vec::new(),
        }],
    });
    assert!(matches!(
        unknown_criterion,
        Err(ProjectDevelopmentError::UnknownSuccessCondition(_))
    ));

    let unsupported_success = ledger.set_intent_return(BoundedIntentReturn {
        run_ref: run_ref(),
        return_ref: "project-context/return-no-evidence".into(),
        intent_source_ref: INTENT_SOURCE.into(),
        context_resolution_ref: CONTEXT_RESOLUTION.into(),
        artifact_refs: Vec::new(),
        claim_refs: Vec::new(),
        evidence_refs: Vec::new(),
        criterion_evaluations: vec![IntentCriterionEvaluation {
            criterion_ref: CRITERION_TESTS.into(),
            state: IntentCriterionState::Satisfied,
            evidence_refs: Vec::new(),
        }],
    });
    assert!(matches!(
        unsupported_success,
        Err(ProjectDevelopmentError::ConclusiveCriterionWithoutEvidence(_))
    ));
}
