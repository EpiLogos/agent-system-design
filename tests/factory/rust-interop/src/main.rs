use serde_json::Value;
use std::{collections::BTreeSet, env, fs};

fn req(ok: bool, message: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(message.to_string())
    }
}

fn kind_ref(value: &str, kind: &str) -> bool {
    value.starts_with(&format!("factory:{kind}:")) && value.split(':').count() == 3
}

fn canonical_ref(value: &str) -> bool {
    let parts: Vec<&str> = value.split(':').collect();
    parts.len() == 3 && parts[0] == "factory" && !parts[1].is_empty() && !parts[2].is_empty()
}

fn subject_key(value: &Value) -> String {
    format!(
        "{}|{}|{}",
        value["subjectRef"].as_str().unwrap_or(""),
        value["stateRef"].as_str().unwrap_or(""),
        value["revision"].as_i64().unwrap_or(-1)
    )
}

fn owners(node: &Value) -> Result<(), String> {
    if let Some(properties) = node.get("properties").and_then(Value::as_object) {
        for (name, child) in properties {
            req(
                child
                    .get("x-semantic-owner")
                    .and_then(Value::as_str)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false),
                &format!("missing owner: {name}"),
            )?;
            owners(child)?;
        }
    }
    if let Some(defs) = node.get("$defs").and_then(Value::as_object) {
        for child in defs.values() {
            owners(child)?;
        }
    }
    if let Some(items) = node.get("items") {
        if items.is_object() {
            owners(items)?;
        }
    }
    Ok(())
}

fn ql_lens_core(value: &str) -> bool {
    let bytes = value.as_bytes();
    match bytes {
        [digit] => (b'0'..=b'5').contains(digit),
        [digit, b'\''] => (b'0'..=b'5').contains(digit),
        _ => false,
    }
}

fn ql_lens_ref(value: &str) -> bool {
    value
        .strip_prefix("mef:lens:L")
        .and_then(|rest| rest.strip_suffix("@1"))
        .map(ql_lens_core)
        .unwrap_or(false)
}

fn ql_sublens_ref(value: &str) -> bool {
    let Some(body) = value
        .strip_prefix("mef:sublens:L")
        .and_then(|rest| rest.strip_suffix("@1"))
    else {
        return false;
    };
    let Some((lens, sublens)) = body.split_once('.') else {
        return false;
    };
    ql_lens_core(lens)
        && sublens.len() == 1
        && sublens
            .as_bytes()
            .first()
            .map(|digit| (b'0'..=b'5').contains(digit))
            .unwrap_or(false)
}

fn ql_form_ref(value: &str) -> bool {
    matches!(
        value,
        "qlform:sixfold@1" | "qlform:four-plus-two@1" | "qlform:direct-conjugate@1"
    )
}

fn ql_address(value: &str) -> bool {
    let parts: Vec<&str> = value.split('/').collect();
    if parts.len() != 4 || parts[0] != "qladdr:sixfold@1" {
        return false;
    }
    if !matches!(parts[1], "direct" | "conjugate") {
        return false;
    }
    let position = parts[2].as_bytes();
    if position.len() != 2 || position[0] != b'P' || !(b'0'..=b'5').contains(&position[1]) {
        return false;
    }
    parts[3]
        .strip_prefix('d')
        .map(|depth| !depth.is_empty() && depth.bytes().all(|byte| byte.is_ascii_digit()))
        .unwrap_or(false)
}

fn validate_ql_composition(contract: &Value, schema: &Value, ql_schema: &Value) -> Result<(), String> {
    let ql_id = ql_schema["$id"].as_str().ok_or("QL schema $id")?;
    req(
        schema["properties"]["qlComposition"]["$ref"].as_str() == Some(ql_id),
        "parent QL schema reference drift",
    )?;
    req(
        schema["properties"]["qlComposition"]["x-semantic-owner"].as_str()
            == Some("Standalone QL/MEF module"),
        "QL semantic owner drift",
    )?;
    req(
        schema["$defs"].get("qlComposition").is_none(),
        "Factory must not duplicate QL composition definition",
    )?;

    let composition = contract["qlComposition"]
        .as_object()
        .ok_or("QL composition object")?;
    let allowed: BTreeSet<&str> = ql_schema["properties"]
        .as_object()
        .ok_or("QL schema properties")?
        .keys()
        .map(String::as_str)
        .collect();
    req(
        composition.keys().all(|key| allowed.contains(key.as_str())),
        "QL composition additional property",
    )?;
    let target = composition
        .get("targetRef")
        .and_then(Value::as_str)
        .ok_or("QL targetRef")?;
    req(!target.is_empty(), "QL targetRef")?;

    let mut coordinate_count = 0;
    if let Some(value) = composition.get("qlFormRef") {
        coordinate_count += 1;
        req(
            value.as_str().map(ql_form_ref).unwrap_or(false),
            "invalid qlFormRef",
        )?;
    }
    if let Some(value) = composition.get("qlAddress") {
        coordinate_count += 1;
        req(
            value.as_str().map(ql_address).unwrap_or(false),
            "invalid qlAddress",
        )?;
    }
    if let Some(value) = composition.get("lensRef") {
        coordinate_count += 1;
        req(
            value.as_str().map(ql_lens_ref).unwrap_or(false),
            "invalid lensRef",
        )?;
    }
    if let Some(value) = composition.get("sublensRef") {
        coordinate_count += 1;
        req(
            value.as_str().map(ql_sublens_ref).unwrap_or(false),
            "invalid sublensRef",
        )?;
    }
    req(coordinate_count > 0, "QL composition requires a coordinate")?;
    Ok(())
}

fn anti_rejected(item: &Value) -> bool {
    let id = item["id"].as_str().unwrap_or("");
    let value = &item["value"];
    match id {
        "action-as-capability-identity-collapse" => {
            let action = value["actionRef"].as_str().unwrap_or("");
            let capability = value["capabilityRef"].as_str().unwrap_or("");
            !(kind_ref(action, "action")
                && kind_ref(capability, "capability")
                && action != capability)
        }
        "binding-as-ref" => !value["bindingId"].as_str().unwrap_or("").starts_with("binding:"),
        "model-as-agent" | "session-as-agent" => {
            !kind_ref(value["agentRef"].as_str().unwrap_or(""), "agent")
        }
        "provider-as-project" => {
            !kind_ref(value["projectRef"].as_str().unwrap_or(""), "project")
        }
        "stale-subject-state-evidence" => {
            subject_key(&value["currentSubjectState"]) != subject_key(&value["evidenceSubjectState"])
        }
        _ => false,
    }
}

fn validate(document: &Value, schema: &Value, ql_schema: &Value) -> Result<(), String> {
    req(
        document["fixtureVersion"] == "factory.interop-fixtures/v1",
        "fixture version",
    )?;
    req(
        schema["$schema"] == "https://json-schema.org/draft/2020-12/schema",
        "schema draft",
    )?;
    owners(schema)?;
    let contract = &document["contract"];
    req(
        contract["contractVersion"] == "factory.interop/v1",
        "contract version",
    )?;

    let required: BTreeSet<String> = schema["required"]
        .as_array()
        .ok_or("schema required")?
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    let fields: BTreeSet<String> = contract
        .as_object()
        .ok_or("contract object")?
        .keys()
        .cloned()
        .collect();
    req(required == fields, "contract surface drift")?;

    let identity = &contract["identityEnvelope"];
    let identity_ref = identity["ref"].as_str().ok_or("identity Ref")?;
    req(canonical_ref(identity_ref), "canonical identity")?;
    req(
        identity["revision"] == identity["subjectState"]["revision"],
        "identity revision drift",
    )?;
    req(
        identity_ref == identity["subjectState"]["subjectRef"].as_str().unwrap_or(""),
        "identity subject drift",
    )?;

    let action = contract["actionDescriptor"]["actionRef"]
        .as_str()
        .ok_or("ActionRef")?;
    let capability = contract["capabilityDescriptor"]["capabilityRef"]
        .as_str()
        .ok_or("CapabilityRef")?;
    req(
        kind_ref(action, "action") && kind_ref(capability, "capability") && action != capability,
        "Action/Capability collapse",
    )?;
    let project = contract["projectBinding"]["projectRef"]
        .as_str()
        .ok_or("ProjectRef")?;
    req(kind_ref(project, "project"), "Project identity")?;
    req(
        kind_ref(
            contract["runEnvelope"]["runRef"].as_str().unwrap_or(""),
            "run",
        ),
        "Run identity",
    )?;
    req(
        kind_ref(
            contract["executionEnvelope"]["executionRef"]
                .as_str()
                .unwrap_or(""),
            "execution",
        ),
        "Execution identity",
    )?;
    req(
        kind_ref(
            contract["executionEnvelope"]["agentRef"]
                .as_str()
                .unwrap_or(""),
            "agent",
        ),
        "Agent identity",
    )?;

    let demand = contract["executionDemand"]
        .as_object()
        .ok_or("ExecutionDemand")?;
    req(
        !demand
            .keys()
            .any(|key| key.to_lowercase().contains("provider")),
        "provider leaked into ExecutionDemand",
    )?;
    let binding = contract["binding"]["bindingId"].as_str().ok_or("bindingId")?;
    req(
        binding.starts_with("binding:") && !binding.starts_with("factory:"),
        "Binding-as-Ref",
    )?;

    let current = subject_key(&identity["subjectState"]);
    for key in [
        "evidenceEnvelope",
        "assessmentEnvelope",
        "closureEnvelope",
        "gateDecisionEnvelope",
    ] {
        req(
            subject_key(&contract[key]["subjectState"]) == current,
            &format!("{key} subject-state"),
        )?;
    }

    let projections = contract["projectionProvenance"]
        .as_array()
        .ok_or("projection corpus")?;
    req(projections.len() >= 2, "projection corpus")?;
    let providers: BTreeSet<&str> = projections
        .iter()
        .filter_map(|projection| projection["providerRef"].as_str())
        .collect();
    req(providers.len() >= 2, "provider change not exercised")?;
    req(
        projections
            .iter()
            .all(|projection| projection["canonicalRef"].as_str() == Some(project)),
        "projection identity drift",
    )?;

    validate_ql_composition(contract, schema, ql_schema)?;
    req(
        canonical_ref(contract["qlComposition"]["targetRef"].as_str().unwrap_or("")),
        "QL target composition",
    )?;

    let expected: BTreeSet<&str> = [
        "action-as-capability-identity-collapse",
        "binding-as-ref",
        "model-as-agent",
        "session-as-agent",
        "provider-as-project",
        "stale-subject-state-evidence",
    ]
    .into_iter()
    .collect();
    let anti = document["antiFixtures"]
        .as_array()
        .ok_or("anti fixtures")?;
    let actual: BTreeSet<&str> = anti
        .iter()
        .filter_map(|item| item["id"].as_str())
        .collect();
    req(actual == expected, "anti fixture completeness")?;
    for item in anti {
        req(
            item["mustReject"] == true && anti_rejected(item),
            &format!(
                "anti fixture accepted: {}",
                item["id"].as_str().unwrap_or("?")
            ),
        )?;
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    req(
        args.len() >= 4,
        "usage: factory-interop-consumer <fixture> <schema> <ql-schema>",
    )?;
    let raw = fs::read_to_string(&args[1]).map_err(|error| error.to_string())?;
    let schema_raw = fs::read_to_string(&args[2]).map_err(|error| error.to_string())?;
    let ql_schema_raw = fs::read_to_string(&args[3]).map_err(|error| error.to_string())?;
    let document: Value = serde_json::from_str(&raw).map_err(|error| error.to_string())?;
    let schema: Value = serde_json::from_str(&schema_raw).map_err(|error| error.to_string())?;
    let ql_schema: Value = serde_json::from_str(&ql_schema_raw).map_err(|error| error.to_string())?;
    let encoded = serde_json::to_string(&document).map_err(|error| error.to_string())?;
    let round_trip: Value = serde_json::from_str(&encoded).map_err(|error| error.to_string())?;
    req(round_trip == document, "round-trip drift")?;
    validate(&round_trip, &schema, &ql_schema)?;
    println!("Rust interop PASS");
    Ok(())
}
