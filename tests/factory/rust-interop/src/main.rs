use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn load(root: &Path, rel: &str) -> Value {
    let raw = fs::read_to_string(root.join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {rel}: {e}"))
}

fn s<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("{key} must be string"))
}

fn canonical_ref(value: &str) -> bool {
    let mut parts = value.split(':');
    let (Some(kind), Some(id)) = (parts.next(), parts.next()) else { return false };
    if parts.next().is_some() || kind.is_empty() || id.len() != 26 { return false; }
    let mut kc = kind.chars();
    if !kc.next().is_some_and(|c| c.is_ascii_lowercase()) || !kc.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') { return false; }
    let mut chars = id.chars();
    if !chars.next().is_some_and(|c| matches!(c, '0'..='7')) { return false; }
    chars.all(|c| matches!(c, '0'..='9' | 'A'..='H' | 'J' | 'K' | 'M' | 'N' | 'P'..='T' | 'V'..='Z'))
}

fn kind_ref(kind: &str, value: &str) -> bool {
    canonical_ref(value) && value.starts_with(&format!("{kind}:"))
}

fn subject_key(value: &Value) -> String {
    format!("{}|{}|{}", s(value, "subjectRef"), value["subjectRevision"].as_u64().unwrap(), s(value, "stateRef"))
}

fn owners(node: &Value, failures: &mut Vec<String>, path: &str) {
    let Some(obj) = node.as_object() else { return };
    if let Some(props) = obj.get("properties").and_then(Value::as_object) {
        for (name, child) in props {
            if !child.get("x-semantic-owner").and_then(Value::as_str).is_some_and(|v| !v.trim().is_empty()) {
                failures.push(format!("{path}/{name}"));
            }
            owners(child, failures, &format!("{path}/{name}"));
        }
    }
    if let Some(defs) = obj.get("$defs").and_then(Value::as_object) {
        for (name, child) in defs { owners(child, failures, &format!("{path}/$defs/{name}")); }
    }
    if let Some(items) = obj.get("items") { owners(items, failures, &format!("{path}/items")); }
    if let Some(any) = obj.get("anyOf").and_then(Value::as_array) {
        for (i, child) in any.iter().enumerate() { owners(child, failures, &format!("{path}/anyOf/{i}")); }
    }
}

fn string_set(value: &Value) -> BTreeSet<String> {
    value.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_owned()).collect()
}

fn whole_has_unmet_obligations(value: &Value) -> bool {
    let required = string_set(&value["operativeWhole"]["requiredObligations"]);
    let satisfied = string_set(&value["operativeWhole"]["satisfiedObligations"]);
    !required.is_subset(&satisfied)
}

fn anti_rejected(item: &Value) -> bool {
    let v = &item["value"];
    match s(item, "id") {
        "action-as-capability-identity-collapse" => kind_ref("action", s(v, "actionRef")) && !kind_ref("capability", s(v, "capabilityRef")),
        "binding-as-ref" => v["binding"].get("bindingRef").and_then(Value::as_str).is_some_and(canonical_ref),
        "model-as-agent" | "session-as-agent" => !kind_ref("agent", s(v, "agentRef")),
        "provider-as-project" => !kind_ref("project", s(v, "projectRef")),
        "stale-subject-state-evidence" => subject_key(&v["currentSubjectState"]) != subject_key(&v["evidenceSubjectState"]),
        "plausible-artifact-partial-evidence-as-full-closure" => {
            s(v, "claimedClosure") == "full"
                && whole_has_unmet_obligations(v)
                && v["claim"]["confidence"].as_f64().is_some_and(|confidence| confidence > 0.9)
        }
        "representative-evidence-without-coverage-contract" => {
            s(v, "claimedClosure") == "full"
                && s(v, "evidenceMode") == "representative"
                && whole_has_unmet_obligations(v)
                && (!v["samplingSufficiencyDeclared"].as_bool().unwrap_or(false)
                    || !v["coverageConditionEvidenced"].as_bool().unwrap_or(false))
        }
        _ => false,
    }
}

fn main() {
    let root = root();
    let schema_set = load(&root, "contracts/factory/interop/schema-set.json");
    let fixture_set = load(&root, "contracts/factory/fixtures/interop/fixture-set.json");
    assert_eq!(schema_set["schemaSetVersion"], "factory.interop/v1");
    assert_eq!(fixture_set["fixtureSetVersion"], "factory.interop-fixtures/v1");
    let schemas: BTreeMap<String, Value> = schema_set["schemas"].as_array().unwrap().iter().map(|e| (s(e, "instanceKey").to_owned(), e.clone())).collect();
    let sections: BTreeMap<String, Value> = fixture_set["sections"].as_array().unwrap().iter().map(|e| (s(e, "instanceKey").to_owned(), e.clone())).collect();
    assert_eq!(schemas.keys().collect::<Vec<_>>(), sections.keys().collect::<Vec<_>>());
    for (key, entry) in &schemas {
        let schema = load(&root, s(entry, "path"));
        let fixture = load(&root, s(&sections[key], "path"));
        assert_eq!(s(&schema, "$id"), s(entry, "id"));
        let mut failures = Vec::new(); owners(&schema, &mut failures, "#"); assert!(failures.is_empty(), "{key}: {failures:?}");
        let encoded = serde_json::to_string(&fixture).unwrap(); let decoded: Value = serde_json::from_str(&encoded).unwrap(); assert_eq!(fixture, decoded);
    }
    let section = |key: &str| load(&root, s(&sections[key], "path"));
    let identity = section("identityCore"); let run = section("runProjection"); let execution = section("execution");
    let action = section("action"); let capability = section("capability"); let context = section("context");
    let evidence = section("evidenceAssessment"); let closure = section("closureGate"); let demand = section("demand");
    let offer = section("workcellOffer"); let binding = section("binding"); let world = section("materialisedWorld"); let ql = section("ql");

    let candidate = s(&identity["identityEnvelope"], "ref");
    assert!(canonical_ref(candidate) && !candidate.starts_with("factory:"));
    assert_eq!(identity["identityEnvelope"]["revision"], identity["identityEnvelope"]["subjectState"]["subjectRevision"]);
    assert_eq!(candidate, s(&identity["identityEnvelope"]["subjectState"], "subjectRef"));
    let project = s(&identity["projectBinding"], "projectRef"); assert!(kind_ref("project", project)); assert!(!s(&identity["projectBinding"], "sourceRef").starts_with("project:"));
    let run_ref = s(&run["runEnvelope"], "runRef"); assert!(kind_ref("run", run_ref)); let map = format!("{run_ref}/map"); assert_eq!(s(&run["runEnvelope"], "runMapAddress"), map);
    assert_eq!(s(&run["runEnvelope"], "projectRef"), project);
    let projections = run["projectionProvenance"].as_array().unwrap(); assert!(projections.len() >= 2);
    assert!(projections.iter().map(|p| s(p, "providerRef")).collect::<BTreeSet<_>>().len() >= 2);
    for p in projections { assert_eq!(s(p, "canonicalRef"), project); assert_eq!(p["projectedRevision"], run["runEnvelope"]["runRevision"]); }

    let ex = &execution["executionEnvelope"];
    assert!(kind_ref("execution", s(ex, "executionRef")) && kind_ref("agent", s(ex, "agentRef")) && kind_ref("agency", s(ex, "agencyRef")) && kind_ref("agent-session", s(ex, "agentSessionRef")));
    assert!(!s(ex, "agentRef").starts_with("model:") && !s(ex, "agentRef").starts_with("agent-session:"));
    let action_ref = s(&action["actionDescriptor"], "actionRef"); let capability_ref = s(&capability["capabilityDescriptor"], "capabilityRef");
    assert!(kind_ref("action", action_ref) && kind_ref("capability", capability_ref) && action_ref != capability_ref);
    assert_eq!(s(&action["actionDescriptor"], "ownerProjectRef"), project); assert!(string_set(&ex["capabilityRefs"]).contains(capability_ref));

    let cx = &context["contextResolution"]; let available = string_set(&cx["availableRefs"]); let retrieved = string_set(&cx["retrievedRefs"]); let loaded = string_set(&cx["loadedRefs"]);
    assert!(loaded.is_subset(&retrieved) && retrieved.is_subset(&available)); assert_eq!(s(cx, "projectRef"), project);
    assert_eq!(s(ex, "contextResolutionRef"), s(cx, "contextResolutionRef")); assert_eq!(s(ex, "generationRef"), s(cx, "generationRef"));
    assert!(string_set(&context["generationProvenance"]["actionRefs"]).contains(action_ref)); assert!(string_set(&context["generationProvenance"]["capabilityRefs"]).contains(capability_ref));

    let current = subject_key(&identity["identityEnvelope"]["subjectState"]);
    for state in [&evidence["evidenceEnvelope"]["subjectState"], &evidence["assessmentEnvelope"]["subjectState"], &closure["closureEnvelope"]["subjectState"], &closure["gateDecisionEnvelope"]["subjectState"]] { assert_eq!(subject_key(state), current); }
    assert_ne!(s(&evidence["assessmentEnvelope"], "producerExecutionRef"), s(&evidence["evidenceEnvelope"], "producerExecutionRef"));
    assert!(string_set(&evidence["assessmentEnvelope"]["independentFromExecutionRefs"]).contains(s(&evidence["evidenceEnvelope"], "producerExecutionRef")));
    assert_eq!(s(&closure["gateDecisionEnvelope"], "closureRef"), s(&closure["closureEnvelope"], "closureRef"));

    let ed = &demand["executionDemand"]; assert!(!ed.as_object().unwrap().keys().any(|k| k.to_ascii_lowercase().contains("provider")));
    assert_eq!(s(ed, "projectRef"), project); assert_eq!(s(ed, "runRef"), run_ref); assert_eq!(s(&demand["candidateMaterialisationDemand"], "candidateRef"), candidate);
    assert!(string_set(&ed["requiredAffordances"]).is_subset(&string_set(&offer["workcellOffer"]["affordances"])));
    assert_eq!(s(&binding["binding"], "workcellRef"), s(&offer["workcellOffer"], "workcellRef")); assert!(binding["binding"].get("bindingRef").is_none() && !canonical_ref(s(&binding["binding"], "bindingKey")));
    assert_eq!(s(&world["materialisedExecutionWorld"], "executionDemandRef"), s(ed, "demandRef")); assert_eq!(s(&world["materialisedExecutionWorld"], "candidateRef"), candidate);
    assert!(canonical_ref(s(&ql["qlComposition"], "targetRef")));

    let anti = load(&root, s(&fixture_set, "antiFixturesPath"));
    let expected: BTreeSet<&str> = ["action-as-capability-identity-collapse","binding-as-ref","model-as-agent","session-as-agent","provider-as-project","stale-subject-state-evidence","plausible-artifact-partial-evidence-as-full-closure","representative-evidence-without-coverage-contract"].into_iter().collect();
    let items = anti["antiFixtures"].as_array().unwrap(); assert_eq!(items.iter().map(|i| s(i, "id")).collect::<BTreeSet<_>>(), expected);
    for item in items { assert_eq!(item["mustReject"].as_bool(), Some(true)); assert!(anti_rejected(item), "anti fixture accepted: {}", s(item, "id")); }
    println!("Rust CR-001 interop PASS ({} schema sections, {} anti-fixtures)", schemas.len(), items.len());
}
