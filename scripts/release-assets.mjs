import {execFileSync} from "node:child_process";
import {readdirSync,readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
const tag=process.env.RELEASE_TAG;
if (!/^v\d+\.\d+\.\d+$/.test(tag || '')) throw Error('invalid tag');
const gh=args=>execFileSync('gh',args,{encoding:'utf8'});
const release=JSON.parse(gh(['release','view',tag,'--json','assets']));
for (const name of readdirSync('dist').filter(n=>!n.startsWith('.'))) {
  const path=join('dist',name), local=readFileSync(path), existing=release.assets.find(a=>a.name===name);
  if (existing) {
    const data=execFileSync('gh',['api','-H','Accept: application/octet-stream',`repos/${process.env.GITHUB_REPOSITORY}/releases/assets/${existing.id}`],{maxBuffer:1024*1024*1024});
    const hash=b=>createHash('sha256').update(b).digest('hex');
    if (hash(data)!==hash(local)) throw Error(`release asset ${name} differs; refusing overwrite`);
  } else gh(['release','upload',tag,path]);
}
