import {execFileSync} from "node:child_process";
import {mkdirSync,writeFileSync,readFileSync} from "node:fs";
import {metadata} from "./release-metadata.mjs";
import {cargoCommand} from "./managed-cargo.mjs";
const filter="package(jj-store) | package(jj-store-server) | package(jj-store-capture)";
const mode=process.argv[2], cohort=process.argv[3] || "ordinary";
if (!['build','run','inventory'].includes(mode) || !['ordinary','heavy'].includes(cohort)) throw Error("invalid CI test selection");
const run=args=>execFileSync(...cargoCommand(args),{stdio:"inherit"});
const archive=`.release/tests-${cohort}.tar.zst`;
mkdirSync('.release',{recursive:true});
if (mode==='build') {
  const packages=metadata().packages.filter(p=>p.name!=='jj-store-embedded-rustfs' && (cohort==='heavy')===['jj-store','jj-store-server','jj-store-capture'].includes(p.name));
  run(['nextest','archive',...packages.flatMap(p=>['-p',p.name]),'--locked','--archive-file',archive]);
} else if (mode==='inventory') {
  const {stdout}=await new Promise((resolve,reject)=>{
    import('node:child_process').then(({execFile})=>execFile(...cargoCommand(['nextest','list','--workspace','--locked','--message-format','json']),{maxBuffer:64*1024*1024},(e,stdout)=>e?reject(e):resolve({stdout})));
  });
  writeFileSync('.release/test-inventory.json',stdout);
} else {
  const shard=process.env.CI_SHARD, count=cohort==='heavy'?8:4;
  if (!/^[1-8]$/.test(shard || '') || Number(shard)>count) throw Error('invalid test shard');
  run(['nextest','run','--archive-file',archive,'--workspace-remap',process.cwd(),'--profile',cohort==='heavy'?'heavy':'ci','--partition',`hash:${shard}/${count}`]);
}
