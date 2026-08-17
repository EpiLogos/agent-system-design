//! Persistent first-party local provider for the Factory Build read model.
//!
//! The provider owns persistence of canonical [`FactoryBuildState`] and delegates
//! all semantic mutation to the Factory-owned action executor. External hosts may
//! hold this provider handle, but they do not read/write the state file directly
//! and never become semantic owners of the stored Project/Run/Candidate state.

use crate::build::{
    FactoryActionAuthority, FactoryActionExecutor, FactoryActionInvocation, FactoryActionReceipt,
    FactoryBuildError, FactoryBuildSelection, FactoryBuildSnapshot, FactoryBuildState,
    FactoryBuildViewProvider,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const FACTORY_BUILD_LOCAL_PROVIDER_STATE: &str = "factory.build-local-provider-state/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StoredFactoryBuildState {
    schema: String,
    state: FactoryBuildState,
}

/// Product-owned local provider binding suitable for a first-party development
/// host. Persistence is an implementation detail of the Factory provider rather
/// than a mutable GUI/O:I store.
#[derive(Debug)]
pub struct FactoryBuildFileProvider {
    path: PathBuf,
    selection: FactoryBuildSelection,
    state: FactoryBuildState,
}

impl FactoryBuildFileProvider {
    pub fn create(
        path: impl Into<PathBuf>,
        selection: FactoryBuildSelection,
        state: FactoryBuildState,
    ) -> Result<Self, FactoryBuildProviderError> {
        let provider = Self {
            path: path.into(),
            selection,
            state,
        };
        provider.validate_selection()?;
        provider.persist()?;
        Ok(provider)
    }

    pub fn open(
        path: impl Into<PathBuf>,
        selection: FactoryBuildSelection,
    ) -> Result<Self, FactoryBuildProviderError> {
        let path = path.into();
        let input = fs::read(&path)?;
        let stored: StoredFactoryBuildState = serde_json::from_slice(&input)?;
        if stored.schema != FACTORY_BUILD_LOCAL_PROVIDER_STATE {
            return Err(FactoryBuildProviderError::UnsupportedSchema(stored.schema));
        }
        let provider = Self {
            path,
            selection,
            state: stored.state,
        };
        provider.validate_selection()?;
        Ok(provider)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn selection(&self) -> &FactoryBuildSelection {
        &self.selection
    }

    pub fn snapshot(&self) -> Result<FactoryBuildSnapshot, FactoryBuildProviderError> {
        Ok(FactoryBuildViewProvider.snapshot(&self.state, &self.selection)?)
    }

    /// Re-read the Factory-owned canonical state file. This is the explicit local
    /// change-observation seam; no watcher or ACTIVE state is fabricated.
    pub fn refresh(&mut self) -> Result<FactoryBuildSnapshot, FactoryBuildProviderError> {
        let input = fs::read(&self.path)?;
        let stored: StoredFactoryBuildState = serde_json::from_slice(&input)?;
        if stored.schema != FACTORY_BUILD_LOCAL_PROVIDER_STATE {
            return Err(FactoryBuildProviderError::UnsupportedSchema(stored.schema));
        }
        self.state = stored.state;
        self.validate_selection()?;
        self.snapshot()
    }

    pub fn execute_action(
        &mut self,
        invocation: &FactoryActionInvocation,
        authority: &FactoryActionAuthority,
    ) -> Result<FactoryActionReceipt, FactoryBuildProviderError> {
        if invocation.run_ref != self.selection.run_ref {
            return Err(FactoryBuildProviderError::SelectionMismatch(format!(
                "Action Run {} does not match provider-selected Run {}",
                invocation.run_ref, self.selection.run_ref
            )));
        }
        let receipt = FactoryActionExecutor.execute(&mut self.state, invocation, authority)?;
        self.persist()?;
        Ok(receipt)
    }

    fn validate_selection(&self) -> Result<(), FactoryBuildProviderError> {
        FactoryBuildViewProvider
            .snapshot(&self.state, &self.selection)
            .map(|_| ())
            .map_err(FactoryBuildProviderError::Factory)
    }

    fn persist(&self) -> Result<(), FactoryBuildProviderError> {
        let parent = self
            .path
            .parent()
            .filter(|path| !path.as_os_str().is_empty());
        if let Some(parent) = parent {
            fs::create_dir_all(parent)?;
        }
        let stored = StoredFactoryBuildState {
            schema: FACTORY_BUILD_LOCAL_PROVIDER_STATE.into(),
            state: self.state.clone(),
        };
        let bytes = serde_json::to_vec_pretty(&stored)?;
        let temporary = temporary_path(&self.path);
        fs::write(&temporary, bytes)?;
        fs::rename(&temporary, &self.path).map_err(|error| {
            let _ = fs::remove_file(&temporary);
            error
        })?;
        Ok(())
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("factory-build-state.json");
    path.with_file_name(format!(".{file_name}.tmp-{}", std::process::id()))
}

#[derive(Debug)]
pub enum FactoryBuildProviderError {
    Io(io::Error),
    Json(serde_json::Error),
    Factory(FactoryBuildError),
    UnsupportedSchema(String),
    SelectionMismatch(String),
}

impl From<io::Error> for FactoryBuildProviderError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for FactoryBuildProviderError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<FactoryBuildError> for FactoryBuildProviderError {
    fn from(error: FactoryBuildError) -> Self {
        Self::Factory(error)
    }
}

impl Display for FactoryBuildProviderError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "Factory Build provider I/O error: {error}"),
            Self::Json(error) => write!(formatter, "Factory Build provider JSON error: {error}"),
            Self::Factory(error) => write!(formatter, "{error}"),
            Self::UnsupportedSchema(schema) => {
                write!(
                    formatter,
                    "unsupported Factory Build provider schema `{schema}`"
                )
            }
            Self::SelectionMismatch(detail) => write!(formatter, "{detail}"),
        }
    }
}

impl Error for FactoryBuildProviderError {}
