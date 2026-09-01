use crate::core::run::{ProjectRef, RunRef};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub const GIT_DEVELOPMENT_WORLD_SCHEMA: &str = "factory.git-development-world/v1";
pub const GIT_RETURN_EVIDENCE_SCHEMA: &str = "factory.git-return-evidence/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDevelopmentBase {
    pub project_ref: ProjectRef,
    pub run_ref: RunRef,
    pub candidate_ref: String,
    pub repository_ref: String,
    pub base_revision: String,
    pub base_worktree_clean: bool,
    #[serde(default)]
    pub source_basis_refs: Vec<String>,
    #[serde(default)]
    pub structural_ground_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitWorktreeBinding {
    pub provider_ref: String,
    pub worktree_ref: String,
    pub repository_ref: String,
    pub current_revision: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_host_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    pub clean: bool,
    #[serde(default)]
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDevelopmentWorld {
    pub schema: String,
    pub development_ref: String,
    pub base: GitDevelopmentBase,
    pub binding: GitWorktreeBinding,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub returned: Option<GitReturnEvidence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recognition: Option<GitRecognitionEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitReturnEvidence {
    pub schema: String,
    pub base_revision: String,
    pub result_revision: String,
    #[serde(default)]
    pub commits: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uncommitted_diff_ref: Option<String>,
    #[serde(default)]
    pub changed_paths: Vec<String>,
    #[serde(default)]
    pub verification_evidence_refs: Vec<String>,
    #[serde(default)]
    pub claim_refs: Vec<String>,
    #[serde(default)]
    pub conflicts: Vec<String>,
    pub provider_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_host_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRecognitionEvidence {
    pub recognition_ref: String,
    pub accepted: bool,
    pub project_revision_before: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_revision_after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_evidence_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDevelopmentRequest {
    pub project_ref: ProjectRef,
    pub run_ref: RunRef,
    pub candidate_ref: String,
    pub repository_ref: String,
    pub base_revision: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_host_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDevelopmentProvision {
    pub provider_ref: String,
    pub worktree_ref: String,
    pub repository_ref: String,
    pub base_revision: String,
    pub current_revision: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_host_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locator: Option<String>,
    pub clean: bool,
}

/// Factory asks for an isolated material Git reality through an external native
/// provider (normally AIKit). The provider owns Git mechanics; Factory owns the
/// developmental Run/Candidate relation and returned evidence.
pub trait GitDevelopmentProvider {
    fn provision_isolated(
        &self,
        request: &GitDevelopmentRequest,
    ) -> Result<GitDevelopmentProvision, GitDevelopmentError>;

    fn inspect(
        &self,
        worktree_ref: &str,
    ) -> Result<GitWorktreeBinding, GitDevelopmentError>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitDevelopmentRegistry {
    worlds: BTreeMap<String, GitDevelopmentWorld>,
}

impl GitDevelopmentRegistry {
    pub fn begin(
        &mut self,
        development_ref: impl Into<String>,
        base: GitDevelopmentBase,
        binding: GitWorktreeBinding,
    ) -> Result<&GitDevelopmentWorld, GitDevelopmentError> {
        let development_ref = non_empty(development_ref.into(), "development_ref")?;
        validate_base(&base)?;
        validate_binding(&binding)?;
        if base.repository_ref != binding.repository_ref {
            return Err(GitDevelopmentError::RepositoryMismatch);
        }
        if base.base_revision != binding.current_revision {
            return Err(GitDevelopmentError::ProvisionedBaseMismatch {
                expected: base.base_revision,
                actual: binding.current_revision,
            });
        }
        if self.worlds.contains_key(&development_ref) {
            return Err(GitDevelopmentError::DuplicateDevelopmentRef(development_ref));
        }
        let world = GitDevelopmentWorld {
            schema: GIT_DEVELOPMENT_WORLD_SCHEMA.to_owned(),
            development_ref: development_ref.clone(),
            base,
            binding,
            returned: None,
            recognition: None,
        };
        self.worlds.insert(development_ref.clone(), world);
        Ok(self.worlds.get(&development_ref).expect("inserted development world"))
    }

    pub fn provision<P: GitDevelopmentProvider>(
        &mut self,
        development_ref: impl Into<String>,
        base: GitDevelopmentBase,
        provider: &P,
    ) -> Result<&GitDevelopmentWorld, GitDevelopmentError> {
        validate_base(&base)?;
        let request = GitDevelopmentRequest {
            project_ref: base.project_ref.clone(),
            run_ref: base.run_ref.clone(),
            candidate_ref: base.candidate_ref.clone(),
            repository_ref: base.repository_ref.clone(),
            base_revision: base.base_revision.clone(),
            preferred_branch: None,
            material_host_ref: None,
        };
        let provision = provider.provision_isolated(&request)?;
        if provision.base_revision != base.base_revision {
            return Err(GitDevelopmentError::ProvisionedBaseMismatch {
                expected: base.base_revision,
                actual: provision.base_revision,
            });
        }
        let binding = GitWorktreeBinding {
            provider_ref: provision.provider_ref,
            worktree_ref: provision.worktree_ref,
            repository_ref: provision.repository_ref,
            current_revision: provision.current_revision,
            branch: provision.branch,
            material_host_ref: provision.material_host_ref,
            locator: provision.locator,
            clean: provision.clean,
            conflicts: Vec::new(),
        };
        self.begin(development_ref, base, binding)
    }

    pub fn get(&self, development_ref: &str) -> Option<&GitDevelopmentWorld> {
        self.worlds.get(development_ref)
    }

    pub fn for_run(&self, run_ref: &RunRef) -> Vec<&GitDevelopmentWorld> {
        self.worlds
            .values()
            .filter(|world| &world.base.run_ref == run_ref)
            .collect()
    }

    pub fn for_candidate(&self, candidate_ref: &str) -> Vec<&GitDevelopmentWorld> {
        self.worlds
            .values()
            .filter(|world| world.base.candidate_ref == candidate_ref)
            .collect()
    }

    /// Update only the material binding. Run/Candidate/development identity and
    /// exact original base remain unchanged across Workcell/host relocation.
    pub fn rebind_material(
        &mut self,
        development_ref: &str,
        binding: GitWorktreeBinding,
    ) -> Result<&GitDevelopmentWorld, GitDevelopmentError> {
        validate_binding(&binding)?;
        let world = self
            .worlds
            .get_mut(development_ref)
            .ok_or_else(|| GitDevelopmentError::NotFound(development_ref.to_owned()))?;
        if world.base.repository_ref != binding.repository_ref {
            return Err(GitDevelopmentError::RepositoryMismatch);
        }
        world.binding = binding;
        Ok(world)
    }

    pub fn return_difference(
        &mut self,
        development_ref: &str,
        mut evidence: GitReturnEvidence,
    ) -> Result<&GitDevelopmentWorld, GitDevelopmentError> {
        validate_return(&evidence)?;
        let world = self
            .worlds
            .get_mut(development_ref)
            .ok_or_else(|| GitDevelopmentError::NotFound(development_ref.to_owned()))?;
        if evidence.base_revision != world.base.base_revision {
            return Err(GitDevelopmentError::ReturnBaseMismatch);
        }
        evidence.schema = GIT_RETURN_EVIDENCE_SCHEMA.to_owned();
        world.returned = Some(evidence);
        Ok(world)
    }

    /// Current Project Git state is external evidence. Divergence never rewrites
    /// the Candidate's original basis; it determines whether integration now
    /// requires merge/rebase/rework/Recognition against a moved reality.
    pub fn basis_state(
        &self,
        development_ref: &str,
        current_project_revision: &str,
    ) -> Result<GitBasisState, GitDevelopmentError> {
        let world = self
            .worlds
            .get(development_ref)
            .ok_or_else(|| GitDevelopmentError::NotFound(development_ref.to_owned()))?;
        Ok(if world.base.base_revision == current_project_revision {
            GitBasisState::Current
        } else {
            GitBasisState::Diverged {
                candidate_base: world.base.base_revision.clone(),
                current_project_revision: current_project_revision.to_owned(),
            }
        })
    }

    pub fn record_recognition(
        &mut self,
        development_ref: &str,
        recognition: GitRecognitionEvidence,
    ) -> Result<&GitDevelopmentWorld, GitDevelopmentError> {
        non_empty(recognition.recognition_ref.clone(), "recognition_ref")?;
        non_empty(
            recognition.project_revision_before.clone(),
            "project_revision_before",
        )?;
        let world = self
            .worlds
            .get_mut(development_ref)
            .ok_or_else(|| GitDevelopmentError::NotFound(development_ref.to_owned()))?;
        if world.returned.is_none() {
            return Err(GitDevelopmentError::RecognitionBeforeReturn);
        }
        world.recognition = Some(recognition);
        Ok(world)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum GitBasisState {
    Current,
    Diverged {
        candidate_base: String,
        current_project_revision: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitDevelopmentError {
    InvalidField(&'static str),
    DirtyBaseWithoutExplicitSnapshot,
    DuplicateDevelopmentRef(String),
    NotFound(String),
    RepositoryMismatch,
    ProvisionedBaseMismatch { expected: String, actual: String },
    ReturnBaseMismatch,
    RecognitionBeforeReturn,
    Provider(String),
}

impl Display for GitDevelopmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidField(field) => write!(f, "{field} must be non-empty"),
            Self::DirtyBaseWithoutExplicitSnapshot => write!(
                f,
                "a dirty inhabited worktree cannot silently become the canonical Git development base"
            ),
            Self::DuplicateDevelopmentRef(reference) => {
                write!(f, "duplicate Git development Ref: {reference}")
            }
            Self::NotFound(reference) => write!(f, "Git development world not found: {reference}"),
            Self::RepositoryMismatch => write!(f, "Git repository relation does not match the development base"),
            Self::ProvisionedBaseMismatch { expected, actual } => write!(
                f,
                "provider materialised Git revision {actual}, expected exact base {expected}"
            ),
            Self::ReturnBaseMismatch => write!(f, "returned Git evidence does not refer to the exact original base"),
            Self::RecognitionBeforeReturn => write!(f, "Recognition cannot be attached before returned Git evidence exists"),
            Self::Provider(message) => write!(f, "Git development provider failed: {message}"),
        }
    }
}

impl Error for GitDevelopmentError {}

fn validate_base(base: &GitDevelopmentBase) -> Result<(), GitDevelopmentError> {
    non_empty(base.candidate_ref.clone(), "candidate_ref")?;
    non_empty(base.repository_ref.clone(), "repository_ref")?;
    non_empty(base.base_revision.clone(), "base_revision")?;
    if !base.base_worktree_clean {
        return Err(GitDevelopmentError::DirtyBaseWithoutExplicitSnapshot);
    }
    Ok(())
}

fn validate_binding(binding: &GitWorktreeBinding) -> Result<(), GitDevelopmentError> {
    non_empty(binding.provider_ref.clone(), "provider_ref")?;
    non_empty(binding.worktree_ref.clone(), "worktree_ref")?;
    non_empty(binding.repository_ref.clone(), "repository_ref")?;
    non_empty(binding.current_revision.clone(), "current_revision")?;
    Ok(())
}

fn validate_return(evidence: &GitReturnEvidence) -> Result<(), GitDevelopmentError> {
    non_empty(evidence.base_revision.clone(), "base_revision")?;
    non_empty(evidence.result_revision.clone(), "result_revision")?;
    non_empty(evidence.provider_ref.clone(), "provider_ref")?;
    Ok(())
}

fn non_empty(value: String, field: &'static str) -> Result<String, GitDevelopmentError> {
    if value.trim().is_empty() {
        Err(GitDevelopmentError::InvalidField(field))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn project() -> ProjectRef {
        ProjectRef::from_str("project:01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap()
    }

    fn run() -> RunRef {
        RunRef::from_str("run:01ARZ3NDEKTSV4RRFFQ69G5FAW").unwrap()
    }

    fn base(candidate: &str, revision: &str) -> GitDevelopmentBase {
        GitDevelopmentBase {
            project_ref: project(),
            run_ref: run(),
            candidate_ref: candidate.to_owned(),
            repository_ref: "repo:example".to_owned(),
            base_revision: revision.to_owned(),
            base_worktree_clean: true,
            source_basis_refs: Vec::new(),
            structural_ground_refs: Vec::new(),
        }
    }

    fn binding(worktree: &str, revision: &str, host: &str) -> GitWorktreeBinding {
        GitWorktreeBinding {
            provider_ref: "aikit:provider:native-git".to_owned(),
            worktree_ref: worktree.to_owned(),
            repository_ref: "repo:example".to_owned(),
            current_revision: revision.to_owned(),
            branch: Some(format!("agent/{worktree}")),
            material_host_ref: Some(host.to_owned()),
            locator: None,
            clean: true,
            conflicts: Vec::new(),
        }
    }

    #[test]
    fn two_parallel_candidates_keep_semantic_identity_separate_from_worktrees() {
        let mut registry = GitDevelopmentRegistry::default();
        registry.begin("git-dev:a", base("candidate:a", "A"), binding("wt:a", "A", "host:one")).unwrap();
        registry.begin("git-dev:b", base("candidate:b", "A"), binding("wt:b", "A", "host:one")).unwrap();
        assert_eq!(registry.for_run(&run()).len(), 2);
        assert_eq!(registry.for_candidate("candidate:a")[0].binding.worktree_ref, "wt:a");
        assert_ne!(
            registry.get("git-dev:a").unwrap().base.candidate_ref,
            registry.get("git-dev:b").unwrap().base.candidate_ref
        );
    }

    #[test]
    fn material_relocation_changes_binding_not_run_candidate_or_base() {
        let mut registry = GitDevelopmentRegistry::default();
        registry.begin("git-dev:a", base("candidate:a", "A"), binding("wt:a", "A", "host:one")).unwrap();
        let before = registry.get("git-dev:a").unwrap().base.clone();
        registry.rebind_material("git-dev:a", binding("wt:a2", "A", "host:two")).unwrap();
        let after = registry.get("git-dev:a").unwrap();
        assert_eq!(after.base, before);
        assert_eq!(after.binding.material_host_ref.as_deref(), Some("host:two"));
    }

    #[test]
    fn moved_current_project_is_explicit_divergence_not_rebased_history() {
        let mut registry = GitDevelopmentRegistry::default();
        registry.begin("git-dev:a", base("candidate:a", "A"), binding("wt:a", "A", "host:one")).unwrap();
        assert_eq!(
            registry.basis_state("git-dev:a", "B").unwrap(),
            GitBasisState::Diverged {
                candidate_base: "A".to_owned(),
                current_project_revision: "B".to_owned()
            }
        );
        assert_eq!(registry.get("git-dev:a").unwrap().base.base_revision, "A");
    }

    #[test]
    fn returned_diff_is_evidence_and_never_recognition_by_itself() {
        let mut registry = GitDevelopmentRegistry::default();
        registry.begin("git-dev:a", base("candidate:a", "A"), binding("wt:a", "A", "host:one")).unwrap();
        registry.return_difference("git-dev:a", GitReturnEvidence {
            schema: String::new(),
            base_revision: "A".to_owned(),
            result_revision: "C".to_owned(),
            commits: vec!["C".to_owned()],
            diff_ref: Some("git-diff:A..C".to_owned()),
            uncommitted_diff_ref: None,
            changed_paths: vec!["src/lib.rs".to_owned()],
            verification_evidence_refs: vec!["evidence:test".to_owned()],
            claim_refs: vec!["claim:works".to_owned()],
            conflicts: Vec::new(),
            provider_ref: "aikit:provider:native-git".to_owned(),
            material_host_ref: Some("host:one".to_owned()),
        }).unwrap();
        assert!(registry.get("git-dev:a").unwrap().returned.is_some());
        assert!(registry.get("git-dev:a").unwrap().recognition.is_none());

        registry.record_recognition("git-dev:a", GitRecognitionEvidence {
            recognition_ref: "recognition:accept".to_owned(),
            accepted: true,
            project_revision_before: "B".to_owned(),
            project_revision_after: Some("D".to_owned()),
            integration_evidence_ref: Some("evidence:merge".to_owned()),
        }).unwrap();
        assert!(registry.get("git-dev:a").unwrap().recognition.as_ref().unwrap().accepted);
    }

    #[test]
    fn dirty_human_worktree_cannot_be_silently_normalised_as_base() {
        let mut dirty = base("candidate:a", "A");
        dirty.base_worktree_clean = false;
        let mut registry = GitDevelopmentRegistry::default();
        assert_eq!(
            registry.begin("git-dev:a", dirty, binding("wt:a", "A", "host:one")).unwrap_err(),
            GitDevelopmentError::DirtyBaseWithoutExplicitSnapshot
        );
    }
}
