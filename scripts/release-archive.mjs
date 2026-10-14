import {mkdtempSync,mkdirSync,cpSync,chmodSync,readFileSync,writeFileSync,rmSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {createHash} from "node:crypto";
import {version} from "./release-metadata.mjs";
const target=process.argv[2];
if (!/^(x86_64|aarch64)-(apple-darwin|unknown-linux-gnu)$/.test(target || '')) throw Error('unsupported archive target');
const temporary=mkdtempSync(join(tmpdir(),'jj-store-archive-'));
try {
  const name=`jj-store-${version()}-${target}`,stage=join(temporary,name);
  mkdirSync(join(stage,'bin'),{recursive:true});
  for (const [source,destination] of [
    [`target/${target}/release/jj-store`,'bin/jj-store'],
    [`target/${target}/release/jj-store-jj`,'bin/jj-store-jj'],
    [`runtime/embedded-rustfs/target/${target}/release/jj-store-rustfs`,'bin/jj-store-rustfs'],
    ['web/dist','web'],['README.md','README.md'],['LICENSE.md','LICENSE.md'],['docs/licenses','licenses']
  ]) cpSync(source,join(stage,destination),{recursive:true});
  for (const name of ['jj-store','jj-store-jj','jj-store-rustfs']) chmodSync(join(stage,'bin',name),0o755);
  if (execFileSync(join(stage,'bin/jj-store'),['--version'],{encoding:'utf8'}).trim()!==`jj-store ${version()}`) throw Error('wrong server version');
  execFileSync(join(stage,'bin/jj-store-jj'),['store','--help']);
  if (!readFileSync(join(stage,'web/index.html')).length || !readFileSync(join(stage,'web/help/index.html')).length) throw Error('missing UI/Help');
  mkdirSync('dist',{recursive:true});
  const archive=`${name}.tar.gz`;
  const epoch=execFileSync('git',['show','-s','--format=%ct','HEAD'],{encoding:'utf8'}).trim();
  execFileSync('python3',['-c',`
import gzip,tarfile,sys
output,stage,name,epoch=sys.argv[1:]
def normalize(info):
 info.uid=info.gid=0;info.uname=info.gname='';info.mtime=int(epoch)
 info.mode=0o755 if info.isdir() or '/bin/' in info.name else 0o644
 return info
with open(output,'wb') as raw:
 with gzip.GzipFile(filename='',mode='wb',fileobj=raw,mtime=int(epoch)) as stream:
  with tarfile.open(fileobj=stream,mode='w',format=tarfile.PAX_FORMAT) as bundle:bundle.add(stage,arcname=name,filter=normalize)
`,resolve('dist',archive),stage,name,epoch]);
  const digest=createHash('sha256').update(readFileSync(join('dist',archive))).digest('hex');
  writeFileSync(join('dist',archive+'.sha256'),`${digest}  ${archive}\n`);
  console.log(archive);
} finally {rmSync(temporary,{recursive:true,force:true});}
