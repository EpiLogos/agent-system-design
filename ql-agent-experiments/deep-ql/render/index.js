const clone=v=>v===undefined?undefined:structuredClone(v);
export function replayState(events){
  const state={run_id:null,circuit_id:null,parent_circuit_id:null,depth:0,face:null,active_position:null,relation:null,exchange:null,closure_state:'open',child_count:0,conjugate_state:'available'};
  for(const e of events){ state.run_id=e.run_id??state.run_id; state.circuit_id=e.circuit_id??state.circuit_id; state.parent_circuit_id=e.parent_circuit_id??state.parent_circuit_id; state.face=e.face??state.face;
    if(e.event_type==='circuit_started') state.depth=e.payload?.depth??state.depth;
    if(e.ql?.to) state.active_position=e.ql.to; if(e.ql?.relation) state.relation=e.ql.relation;
    if(e.event_type==='projection') state.exchange={projection:e.payload?.projection??e.ql?.projection};
    if(e.event_type==='return_received') state.exchange={...(state.exchange??{}),return:e.payload?.returned?.difference??e.ql?.return};
    if(e.event_type==='circuit_closed') state.closure_state='closed'; if(e.event_type==='circuit_reopened') state.closure_state='open';
    if(e.event_type==='child_started') state.child_count++; if(e.event_type==='conjugate_started') state.conjugate_state='active'; if(e.event_type==='conjugate_completed') state.conjugate_state='completed'; }
  return clone(state);
}
export function renderRun(events){const s=replayState(events); return [`QL  ${s.run_id} / ${s.circuit_id}  depth=${s.depth}`,`FACE  ${(s.face??'unknown').toUpperCase()}`,`ACTIVE  ${s.active_position??'-'}`,`RELATION  ${s.relation??'-'}`,`EXCHANGE  ${JSON.stringify(s.exchange??{})}`,`CLOSURE  ${s.closure_state}`,`CHILDREN  ${s.child_count}`,`CONJUGATE  ${s.conjugate_state}`].join('\n');}
