import fs from 'node:fs';
type Fixture={interop:{version:string,runEnvelope:{runRef:string},actionDescriptor:{actionRef:string},capabilityDescriptor:{capabilityRef:string}}};
const d:Fixture=JSON.parse(fs.readFileSync(process.argv[2],'utf8'));
if(d.interop.version!=='factory.interop/v1-alpha')throw Error('version');
if(d.interop.actionDescriptor.actionRef===d.interop.capabilityDescriptor.capabilityRef)throw Error('collapse');
if(!d.interop.runEnvelope.runRef.startsWith('factory:run:'))throw Error('run identity');
console.log('TypeScript interop PASS');
