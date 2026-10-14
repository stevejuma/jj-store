import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";

export function metadata(root = ".") {
  return JSON.parse(execFileSync("python3", ["-c", `
import tomllib,json,sys
from pathlib import Path
r=Path(sys.argv[1]); w=tomllib.loads((r/'Cargo.toml').read_text())
items=[]
for rel in w['workspace']['members']+['runtime/embedded-rustfs']:
 d=tomllib.loads((r/rel/'Cargo.toml').read_text()); p=d['package']
 deps=[]
 for name,value in d.get('dependencies',{}).items():
  if isinstance(value,dict) and value.get('workspace'):
   v=w['workspace']['dependencies'].get(name,{})
   if isinstance(v,dict) and 'path' in v: deps.append(name)
 items.append(dict(name=p['name'],path=rel,version=w['workspace']['package']['version'] if isinstance(p['version'],dict) else p['version'],dependencies=deps))
print(json.dumps(dict(version=w['workspace']['package']['version'],packages=items)))
`, root], { encoding: "utf8" }));
}
export function orderedPackages(data) {
  const pending = new Map(data.packages.map(p => [p.name, p]));
  const result = [];
  while (pending.size) {
    const ready = [...pending.values()].filter(p => p.dependencies.every(d => !pending.has(d))).sort((a,b) => a.name.localeCompare(b.name));
    if (!ready.length) throw Error("publication dependency cycle");
    for (const p of ready) { result.push(p); pending.delete(p.name); }
  }
  return result;
}
export function validateVersion(data, packageVersion) {
  if (!/^\d+\.\d+\.\d+$/.test(data.version) || data.version !== packageVersion || data.packages.some(p => p.version !== data.version))
    throw Error("Changesets, Cargo and helper versions must agree");
}
export function version(root = ".") {
  const data = metadata(root);
  validateVersion(data, JSON.parse(readFileSync(`${root}/package.json`, "utf8")).version);
  return data.version;
}
export function validateTag(tag, selectedVersion) {
  if (tag !== `v${selectedVersion}`) throw Error("tag does not match release version");
}
