use crate::core::run::RunRef;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const STRUCTURAL_GROUND_VERSION: &str = "factory.structural-ground/v1";

/// Opaque source identity and revision supplied by the target world.
///
/// Factory deliberately does not interpret repository-specific coordinate,
/// ontology, graph, or schema syntax here. It needs only stable identity,
/// provenance and the acceptance relation declared by the target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralSourceRef {
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(default)]
    pub revision: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralBinding {
    pub identity: String,
    pub implementation_ref: String,
    pub relation: String,
    #[serde(default)]
    pub implementation_revision: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralRelation {
    pub from_identity: String,
    pub to_identity: String,
    pub relation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralGap {
    pub id: String,
    pub detail: String,
    #[serde(default)]
    pub owner_ref: Option<String>,
}

/// The smallest Factory-owned account required to preserve an already-authored
/// target structure through development.
///
/// The target still owns the manifests and identities referenced here. Factory
/// owns only the obligation to keep the declared ground inspectable and to fail
/// structural acceptance when observed implementation silently diverges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralGround {
    pub version: String,
    pub id: String,
    pub source_refs: Vec<StructuralSourceRef>,
    #[serde(default)]
    pub structural_manifest_refs: Vec<String>,
    #[serde(default)]
    pub relation_manifest_refs: Vec<String>,
    #[serde(default)]
    pub in_scope_identities: Vec<String>,
    #[serde(default)]
    pub constitutive_relations: Vec<StructuralRelation>,
    #[serde(default)]
    pub existing_implementation_bindings: Vec<StructuralBinding>,
    #[serde(default)]
    pub unresolved_source_gaps: Vec<StructuralGap>,
    #[serde(default)]
    pub return_or_mutation_law: Option<String>,
}

impl StructuralGround {
    pub fn validate(&self) -> Result<(), StructuralGroundError> {
        if self.version != STRUCTURAL_GROUND_VERSION {
            return Err(StructuralGroundError::InvalidVersion(self.version.clone()));
        }
        require_text("id", &self.id)?;
        if self.source_refs.is_empty() {
            return Err(StructuralGroundError::NoSourceRefs);
        }
        if self.structural_manifest_refs.is_empty() {
            return Err(StructuralGroundError::NoStructuralManifestRefs);
        }

        let identities = unique_non_empty("in_scope_identities", &self.in_scope_identities)?;

        for source in &self.source_refs {
            require_text("source_ref", &source.reference)?;
            if let Some(revision) = &source.revision {
                require_text("source_revision", revision)?;
            }
        }
        for reference in self
            .structural_manifest_refs
            .iter()
            .chain(self.relation_manifest_refs.iter())
        {
            require_text("manifest_ref", reference)?;
        }

        for binding in &self.existing_implementation_bindings {
            require_text("binding.identity", &binding.identity)?;
            require_text("binding.implementation_ref", &binding.implementation_ref)?;
            require_text("binding.relation", &binding.relation)?;
            if !identities.contains(binding.identity.as_str()) {
                return Err(StructuralGroundError::UnknownIdentity(
                    binding.identity.clone(),
                ));
            }
            if let Some(revision) = &binding.implementation_revision {
                require_text("binding.implementation_revision", revision)?;
            }
        }

        for relation in &self.constitutive_relations {
            require_text("relation.from_identity", &relation.from_identity)?;
            require_text("relation.to_identity", &relation.to_identity)?;
            require_text("relation.relation", &relation.relation)?;
            if !identities.contains(relation.from_identity.as_str()) {
                return Err(StructuralGroundError::UnknownIdentity(
                    relation.from_identity.clone(),
                ));
            }
            if !identities.contains(relation.to_identity.as_str()) {
                return Err(StructuralGroundError::UnknownIdentity(
                    relation.to_identity.clone(),
                ));
            }
        }

        let mut gap_ids = BTreeSet::new();
        for gap in &self.unresolved_source_gaps {
            require_text("gap.id", &gap.id)?;
            require_text("gap.detail", &gap.detail)?;
            if !gap_ids.insert(gap.id.as_str()) {
                return Err(StructuralGroundError::DuplicateGap(gap.id.clone()));
            }
        }
        if let Some(law) = &self.return_or_mutation_law {
            require_text("return_or_mutation_law", law)?;
        }
        Ok(())
    }
}

/// What the current implementation/material evidence actually shows.
///
/// Observation is intentionally shaped in the same opaque terms as the target's
/// declaration. Provider-specific code indexes or graph stores may produce these
/// values, but Factory does not promote their local IDs into target identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralFidelityObservation {
    pub source_refs: Vec<StructuralSourceRef>,
    #[serde(default)]
    pub identities: Vec<String>,
    #[serde(default)]
    pub implementation_bindings: Vec<StructuralBinding>,
    #[serde(default)]
    pub constitutive_relations: Vec<StructuralRelation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuralFidelityIssueKind {
    MissingSourceRevision,
    MissingIdentity,
    MissingBinding,
    StaleBinding,
    MissingConstitutiveRelation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralFidelityIssue {
    pub kind: StructuralFidelityIssueKind,
    pub subject: String,
    pub detail: String,
}

/// Run-addressable structural-fidelity evidence. Behavioural tests remain
/// separate evidence; this receipt says only what structural claim was checked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuralFidelityEvidence {
    pub version: String,
    pub run_ref: RunRef,
    pub structural_ground_id: String,
    pub passed: bool,
    #[serde(default)]
    pub issues: Vec<StructuralFidelityIssue>,
    #[serde(default)]
    pub unresolved_source_gaps: Vec<StructuralGap>,
}

pub fn verify_structural_ground(
    run_ref: RunRef,
    ground: &StructuralGround,
    observed: &StructuralFidelityObservation,
) -> Result<StructuralFidelityEvidence, StructuralGroundError> {
    ground.validate()?;
    let mut issues = Vec::new();

    let observed_sources = observed
        .source_refs
        .iter()
        .map(|source| (source.reference.as_str(), source.revision.as_deref()))
        .collect::<BTreeMap<_, _>>();
    for expected in &ground.source_refs {
        if let Some(revision) = expected.revision.as_deref() {
            if observed_sources
                .get(expected.reference.as_str())
                .copied()
                .flatten()
                != Some(revision)
            {
                issues.push(StructuralFidelityIssue {
                    kind: StructuralFidelityIssueKind::MissingSourceRevision,
                    subject: expected.reference.clone(),
                    detail: format!(
                        "expected source revision {revision} is not the observed revision"
                    ),
                });
            }
        }
    }

    let observed_identities = observed
        .identities
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for identity in &ground.in_scope_identities {
        if !observed_identities.contains(identity.as_str()) {
            issues.push(StructuralFidelityIssue {
                kind: StructuralFidelityIssueKind::MissingIdentity,
                subject: identity.clone(),
                detail: "declared in-scope structural identity is absent from observed implementation mapping".into(),
            });
        }
    }

    for expected in &ground.existing_implementation_bindings {
        let candidates = observed
            .implementation_bindings
            .iter()
            .filter(|value| {
                value.identity == expected.identity
                    && value.implementation_ref == expected.implementation_ref
                    && value.relation == expected.relation
            })
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            issues.push(StructuralFidelityIssue {
                kind: StructuralFidelityIssueKind::MissingBinding,
                subject: expected.identity.clone(),
                detail: format!(
                    "expected `{}` binding to {} is absent",
                    expected.relation, expected.implementation_ref
                ),
            });
            continue;
        }
        if let Some(revision) = expected.implementation_revision.as_deref() {
            if !candidates
                .iter()
                .any(|value| value.implementation_revision.as_deref() == Some(revision))
            {
                issues.push(StructuralFidelityIssue {
                    kind: StructuralFidelityIssueKind::StaleBinding,
                    subject: expected.identity.clone(),
                    detail: format!(
                        "implementation binding no longer resolves revision {revision}"
                    ),
                });
            }
        }
    }

    let observed_relations = observed
        .constitutive_relations
        .iter()
        .collect::<BTreeSet<_>>();
    for expected in &ground.constitutive_relations {
        if !observed_relations.contains(expected) {
            issues.push(StructuralFidelityIssue {
                kind: StructuralFidelityIssueKind::MissingConstitutiveRelation,
                subject: format!("{}→{}", expected.from_identity, expected.to_identity),
                detail: format!(
                    "declared constitutive `{}` relation is absent",
                    expected.relation
                ),
            });
        }
    }

    Ok(StructuralFidelityEvidence {
        version: STRUCTURAL_GROUND_VERSION.into(),
        run_ref,
        structural_ground_id: ground.id.clone(),
        passed: issues.is_empty(),
        issues,
        unresolved_source_gaps: ground.unresolved_source_gaps.clone(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralGroundError {
    InvalidVersion(String),
    EmptyField(&'static str),
    NoSourceRefs,
    NoStructuralManifestRefs,
    DuplicateIdentity(String),
    UnknownIdentity(String),
    DuplicateGap(String),
}

fn require_text(field: &'static str, value: &str) -> Result<(), StructuralGroundError> {
    if value.trim().is_empty() {
        return Err(StructuralGroundError::EmptyField(field));
    }
    Ok(())
}

fn unique_non_empty<'a>(
    field: &'static str,
    values: &'a [String],
) -> Result<BTreeSet<&'a str>, StructuralGroundError> {
    let mut result = BTreeSet::new();
    for value in values {
        require_text(field, value)?;
        if !result.insert(value.as_str()) {
            return Err(StructuralGroundError::DuplicateIdentity(value.clone()));
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const EPI_REVISION: &str = "daa660cbc1b8c5da83828698665a753852cb0287";
    const QL_HEAD: &str = "de7d50c9f7dcfec33cfa0fd5f8a8a1068b4fbe84";
    const RUN_REF: &str = "run:01ARZ3NDEKTSV4RRFFQ69G5FAV";

    fn source(reference: &str, revision: &str) -> StructuralSourceRef {
        StructuralSourceRef {
            reference: reference.into(),
            revision: Some(revision.into()),
        }
    }

    fn binding(revision: &str) -> StructuralBinding {
        StructuralBinding {
            identity: "formal:sixfold-complement".into(),
            implementation_ref: "github:EpiLogos/QL-MEF:c/src/primitive.c#ql_position_invert"
                .into(),
            relation: "implemented-by".into(),
            implementation_revision: Some(revision.into()),
        }
    }

    #[test]
    fn epi_holographic_specimen_is_structurally_faithful_without_hiding_bimba_gap() {
        let ground = StructuralGround {
            version: STRUCTURAL_GROUND_VERSION.into(),
            id: "epi-holographic-specimen/position-inversion/v1".into(),
            source_refs: vec![
                source("github:EpiLogos/Epi-Logos-C-Experiments", EPI_REVISION),
                source("github:EpiLogos/QL-MEF", QL_HEAD),
            ],
            structural_manifest_refs: vec![
                "github:EpiLogos/QL-MEF:docs/integrations/epi-logos/EPI-HOLOGRAPHIC-KERNEL-MANIFEST.json".into(),
            ],
            relation_manifest_refs: vec![
                "github:EpiLogos/QL-MEF:docs/integrations/epi-logos/EPI-HOLOGRAPHIC-KERNEL-ORIENTATION.md".into(),
            ],
            in_scope_identities: vec!["formal:sixfold-complement".into()],
            constitutive_relations: vec![],
            existing_implementation_bindings: vec![binding(QL_HEAD)],
            unresolved_source_gaps: vec![StructuralGap {
                id: "bimba-live".into(),
                detail: "exact live Bimba graph node for this low-level formal law is not verified in the current environment".into(),
                owner_ref: Some("github:EpiLogos/Epi-Logos-C-Experiments".into()),
            }],
            return_or_mutation_law: Some(
                "returned implementation reality may revise target semantic ground only through an explicit target-owned source change".into(),
            ),
        };
        let observed = StructuralFidelityObservation {
            source_refs: ground.source_refs.clone(),
            identities: vec!["formal:sixfold-complement".into()],
            implementation_bindings: vec![binding(QL_HEAD)],
            constitutive_relations: vec![],
        };

        let run_ref = RunRef::from_str(RUN_REF).unwrap();
        let evidence = verify_structural_ground(run_ref, &ground, &observed).unwrap();

        assert!(evidence.passed, "structural issues: {:?}", evidence.issues);
        assert_eq!(evidence.structural_ground_id, ground.id);
        assert_eq!(evidence.unresolved_source_gaps.len(), 1);
        assert_eq!(evidence.unresolved_source_gaps[0].id, "bimba-live");
    }

    #[test]
    fn stale_implementation_revision_fails_structural_fidelity() {
        let ground = StructuralGround {
            version: STRUCTURAL_GROUND_VERSION.into(),
            id: "test-ground".into(),
            source_refs: vec![source("source:target", "v1")],
            structural_manifest_refs: vec!["source:manifest".into()],
            relation_manifest_refs: vec![],
            in_scope_identities: vec!["coordinate:A".into()],
            constitutive_relations: vec![],
            existing_implementation_bindings: vec![StructuralBinding {
                identity: "coordinate:A".into(),
                implementation_ref: "code:A".into(),
                relation: "implemented-by".into(),
                implementation_revision: Some("old".into()),
            }],
            unresolved_source_gaps: vec![],
            return_or_mutation_law: None,
        };
        let observed = StructuralFidelityObservation {
            source_refs: ground.source_refs.clone(),
            identities: vec!["coordinate:A".into()],
            implementation_bindings: vec![StructuralBinding {
                identity: "coordinate:A".into(),
                implementation_ref: "code:A".into(),
                relation: "implemented-by".into(),
                implementation_revision: Some("new".into()),
            }],
            constitutive_relations: vec![],
        };
        let run_ref = RunRef::from_str(RUN_REF).unwrap();
        let evidence = verify_structural_ground(run_ref, &ground, &observed).unwrap();
        assert!(!evidence.passed);
        assert!(evidence
            .issues
            .iter()
            .any(|issue| issue.kind == StructuralFidelityIssueKind::StaleBinding));
    }
}
