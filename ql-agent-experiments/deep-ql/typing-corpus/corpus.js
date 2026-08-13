const positions=['P0','P1','P2','P3','P4','P5'];
const carriers=['model','tool','environment','human','artifact'];
const destinationFor=(i,p)=> p==='P5' && i%2===0 ? `P${i%5}` : positions[(positions.indexOf(p)+(i%3))%6];
export const corpus = Object.freeze(Array.from({length:100},(_,i)=>{
  const p = i<8?'P0':i<20?'P1':i<32?'P2':i<44?'P3':i<56?'P4':i<64?'P5':positions[i%6];
  const dest=destinationFor(i,p); const relation=`R${p[1]}${dest[1]}`;
  const categories=[];
  if(i<12) categories.push('ambiguous');
  if(i>=64&&i<74) categories.push('p5-reopening');
  if(i>=74&&i<84) categories.push('cross-carrier-same-function');
  if(i>=84&&i<94) categories.push('same-carrier-cross-function');
  return {
    id:`QLT-${String(i+1).padStart(3,'0')}`,
    act:{intent:`Deterministic semantic typing act ${i+1}`,carrier:{kind:carriers[i%carriers.length],name:`carrier-${i%5}`}},
    human_reference:{primary_position:p,secondary_positions:i<12?[positions[(positions.indexOf(p)+1)%6]]:[],relation,face:'direct',closure:i>=64&&i<74?'reopen':'not_applicable',reopening_destination:i>=64&&i<74?dest:null},
    model_claimed:{primary_position:i%9===0?positions[(positions.indexOf(p)+1)%6]:p,secondary_positions:[],relation:i%11===0?`R${p[1]}${p[1]}`:relation,face:'direct'},
    retrospective:{primary_position:p,secondary_positions:i<6?[positions[(positions.indexOf(p)+1)%6]]:[],relation,face:'direct'},
    structural_facts:{carrier:carriers[i%carriers.length],side_effect:i%4===0,fixture_index:i+1},
    ambiguity_notes:i<12?'Intentionally multi-functional; secondary position retained.':'',categories,
    review:{status:'pending-human-review',reviewer:null}
  };
}));

export function corpusStats(records=corpus){
  const byPosition=Object.fromEntries(positions.map(p=>[p,records.filter(r=>r.human_reference.primary_position===p).length]));
  const category=(name)=>records.filter(r=>r.categories.includes(name)).length;
  return {count:records.length,by_position:byPosition,ambiguous:category('ambiguous'),p5_reopening:category('p5-reopening'),cross_carrier_same_function:category('cross-carrier-same-function'),same_carrier_cross_function:category('same-carrier-cross-function'),human_reviewed:records.filter(r=>r.review.status==='human-reviewed').length,pending_human_review:records.filter(r=>r.review.status==='pending-human-review').length};
}

const exact=(a,b)=>a===b?1:0;
const avg=(xs)=>xs.length?xs.reduce((a,b)=>a+b,0)/xs.length:null;
export function agreementMetrics(records=corpus){
  const compare=(left,right)=>({
    position_exact_agreement:avg(records.map(r=>exact(r[left].primary_position,r[right].primary_position))),
    relation_exact_agreement:avg(records.map(r=>exact(r[left].relation,r[right].relation))),
    face_agreement:avg(records.map(r=>exact(r[left].face,r[right].face))),
    primary_vs_secondary_position_agreement:avg(records.map(r=>r[left].primary_position===r[right].primary_position || (r[right].secondary_positions??[]).includes(r[left].primary_position)?1:0)),
    closure_reopen_agreement: left==='human_reference'||right==='human_reference' ? avg(records.map(r=>exact(r[left].closure??'not_applicable',r[right].closure??'not_applicable'))) : null,
    reopening_destination_agreement: left==='human_reference'||right==='human_reference' ? avg(records.filter(r=>(r[left].reopening_destination??r[right].reopening_destination)).map(r=>exact(r[left].reopening_destination??null,r[right].reopening_destination??null))) : null
  });
  return {claimed_human:compare('model_claimed','human_reference'),retrospective_human:compare('retrospective','human_reference'),claimed_retrospective:compare('model_claimed','retrospective'),review:{human_reviewed:records.filter(r=>r.review.status==='human-reviewed').length,pending_human_review:records.filter(r=>r.review.status==='pending-human-review').length}};
}
