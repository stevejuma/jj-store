import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { checkPackages } from "./package-check.mjs";

function workspace(dependencies) {
  const root = mkdtempSync(join(tmpdir(), "jj-store-package-check-"));
  const write = (rel, body) => {
    const path = join(root, rel);
    mkdirSync(join(path, ".."), { recursive: true });
    writeFileSync(path, body);
  };
  write(
    "Cargo.toml",
    '[workspace]\nmembers = ["crates/demo"]\nresolver = "2"\n\n[workspace.package]\nversion = "0.1.0"\n',
  );
  for (const rel of ["crates/demo", "runtime/embedded-rustfs"]) {
    write(
      `${rel}/Cargo.toml`,
      `[package]\nname = "${rel.split("/").at(-1)}"\nversion = "0.1.0"\nedition = "2024"\n${dependencies(rel)}`,
    );
    for (const name of ["README.md", "LICENSE.md", "NOTICE.md"])
      write(`${rel}/${name}`, `${name}\n`);
    write(`${rel}/src/lib.rs`, "");
  }
  return root;
}

test("package check admits only the published Lapis git tags", () => {
  const admitted = workspace((rel) =>
    rel.endsWith("demo")
      ? '\n[dependencies]\nlapis-ai-runtime = { git = "https://github.com/lapismd/lapis-ai", tag = "v0.2.0", version = "0.2.0" }\n'
      : "",
  );
  assert.doesNotThrow(() => checkPackages(admitted, "pipe"));
  const foreign = workspace((rel) =>
    rel.endsWith("demo")
      ? '\n[dependencies]\nevil = { git = "https://github.com/example/evil", tag = "v1.0.0", version = "1.0.0" }\n'
      : "",
  );
  const branch = workspace((rel) =>
    rel.endsWith("demo")
      ? '\n[dependencies]\nlapis-ai-runtime = { git = "https://github.com/lapismd/lapis-ai", branch = "main", version = "0.2.0" }\n'
      : "",
  );
  for (const root of [foreign, branch])
    assert.throws(
      () => checkPackages(root, "pipe"),
      (error) =>
        /crates\/demo has a Git dependency/.test(String(error.stderr ?? "")),
    );
});
