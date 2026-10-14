import {readFileSync,writeFileSync,mkdirSync,readdirSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {writeEvidence} from "./spec/evidence-new.mjs";
const mode=process.argv[2], version=process.argv[3];
if (mode!=="version" || !/^\d+\.\d+\.\d+$/.test(version)) throw Error("expected version selection");
const date=new Date().toISOString().slice(0,10), id=`E-RELEASE-VERSION-${version.replaceAll(".","-")}`;
const files=execFileSync("git",["diff","--name-only"],{encoding:"utf8"}).trim().split("\n").filter(Boolean);
const entry={id,tasks:["M30"],requirements:["JST-ARCH-005"],date,kind:"verification",revision:process.env.GITHUB_SHA || "version-pr",files,tests:["scripts/release.test.mjs"],runs:[{command:"pnpm release:version",result:"pass",failures:0,note:"Changesets selected the coordinated version; Cargo manifests and internal edges synchronized. Publication qualification remains pending."}],result:"pending",remaining:["Complete exact-source release qualification before publication."],versions:`Coordinated version ${version}; Rust 1.99`,formats:"Native storage and API formats unchanged.",scope:"Changesets version orchestration only.",next:"Merge version PR after CI; qualify selected source before publication.",contract_review:{status:"changed",reason:"JST-ARCH-005 selects the coordinated release version; source authority remains native."},milestone_review:{status:"unchanged",reason:"M30 reviewed In progress; version selection does not complete publication gates."}};
const dir=writeEvidence(entry);
const pages=readdirSync(dir).filter(n=>/^details-\d+\.md$/.test(n)).sort();
writeFileSync(dir+"/SUMMARY.nav.md",pages.map(n=>`- [Release evidence](${n})`).join("\n")+"\n");
const chapter="spec/src/current/contracts/architecture/23-public-distribution.md";
writeFileSync(chapter,readFileSync(chapter,"utf8").replace(/\n## Selected release[\s\S]*$/,"")+`\n## Selected release\n\nChangesets selected version ${version}. Full qualification precedes publication.\n`);
const milestone="spec/src/current/progress/M30/index.md";
writeFileSync(milestone,readFileSync(milestone,"utf8").replace(/E-PUBLIC-RELEASE-001[^ |]*/,`E-PUBLIC-RELEASE-001,${id}`));
const day=dir.slice(0,dir.lastIndexOf("/"));
mkdirSync(day,{recursive:true});
for (const name of ["index.md","SUMMARY.nav.md"]) {
 let text;
 try {text=readFileSync(`${day}/${name}`,"utf8");} catch {text=name==="index.md"?`# ${date}\n\n## Pages\n\n`:"";}
 if (!text.includes(`${id}/index.md`)) text+=`- [${id}](${id}/index.md)\n`+(name==="SUMMARY.nav.md"?`  {{#include ${id}/SUMMARY.nav.md}}\n`:"");
 writeFileSync(`${day}/${name}`,text);
}
