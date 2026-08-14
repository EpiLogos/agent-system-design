use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};

pub const EXPECTED_DETERMINATIONS: &[&str] = &[
    "CONSTITUTIONAL DETERMINATION",
    "CURRENT DESIGN",
    "OBSERVED",
    "VERIFIED",
    "SOURCE-INSPECTION BLOCKED",
    "OPEN DECISION",
    "GENUINE HUMAN AUTHORSHIP",
    "OPEN SOCKET",
    "RESEARCH CLAIM",
    "SUPERSEDED",
];

pub const ALLOWED_DOCUMENT_STATUSES: &[&str] = &[
    "constitutional",
    "current design",
    "module specification",
    "refined by",
    "superseded",
    "experimental / research claim",
    "reference/source material",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorityManifest {
    pub schema_version: String,
    pub repository: RepositoryIdentity,
    pub determination_statuses: Vec<String>,
    pub document_statuses: BTreeMap<String, String>,
    pub sources: Vec<AuthoritySource>,
    #[serde(default)]
    pub retrieved_references: Vec<RetrievedReference>,
    pub promotion_rules: Vec<PromotionRule>,
    pub invariants: BTreeMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryIdentity {
    pub id: String,
    pub product: String,
    pub baseline_ref: String,
    pub baseline_commit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthoritySource {
    pub id: String,
    pub path: String,
    pub status: String,
    pub determination_status: String,
    pub precedence: i64,
    pub scope: String,
    pub governs: bool,
    pub provenance: Value,
    #[serde(default)]
    pub declared_status: Option<String>,
    #[serde(default)]
    pub refined_by: Vec<String>,
    #[serde(default)]
    pub authority_under: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RetrievedReference {
    pub id: String,
    pub path: String,
    pub status: String,
    pub determination_status: String,
    pub governs: bool,
    pub reason: String,
    pub provenance: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PromotionRule {
    pub from: String,
    pub to: String,
    pub allowed_silently: bool,
}

pub fn parse_manifest(input: &str) -> Result<AuthorityManifest, serde_json::Error> {
    serde_json::from_str(input)
}

pub fn validate_manifest(manifest: &AuthorityManifest) -> Vec<String> {
    let mut errors = Vec::new();

    if manifest.schema_version != "factory.authority-manifest/v1" {
        errors.push("unsupported schemaVersion".to_owned());
    }

    let actual: HashSet<&str> = manifest
        .determination_statuses
        .iter()
        .map(String::as_str)
        .collect();
    let expected: HashSet<&str> = EXPECTED_DETERMINATIONS.iter().copied().collect();
    if actual.len() != manifest.determination_statuses.len() {
        errors.push("duplicate determination status".to_owned());
    }
    if actual != expected {
        errors.push("determination status set differs from root programme".to_owned());
    }

    if manifest.sources.is_empty() {
        errors.push("no authority sources".to_owned());
    }

    let mut ids = HashSet::new();
    let mut paths = HashSet::new();
    for source in &manifest.sources {
        if !ids.insert(source.id.as_str()) && !errors.iter().any(|e| e == "duplicate source id") {
            errors.push("duplicate source id".to_owned());
        }
        if !paths.insert(source.path.as_str())
            && !errors.iter().any(|e| e == "duplicate source path")
        {
            errors.push("duplicate source path".to_owned());
        }
        if !ALLOWED_DOCUMENT_STATUSES.contains(&source.status.as_str()) {
            errors.push(format!("invalid document status for {}", source.id));
        }
        if !EXPECTED_DETERMINATIONS.contains(&source.determination_status.as_str()) {
            errors.push(format!("invalid determination status for {}", source.id));
        }
        if !has_non_empty_string(&source.provenance, "ref")
            || !has_non_empty_string(&source.provenance, "blobSha")
        {
            errors.push(format!("missing provenance for {}", source.id));
        }
        if source.governs
            && matches!(
                source.status.as_str(),
                "experimental / research claim" | "reference/source material" | "superseded"
            )
        {
            errors.push(format!("silent authority promotion for {}", source.id));
        }
    }

    match manifest
        .sources
        .iter()
        .find(|source| source.id == "constitutional-index")
    {
        Some(index) => {
            if !index.governs || index.status != "constitutional" {
                errors.push("constitutional index must govern as constitutional".to_owned());
            }
            if manifest.sources.iter().any(|source| {
                source.id != index.id && source.precedence >= index.precedence
            }) {
                errors.push("constitutional index must have unique highest precedence".to_owned());
            }
        }
        None => errors.push("constitutional index must govern as constitutional".to_owned()),
    }

    for reference in &manifest.retrieved_references {
        if reference.governs {
            errors.push(format!(
                "retrieved reference may not silently govern: {}",
                reference.id
            ));
        }
    }

    for rule in &manifest.promotion_rules {
        if rule.allowed_silently {
            errors.push(format!(
                "promotion must require evidence/authority: {} -> {}",
                rule.from, rule.to
            ));
        }
    }

    errors
}

pub fn render_markdown(manifest: &AuthorityManifest) -> String {
    let mut lines = vec![
        "# Factory authority manifest (generated)".to_owned(),
        String::new(),
        "> Generated from `contracts/factory/authority-manifest.json`; do not edit by hand."
            .to_owned(),
        String::new(),
        format!(
            "Pinned baseline: `{}` (`{}`)",
            manifest.repository.baseline_commit, manifest.repository.baseline_ref
        ),
        String::new(),
        "| Precedence | Source | Status | Determination | Governs | Scope |".to_owned(),
        "|---:|---|---|---|---|---|".to_owned(),
    ];

    let mut sources = manifest.sources.iter().collect::<Vec<_>>();
    sources.sort_by(|left, right| right.precedence.cmp(&left.precedence));
    for source in sources {
        lines.push(format!(
            "| {} | `{}` | {} | {} | {} | {} |",
            source.precedence,
            source.path,
            source.status,
            source.determination_status,
            if source.governs { "yes" } else { "no" },
            source.scope
        ));
    }

    lines.push(String::new());
    lines.push("## Retrieved but non-governing references".to_owned());
    lines.push(String::new());
    for reference in &manifest.retrieved_references {
        lines.push(format!(
            "- `{}` — {}: {}",
            reference.path, reference.status, reference.reason
        ));
    }
    lines.push(String::new());
    lines.push("## Promotion rule".to_owned());
    lines.push(String::new());
    lines.push("No source becomes more authoritative because files agree, generated prose looks formal, or a provider exposes it.".to_owned());
    lines.push(String::new());

    lines.join("\n")
}

fn has_non_empty_string(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .and_then(Value::as_str)
        .is_some_and(|candidate| !candidate.is_empty())
}
