use crate::core::run::RunRef;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub const GIT_DEVELOPMENT_VERSION: &str = "factory.git-development/v1";

/// Exact Git/material condition from which a Factory Run is allowed to develop.
///
/// Git facts are retained as developmental provenance. They do not become
/// Project, Run, Candidate, Journey or World identity.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitDevelopmentBase {
    pub run_ref: RunRef,
    pub base_ref: String,
    pub project_ref: String,
    pub repository_ref: String,
    pub base_commit: String,
    pub worktree_path: String,
    #[serde(default)]
    pub branch: Option<String>,
    pub working_tree_clean: bool,
    pub isolated_worktree: bool,
    #[serde(default)]
    pub material_binding_ref: Option<String>,
    #[serde(default)]
    pub ground_refs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitDevelopmentReturn {
    pub run_ref: RunRef,
    pub base_ref: String,
    pub return_ref: String,
    pub worktree_path: String,
    #[serde(default)]
    pub material_binding_ref: Option<String>,
    pub observed_head: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub commit_refs: Vec<String>,
    #[serde(default)]
    pub changed_paths: Vec<String>,
    #[serde(default)]
    pub diff_ref: Option<String>,
    #[serde(default)]
    pub verification_refs: Vec<String>,
}

/// Three-way developmental condition when the accepted Project moved while a
/// candidate developed from an earlier exact base.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitDevelopmentDivergence {
    pub base_commit: String,
    pub current_accepted_commit: String,
    pub candidate_commit: String,
    pub reconciliation_required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GitDevelopmentError {
    MissingExactBase,
    DirtyPrimaryWorktree,
    WrongRun { expected: RunRef, actual: RunRef },
    WrongBase { expected: String, actual: String },
}

impl Display for GitDevelopmentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingExactBase => write!(formatter, "development requires an exact Git base commit"),
            Self::DirtyPrimaryWorktree => write!(
                formatter,
                "dirty primary human worktree cannot be silently admitted as the Factory development base"
            ),
            Self::WrongRun { expected, actual } => {
                write!(formatter, "Git development return belongs to {actual}, expected {expected}")
            }
            Self::WrongBase { expected, actual } => {
                write!(formatter, "Git development return is based on {actual}, expected {expected}")
            }
        }
    }
}

impl Error for GitDevelopmentError {}

pub fn admit_git_development_base(base: &GitDevelopmentBase) -> Result<(), GitDevelopmentError> {
    if base.base_commit.trim().is_empty() {
        return Err(GitDevelopmentError::MissingExactBase);
    }
    if !base.isolated_worktree && !base.working_tree_clean {
        return Err(GitDevelopmentError::DirtyPrimaryWorktree);
    }
    Ok(())
}

pub fn validate_git_development_return(
    base: &GitDevelopmentBase,
    returned: &GitDevelopmentReturn,
) -> Result<(), GitDevelopmentError> {
    if returned.run_ref != base.run_ref {
        return Err(GitDevelopmentError::WrongRun {
            expected: base.run_ref.clone(),
            actual: returned.run_ref.clone(),
        });
    }
    if returned.base_ref != base.base_ref {
        return Err(GitDevelopmentError::WrongBase {
            expected: base.base_ref.clone(),
            actual: returned.base_ref.clone(),
        });
    }
    Ok(())
}

pub fn git_development_divergence(
    base: &GitDevelopmentBase,
    current_accepted_commit: impl Into<String>,
    returned: &GitDevelopmentReturn,
) -> GitDevelopmentDivergence {
    let current_accepted_commit = current_accepted_commit.into();
    GitDevelopmentDivergence {
        reconciliation_required: current_accepted_commit != base.base_commit,
        base_commit: base.base_commit.clone(),
        current_accepted_commit,
        candidate_commit: returned.observed_head.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_ref(value: &str) -> RunRef {
        value.parse().expect("valid RunRef")
    }

    fn base() -> GitDevelopmentBase {
        GitDevelopmentBase {
            run_ref: run_ref("run:01J00000000000000000000000"),
            base_ref: "git-base:1".to_string(),
            project_ref: "project:one".to_string(),
            repository_ref: "repo:one".to_string(),
            base_commit: "aaaaaaaa".to_string(),
            worktree_path: "/work/project".to_string(),
            branch: Some("factory/run-1".to_string()),
            working_tree_clean: true,
            isolated_worktree: true,
            material_binding_ref: Some("workcell:binding:1".to_string()),
            ground_refs: vec!["source:ground".to_string()],
        }
    }

    #[test]
    fn rejects_dirty_primary_human_worktree_as_implicit_base() {
        let mut condition = base();
        condition.isolated_worktree = false;
        condition.working_tree_clean = false;
        assert_eq!(
            admit_git_development_base(&condition),
            Err(GitDevelopmentError::DirtyPrimaryWorktree)
        );
    }

    #[test]
    fn detects_project_divergence_without_equating_merge_with_recognition() {
        let condition = base();
        let returned = GitDevelopmentReturn {
            run_ref: condition.run_ref.clone(),
            base_ref: condition.base_ref.clone(),
            return_ref: "git-return:1".to_string(),
            worktree_path: condition.worktree_path.clone(),
            material_binding_ref: condition.material_binding_ref.clone(),
            observed_head: "cccccccc".to_string(),
            branch: condition.branch.clone(),
            commit_refs: vec!["cccccccc".to_string()],
            changed_paths: vec!["src/lib.rs".to_string()],
            diff_ref: Some("diff:a-c".to_string()),
            verification_refs: vec!["test:1".to_string()],
        };
        let divergence = git_development_divergence(&condition, "bbbbbbbb", &returned);
        assert!(divergence.reconciliation_required);
        assert_eq!(divergence.base_commit, "aaaaaaaa");
        assert_eq!(divergence.current_accepted_commit, "bbbbbbbb");
        assert_eq!(divergence.candidate_commit, "cccccccc");
    }
}
