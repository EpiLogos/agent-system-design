import test from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';

const fixture = JSON.parse(fs.readFileSync(new URL('../research/harmonics.json', import.meta.url), 'utf8'));

test('harmonic research fixture is namespaced, complete and non-controlling', () => {
  assert.equal(fixture.namespace, 'ql.harmonic');
  assert.equal(fixture.status, 'research');
  assert.equal(fixture.control_effect, 'none');
  for (const pairs of Object.values(fixture.families)) {
    assert.equal(pairs.length, 3);
    assert.deepEqual(pairs.flat().sort(), ['P0', 'P1', 'P2', 'P3', 'P4', 'P5']);
  }
});
