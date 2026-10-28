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
const source = join(root, "packages/jj-conversations");
const run = (command, args, cwd = root) => {
  try {
    return execFileSync(command, args, {
      cwd,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    });
  } catch (error) {
    const detail = [error.stdout, error.stderr]
      .filter(Boolean)
      .join("\n")
      .trim();
    if (detail) error.message = `${error.message}\n${detail}`;
    throw error;
  }
};

test("conversation package exports only its built independent client boundary", () => {
  const pkg = JSON.parse(readFileSync(join(source, "package.json")));
  assert.equal(pkg.name, "@lapismd/jj-conversations");
  assert.equal(pkg.private, undefined);
  assert.equal(pkg.exports["."].import, "./dist/index.js");
  assert.equal(pkg.exports["."].types, "./dist/index.d.ts");
  for (const field of [
    "dependencies",
    "peerDependencies",
    "optionalDependencies",
  ])
    assert.deepEqual(pkg[field] ?? {}, {});
  assert.equal(pkg.scripts.postinstall, undefined);
  assert.ok(
    pkg.files.includes("README.md") &&
      pkg.files.includes("LICENSE.md") &&
      pkg.files.includes("NOTICE.md"),
  );
  const config = JSON.parse(readFileSync(join(source, "tsconfig.json")));
  assert.equal(config.compilerOptions.paths, undefined);
  assert.equal(config.compilerOptions.declaration, true);
});

test(
  "independent consumer imports and typechecks the actual npm archive without source aliases",
  { timeout: 60000 },
  () => {
    const temporary = realpathSync(
      mkdtempSync(join(tmpdir(), "jj-conversation-consumer-")),
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
        "LICENSE.md",
        "NOTICE.md",
        "README.md",
      ])
        assert.ok(files.includes(`package/${file}`), file);
      assert.ok(
        !files.includes("package/src/") && !files.includes("node_modules/"),
      );
      writeFileSync(
        join(temporary, "package.json"),
        JSON.stringify({
          name: "independent-conversation-consumer",
          private: true,
          type: "module",
          packageManager: "pnpm@10.32.1",
          dependencies: { "@lapismd/jj-conversations": `file:${archive}` },
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
import { ConversationClient, agentMentionToken, mentionedAgentIds } from '@lapismd/jj-conversations';
const resolved = fileURLToPath(import.meta.resolve('@lapismd/jj-conversations'));
assert.ok(resolved.startsWith(${JSON.stringify(temporary + "/")}));
assert.deepEqual(mentionedAgentIds(agentMentionToken('Exact 🦉')), ['Exact 🦉']);
let posts=0;
const client = new ConversationClient('vault', {submit: async () => {posts++;throw Error('unknown');},lookup: async id => ({request_id:id, delivery:'unknown', capture:'pending', sequence:9007199254740995n})}, JSON.stringify);
await assert.rejects(client.send({request_id:'request',text:'Hello'}));
await assert.rejects(client.send({request_id:'request',text:'Hello'}));
assert.equal(posts,1);
assert.equal((await client.refresh('request')).sequence,9007199254740995n);
`,
      );
      run(process.execPath, ["acceptance.mjs"], temporary);
      writeFileSync(
        join(temporary, "acceptance.ts"),
        `import { promptDeliveryObservations } from '@lapismd/jj-conversations';
const sequence: bigint = promptDeliveryObservations([{sequence:9007199254740995n,payload:{type:'started',requestId:'r'}}]).get('r')!.sequence;
void sequence;
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
        `independent conversation archive sha256=${createHash("sha256").update(readFileSync(archive)).digest("hex")}`,
      );
    } finally {
      rmSync(temporary, { recursive: true, force: true });
    }
  },
);

test(
  "a fresh source workspace builds public client exports during ordinary install",
  { timeout: 60000 },
  () => {
    const temporary = realpathSync(
      mkdtempSync(join(tmpdir(), "jj-conversation-workspace-")),
    );
    try {
      const target = join(temporary, "packages/jj-conversations");
      mkdirSync(target, { recursive: true });
      for (const file of ["package.json", "tsconfig.json", "src"])
        cpSync(join(source, file), join(target, file), { recursive: true });
      writeFileSync(
        join(temporary, "package.json"),
        JSON.stringify({
          name: "source-workspace-fixture",
          private: true,
          packageManager: "pnpm@10.32.1",
          dependencies: { "@lapismd/jj-conversations": "^0.1.0" },
          pnpm: { onlyBuiltDependencies: [] },
        }),
      );
      writeFileSync(
        join(temporary, "pnpm-workspace.yaml"),
        "packages:\n  - .\n  - packages/jj-conversations\nlinkWorkspacePackages: true\n",
      );
      assert.equal(existsSync(join(target, "dist/index.js")), false);
      run("pnpm", ["install", "--offline", "--ignore-pnpmfile"], temporary);
      assert.equal(existsSync(join(target, "dist/index.js")), true);
      assert.equal(existsSync(join(target, "dist/index.d.ts")), true);
    } finally {
      rmSync(temporary, { recursive: true, force: true });
    }
  },
);
