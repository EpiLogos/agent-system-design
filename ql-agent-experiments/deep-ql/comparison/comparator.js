export function compareTraces(left, right) {
  const relations = (events) => events.filter((event) => event.event_type === 'transition').map((event) => event.ql?.relation);
  const a = relations(left);
  const b = relations(right);
  return {
    left_relations: a,
    right_relations: b,
    equal: JSON.stringify(a) === JSON.stringify(b)
  };
}
