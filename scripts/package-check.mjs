import { execFileSync } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { metadata, version } from "./release-metadata.mjs";

const script = `
import tomllib,re,sys
from pathlib import Path
root=Path(sys.argv[1]).resolve();w=tomllib.loads((root/'Cargo.toml').read_text())
pins={
 ('lapis-agent-redaction','https://github.com/lapismd/agent-redaction','lapis-agent-redaction@0.1.0','0.1.0'),
 ('lapis-ai-capture','https://github.com/lapismd/lapis-ai','v0.2.0','0.2.0'),
 ('lapis-ai-runtime','https://github.com/lapismd/lapis-ai','v0.2.0','0.2.0'),
 ('lapis-ai-config','https://github.com/lapismd/lapis-ai','v0.2.0','0.2.0'),
}
for rel in w['workspace']['members']+['runtime/embedded-rustfs']:
 base=root/rel;d=tomllib.loads((base/'Cargo.toml').read_text());p=d['package']
 git_only={'jj-history-service','jj-vault','tauri-plugin-jj-store','jj-store-embedded-rustfs'}
 if p['name'] in git_only: assert p.get('publish') is False,rel+' must stay git-only'
 else: assert p.get('publish') is not False,rel+' remains private'
 assert p['name']!='jj-store' or p.get('publish') is not False
 for name in ['README.md','LICENSE.md','NOTICE.md']:assert (base/name).is_file(),rel+' missing '+name
 for section in ['dependencies','dev-dependencies','build-dependencies']:
  for name,v in d.get(section,{}).items():
   if isinstance(v,dict) and v.get('workspace'):v=w['workspace']['dependencies'].get(name,{})
   if isinstance(v,dict):
    if 'git' in v:
     assert 'path' not in v and 'branch' not in v and 'rev' not in v and (name,v.get('git'),v.get('tag'),str(v.get('version'))) in pins,rel+' has a Git dependency'
    assert 'path' not in v or 'version' in v,rel+' has an unversioned local dependency'
 for source in (base/'src').rglob('*.rs'):
  for value in re.findall(r'include_(?:str|bytes)!\\(\\s*"([^"]+)"',source.read_text()):
   path=(source.parent/value).resolve()
   assert path.is_relative_to(base),str(source)+' has an external compilation input '+value
   assert path.is_file(),str(source)+' missing '+value
print(f'All {len(w["workspace"]["members"])+1} Cargo packages have admitted dependencies and contained compilation inputs.')
`;

export function checkPackages(root, stdio = "inherit") {
  execFileSync("python3", ["-c", script, root], { stdio });
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  version();
  checkPackages(process.cwd());
  console.log(`Coordinated package metadata: ${metadata().version}`);
}
