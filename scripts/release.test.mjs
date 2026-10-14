import test from "node:test";
import assert from "node:assert/strict";
import {orderedPackages, validateVersion, validateTag, metadata} from "./release-metadata.mjs";
test("publication order resolves internal edges and rejects cycles", () => {
  const packages=[{name:"server",dependencies:["model"]},{name:"model",dependencies:[]}];
  assert.deepEqual(orderedPackages({packages}).map(p=>p.name),["model","server"]);
  assert.throws(()=>orderedPackages({packages:[{name:"a",dependencies:["b"]},{name:"b",dependencies:["a"]}]}),/cycle/);
});
test("release rejects divergent helper, Changesets and tag identities", () => {
  const data={version:"0.1.0",packages:[{version:"0.1.1"}]};
  assert.throws(()=>validateVersion(data,"0.1.0"),/agree/);
  assert.throws(()=>validateTag("v0.2.0","0.1.0"),/match/);
  validateTag("v0.1.0","0.1.0");
});
test("all public package versions have an acyclic coordinated graph", () => {
  const data=metadata();
  assert.equal(data.packages.length,20);
  assert.equal(orderedPackages(data).at(-1).name,"jj-store");
  validateVersion(data,data.version);
});
