use crate::core::run::RunRef;
use crate::project_development::{ProjectDevelopmentLedger, PROJECT_DEVELOPMENT_VERSION};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

/// Owner-native persistence boundary for Factory's run-scoped developmental truth.
///
/// Stores persist the existing `ProjectDevelopmentLedger`; they do not mint Project,
/// Run, Intent, ContextResolution, Candidate, or Evidence identity. Provider paths are
/// material state only and must never be treated as semantic refs.
pub trait ProjectDevelopmentStore {
    fn save(&self, ledger: &ProjectDevelopmentLedger) -> Result<(), ProjectDevelopmentStoreError>;

    fn load(
        &self,
        run_ref: &RunRef,
    ) -> Result<Option<ProjectDevelopmentLedger>, ProjectDevelopmentStoreError>;
}

/// Small local reference provider used by deterministic acceptance and local Factory
/// operation. One JSON document is retained per canonical RunRef. The filename uses
/// only the Run ULID as provider addressing; the canonical RunRef remains inside and
/// is revalidated on every load.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileProjectDevelopmentStore {
    root: PathBuf,
}

impl FileProjectDevelopmentStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn path_for(&self, run_ref: &RunRef) -> PathBuf {
        self.root.join(format!("{}.json", run_ref.as_ref().id()))
    }
}

#[derive(Debug)]
pub enum ProjectDevelopmentStoreError {
    Io(std::io::Error),
    Json(serde_json::Error),
    VersionMismatch { expected: String, actual: String },
    RunMismatch { expected: RunRef, actual: RunRef },
}

impl Display for ProjectDevelopmentStoreError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "project-development store I/O failed: {error}"),
            Self::Json(error) => {
                write!(formatter, "project-development store JSON failed: {error}")
            }
            Self::VersionMismatch { expected, actual } => write!(
                formatter,
                "project-development store version {actual} does not match {expected}"
            ),
            Self::RunMismatch { expected, actual } => write!(
                formatter,
                "project-development store returned {actual}, expected {expected}"
            ),
        }
    }
}

impl Error for ProjectDevelopmentStoreError {}

impl From<std::io::Error> for ProjectDevelopmentStoreError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for ProjectDevelopmentStoreError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl ProjectDevelopmentStore for FileProjectDevelopmentStore {
    fn save(&self, ledger: &ProjectDevelopmentLedger) -> Result<(), ProjectDevelopmentStoreError> {
        fs::create_dir_all(&self.root)?;
        let encoded = serde_json::to_vec_pretty(ledger)?;
        fs::write(self.path_for(&ledger.run_ref), encoded)?;
        Ok(())
    }

    fn load(
        &self,
        run_ref: &RunRef,
    ) -> Result<Option<ProjectDevelopmentLedger>, ProjectDevelopmentStoreError> {
        let path = self.path_for(run_ref);
        let encoded = match fs::read(path) {
            Ok(encoded) => encoded,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        let ledger: ProjectDevelopmentLedger = serde_json::from_slice(&encoded)?;
        if ledger.version != PROJECT_DEVELOPMENT_VERSION {
            return Err(ProjectDevelopmentStoreError::VersionMismatch {
                expected: PROJECT_DEVELOPMENT_VERSION.to_owned(),
                actual: ledger.version,
            });
        }
        if &ledger.run_ref != run_ref {
            return Err(ProjectDevelopmentStoreError::RunMismatch {
                expected: run_ref.clone(),
                actual: ledger.run_ref,
            });
        }
        Ok(Some(ledger))
    }
}
