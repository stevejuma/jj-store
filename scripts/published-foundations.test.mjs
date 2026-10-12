import test from "node:test";
import assert from "node:assert/strict";
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { parse } from "yaml";

test("released_foundations_remain_registry_owned_without_active_overrides", () => {
  const manifest = JSON.parse(readFileSync("package.json", "utf8"));
  const lock = parse(readFileSync("pnpm-lock.yaml", "utf8"));
  assert.equal(existsSync(".pnpmfile.cjs"), false);
  assert.equal(manifest.pnpm.overrides, undefined);
  for (const name of [
    "@lapismd/design-core",
    "@lapismd/mira",
    "@lapismd/spec-validator",
  ]) {
    assert.match(manifest.devDependencies[name], /^[~^]?\d+\.\d+\.\d+$/);
    const dependency = lock.importers["."].devDependencies[name];
    assert.equal(dependency.specifier, manifest.devDependencies[name]);
    assert.doesNotMatch(dependency.version, /(?:file:|link:|workspace:)/);
    const installed = JSON.parse(
      readFileSync(`node_modules/${name}/package.json`, "utf8"),
    );
    assert.equal(installed.name, name);
    assert.ok(dependency.version.startsWith(installed.version));
    assert.ok(
      realpathSync(`node_modules/${name}`).startsWith(
        realpathSync("node_modules/.pnpm") + "/",
      ),
    );
    const artifact = lock.packages[`${name}@${installed.version}`];
    assert.ok(artifact.resolution.integrity);
    assert.equal(artifact.resolution.tarball, undefined);
  }
});
