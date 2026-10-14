import test from "node:test";
import assert from "node:assert/strict";
import {mkdtempSync,writeFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {execFileSync} from "node:child_process";
import {revisionReader} from "./spec/revision-reader.mjs";
import {impact} from "./ci-plan.mjs";
test("docs-only impact stays light; shared and service changes select heavy lanes",()=>{
  assert.deepEqual(impact(["spec/src/current/index.md"]),{rust:false,web:false,heavy:false,full:false});
  assert.equal(impact(["crates/jj-store-service/src/lib.rs"]).heavy,true);
  assert.equal(impact(["Cargo.lock"]).full,true);
});
test("Git revision reader pins content and all merge parents",()=>{
  const root=mkdtempSync(join(tmpdir(),"jj-store-ci-git-"));
  const git=(...args)=>execFileSync("git",args,{cwd:root,encoding:"utf8"}).trim();
  try {
    git("init","-q");git("config","user.name","Fixture");git("config","user.email","fixture@example.invalid");
    writeFileSync(join(root,"record.md"),"original\n");git("add","record.md");git("commit","-qm","baseline");
    const first=git("rev-parse","HEAD");
    writeFileSync(join(root,"record.md"),"selected\n");git("commit","-qam","selected");
    const reader=revisionReader(root,first);
    writeFileSync(join(root,"record.md"),"uncommitted\n");
    assert.equal(reader.read("record.md"),"original\n");reader.verify();
    const empty=git("hash-object","-t","tree","--stdin");
    assert.deepEqual(revisionReader(root,empty).files,[]);
  } finally {rmSync(root,{recursive:true,force:true});}
});
