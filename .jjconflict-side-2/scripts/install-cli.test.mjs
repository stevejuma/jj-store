import assert from "node:assert/strict";
import {
  mkdtemp,
  mkdir,
  readFile,
  writeFile,
  rm,
  lstat,
  symlink,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { installCli } from "./install-cli.mjs";
test("CLI installer copies both tools without replacing jj or following target symlinks", async (t) => {
  const root = await mkdtemp(join(tmpdir(), "jj-store-install-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  const source = join(root, "source"),
    destination = join(root, "bin");
  await mkdir(source);
  await mkdir(destination);
  await writeFile(join(source, "jj-store"), "server");
  await writeFile(join(source, "jj"), "custom jj");
  await writeFile(join(destination, "jj"), "existing jj");
  await writeFile(join(root, "symlink-target"), "untouched");
  await symlink(join(root, "symlink-target"), join(destination, "jj-store"));
  await installCli({ source, destination });
  assert.equal(
    await readFile(join(root, "symlink-target"), "utf8"),
    "untouched",
  );
  assert.equal(await readFile(join(destination, "jj"), "utf8"), "existing jj");
  assert.equal(await readFile(join(destination, "jj-store"), "utf8"), "server");
  assert.equal(
    await readFile(join(destination, "jj-store-jj"), "utf8"),
    "custom jj",
  );
  assert.equal(
    (await lstat(join(destination, "jj-store"))).mode & 0o777,
    0o755,
  );
  await installCli({ source, destination });
  assert.equal(await readFile(join(destination, "jj"), "utf8"), "existing jj");
});
test("CLI installer preflights missing artifacts before replacing installed tools", async (t) => {
  const root = await mkdtemp(join(tmpdir(), "jj-store-install-missing-"));
  t.after(() => rm(root, { recursive: true, force: true }));
  await writeFile(join(root, "jj-store"), "existing server");
  await assert.rejects(
    installCli({ source: join(root, "missing"), destination: root }),
  );
  assert.equal(
    await readFile(join(root, "jj-store"), "utf8"),
    "existing server",
  );
});
