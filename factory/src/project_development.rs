use crate::core::run::RunRef;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub const PROJECT_DEVELOPMENT_VERSION: &str = "factory.project-development/v1";

/// Run-scoped orientation supplied by native owners. Factory retains these refs;
/// it does not become the owner or resolver of ProjectCentral, Wiki, ProjectMap,
/// CodeIndex, or frontier state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectOrientationCondition {
    pub run_ref: RunRef,
    pub condition_ref: String,
    pub project_ref: String,
    pub projectcentral_contract_ref: Option<String>,
    pub projectcentral_revision: Option<String>,
    pub human_ground_refs: Vec<String>,
    pub semantic_wiki_refs: Vec<String>,
    pub structural_ground_ref: Option<String>,
    pub knowledge_application_ref: Option<String>,
    pub project_map_ref: Option<String>,
    pub frontier_refs: Vec<String>,
}

/// One attributable semantic <-> source <-> code route returned by the native
/// reflection/knowledge provider. `local_source_ref` is source material, not an
/// assertion that the source is implementation truth.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReflectionAnchor {
    pub run_ref: RunRef,
    pub anchor_ref: String,
    pub semantic_ref: String,
    pub local_source_ref: Option<String>,
    pub code_refs: Vec<String>,
    pub verification_refs: Vec<String>,
    pub route_ref: Option<String>,
    pub provider_ref: String,
    pub provider_revision: String,
    pub relation: String,
}

/// Smallest sufficient resolved praxis condition for a consequential Run.
/// Resolution remains AIKit-owned; Factory records the returned condition.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PraxisCondition {
    pub run_ref: RunRef,
    pub condition_ref: String,
    pub focus_ref: Option<String>,
    pub method_ref: Option<String>,
    pub skill_refs: Vec<String>,
    pub skill_set_refs: Vec<String>,
    pub profile_ref: Option<String>,
    pub usage_overlay_refs: Vec<String>,
    pub context_source_refs: Vec<String>,
    pub action_refs: Vec<String>,
    pub model_ref: Option<String>,
    pub harness_ref: Option<String>,
    pub harness_composition_ref: Option<String>,
    pub agency_ref: Option<String>,
    pub material_condition_refs: Vec<String>,
    pub resolution_ref: Option<String>,
    pub provider_ref: String,
    pub provider_revision: String,
}

/// Forward + returned capability/praxis relation. QL affinity is descriptive
/// metadata only and is never activation authority.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityPraxisRow {
    pub run_ref: RunRef,
    pub row_ref: String,
    pub capability_ref: String,
    pub skill_ref: Option<String>,
    pub method_ref: Option<String>,
    pub project_target_refs: Vec<String>,
    pub ql_affinity: Option<String>,
    pub use_type: String,
    pub context_source_refs: Vec<String>,
    pub action_refs: Vec<String>,
    pub model_ref: Option<String>,
    pub harness_ref: Option<String>,
    pub agency_ref: Option<String>,
    pub material_condition_refs: Vec<String>,
    pub verification_expectation: String,
    pub observed_fitness_evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DevelopmentObservationKind {
    ProjectReflectionDiscrepancy,
    PraxisFitness,
    InsufficientEvidence,
}

/// A pressure discovered by development that must return through the native
/// owner's mutation/Recognition path rather than being silently applied here.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OwnerReturnProposal {
    pub owner_ref: String,
    pub source_ref: Option<String>,
    pub proposal_ref: String,
    pub recognition_required: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DevelopmentObservation {
    pub run_ref: RunRef,
    pub observation_ref: String,
    pub kind: DevelopmentObservationKind,
    pub statement: String,
    pub subject_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub owner_return: Option<OwnerReturnProposal>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct HumanProjectReview {
    pub run_ref: RunRef,
    pub intention_and_ground_refs: Vec<String>,
    pub semantic_refs: Vec<String>,
    pub local_source_refs: Vec<String>,
    pub code_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub discrepancies: Vec<String>,
    pub recognition_returns: Vec<OwnerReturnProposal>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectDevelopmentLedger {
    pub version: String,
    pub run_ref: RunRef,
    pub orientation: Option<ProjectOrientationCondition>,
    pub reflection_anchors: Vec<ReflectionAnchor>,
    pub praxis: Option<PraxisCondition>,
    pub capability_rows: Vec<CapabilityPraxisRow>,
    pub observations: Vec<DevelopmentObservation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProjectDevelopmentError {
    WrongRun { expected: RunRef, actual: RunRef },
    DuplicateRef(String),
}

impl Display for ProjectDevelopmentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongRun { expected, actual } => {
                write!(formatter, "development record belongs to {actual}, expected {expected}")
            }
            Self::DuplicateRef(reference) => write!(formatter, "duplicate development ref: {reference}"),
        }
    }
}

impl Error for ProjectDevelopmentError {}

impl ProjectDevelopmentLedger {
    pub fn new(run_ref: RunRef) -> Self {
        Self {
            version: PROJECT_DEVELOPMENT_VERSION.to_owned(),
            run_ref,
            orientation: None,
            reflection_anchors: Vec::new(),
            praxis: None,
            capability_rows: Vec::new(),
            observations: Vec::new(),
        }
    }

    pub fn set_orientation(
        &mut self,
        condition: ProjectOrientationCondition,
    ) -> Result<(), ProjectDevelopmentError> {
        self.ensure_run(&condition.run_ref)?;
        self.ensure_unique(&condition.condition_ref)?;
        self.orientation = Some(condition);
        Ok(())
    }

    pub fn add_reflection_anchor(
        &mut self,
        anchor: ReflectionAnchor,
    ) -> Result<(), ProjectDevelopmentError> {
        self.ensure_run(&anchor.run_ref)?;
        self.ensure_unique(&anchor.anchor_ref)?;
        self.reflection_anchors.push(anchor);
        Ok(())
    }

    pub fn set_praxis(&mut self, condition: PraxisCondition) -> Result<(), ProjectDevelopmentError> {
        self.ensure_run(&condition.run_ref)?;
        self.ensure_unique(&condition.condition_ref)?;
        self.praxis = Some(condition);
        Ok(())
    }

    pub fn add_capability_row(
        &mut self,
        row: CapabilityPraxisRow,
    ) -> Result<(), ProjectDevelopmentError> {
        self.ensure_run(&row.run_ref)?;
        self.ensure_unique(&row.row_ref)?;
        self.capability_rows.push(row);
        Ok(())
    }

    pub fn add_observation(
        &mut self,
        observation: DevelopmentObservation,
    ) -> Result<(), ProjectDevelopmentError> {
        self.ensure_run(&observation.run_ref)?;
        self.ensure_unique(&observation.observation_ref)?;
        self.observations.push(observation);
        Ok(())
    }

    /// Reverse traversal over retained AIKit/provider results. This is not a
    /// Factory CodeIndex lookup and does not infer new mappings.
    pub fn anchors_for_code(&self, code_ref: &str) -> Vec<&ReflectionAnchor> {
        self.reflection_anchors
            .iter()
            .filter(|anchor| anchor.code_refs.iter().any(|candidate| candidate == code_ref))
            .collect()
    }

    /// Human-altitude review: intention -> meaning -> changed executable refs ->
    /// evidence -> discrepancy -> Recognition return. Exact refs remain available
    /// for progressive inspection without making graph topology the review surface.
    pub fn human_review(&self) -> HumanProjectReview {
        let intention_and_ground_refs = self
            .orientation
            .as_ref()
            .map(|orientation| orientation.human_ground_refs.clone())
            .unwrap_or_default();

        let mut semantic_refs = Vec::new();
        let mut local_source_refs = Vec::new();
        let mut code_refs = Vec::new();
        let mut evidence_refs = Vec::new();

        for anchor in &self.reflection_anchors {
            push_unique(&mut semantic_refs, anchor.semantic_ref.clone());
            if let Some(source_ref) = &anchor.local_source_ref {
                push_unique(&mut local_source_refs, source_ref.clone());
            }
            for code_ref in &anchor.code_refs {
                push_unique(&mut code_refs, code_ref.clone());
            }
            for evidence_ref in &anchor.verification_refs {
                push_unique(&mut evidence_refs, evidence_ref.clone());
            }
        }

        let mut discrepancies = Vec::new();
        let mut recognition_returns = Vec::new();
        for observation in &self.observations {
            for evidence_ref in &observation.evidence_refs {
                push_unique(&mut evidence_refs, evidence_ref.clone());
            }
            if matches!(
                observation.kind,
                DevelopmentObservationKind::ProjectReflectionDiscrepancy
            ) {
                discrepancies.push(observation.statement.clone());
            }
            if let Some(owner_return) = &observation.owner_return {
                if owner_return.recognition_required {
                    recognition_returns.push(owner_return.clone());
                }
            }
        }

        HumanProjectReview {
            run_ref: self.run_ref.clone(),
            intention_and_ground_refs,
            semantic_refs,
            local_source_refs,
            code_refs,
            evidence_refs,
            discrepancies,
            recognition_returns,
        }
    }

    fn ensure_run(&self, actual: &RunRef) -> Result<(), ProjectDevelopmentError> {
        if actual == &self.run_ref {
            Ok(())
        } else {
            Err(ProjectDevelopmentError::WrongRun {
                expected: self.run_ref.clone(),
                actual: actual.clone(),
            })
        }
    }

    fn ensure_unique(&self, candidate: &str) -> Result<(), ProjectDevelopmentError> {
        let duplicate = self
            .orientation
            .as_ref()
            .map(|record| record.condition_ref.as_str() == candidate)
            .unwrap_or(false)
            || self
                .praxis
                .as_ref()
                .map(|record| record.condition_ref.as_str() == candidate)
                .unwrap_or(false)
            || self
                .reflection_anchors
                .iter()
                .any(|record| record.anchor_ref == candidate)
            || self.capability_rows.iter().any(|record| record.row_ref == candidate)
            || self
                .observations
                .iter()
                .any(|record| record.observation_ref == candidate);

        if duplicate {
            Err(ProjectDevelopmentError::DuplicateRef(candidate.to_owned()))
        } else {
            Ok(())
        }
    }
}

fn push_unique(target: &mut Vec<String>, value: String) {
    if !target.contains(&value) {
        target.push(value);
    }
}
