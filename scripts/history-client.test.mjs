import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import {
  cpSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  realpathSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
const root = resolve(import.meta.dirname, "..");
const source = join(root, "packages/jj-history");
const run = (command, args, cwd = root) =>
  execFileSync(command, args, {
    cwd,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });

test(
  "history runtime and public type regressions run in normal governance",
  { timeout: 60000 },
  () => {
    run("pnpm", ["--dir", source, "test"]);
  },
);

test("history package exports built independent entries and matches its native schema", () => {
  run(process.execPath, ["scripts/history-contracts.mjs", "--check"]);
  const pkg = JSON.parse(readFileSync(join(source, "package.json")));
  assert.equal(pkg.name, "@lapismd/jj-history");
  assert.equal(pkg.private, undefined);
  assert.equal(pkg.exports["."].import, "./dist/index.js");
  assert.equal(pkg.exports["./tauri"].types, "./dist/tauri.d.ts");
  assert.deepEqual(pkg.dependencies, { "lossless-json": "^4.3.1" });
  for (const field of ["peerDependencies", "optionalDependencies"])
    assert.deepEqual(pkg[field] ?? {}, {});
  assert.equal(pkg.scripts.postinstall, undefined);
  assert.equal(
    JSON.parse(readFileSync(join(source, "tsconfig.json"))).compilerOptions
      .paths,
    undefined,
  );
});

test(
  "independent history archive preserves exact integers, public types and Tauri text invocation",
  { timeout: 60000 },
  () => {
    const temporary = realpathSync(
      mkdtempSync(join(tmpdir(), "jj-history-consumer-")),
    );
    try {
      run("pnpm", ["--dir", source, "pack", "--pack-destination", temporary]);
      const archives = readdirSync(temporary).filter((name) =>
        name.endsWith(".tgz"),
      );
      assert.equal(archives.length, 1);
      const archive = join(temporary, archives[0]);
      const files = run("tar", ["-tf", archive]);
      for (const file of [
        "dist/index.js",
        "dist/index.d.ts",
        "dist/tauri.js",
        "dist/tauri.d.ts",
        "dist/schema.json",
        "README.md",
        "LICENSE.md",
        "NOTICE.md",
      ])
        assert.ok(files.includes(`package/${file}`), file);
      assert.ok(
        !files.includes("package/src/") && !files.includes("node_modules/"),
      );
      writeFileSync(
        join(temporary, "package.json"),
        JSON.stringify({
          name: "independent-history-consumer",
          private: true,
          type: "module",
          packageManager: "pnpm@10.32.1",
          dependencies: { "@lapismd/jj-history": `file:${archive}` },
          pnpm: { onlyBuiltDependencies: [] },
        }),
      );
      run(
        "pnpm",
        ["install", "--offline", "--ignore-scripts", "--ignore-pnpmfile"],
        temporary,
      );
      writeFileSync(
        join(temporary, "acceptance.mjs"),
        `import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { HistoryClient, HistoryProtocolError } from '@lapismd/jj-history';
import { tauriHistory } from '@lapismd/jj-history/tauri';
assert.ok(fileURLToPath(import.meta.resolve('@lapismd/jj-history')).startsWith(${JSON.stringify(temporary + "/")}));
const operation='ab'.repeat(64), repository='cd'.repeat(16);
let calls=[];
const client=tauriHistory('scoped',async(command,args)=>{calls.push([command,args]);return '{"version":1,"data":{"version":1,"profile":"local","repository":"'+repository+'","generation":18446744073709551615,"heads":["'+operation+'"]}}';});
assert.equal((await client.heads()).generation,18446744073709551615n);
assert.equal(calls[0][0],'plugin:jj-store|heads');
assert.deepEqual(Object.keys(calls[0][1]),['request']);
assert.equal(JSON.parse(calls[0][1].request).data.vault,'scoped');
await assert.rejects(client.open('bad'),HistoryProtocolError);
assert.equal(calls.length,1);
let attempts=0;const uncertain=new HistoryClient('scoped',{invoke:async()=>{attempts++;throw Error('unknown');}});
await assert.rejects(uncertain.open(operation));assert.equal(attempts,1);
`,
      );
      run(process.execPath, ["acceptance.mjs"], temporary);
      writeFileSync(
        join(temporary, "acceptance.ts"),
        `import { HistoryClient, type LocalGraphTimestamp } from '@lapismd/jj-history';
import { tauriHistory } from '@lapismd/jj-history/tauri';
const timestamp:LocalGraphTimestamp={millis:-9223372036854775808n,offset_minutes:0};
const invoke=async <T>(command:string,args?:Record<string,unknown>):Promise<T>=>{throw Error('fixture');};
const client:HistoryClient=tauriHistory('vault',invoke);
void timestamp;void client.read('h',{kind:'graph',query:[{graph:'operations'},{limit:1}]});
// @ts-expect-error Pins retain full width, not arbitrary display numbers.
const rounded:number=timestamp.millis;
// @ts-expect-error There is no renderer source-path authority.
void client.read('h',{kind:'query_info',root:'/outside'});
`,
      );
      writeFileSync(
        join(temporary, "tsconfig.json"),
        JSON.stringify({
          compilerOptions: {
            target: "ES2022",
            module: "NodeNext",
            moduleResolution: "NodeNext",
            strict: true,
            noEmit: true,
            types: [],
          },
          include: ["acceptance.ts"],
        }),
      );
      run(
        process.execPath,
        [join(root, "node_modules/typescript/bin/tsc"), "-p", "tsconfig.json"],
        temporary,
      );
      console.log(
        `independent history archive sha256=${createHash("sha256").update(readFileSync(archive)).digest("hex")}`,
      );
    } finally {
      rmSync(temporary, { recursive: true, force: true });
    }
  },
);

test(
  "a fresh source workspace builds independent history entries on ordinary install",
  { timeout: 60000 },
  () => {
    const temporary = realpathSync(
      mkdtempSync(join(tmpdir(), "jj-history-workspace-")),
    );
    try {
      const target = join(temporary, "packages/jj-history");
      mkdirSync(target, { recursive: true });
      for (const file of ["package.json", "tsconfig.json", "src"])
        cpSync(join(source, file), join(target, file), { recursive: true });
      writeFileSync(
        join(temporary, "package.json"),
        JSON.stringify({
          name: "history-workspace-fixture",
          private: true,
          packageManager: "pnpm@10.32.1",
          dependencies: { "@lapismd/jj-history": "^0.1.0" },
          pnpm: { onlyBuiltDependencies: [] },
        }),
      );
      writeFileSync(
        join(temporary, "pnpm-workspace.yaml"),
        "packages:\n  - .\n  - packages/jj-history\nlinkWorkspacePackages: true\n",
      );
      assert.equal(existsSync(join(target, "dist/index.js")), false);
      run("pnpm", ["install", "--offline", "--ignore-pnpmfile"], temporary);
      for (const name of [
        "index.js",
        "index.d.ts",
        "tauri.js",
        "tauri.d.ts",
        "schema.json",
      ])
        assert.ok(existsSync(join(target, "dist", name)), name);
    } finally {
      rmSync(temporary, { recursive: true, force: true });
    }
  },
);
