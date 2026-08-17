#!/usr/bin/env python3
import json
from pathlib import Path
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]

def load(rel):
    return json.loads((ROOT / rel).read_text(encoding='utf-8'))

def owner_failures(node, where='#'):
    failures = []
    if not isinstance(node, dict):
        return failures
    props = node.get('properties')
    if isinstance(props, dict):
        for name, child in props.items():
            owner = child.get('x-semantic-owner') if isinstance(child, dict) else None
            if not isinstance(owner, str) or not owner.strip():
                failures.append(f'{where}/{name}')
            failures.extend(owner_failures(child, f'{where}/{name}'))
    defs = node.get('$defs')
    if isinstance(defs, dict):
        for name, child in defs.items():
            failures.extend(owner_failures(child, f'{where}/$defs/{name}'))
    items = node.get('items')
    if isinstance(items, dict):
        failures.extend(owner_failures(items, f'{where}/items'))
    for i, child in enumerate(node.get('anyOf', [])):
        failures.extend(owner_failures(child, f'{where}/anyOf/{i}'))
    return failures

schema_set = load('contracts/factory/interop/schema-set.json')
fixture_set = load('contracts/factory/fixtures/interop/fixture-set.json')
assert schema_set['schemaSetVersion'] == 'factory.interop/v1'
assert fixture_set['fixtureSetVersion'] == 'factory.interop-fixtures/v1'
assert fixture_set['contractVersion'] == 'factory.interop/v1'
schemas = {entry['instanceKey']: entry for entry in schema_set['schemas']}
sections = {entry['instanceKey']: entry for entry in fixture_set['sections']}
assert schemas.keys() == sections.keys(), 'schema/fixture manifest drift'
for key, entry in schemas.items():
    schema = load(entry['path'])
    fixture = load(sections[key]['path'])
    assert schema['$id'] == entry['id'], f'{key}: schema id drift'
    Draft202012Validator.check_schema(schema)
    errors = sorted(Draft202012Validator(schema).iter_errors(fixture), key=lambda e: list(e.path))
    assert not errors, f"{key}: " + '; '.join(e.message for e in errors)
    missing = owner_failures(schema)
    assert not missing, f'{key}: fields without one semantic owner: {missing}'
print(f'JSON Schema CR-001 interop PASS ({len(schemas)} schemas)')
