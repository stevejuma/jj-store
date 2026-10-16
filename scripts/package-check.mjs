import { execFileSync } from "node:child_process";
import { metadata, version } from "./release-metadata.mjs";
version();
execFileSync(
  "python3",
  [
    "-c",
    `
import tomllib,re,sys
from pathlib import Path
root=Path('.').resolve();w=tomllib.loads((root/'Cargo.toml').read_text())
for rel in w['workspace']['members']+['runtime/embedded-rustfs']:
 base=root/rel;d=tomllib.loads((base/'Cargo.toml').read_text());p=d['package']
 assert p.get('publish') is not False,rel+' remains private'
 for name in ['README.md','LICENSE.md','NOTICE.md']:assert (base/name).is_file(),rel+' missing '+name
 for section in ['dependencies','dev-dependencies','build-dependencies']:
  for name,v in d.get(section,{}).items():
   if isinstance(v,dict) and v.get('workspace'):v=w['workspace']['dependencies'].get(name,{})
   if isinstance(v,dict):
    assert 'git' not in v,rel+' has a Git dependency'
    assert 'path' not in v or 'version' in v,rel+' has an unversioned local dependency'
 for source in (base/'src').rglob('*.rs'):
  for value in re.findall(r'include_(?:str|bytes)!\\(\\s*"([^"]+)"',source.read_text()):
   path=(source.parent/value).resolve()
   assert path.is_relative_to(base),str(source)+' has an external compilation input '+value
   assert path.is_file(),str(source)+' missing '+value
print(f'All {len(w["workspace"]["members"])+1} Cargo packages have registry dependencies and contained compilation inputs.')
`,
  ],
  { stdio: "inherit" },
);
console.log(`Coordinated package metadata: ${metadata().version}`);
